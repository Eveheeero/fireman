use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstVariableAccessType, GetRelatedVariables,
        ProcessedOptimization,
    },
    ir::{
        Register,
        data::{IrData, IrDataOperation},
        operator::IrUnaryOperator,
    },
    prelude::DecompileError,
};
use hashbrown::HashSet;

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

    let first_arg_undetectable_statement_index =
        super::utils::get_first_arg_undetectable_statement_index(body.iter());
    for (i, stmt) in body.iter().enumerate() {
        let stmt = &stmt.statement;
        let related_vars = stmt.get_related_variables();

        /* analyze registers before undetectable statements */
        if i < first_arg_undetectable_statement_index.unwrap_or(usize::MAX) {
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

    /*
    TODO ordering parameters
    x86 cdecl - arg passed with stack, sp cleaned by caller
    x86 stdcall - arg passed with stack, sp cleaned by callee
    x86 fastcall - ecx, edx, stack..., sp cleaned by callee
    x86 thiscall - ecx, edx, stack..., sp cleaned by callee (?)
    x86 vectorcall - xmm0, xmm1, xmm2, xmm3, stack..., sp cleaned by callee (?)
    x64 - rcx(xmm0), rdx(xmm1), r8(xmm2), r9(xmm3), stack..., sp cleaned by callee
     */
    // let parameters;

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function
            .processed_optimizations
            .push(ProcessedOptimization::ParameterAnalyzation);
        // function.parameters = parameters;
        function.body = body;
    }
    Ok(())
}
