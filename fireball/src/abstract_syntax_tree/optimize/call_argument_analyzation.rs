use crate::{
    abstract_syntax_tree::{
        ArcAstVariableMap, Ast, AstCall, AstExpression, AstFunction, AstFunctionId,
        AstFunctionVersion, AstParameter, AstParameterLocation, AstStatement, AstStatementOrigin,
        AstValueOrigin, AstValueType, AstVariableId, ProcessedOptimization, Wrapped,
        WrappedAstStatement,
    },
    ir::{
        Register, VirtualMachine,
        analyze::variables::resolve_operand,
        data::{IrData, IrDataOperation},
        operator::{IrBinaryOperator, IrUnaryOperator},
        statements::IrStatement,
        x86_64::X64Range,
    },
    prelude::DecompileError,
    utils::version_map::VersionMap,
};
use either::Either;
use hashbrown::{HashMap, HashSet};

pub(super) fn analyze_call_arguments(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let variables;
    let mut body;
    let function_ir;
    let function_return_type;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();

        body = std::mem::take(&mut function.body);
        variables = function.variables.clone();
        function_ir = function.ir.clone();
        function_return_type = function.return_type.clone();
    }

    // We rebuild call nodes from IR (JumpByCall) and infer args from surrounding assignments.
    let reg_name_to_var = build_register_name_to_var_map(&variables);
    let var_id_to_reg = build_var_id_to_register_name_map(&variables);
    let _data_to_var = build_data_location_to_var_map(&variables);

    let var_id_to_rsp_offset = build_var_id_to_rsp_offset_map(&variables);
    let rsp_offset_to_var = build_rsp_offset_to_var_map(&var_id_to_rsp_offset);

    // Conservative mode:
    // Avoid collapsing IR groups into new call nodes, which can misclassify CFG jumps.
    // Instead, only enrich already-existing call nodes with inferred arguments.
    enrich_existing_call_args_recursive(
        ast,
        function_id,
        &variables,
        &mut body,
        &reg_name_to_var,
        &var_id_to_reg,
        &var_id_to_rsp_offset,
        &rsp_offset_to_var,
    );

    // Convert goto(function) forms into call(...) when safe/possible.
    rebuild_goto_as_call_if_possible(ast, function_id, &mut body);

    // Goto -> call conversion can create new call sites; enrich args again.
    enrich_existing_call_args_recursive(
        ast,
        function_id,
        &variables,
        &mut body,
        &reg_name_to_var,
        &var_id_to_reg,
        &var_id_to_rsp_offset,
        &rsp_offset_to_var,
    );

    // If this function contains function-call sites with inferred arguments,
    // propagate that information back into callee signatures when they are missing.
    propagate_observed_call_args_to_callee_parameters(ast, &body);

    // Function-level CFG safety: once a top-level goto/return is emitted,
    // subsequent statements are unreachable in this linear AST view.
    let split_tail = truncate_after_terminal_control_flow(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function
            .processed_optimizations
            .push(ProcessedOptimization::CallArgumentAnalyzation);
        function.body = body;
    }

    if let Some((target_addr, tail_body)) = split_tail {
        materialize_split_tail_function(
            ast,
            target_addr,
            tail_body,
            variables,
            function_ir,
            function_return_type,
        );
    }

    Ok(())
}

fn propagate_observed_call_args_to_callee_parameters(ast: &Ast, body: &[WrappedAstStatement]) {
    let mut observed: HashMap<AstFunctionId, usize> = HashMap::new();
    collect_observed_call_arg_counts(body, &mut observed);
    if observed.is_empty() {
        return;
    }

    let mut funcs = ast.functions.write().unwrap();
    for (target, argc) in observed {
        let Some(version) = ast.function_versions.get(&target).copied() else {
            continue;
        };
        let Some(func) = funcs.get_mut(&target).and_then(|m| m.get_mut(&version)) else {
            continue;
        };

        if func.parameters.len() >= argc {
            continue;
        }

        let mut params = func.parameters.clone();
        for idx in params.len()..argc {
            let location = if idx == 0 {
                AstParameterLocation::Register(
                    IrData::Register(<VirtualMachine as X64Range>::rcx()).into(),
                )
            } else if idx == 1 {
                AstParameterLocation::Register(
                    IrData::Register(<VirtualMachine as X64Range>::rdx()).into(),
                )
            } else if idx == 2 {
                AstParameterLocation::Register(
                    IrData::Register(<VirtualMachine as X64Range>::r8()).into(),
                )
            } else if idx == 3 {
                AstParameterLocation::Register(
                    IrData::Register(<VirtualMachine as X64Range>::r9()).into(),
                )
            } else {
                let off_rbp = 0x30 + ((idx - 4) as isize * 8);
                AstParameterLocation::Stack(off_rbp)
            };
            params.push(AstParameter {
                location,
                id: Either::Right(format!("p{}", idx + 1)),
            });
        }
        func.parameters = params;
    }
}

fn truncate_after_terminal_control_flow(
    body: &mut Vec<WrappedAstStatement>,
) -> Option<(u64, Vec<WrappedAstStatement>)> {
    let cut = body.iter().position(|stmt| {
        matches!(
            stmt.statement,
            AstStatement::Call(_) | AstStatement::Goto(_) | AstStatement::Return(_)
        )
    });
    let Some(idx) = cut else {
        return None;
    };

    let mut split_target = None;
    if let AstStatement::Goto(jump) = &body[idx].statement {
        split_target = jump_target_constant_address(jump);
    }

    let tail = if idx + 1 < body.len() {
        body[idx + 1..].to_vec()
    } else {
        Vec::new()
    };
    body.truncate(idx + 1);

    if let Some(addr) = split_target
        && !tail.is_empty()
    {
        return Some((addr, tail));
    }

    None
}

fn materialize_split_tail_function(
    ast: &mut Ast,
    target_addr: u64,
    tail_body: Vec<WrappedAstStatement>,
    variables: ArcAstVariableMap,
    function_ir: std::sync::Arc<crate::ir::analyze::ir_function::IrFunction>,
    function_return_type: AstValueType,
) {
    let target = AstFunctionId {
        address: target_addr,
    };
    let mut functions = ast.functions.write().unwrap();
    if functions.contains_key(&target) {
        return;
    }

    let new_func = AstFunction {
        name: None,
        id: target,
        ir: function_ir,
        return_type: function_return_type,
        parameters: Vec::new(),
        variables,
        body: tail_body,
        processed_optimizations: Vec::new(),
    };
    functions.insert(target, VersionMap::new(AstFunctionVersion(1), new_func));
    ast.function_versions.insert(target, AstFunctionVersion(1));
}

fn collect_observed_call_arg_counts(
    body: &[WrappedAstStatement],
    out: &mut HashMap<AstFunctionId, usize>,
) {
    for stmt in body.iter() {
        match &stmt.statement {
            AstStatement::Call(AstCall::Function { target, args }) => {
                if !args.is_empty() {
                    let entry = out.entry(*target).or_insert(0);
                    *entry = (*entry).max(args.len());
                }
            }
            AstStatement::If(_, t, f) => {
                collect_observed_call_arg_counts(t, out);
                if let Some(f) = f {
                    collect_observed_call_arg_counts(f, out);
                }
            }
            AstStatement::While(_, b) | AstStatement::Block(b) => {
                collect_observed_call_arg_counts(b, out);
            }
            AstStatement::For(_, _, _, b) => {
                collect_observed_call_arg_counts(b, out);
            }
            _ => {}
        }
    }
}

fn rebuild_goto_as_call_if_possible(
    ast: &Ast,
    scope: AstFunctionId,
    body: &mut Vec<WrappedAstStatement>,
) {
    for stmt in body.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, t, f) => {
                rebuild_goto_as_call_if_possible(ast, scope, t);
                if let Some(f) = f {
                    rebuild_goto_as_call_if_possible(ast, scope, f);
                }
            }
            AstStatement::While(_, b) | AstStatement::Block(b) => {
                rebuild_goto_as_call_if_possible(ast, scope, b);
            }
            AstStatement::For(_, _, _, b) => rebuild_goto_as_call_if_possible(ast, scope, b),
            _ => {}
        }
    }

    let original = std::mem::take(body);
    let total_len = original.len();
    let mut rebuilt: Vec<WrappedAstStatement> = Vec::with_capacity(total_len + 1);
    for idx in 0..total_len {
        let mut stmt = original[idx].clone();
        let mut should_stop_after = false;
        if let AstStatement::Goto(jump) = &stmt.statement {
            let tail_position = original_suffix_is_non_effectful(&original, idx + 1);
            let origin_is_unconditional_jump = stmt_origin_is_unconditional_jump(&stmt);

            if let Some(target) = jump_target_to_function_id(ast, jump) {
                if tail_position && origin_is_unconditional_jump && target != scope {
                    stmt.statement = AstStatement::Call(AstCall::Function {
                        target,
                        args: Vec::new(),
                    });
                    should_stop_after = true;
                }
            }
        }

        rebuilt.push(stmt);

        if should_stop_after {
            rebuilt.push(WrappedAstStatement {
                statement: AstStatement::Return(None),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            });
            break;
        }
    }

    *body = rebuilt;
}

fn original_suffix_is_non_effectful(body: &[WrappedAstStatement], start: usize) -> bool {
    if start >= body.len() {
        return true;
    }
    body[start..].iter().all(|x| {
        matches!(
            x.statement,
            AstStatement::Comment(_) | AstStatement::Empty | AstStatement::Label(_)
        )
    })
}

fn stmt_origin_is_unconditional_jump(stmt: &WrappedAstStatement) -> bool {
    let Some(ir_stmt) = stmt_origin_ir_statement(stmt) else {
        return false;
    };
    if !matches!(ir_stmt, IrStatement::Jump { .. }) {
        return false;
    }

    let Some(inst) = stmt_origin_instruction(stmt) else {
        return false;
    };

    inst.is_jmp() && !inst.is_jcc() && !inst.is_call()
}

fn jump_target_to_function_id(
    ast: &Ast,
    jump: &crate::abstract_syntax_tree::AstJumpTarget,
) -> Option<AstFunctionId> {
    if let Some(addr) = jump_target_constant_address(jump) {
        resolve_function_id_by_address(ast, addr)
    } else {
        None
    }
}

fn jump_target_constant_address(jump: &crate::abstract_syntax_tree::AstJumpTarget) -> Option<u64> {
    match jump {
        crate::abstract_syntax_tree::AstJumpTarget::Function { target } => Some(target.address),
        // Instruction-based jump targets are often intra-function control-flow labels.
        // Do not treat them as callable function addresses here.
        crate::abstract_syntax_tree::AstJumpTarget::Instruction { .. } => None,
        crate::abstract_syntax_tree::AstJumpTarget::Unknown(s) => {
            let stripped = s.trim_start_matches("0x").trim_start_matches("0X");
            u64::from_str_radix(stripped, 16).ok()
        }
        crate::abstract_syntax_tree::AstJumpTarget::Variable {
            var_map, var_id, ..
        } => {
            let vars = var_map.read().unwrap();
            let var = vars.get(var_id)?;
            let c = var.const_value.as_ref()?;
            match &c.item {
                crate::abstract_syntax_tree::AstValue::Pointer(p) => match &p.item {
                    crate::abstract_syntax_tree::AstValue::Num(n) => {
                        let (sign, digits) = n.to_u64_digits();
                        if sign == num_bigint::Sign::Minus {
                            None
                        } else {
                            Some(*digits.first().unwrap_or(&0))
                        }
                    }
                    _ => None,
                },
                crate::abstract_syntax_tree::AstValue::Num(n) => {
                    let (sign, digits) = n.to_u64_digits();
                    if sign == num_bigint::Sign::Minus {
                        None
                    } else {
                        Some(*digits.first().unwrap_or(&0))
                    }
                }
                _ => None,
            }
        }
    }
}

fn function_like_name_from_address(ast: &Ast, addr: u64) -> String {
    if let Some(fid) = resolve_function_id_by_address(ast, addr) {
        fid.get_default_name()
    } else {
        AstFunctionId { address: addr }.get_default_name()
    }
}

fn resolve_function_id_by_address(ast: &Ast, addr: u64) -> Option<AstFunctionId> {
    let exact = AstFunctionId { address: addr };
    if ast.functions.read().unwrap().contains_key(&exact) {
        Some(exact)
    } else {
        None
    }
}

fn rebuild_call_groups_recursive(
    ast: &Ast,
    scope: AstFunctionId,
    variables: &ArcAstVariableMap,
    stmts: &mut Vec<WrappedAstStatement>,
    reg_name_to_var: &HashMap<Register, AstVariableId>,
    var_id_to_reg: &HashMap<AstVariableId, Register>,
    data_to_var: &HashMap<IrData, AstVariableId>,
    var_id_to_rsp_offset: &HashMap<AstVariableId, isize>,
    rsp_offset_to_var: &HashMap<isize, AstVariableId>,
) {
    // First, recurse into nested statement lists so calls inside blocks/ifs also get rebuilt.
    for ws in stmts.iter_mut() {
        match &mut ws.statement {
            AstStatement::If(_, t, f) => {
                rebuild_call_groups_recursive(
                    ast,
                    scope,
                    variables,
                    t,
                    reg_name_to_var,
                    var_id_to_reg,
                    data_to_var,
                    var_id_to_rsp_offset,
                    rsp_offset_to_var,
                );
                if let Some(f) = f {
                    rebuild_call_groups_recursive(
                        ast,
                        scope,
                        variables,
                        f,
                        reg_name_to_var,
                        var_id_to_reg,
                        data_to_var,
                        var_id_to_rsp_offset,
                        rsp_offset_to_var,
                    );
                }
            }
            AstStatement::While(_, b) | AstStatement::Block(b) => {
                rebuild_call_groups_recursive(
                    ast,
                    scope,
                    variables,
                    b,
                    reg_name_to_var,
                    var_id_to_reg,
                    data_to_var,
                    var_id_to_rsp_offset,
                    rsp_offset_to_var,
                );
            }
            AstStatement::For(_init, _cond, _update, b) => {
                rebuild_call_groups_recursive(
                    ast,
                    scope,
                    variables,
                    b,
                    reg_name_to_var,
                    var_id_to_reg,
                    data_to_var,
                    var_id_to_rsp_offset,
                    rsp_offset_to_var,
                );
            }
            _ => {}
        }
    }

    // Then rebuild call-groups at this level.
    let body = std::mem::take(stmts);
    let call_sites = find_call_sites(&body);
    if call_sites.is_empty() {
        *stmts = body;
        enrich_existing_call_args_recursive(
            ast,
            scope,
            variables,
            stmts,
            reg_name_to_var,
            var_id_to_reg,
            var_id_to_rsp_offset,
            rsp_offset_to_var,
        );
        return;
    }

    let mut rebuilt_groups: HashMap<usize, (usize, Vec<WrappedAstStatement>)> = HashMap::new();
    for site in call_sites.into_iter() {
        let existing_call_stmt = resolve_call_target(
            ast,
            scope,
            variables,
            &body[site.call_stmt_idx],
            data_to_var,
        )
        .or_else(|| {
            // If target resolution fails, still reuse an existing AST call node so
            // we can infer arguments and collapse call-mechanics statements.
            match &body[site.call_stmt_idx].statement {
                AstStatement::Call(_) => Some(body[site.call_stmt_idx].clone()),
                _ => None,
            }
        });
        let Some(existing_call_stmt) = existing_call_stmt else {
            continue;
        };

        let args = infer_call_args(
            ast,
            variables,
            &body,
            site.group_start,
            &existing_call_stmt,
            reg_name_to_var,
            var_id_to_reg,
            var_id_to_rsp_offset,
            rsp_offset_to_var,
        );

        let call_stmt = set_call_args(existing_call_stmt, args);

        // Rebuild this instruction-group as a single call statement.
        rebuilt_groups.insert(site.group_start, (site.group_end, vec![call_stmt]));
    }

    if rebuilt_groups.is_empty() {
        *stmts = body;
        return;
    }

    let mut new_body: Vec<WrappedAstStatement> = Vec::with_capacity(body.len());
    let mut i = 0;
    while i < body.len() {
        if let Some((end, stmts_group)) = rebuilt_groups.get(&i) {
            new_body.extend(stmts_group.iter().cloned());
            i = *end;
        } else {
            new_body.push(body[i].clone());
            i += 1;
        }
    }

    *stmts = new_body;

    enrich_existing_call_args_recursive(
        ast,
        scope,
        variables,
        stmts,
        reg_name_to_var,
        var_id_to_reg,
        var_id_to_rsp_offset,
        rsp_offset_to_var,
    );
}

fn enrich_existing_call_args_recursive(
    ast: &Ast,
    scope: AstFunctionId,
    variables: &ArcAstVariableMap,
    stmts: &mut Vec<WrappedAstStatement>,
    reg_name_to_var: &HashMap<Register, AstVariableId>,
    var_id_to_reg: &HashMap<AstVariableId, Register>,
    var_id_to_rsp_offset: &HashMap<AstVariableId, isize>,
    rsp_offset_to_var: &HashMap<isize, AstVariableId>,
) {
    for i in 0..stmts.len() {
        // Recurse first
        match &mut stmts[i].statement {
            AstStatement::If(_, t, f) => {
                enrich_existing_call_args_recursive(
                    ast,
                    scope,
                    variables,
                    t,
                    reg_name_to_var,
                    var_id_to_reg,
                    var_id_to_rsp_offset,
                    rsp_offset_to_var,
                );
                if let Some(f) = f {
                    enrich_existing_call_args_recursive(
                        ast,
                        scope,
                        variables,
                        f,
                        reg_name_to_var,
                        var_id_to_reg,
                        var_id_to_rsp_offset,
                        rsp_offset_to_var,
                    );
                }
            }
            AstStatement::While(_, b) | AstStatement::Block(b) => {
                enrich_existing_call_args_recursive(
                    ast,
                    scope,
                    variables,
                    b,
                    reg_name_to_var,
                    var_id_to_reg,
                    var_id_to_rsp_offset,
                    rsp_offset_to_var,
                );
            }
            AstStatement::For(_, _, _, b) => {
                enrich_existing_call_args_recursive(
                    ast,
                    scope,
                    variables,
                    b,
                    reg_name_to_var,
                    var_id_to_reg,
                    var_id_to_rsp_offset,
                    rsp_offset_to_var,
                );
            }
            _ => {}
        }

        rewrite_indirect_call_target_from_nearby_assignment(stmts, i);

        let should_infer = match &stmts[i].statement {
            AstStatement::Call(AstCall::Function { args, .. })
            | AstStatement::Call(AstCall::Variable { args, .. })
            | AstStatement::Call(AstCall::Unknown(_, args)) => args.is_empty(),
            _ => false,
        };
        if !should_infer {
            continue;
        }

        let inferred = {
            let call_stmt = &stmts[i];
            infer_call_args(
                ast,
                variables,
                stmts.as_slice(),
                i,
                call_stmt,
                reg_name_to_var,
                var_id_to_reg,
                var_id_to_rsp_offset,
                rsp_offset_to_var,
            )
        };
        if inferred.is_empty() {
            continue;
        }

        match &mut stmts[i].statement {
            AstStatement::Call(AstCall::Function { args, .. })
            | AstStatement::Call(AstCall::Variable { args, .. })
            | AstStatement::Call(AstCall::Unknown(_, args)) => {
                *args = inferred;
            }
            _ => {}
        }
    }
}

fn rewrite_indirect_call_target_from_nearby_assignment(
    stmts: &mut [WrappedAstStatement],
    call_idx: usize,
) {
    let (target_var, current_args) = match &stmts[call_idx].statement {
        AstStatement::Call(AstCall::Variable { var_id, args, .. }) => (*var_id, args.clone()),
        _ => return,
    };

    let mut j = call_idx;
    while j > 0 {
        j -= 1;
        if is_barrier(&stmts[j].statement) {
            break;
        }

        let AstStatement::Assignment(lhs, _rhs) = &stmts[j].statement else {
            continue;
        };
        let AstExpression::Variable(_, lhs_id) = lhs.item else {
            continue;
        };
        if lhs_id != target_var {
            continue;
        }

        let Some(slot) = stmt_origin_rip_relative_load_slot(&stmts[j]) else {
            break;
        };
        stmts[call_idx].statement =
            AstStatement::Call(AstCall::Unknown(format!("*0x{:X}", slot), current_args));
        break;
    }
}

fn stmt_origin_rip_relative_load_slot(stmt: &WrappedAstStatement) -> Option<u64> {
    let ir_stmt = stmt_origin_ir_statement(stmt)?;
    let IrStatement::Assignment { from, .. } = ir_stmt else {
        return None;
    };
    let instruction_args = stmt_origin_instruction_args(stmt)?;
    let resolved_from = resolve_operand(from, instruction_args);

    let inst = stmt_origin_instruction(stmt)?;
    let inst_len = inst.bytes.as_ref().map(|x| x.len() as u64).unwrap_or(0);
    let AstStatementOrigin::Ir(desc) = &stmt.origin else {
        return None;
    };
    let ir_index = desc.descriptor().ir_index() as usize;
    let rip_next_ip = desc.ir().get_ir()[ir_index].address.get_virtual_address() + inst_len;

    rip_relative_deref_address(resolved_from.as_ref(), rip_next_ip)
}

fn rip_relative_deref_address(data: &IrData, rip_next_ip: u64) -> Option<u64> {
    fn signed_const(data: &IrData) -> Option<i128> {
        match data {
            IrData::Constant(v) => Some(*v as i128),
            IrData::Operation(IrDataOperation::Unary {
                operator: IrUnaryOperator::Negation,
                arg,
            }) => arg.constant().map(|v| -(v as i128)),
            _ => None,
        }
    }
    fn unwrap_cast(data: &IrData) -> &IrData {
        match data {
            IrData::Operation(IrDataOperation::Unary {
                operator: IrUnaryOperator::ZeroExtend | IrUnaryOperator::SignExtend,
                arg,
            }) => arg.as_ref(),
            _ => data,
        }
    }

    let data = unwrap_cast(data);
    let IrData::Dereference(inner) = data else {
        return None;
    };
    let inner = unwrap_cast(inner.as_ref());

    let IrData::Operation(IrDataOperation::Binary {
        operator,
        arg1,
        arg2,
    }) = inner
    else {
        return None;
    };

    let is_rip = |d: &IrData| matches!(d, IrData::Register(r) if r.is_ip());

    match operator {
        IrBinaryOperator::Add => {
            if is_rip(arg1.as_ref()) {
                let disp = signed_const(arg2.as_ref())?;
                return u64::try_from((rip_next_ip as i128) + disp).ok();
            }
            if is_rip(arg2.as_ref()) {
                let disp = signed_const(arg1.as_ref())?;
                return u64::try_from((rip_next_ip as i128) + disp).ok();
            }
            None
        }
        IrBinaryOperator::Sub => {
            if is_rip(arg1.as_ref()) {
                let disp = signed_const(arg2.as_ref())?;
                return u64::try_from((rip_next_ip as i128) - disp).ok();
            }
            None
        }
        _ => None,
    }
}

#[derive(Debug, Clone, Copy)]
struct CallSite {
    group_start: usize,
    group_end: usize,
    call_stmt_idx: usize,
}

fn find_call_sites(body: &[WrappedAstStatement]) -> Vec<CallSite> {
    let mut sites = Vec::new();
    let mut idx = 0;
    while idx < body.len() {
        let Some(ir_index) = stmt_ir_index(&body[idx]) else {
            idx += 1;
            continue;
        };

        let start = find_ir_group_start_index(body, idx, ir_index);
        let end = find_ir_group_end_index(body, idx, ir_index);

        let mut call_stmt_idx: Option<usize> = None;
        for i in start..end {
            if is_origin_jump_by_call(&body[i]) {
                call_stmt_idx = Some(i);
                break;
            }
        }
        if let Some(call_stmt_idx) = call_stmt_idx {
            sites.push(CallSite {
                group_start: start,
                group_end: end,
                call_stmt_idx,
            });
        }

        idx = end;
    }
    sites
}

fn stmt_ir_index(stmt: &WrappedAstStatement) -> Option<u32> {
    match &stmt.origin {
        AstStatementOrigin::Ir(desc) => Some(desc.descriptor().ir_index()),
        _ => None,
    }
}

fn find_ir_group_start_index(body: &[WrappedAstStatement], mut idx: usize, ir_index: u32) -> usize {
    while idx > 0 {
        let Some(prev_ir_index) = stmt_ir_index(&body[idx - 1]) else {
            break;
        };
        if prev_ir_index != ir_index {
            break;
        }
        idx -= 1;
    }
    idx
}

fn find_ir_group_end_index(body: &[WrappedAstStatement], mut idx: usize, ir_index: u32) -> usize {
    while idx < body.len() {
        let Some(next_ir_index) = stmt_ir_index(&body[idx]) else {
            break;
        };
        if next_ir_index != ir_index {
            break;
        }
        idx += 1;
    }
    idx
}

fn is_origin_jump_by_call(stmt: &WrappedAstStatement) -> bool {
    let Some(ir_stmt) = stmt_origin_ir_statement(stmt) else {
        return false;
    };
    matches!(ir_stmt, IrStatement::JumpByCall { .. })
}

fn stmt_origin_ir_statement(stmt: &WrappedAstStatement) -> Option<&IrStatement> {
    let AstStatementOrigin::Ir(desc) = &stmt.origin else {
        return None;
    };
    let ir_index = desc.descriptor().ir_index() as usize;
    let stmt_index = (*desc.descriptor().statement_index()).as_ref().copied()? as usize;
    let ir_block = &desc.ir().get_ir()[ir_index];
    let stmts = ir_block.statements.as_ref()?;
    stmts.get(stmt_index)
}

fn stmt_origin_instruction_args(stmt: &WrappedAstStatement) -> Option<&[iceball::Argument]> {
    let AstStatementOrigin::Ir(desc) = &stmt.origin else {
        return None;
    };
    let ir_index = desc.descriptor().ir_index() as usize;
    Some(&desc.ir().get_instructions()[ir_index].inner.arguments)
}

fn stmt_origin_instruction(stmt: &WrappedAstStatement) -> Option<&iceball::Instruction> {
    let AstStatementOrigin::Ir(desc) = &stmt.origin else {
        return None;
    };
    let ir_index = desc.descriptor().ir_index() as usize;
    Some(&desc.ir().get_instructions()[ir_index].inner)
}

fn resolve_call_target(
    ast: &Ast,
    scope: AstFunctionId,
    variables: &ArcAstVariableMap,
    call_stmt: &WrappedAstStatement,
    data_to_var: &HashMap<IrData, AstVariableId>,
) -> Option<WrappedAstStatement> {
    // JumpByCall target in IR (resolved operands) -> AstCall
    let ir_stmt = stmt_origin_ir_statement(call_stmt)?;
    let IrStatement::JumpByCall { target } = ir_stmt else {
        return None;
    };
    let instruction_args = stmt_origin_instruction_args(call_stmt)?;
    let resolved_target = resolve_operand(target, instruction_args);

    let call = match resolved_target.as_ref() {
        IrData::Constant(c) => {
            let addr = *c as u64;
            if let Some(fid) = resolve_function_id_by_address(ast, addr) {
                AstCall::Function {
                    target: fid,
                    args: Vec::new(),
                }
            } else {
                AstCall::Unknown(function_like_name_from_address(ast, addr), Vec::new())
            }
        }
        _ => {
            // Try mapping IR data location to an AST variable.
            let var_id = data_to_var.get(resolved_target.as_ref()).copied();
            if let Some(var_id) = var_id {
                AstCall::Variable {
                    scope,
                    var_map: variables.clone(),
                    var_id,
                    args: Vec::new(),
                }
            } else {
                AstCall::Unknown(resolved_target.to_string(), Vec::new())
            }
        }
    };

    let mut out = call_stmt.clone();
    out.statement = AstStatement::Call(call);

    Some(out)
}

fn set_call_args(
    mut call_stmt: WrappedAstStatement,
    args: Vec<Wrapped<AstExpression>>,
) -> WrappedAstStatement {
    if let AstStatement::Call(call) = &mut call_stmt.statement {
        match call {
            AstCall::Function { args: a, .. }
            | AstCall::Variable { args: a, .. }
            | AstCall::Unknown(_, a) => {
                *a = args;
            }
            AstCall::Builtin(_, _) => {}
        }
    }
    call_stmt
}

fn infer_call_args(
    ast: &Ast,
    variables: &ArcAstVariableMap,
    body: &[WrappedAstStatement],
    call_group_start_idx: usize,
    call_stmt: &WrappedAstStatement,
    reg_name_to_var: &HashMap<Register, AstVariableId>,
    var_id_to_reg: &HashMap<AstVariableId, Register>,
    var_id_to_rsp_offset: &HashMap<AstVariableId, isize>,
    rsp_offset_to_var: &HashMap<isize, AstVariableId>,
) -> Vec<Wrapped<AstExpression>> {
    let callee_params = match &call_stmt.statement {
        AstStatement::Call(AstCall::Function { target, .. }) => {
            get_function_parameters(ast, *target)
        }
        _ => None,
    };

    let fallback_args = infer_args_by_calling_convention(
        variables,
        body,
        call_group_start_idx,
        reg_name_to_var,
        var_id_to_reg,
        var_id_to_rsp_offset,
    );

    if let Some(params) = callee_params
        && !params.is_empty()
    {
        let mut args = infer_args_from_callee_params(
            variables,
            body,
            call_group_start_idx,
            &params,
            reg_name_to_var,
            var_id_to_reg,
            var_id_to_rsp_offset,
            rsp_offset_to_var,
        );

        let known_count = args.iter().filter(|x| !is_unknown_expr(x)).count();
        if known_count == 0 && !fallback_args.is_empty() {
            return fallback_args;
        }
        if known_count * 2 < args.len() && !fallback_args.is_empty() {
            return fallback_args;
        }

        // Keep callee-based ordering but patch unknown slots from fallback when possible.
        for (i, arg) in args.iter_mut().enumerate() {
            if is_unknown_expr(arg)
                && let Some(fallback) = fallback_args.get(i)
                && !is_unknown_expr(fallback)
            {
                *arg = fallback.clone();
            }
        }

        return args;
    }

    fallback_args
}

fn infer_args_from_callee_params(
    variables: &ArcAstVariableMap,
    body: &[WrappedAstStatement],
    call_group_start_idx: usize,
    params: &[crate::abstract_syntax_tree::AstParameter],
    reg_name_to_var: &HashMap<Register, AstVariableId>,
    var_id_to_reg: &HashMap<AstVariableId, Register>,
    var_id_to_rsp_offset: &HashMap<AstVariableId, isize>,
    rsp_offset_to_var: &HashMap<isize, AstVariableId>,
) -> Vec<Wrapped<AstExpression>> {
    let mut interested: HashSet<Register> = HashSet::new();
    for p in params.iter() {
        let AstParameterLocation::Register(reg_loc) = &p.location else {
            continue;
        };
        if let Some(r) = ir_register(reg_loc.as_ref()) {
            interested.insert(r);
        }
    }
    let reg_writes =
        scan_backward_for_register_writes(body, call_group_start_idx, var_id_to_reg, &interested);

    // Stack args: map callee RBP+off to caller RSP+off (Win64: rbp = rsp after `push rbp`).
    let mut interested_stack_offsets: HashSet<isize> = HashSet::new();
    for p in params.iter() {
        let AstParameterLocation::Stack(off_rbp) = &p.location else {
            continue;
        };
        let off_rsp = *off_rbp - 0x10;
        if off_rsp >= 0 {
            interested_stack_offsets.insert(off_rsp);
        }
    }
    let stack_writes = if interested_stack_offsets.is_empty() {
        HashMap::new()
    } else {
        scan_backward_for_stack_slot_writes(
            body,
            call_group_start_idx,
            var_id_to_rsp_offset,
            var_id_to_reg,
            &interested_stack_offsets,
        )
    };

    let mut out: Vec<Wrapped<AstExpression>> = Vec::new();
    for p in params.iter() {
        match &p.location {
            AstParameterLocation::Register(reg_loc) => {
                let Some(r) = ir_register(reg_loc.as_ref()) else {
                    out.push(wrap_unknown_expr());
                    continue;
                };
                let arg = reg_writes
                    .get(&r)
                    .cloned()
                    .or_else(|| {
                        reg_name_to_var
                            .get(&r)
                            .copied()
                            .map(|id| wrap_var_expr(variables.clone(), id))
                    })
                    .unwrap_or_else(wrap_unknown_expr);
                out.push(arg);
            }
            AstParameterLocation::Stack(_) => {
                let AstParameterLocation::Stack(off_rbp) = &p.location else {
                    out.push(wrap_unknown_expr());
                    continue;
                };
                let off_rsp = *off_rbp - 0x10;
                if let Some(v) = stack_writes.get(&off_rsp) {
                    out.push(v.clone());
                } else if let Some(var_id) = rsp_offset_to_var.get(&off_rsp).copied() {
                    out.push(wrap_var_expr(variables.clone(), var_id));
                } else {
                    out.push(wrap_unknown_expr());
                }
            }
        }
    }

    out
}

fn infer_args_by_calling_convention(
    variables: &ArcAstVariableMap,
    body: &[WrappedAstStatement],
    call_group_start_idx: usize,
    reg_name_to_var: &HashMap<Register, AstVariableId>,
    var_id_to_reg: &HashMap<AstVariableId, Register>,
    var_id_to_rsp_offset: &HashMap<AstVariableId, isize>,
) -> Vec<Wrapped<AstExpression>> {
    // Windows x64 calling convention (best-effort): rcx, rdx, r8, r9.
    // We accept width-variants since IR often uses edx/r8d.
    let arg_regs = [
        [
            <VirtualMachine as X64Range>::rcx(),
            <VirtualMachine as X64Range>::ecx(),
            <VirtualMachine as X64Range>::cx(),
            <VirtualMachine as X64Range>::cl(),
        ],
        [
            <VirtualMachine as X64Range>::rdx(),
            <VirtualMachine as X64Range>::edx(),
            <VirtualMachine as X64Range>::dx(),
            <VirtualMachine as X64Range>::dl(),
        ],
        [
            <VirtualMachine as X64Range>::r8(),
            <VirtualMachine as X64Range>::r8d(),
            <VirtualMachine as X64Range>::r8w(),
            <VirtualMachine as X64Range>::r8b(),
        ],
        [
            <VirtualMachine as X64Range>::r9(),
            <VirtualMachine as X64Range>::r9d(),
            <VirtualMachine as X64Range>::r9w(),
            <VirtualMachine as X64Range>::r9b(),
        ],
    ];

    let mut interested: HashSet<Register> = HashSet::new();
    for regs in arg_regs.iter() {
        for reg in regs.iter() {
            interested.insert(reg.clone());
        }
    }
    let reg_writes =
        scan_backward_for_register_writes(body, call_group_start_idx, var_id_to_reg, &interested);

    // Only include registers that look like explicit argument setup in the window.
    let mut out: Vec<Wrapped<AstExpression>> = Vec::new();
    for regs in arg_regs.iter() {
        let mut chosen = None;
        for reg in regs.iter() {
            if let Some(v) = reg_writes.get(reg) {
                chosen = Some(v.clone());
                break;
            }
        }
        if chosen.is_none() {
            for reg in regs.iter() {
                if let Some(var_id) = reg_name_to_var.get(reg).copied() {
                    chosen = Some(wrap_var_expr(variables.clone(), var_id));
                    break;
                }
            }
        }
        if let Some(v) = chosen {
            out.push(v);
        }
    }

    // If we couldn't prove anything, don't emit noisy unknown args.
    if out.is_empty() {
        // No proven register setup; don't emit noisy args.
        return Vec::new();
    }

    // Best-effort stack args (Win64): only attempt when all 4 reg args are proven.
    if out.len() < 4 {
        return out;
    }

    // Scan for writes to [rsp + 0x20], [rsp + 0x28], ... (arg5+; ignore shadow-space/home-space).
    let mut interested_stack_offsets: HashSet<isize> = HashSet::new();
    for i in 0..16 {
        interested_stack_offsets.insert(0x20 + (i * 8) as isize);
    }
    let stack_writes = scan_backward_for_stack_slot_writes(
        body,
        call_group_start_idx,
        var_id_to_rsp_offset,
        var_id_to_reg,
        &interested_stack_offsets,
    );
    if !stack_writes.contains_key(&0x20) {
        return out;
    }
    let mut off = 0x20isize;
    while let Some(v) = stack_writes.get(&off) {
        out.push(v.clone());
        off += 8;
        if off > 0x20 + (16 * 8) {
            break;
        }
    }

    // If we have no callee info, avoid printing implicit default values.
    // But keep proven args.
    out
}

fn scan_backward_for_register_writes(
    body: &[WrappedAstStatement],
    call_group_start_idx: usize,
    var_id_to_reg: &HashMap<AstVariableId, Register>,
    interested: &HashSet<Register>,
) -> HashMap<Register, Wrapped<AstExpression>> {
    let mut out: HashMap<Register, Wrapped<AstExpression>> = HashMap::new();

    let mut i = call_group_start_idx;
    while i > 0 {
        i -= 1;
        let stmt = &body[i].statement;

        if is_barrier(stmt) {
            break;
        }

        match stmt {
            AstStatement::Declaration(var, init) => {
                if let Some(reg) = var_id_to_reg.get(&var.id)
                    && interested.contains(reg)
                    && !out.contains_key(reg)
                {
                    if let Some(init) = init {
                        out.insert(reg.clone(), init.clone());
                    }
                }
            }
            AstStatement::Assignment(lhs, rhs) => {
                if let AstExpression::Variable(_, lhs_id) = lhs.item {
                    if let Some(reg) = var_id_to_reg.get(&lhs_id)
                        && interested.contains(reg)
                        && !out.contains_key(reg)
                    {
                        out.insert(reg.clone(), rhs.clone());
                    }
                }
            }
            _ => {}
        }

        if out.len() >= interested.len() {
            break;
        }
    }

    out
}

fn scan_backward_for_stack_slot_writes(
    body: &[WrappedAstStatement],
    call_group_start_idx: usize,
    var_id_to_rsp_offset: &HashMap<AstVariableId, isize>,
    var_id_to_reg: &HashMap<AstVariableId, Register>,
    interested_offsets: &HashSet<isize>,
) -> HashMap<isize, Wrapped<AstExpression>> {
    let mut out: HashMap<isize, Wrapped<AstExpression>> = HashMap::new();

    let mut i = call_group_start_idx;
    while i > 0 {
        i -= 1;
        let stmt = &body[i].statement;

        if is_barrier(stmt) {
            break;
        }
        if is_stack_pointer_write(stmt, var_id_to_reg) {
            break;
        }

        match stmt {
            AstStatement::Declaration(var, init) => {
                if let Some(off) = var_id_to_rsp_offset.get(&var.id).copied()
                    && interested_offsets.contains(&off)
                    && !out.contains_key(&off)
                {
                    if let Some(init) = init {
                        out.insert(off, init.clone());
                    }
                }
            }
            AstStatement::Assignment(lhs, rhs) => {
                if let AstExpression::Variable(_, lhs_id) = lhs.item {
                    if let Some(off) = var_id_to_rsp_offset.get(&lhs_id).copied()
                        && interested_offsets.contains(&off)
                        && !out.contains_key(&off)
                    {
                        out.insert(off, rhs.clone());
                    }
                }
            }
            _ => {}
        }

        if out.len() >= interested_offsets.len() {
            break;
        }
    }

    out
}

fn is_stack_pointer_write(
    stmt: &AstStatement,
    var_id_to_reg: &HashMap<AstVariableId, Register>,
) -> bool {
    match stmt {
        AstStatement::Declaration(var, _) => {
            var_id_to_reg.get(&var.id).is_some_and(Register::is_sp)
        }
        AstStatement::Assignment(lhs, _) => {
            if let AstExpression::Variable(_, id) = lhs.item {
                var_id_to_reg.get(&id).is_some_and(Register::is_sp)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn is_barrier(stmt: &AstStatement) -> bool {
    match stmt {
        AstStatement::Call(_)
        | AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Ir(_)
        | AstStatement::Return(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_) => true,

        AstStatement::If(_, _, _)
        | AstStatement::While(_, _)
        | AstStatement::For(_, _, _, _)
        | AstStatement::Block(_) => true,

        AstStatement::Declaration(_, _)
        | AstStatement::Assignment(_, _)
        | AstStatement::Label(_)
        | AstStatement::Comment(_)
        | AstStatement::Empty => false,
    }
}

fn get_function_parameters(
    ast: &Ast,
    function_id: AstFunctionId,
) -> Option<Vec<crate::abstract_syntax_tree::AstParameter>> {
    let functions = ast.functions.read().unwrap();
    let version = *ast.function_versions.get(&function_id)?;
    let func = functions.get(&function_id)?.get(&version)?;
    Some(func.parameters.clone())
}

fn build_register_name_to_var_map(
    variables: &ArcAstVariableMap,
) -> HashMap<Register, AstVariableId> {
    let mut map: HashMap<Register, AstVariableId> = HashMap::new();
    let vars = variables.read().unwrap();
    for (var_id, var) in vars.iter() {
        let Some(access_map) = var.data_access_ir.as_ref() else {
            continue;
        };
        for da in access_map.values().flat_map(|x| x.iter()) {
            let IrData::Register(reg) = da.location().as_ref() else {
                continue;
            };
            map.entry(reg.clone()).or_insert(*var_id);
        }
    }
    map
}

fn build_var_id_to_register_name_map(
    variables: &ArcAstVariableMap,
) -> HashMap<AstVariableId, Register> {
    let vars = variables.read().unwrap();
    let mut map: HashMap<AstVariableId, Register> = HashMap::new();
    for (var_id, var) in vars.iter() {
        let Some(access_map) = var.data_access_ir.as_ref() else {
            continue;
        };
        for da in access_map.values().flat_map(|x| x.iter()) {
            let IrData::Register(reg) = da.location().as_ref() else {
                continue;
            };
            map.entry(*var_id).or_insert(reg.clone());
            break;
        }
    }
    map
}

fn build_data_location_to_var_map(variables: &ArcAstVariableMap) -> HashMap<IrData, AstVariableId> {
    let vars = variables.read().unwrap();
    let mut map: HashMap<IrData, AstVariableId> = HashMap::new();
    for (var_id, var) in vars.iter() {
        let Some(access_map) = var.data_access_ir.as_ref() else {
            continue;
        };
        for da in access_map.values().flat_map(|x| x.iter()) {
            map.entry(da.location().as_ref().clone()).or_insert(*var_id);
        }
    }
    map
}

fn build_var_id_to_rsp_offset_map(variables: &ArcAstVariableMap) -> HashMap<AstVariableId, isize> {
    let vars = variables.read().unwrap();
    let mut map: HashMap<AstVariableId, isize> = HashMap::new();
    for (var_id, var) in vars.iter() {
        let Some(access_map) = var.data_access_ir.as_ref() else {
            continue;
        };
        for da in access_map.values().flat_map(|x| x.iter()) {
            let Some(off) = rsp_offset_from_location(da.location().as_ref()) else {
                continue;
            };
            map.entry(*var_id).or_insert(off);
            break;
        }
    }
    map
}

fn build_rsp_offset_to_var_map(
    var_id_to_rsp_offset: &HashMap<AstVariableId, isize>,
) -> HashMap<isize, AstVariableId> {
    let mut map: HashMap<isize, AstVariableId> = HashMap::new();
    for (var_id, off) in var_id_to_rsp_offset.iter() {
        map.entry(*off).or_insert(*var_id);
    }
    map
}

fn rsp_offset_from_location(loc: &IrData) -> Option<isize> {
    let IrData::Dereference(inner) = loc else {
        return None;
    };
    rsp_offset_from_expr(inner.as_ref())
}

fn rsp_offset_from_expr(expr: &IrData) -> Option<isize> {
    fn is_sp_expr(data: &IrData) -> bool {
        matches!(data, IrData::Register(r) if r.is_sp())
    }
    fn signed_const(data: &IrData) -> Option<isize> {
        match data {
            IrData::Constant(v) => Some(*v as isize),
            IrData::Operation(IrDataOperation::Unary {
                operator: IrUnaryOperator::Negation,
                arg,
            }) => arg.constant().map(|v| 0 - v as isize),
            _ => None,
        }
    }

    // [rsp]
    if is_sp_expr(expr) {
        return Some(0);
    }

    // [rsp +/- const]
    match expr {
        IrData::Operation(IrDataOperation::Binary {
            operator: IrBinaryOperator::Add,
            arg1,
            arg2,
        }) if is_sp_expr(arg1.as_ref()) => signed_const(arg2.as_ref()),
        IrData::Operation(IrDataOperation::Binary {
            operator: IrBinaryOperator::Sub,
            arg1,
            arg2,
        }) if is_sp_expr(arg1.as_ref()) => signed_const(arg2.as_ref()).map(|v| 0 - v),
        _ => None,
    }
}

// NOTE: previously we tried to selectively keep some statements in the same IR-instruction group.
// In practice, the call-related groups are almost entirely call mechanics, so we collapse to 1 stmt.

fn ir_register(data: &IrData) -> Option<Register> {
    match data {
        IrData::Register(r) => Some(r.clone()),
        _ => None,
    }
}

fn wrap_var_expr(var_map: ArcAstVariableMap, var_id: AstVariableId) -> Wrapped<AstExpression> {
    Wrapped {
        item: AstExpression::Variable(var_map, var_id),
        origin: AstValueOrigin::Unknown,
        comment: None,
    }
}

fn wrap_unknown_expr() -> Wrapped<AstExpression> {
    Wrapped {
        item: AstExpression::Unknown,
        origin: AstValueOrigin::Unknown,
        comment: None,
    }
}

fn is_unknown_expr(expr: &Wrapped<AstExpression>) -> bool {
    matches!(expr.item, AstExpression::Unknown)
}
