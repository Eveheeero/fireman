use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstParameter, AstVariableAccessType, AstVariableId,
        GetRelatedVariables, ProcessedOptimization,
    },
    ir::{
        Register,
        data::{IrData, IrDataOperation},
        operator::IrUnaryOperator,
    },
    prelude::*,
};
use hashbrown::{HashMap, HashSet};

pub(super) fn analyze_parameters(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let variables;
    let body;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();

        body = std::mem::take(&mut function.body);
        variables = function.variables.clone();
    }
    let mut written_registers: HashSet<Register> = HashSet::new();
    let mut read_before_write_registers: HashSet<Register> = HashSet::new();
    let mut used_offset_from_base_pointers: HashSet<isize> = HashSet::new();
    // Maps to link used locations back to existing AST variables
    let mut reg_to_var: HashMap<Register, AstVariableId> = HashMap::new();
    let mut offset_to_var: HashMap<isize, AstVariableId> = HashMap::new();

    let first_arg_undetectable_statement_index =
        super::utils::get_first_arg_undetectable_statement_index(body.iter());
    for (i, stmt) in body.iter().enumerate() {
        let stmt = &stmt.statement;
        let related_vars = stmt.get_related_variables();

        /* analyze registers before undetectable statements */
        if i < first_arg_undetectable_statement_index.unwrap_or(usize::MAX) {
            // Map registers to variables when detectable early
            for (access_type, var_id) in related_vars.iter().copied() {
                let location = super::utils::var_id_to_access_location(&variables, var_id);
                if let Some(loc) = location {
                    if let IrData::Register(register) = loc.as_ref() {
                        reg_to_var.entry(register.clone()).or_insert(var_id);
                    }
                }
            }

            let related_registers = related_vars.iter().filter_map(|x| {
                let access_type = x.0;
                let location = super::utils::var_id_to_access_location(&variables, x.1);
                let location = location.and_then(|x| {
                    if let IrData::Register(register) = x.as_ref() {
                        Some(register.clone())
                    } else {
                        None
                    }
                });
                if let Some(location) = location {
                    Some((access_type, location))
                } else {
                    None
                }
            });
            for (access_type, register) in related_registers {
                match access_type {
                    AstVariableAccessType::Read => {
                        if written_registers.contains(&register) {
                            read_before_write_registers.insert(register);
                        }
                    }
                    AstVariableAccessType::Write => {
                        written_registers.insert(register);
                    }
                }
            }

            /* TODO what if there is register used after undetectable statement? */
        }

        /*
        analyze stack related accesses
        if write with base pointer, it might return with reference
        if read with base pointer, it might arg passing
         */
        'a: {
            let is_bp_related = related_vars.iter().any(|x| {
                super::utils::var_id_to_access_location(&variables, x.1)
                    .and_then(|x| x.get_offset_from_base_pointer())
                    .is_some()
            });
            if !is_bp_related {
                break 'a;
            }
            // Build mapping from positive stack offsets to variables
            for (_, var_id) in related_vars.iter().copied() {
                let maybe_offset = super::utils::var_id_to_access_location(&variables, var_id)
                    .and_then(|x| x.get_offset_from_base_pointer());
                if let Some(mut offset) = maybe_offset {
                    let mut neg = false;
                    if let IrData::Operation(IrDataOperation::Unary {
                        operator: IrUnaryOperator::Negation,
                        arg,
                    }) = offset.as_ref()
                    {
                        neg = true;
                        offset = arg.clone();
                    }
                    if let Some(v) = offset.constant() {
                        let signed = if !neg { v as isize } else { 0 - v as isize };
                        if signed > 0 {
                            offset_to_var.entry(signed).or_insert(var_id);
                        }
                    }
                }
            }

            let offset_from_bp = related_vars.iter().filter_map(|x| {
                super::utils::var_id_to_access_location(&variables, x.1)
                    .and_then(|x| x.get_offset_from_base_pointer())
            });
            for mut offset in offset_from_bp {
                let mut neg = false;
                if let IrData::Operation(IrDataOperation::Unary {
                    operator: IrUnaryOperator::Negation,
                    arg,
                }) = offset.as_ref()
                {
                    neg = true;
                    offset = arg.clone();
                }
                let Some(offset) = offset.constant() else {
                    continue;
                };
                if !neg {
                    used_offset_from_base_pointers.insert(offset as isize);
                } else {
                    used_offset_from_base_pointers.insert(0 - offset as isize);
                }
            }
        }
    }

    let parameters = used_locations_to_parameters(
        written_registers,
        used_offset_from_base_pointers,
        &reg_to_var,
        &offset_to_var,
    );

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function
            .processed_optimizations
            .push(ProcessedOptimization::ParameterAnalyzation);
        function.parameters = parameters;
        function.body = body;
    }
    Ok(())
}

enum CallingConvention {
    X86Cdecl,
    X86Stdcall,
    X86Fastcall,
    X86Thiscall,
    X86Vectorcall,
    X64,
    Unknown,
}

/// ordering parameters
/// x86 cdecl - arg passed with stack, sp cleaned by caller
/// x86 stdcall - arg passed with stack, sp cleaned by callee
/// x86 fastcall - ecx, edx, stack..., sp cleaned by callee
/// x86 thiscall - ecx, edx, stack..., sp cleaned by callee (?)
/// x86 vectorcall - xmm0, xmm1, xmm2, xmm3, stack..., sp cleaned by callee (?)
/// x64 - rcx(xmm0), rdx(xmm1), r8(xmm2), r9(xmm3), stack..., sp cleaned by callee
fn used_locations_to_parameters(
    used_registers: HashSet<Register>,
    used_offset_from_base_pointers: HashSet<isize>,
    reg_to_var: &HashMap<Register, AstVariableId>,
    offset_to_var: &HashMap<isize, AstVariableId>,
) -> Vec<AstParameter> {
    let calling_convention =
        detecting_calling_convention(&used_registers, &used_offset_from_base_pointers);

    match calling_convention {
        CallingConvention::X64 => parameter_ordering::order_params_x64(
            &used_registers,
            &used_offset_from_base_pointers,
            reg_to_var,
            offset_to_var,
        ),
        CallingConvention::X86Cdecl => parameter_ordering::order_params_x86_cdecl(
            &used_registers,
            &used_offset_from_base_pointers,
            reg_to_var,
            offset_to_var,
        ),
        CallingConvention::X86Stdcall => parameter_ordering::order_params_x86_stdcall(
            &used_registers,
            &used_offset_from_base_pointers,
            reg_to_var,
            offset_to_var,
        ),
        CallingConvention::X86Fastcall => parameter_ordering::order_params_x86_fastcall(
            &used_registers,
            &used_offset_from_base_pointers,
            reg_to_var,
            offset_to_var,
        ),
        CallingConvention::X86Thiscall => parameter_ordering::order_params_x86_thiscall(
            &used_registers,
            &used_offset_from_base_pointers,
            reg_to_var,
            offset_to_var,
        ),
        CallingConvention::X86Vectorcall => parameter_ordering::order_params_x86_vectorcall(
            &used_registers,
            &used_offset_from_base_pointers,
            reg_to_var,
            offset_to_var,
        ),
        CallingConvention::Unknown => parameter_ordering::order_params_unknown(
            &used_registers,
            &used_offset_from_base_pointers,
            reg_to_var,
            offset_to_var,
        ),
    }
}

fn detecting_calling_convention(
    _used_registers: &HashSet<Register>,
    _used_offset_from_base_pointers: &HashSet<isize>,
) -> CallingConvention {
    // TODO need detecting calling convention with metadata
    warn!("detecting calling convention is unimplemented now. defaulting to unknown");
    CallingConvention::Unknown
}

mod parameter_ordering {
    use crate::{
        abstract_syntax_tree::{AstParameter, AstParameterLocation, AstVariableId},
        ir::{Register, VirtualMachine, x86_64::X64Range},
    };
    use hashbrown::{HashMap, HashSet};
    use either::Either;

    // Common helper functions shared by parameter ordering strategies
    fn push_reg_param(
        params: &mut Vec<AstParameter>,
        added_regs: &mut HashSet<Register>,
        reg: Register,
        reg_to_var: &HashMap<Register, AstVariableId>,
    ) {
        let related = reg_to_var.get(&reg).copied();
        let related_either = match related {
            Some(id) => Either::Left(id),
            None => Either::Right(format!("p_{}", reg.name())),
        };
        params.push(AstParameter {
            location: AstParameterLocation::Register((&reg).into()),
            related_var_or_temp_name: related_either,
        });
        added_regs.insert(reg);
    }

    fn push_stack_param(
        params: &mut Vec<AstParameter>,
        offset: isize,
        offset_to_var: &HashMap<isize, AstVariableId>,
    ) {
        let related = offset_to_var.get(&offset).copied();
        let related_either = match related {
            Some(id) => Either::Left(id),
            None => Either::Right(format!("p_{}", offset)),
        };
        params.push(AstParameter {
            location: AstParameterLocation::Stack(offset),
            related_var_or_temp_name: related_either,
        });
    }

    fn add_register_if_used(
        params: &mut Vec<AstParameter>,
        added_regs: &mut HashSet<Register>,
        used_registers: &HashSet<Register>,
        candidate: Register,
        reg_to_var: &HashMap<Register, AstVariableId>,
    ) {
        if used_registers.contains(&candidate) && !added_regs.contains(&candidate) {
            push_reg_param(params, added_regs, candidate, reg_to_var);
        }
    }

    pub(super) fn order_params_x64(
        used_registers: &HashSet<Register>,
        used_offset_from_base_pointers: &HashSet<isize>,
        reg_to_var: &HashMap<Register, AstVariableId>,
        offset_to_var: &HashMap<isize, AstVariableId>,
    ) -> Vec<AstParameter> {
        let mut params: Vec<AstParameter> = Vec::new();
        let mut added_regs: HashSet<Register> = HashSet::new();

        // RCX/XMM0
        let rcx_family = [
            <VirtualMachine as X64Range>::rcx(),
            <VirtualMachine as X64Range>::ecx(),
            <VirtualMachine as X64Range>::cx(),
            <VirtualMachine as X64Range>::cl(),
            <VirtualMachine as X64Range>::ch(),
        ];
        for r in rcx_family {
            add_register_if_used(&mut params, &mut added_regs, used_registers, r, reg_to_var);
        }
        add_register_if_used(
            &mut params,
            &mut added_regs,
            used_registers,
            <VirtualMachine as X64Range>::xmm0(),
            reg_to_var,
        );

        // RDX/XMM1
        let rdx_family = [
            <VirtualMachine as X64Range>::rdx(),
            <VirtualMachine as X64Range>::edx(),
            <VirtualMachine as X64Range>::dx(),
            <VirtualMachine as X64Range>::dl(),
            <VirtualMachine as X64Range>::dh(),
        ];
        for r in rdx_family {
            add_register_if_used(&mut params, &mut added_regs, used_registers, r, reg_to_var);
        }
        add_register_if_used(
            &mut params,
            &mut added_regs,
            used_registers,
            <VirtualMachine as X64Range>::xmm1(),
            reg_to_var,
        );

        // R8/XMM2
        let r8_family = [
            <VirtualMachine as X64Range>::r8(),
            <VirtualMachine as X64Range>::r8d(),
            <VirtualMachine as X64Range>::r8w(),
            <VirtualMachine as X64Range>::r8b(),
        ];
        for r in r8_family {
            add_register_if_used(&mut params, &mut added_regs, used_registers, r, reg_to_var);
        }
        add_register_if_used(
            &mut params,
            &mut added_regs,
            used_registers,
            <VirtualMachine as X64Range>::xmm2(),
            reg_to_var,
        );

        // R9/XMM3
        let r9_family = [
            <VirtualMachine as X64Range>::r9(),
            <VirtualMachine as X64Range>::r9d(),
            <VirtualMachine as X64Range>::r9w(),
            <VirtualMachine as X64Range>::r9b(),
        ];
        for r in r9_family {
            add_register_if_used(&mut params, &mut added_regs, used_registers, r, reg_to_var);
        }
        add_register_if_used(
            &mut params,
            &mut added_regs,
            used_registers,
            <VirtualMachine as X64Range>::xmm3(),
            reg_to_var,
        );

        // Any remaining used registers in deterministic order
        if added_regs.len() < used_registers.len() {
            let mut remaining: Vec<_> = used_registers
                .iter()
                .filter(|reg| !added_regs.contains(*reg))
                .cloned()
                .collect();
            remaining.sort_by_key(|r| r.name());
            for r in remaining {
                push_reg_param(&mut params, &mut added_regs, r, reg_to_var);
            }
        }

        // Stack args: positive offsets ascending
        let mut stack_offsets: Vec<isize> = used_offset_from_base_pointers
            .iter()
            .copied()
            .filter(|o| *o > 0)
            .collect();
        stack_offsets.sort();
        for off in stack_offsets {
            push_stack_param(&mut params, off, offset_to_var);
        }

        params
    }

    pub(super) fn order_params_x86_fastcall(
        used_registers: &HashSet<Register>,
        used_offset_from_base_pointers: &HashSet<isize>,
        reg_to_var: &HashMap<Register, AstVariableId>,
        offset_to_var: &HashMap<isize, AstVariableId>,
    ) -> Vec<AstParameter> {
        let mut params: Vec<AstParameter> = Vec::new();
        let mut added_regs: HashSet<Register> = HashSet::new();

        let ecx_family = [
            <VirtualMachine as X64Range>::rcx(),
            <VirtualMachine as X64Range>::ecx(),
            <VirtualMachine as X64Range>::cx(),
            <VirtualMachine as X64Range>::cl(),
            <VirtualMachine as X64Range>::ch(),
        ];
        for r in ecx_family {
            add_register_if_used(&mut params, &mut added_regs, used_registers, r, reg_to_var);
        }
        let edx_family = [
            <VirtualMachine as X64Range>::rdx(),
            <VirtualMachine as X64Range>::edx(),
            <VirtualMachine as X64Range>::dx(),
            <VirtualMachine as X64Range>::dl(),
            <VirtualMachine as X64Range>::dh(),
        ];
        for r in edx_family {
            add_register_if_used(&mut params, &mut added_regs, used_registers, r, reg_to_var);
        }
        let mut stack_offsets: Vec<isize> = used_offset_from_base_pointers
            .iter()
            .copied()
            .filter(|o| *o > 0)
            .collect();
        stack_offsets.sort();
        for off in stack_offsets {
            push_stack_param(&mut params, off, offset_to_var);
        }

        params
    }

    pub(super) fn order_params_x86_thiscall(
        used_registers: &HashSet<Register>,
        used_offset_from_base_pointers: &HashSet<isize>,
        reg_to_var: &HashMap<Register, AstVariableId>,
        offset_to_var: &HashMap<isize, AstVariableId>,
    ) -> Vec<AstParameter> {
        // For our purposes, treat like fastcall (ECX as this, then EDX), then stack
        order_params_x86_fastcall(
            used_registers,
            used_offset_from_base_pointers,
            reg_to_var,
            offset_to_var,
        )
    }

    pub(super) fn order_params_x86_vectorcall(
        used_registers: &HashSet<Register>,
        used_offset_from_base_pointers: &HashSet<isize>,
        reg_to_var: &HashMap<Register, AstVariableId>,
        offset_to_var: &HashMap<isize, AstVariableId>,
    ) -> Vec<AstParameter> {
        use crate::ir::{VirtualMachine, x86_64::X64Range};
        let mut params: Vec<AstParameter> = Vec::new();
        let mut added_regs: HashSet<Register> = HashSet::new();

        add_register_if_used(
            &mut params,
            &mut added_regs,
            used_registers,
            <VirtualMachine as X64Range>::xmm0(),
            reg_to_var,
        );
        add_register_if_used(
            &mut params,
            &mut added_regs,
            used_registers,
            <VirtualMachine as X64Range>::xmm1(),
            reg_to_var,
        );
        add_register_if_used(
            &mut params,
            &mut added_regs,
            used_registers,
            <VirtualMachine as X64Range>::xmm2(),
            reg_to_var,
        );
        add_register_if_used(
            &mut params,
            &mut added_regs,
            used_registers,
            <VirtualMachine as X64Range>::xmm3(),
            reg_to_var,
        );

        let mut stack_offsets: Vec<isize> = used_offset_from_base_pointers
            .iter()
            .copied()
            .filter(|o| *o > 0)
            .collect();
        stack_offsets.sort();
        for off in stack_offsets {
            push_stack_param(&mut params, off, offset_to_var);
        }

        params
    }

    pub(super) fn order_params_x86_cdecl(
        used_registers: &HashSet<Register>,
        used_offset_from_base_pointers: &HashSet<isize>,
        reg_to_var: &HashMap<Register, AstVariableId>,
        offset_to_var: &HashMap<isize, AstVariableId>,
    ) -> Vec<AstParameter> {
        order_params_stack_only(
            used_registers,
            used_offset_from_base_pointers,
            reg_to_var,
            offset_to_var,
        )
    }

    pub(super) fn order_params_x86_stdcall(
        used_registers: &HashSet<Register>,
        used_offset_from_base_pointers: &HashSet<isize>,
        reg_to_var: &HashMap<Register, AstVariableId>,
        offset_to_var: &HashMap<isize, AstVariableId>,
    ) -> Vec<AstParameter> {
        order_params_stack_only(
            used_registers,
            used_offset_from_base_pointers,
            reg_to_var,
            offset_to_var,
        )
    }

    pub(super) fn order_params_unknown(
        used_registers: &HashSet<Register>,
        used_offset_from_base_pointers: &HashSet<isize>,
        reg_to_var: &HashMap<Register, AstVariableId>,
        offset_to_var: &HashMap<isize, AstVariableId>,
    ) -> Vec<AstParameter> {
        order_params_stack_only(
            used_registers,
            used_offset_from_base_pointers,
            reg_to_var,
            offset_to_var,
        )
    }

    fn order_params_stack_only(
        used_registers: &HashSet<Register>,
        used_offset_from_base_pointers: &HashSet<isize>,
        reg_to_var: &HashMap<Register, AstVariableId>,
        offset_to_var: &HashMap<isize, AstVariableId>,
    ) -> Vec<AstParameter> {
        let mut params: Vec<AstParameter> = Vec::new();

        // Stack-only (cdecl/stdcall), order by ascending positive base-pointer offsets
        let mut stack_offsets: Vec<isize> = used_offset_from_base_pointers
            .iter()
            .copied()
            .filter(|o| *o > 0)
            .collect();
        stack_offsets.sort();
        for off in stack_offsets {
            push_stack_param(&mut params, off, offset_to_var);
        }
        // Any registers detected fall back afterwards, deterministic by name
        if !used_registers.is_empty() {
            let mut remaining: Vec<_> = used_registers.iter().cloned().collect();
            remaining.sort_by_key(|r| r.name());
            for r in remaining {
                push_reg_param(&mut params, &mut HashSet::new(), r, reg_to_var);
            }
        }

        params
    }
}
