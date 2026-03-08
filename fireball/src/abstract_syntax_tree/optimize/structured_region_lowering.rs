use crate::{
    abstract_syntax_tree::{
        ArcAstVariableMap, Ast, AstBinaryOperator, AstExpression, AstFunctionId,
        AstFunctionVersion, AstJumpTarget, AstLiteral, AstStatement, AstStatementOrigin,
        AstUnaryOperator, AstValueOrigin, AstValueType, AstVariable, Wrapped, WrappedAstStatement,
    },
    ir::analyze::{IrFunction, StructuredRegion},
    prelude::DecompileError,
};
use hashbrown::{HashMap, HashSet};

pub(super) fn lower_structured_regions(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let mut body;
    let ir;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|versions| versions.get_mut(&function_version))
            .unwrap();
        ir = function.ir.clone();
        body = std::mem::take(&mut function.body);
    }

    let variables = ast.get_variables(&function_id, &function_version)?;
    let lowered = ir.get_structured().and_then(|structured| {
        try_lower_body(ast, function_id, &variables, &body, &ir, structured)
    });
    if let Some(lowered) = lowered {
        body = lowered;
    }

    let mut functions = ast.functions.write().unwrap();
    let function = functions
        .get_mut(&function_id)
        .and_then(|versions| versions.get_mut(&function_version))
        .unwrap();
    function.body = body;
    Ok(())
}

fn try_lower_body(
    ast: &mut Ast,
    function_id: AstFunctionId,
    variables: &ArcAstVariableMap,
    body: &[WrappedAstStatement],
    ir: &IrFunction,
    structured: &StructuredRegion,
) -> Option<Vec<WrappedAstStatement>> {
    let mut statements_by_block = statements_by_block(body, ir)?;
    let lowered = lower_region(
        ast,
        function_id,
        variables,
        structured,
        &mut statements_by_block,
        ir,
    )?;
    if statements_by_block.values().any(|stmts| !stmts.is_empty()) {
        return None;
    }
    Some(lowered)
}

fn statements_by_block(
    body: &[WrappedAstStatement],
    ir: &IrFunction,
) -> Option<HashMap<usize, Vec<WrappedAstStatement>>> {
    let mut grouped = HashMap::<usize, Vec<WrappedAstStatement>>::new();
    let ir_block_ids = ir.get_ir_block_ids();
    for statement in body.iter().cloned() {
        let AstStatementOrigin::Ir(descriptor) = &statement.origin else {
            return None;
        };
        let ir_index = descriptor.descriptor().ir_index() as usize;
        let block_id = *ir_block_ids.get(ir_index)?;
        grouped.entry(block_id).or_default().push(statement);
    }
    Some(grouped)
}

fn lower_region(
    ast: &mut Ast,
    function_id: AstFunctionId,
    variables: &ArcAstVariableMap,
    region: &StructuredRegion,
    statements_by_block: &mut HashMap<usize, Vec<WrappedAstStatement>>,
    ir: &IrFunction,
) -> Option<Vec<WrappedAstStatement>> {
    match region {
        StructuredRegion::Sequence(regions) => {
            let mut lowered = Vec::new();
            for region in regions {
                lowered.extend(lower_region(
                    ast,
                    function_id,
                    variables,
                    region,
                    statements_by_block,
                    ir,
                )?);
            }
            Some(lowered)
        }
        StructuredRegion::IfThenElse {
            head_block,
            then_region,
            else_region,
        } => lower_if_region(
            ast,
            function_id,
            variables,
            *head_block,
            then_region,
            else_region.as_deref(),
            statements_by_block,
            ir,
        ),
        StructuredRegion::Switch {
            head_block,
            cases,
            default,
        } => lower_switch_region(
            ast,
            function_id,
            variables,
            *head_block,
            cases,
            default.as_deref(),
            statements_by_block,
            ir,
        ),
        StructuredRegion::Block(block_id) => {
            Some(statements_by_block.remove(block_id).unwrap_or_default())
        }
        StructuredRegion::Goto(block_id) => Some(vec![synthetic_statement(AstStatement::Goto(
            AstJumpTarget::Unknown(synthetic_label(*block_id)),
        ))]),
        StructuredRegion::Label(block_id) => Some(vec![synthetic_statement(AstStatement::Label(
            synthetic_label(*block_id),
        ))]),
        StructuredRegion::Break => Some(vec![synthetic_statement(AstStatement::Break)]),
        StructuredRegion::Continue => Some(vec![synthetic_statement(AstStatement::Continue)]),
        StructuredRegion::While { header_block, body } => lower_loop_region(
            ast,
            function_id,
            variables,
            *header_block,
            body,
            statements_by_block,
            ir,
            false,
        ),
        StructuredRegion::DoWhile { body, latch_block } => lower_loop_region(
            ast,
            function_id,
            variables,
            *latch_block,
            body,
            statements_by_block,
            ir,
            true,
        ),
    }
}

fn lower_if_region(
    ast: &mut Ast,
    function_id: AstFunctionId,
    variables: &ArcAstVariableMap,
    head_block: usize,
    then_region: &StructuredRegion,
    else_region: Option<&StructuredRegion>,
    statements_by_block: &mut HashMap<usize, Vec<WrappedAstStatement>>,
    ir: &IrFunction,
) -> Option<Vec<WrappedAstStatement>> {
    let control_block = statements_by_block.remove(&head_block).unwrap_or_default();
    let (mut prefix, mut if_statement, suffix) = extract_if_for_structuring(control_block)?;
    abstract_condition_if_needed(ast, function_id, variables, &mut prefix, &mut if_statement)?;
    let then_body = lower_region(
        ast,
        function_id,
        variables,
        then_region,
        statements_by_block,
        ir,
    )?;
    let else_body = match else_region {
        Some(region) => Some(lower_region(
            ast,
            function_id,
            variables,
            region,
            statements_by_block,
            ir,
        )?),
        None => None,
    };

    let WrappedAstStatement {
        statement,
        origin,
        comment,
    } = if_statement;
    let AstStatement::If(condition, _, _) = statement else {
        return None;
    };
    prefix.push(WrappedAstStatement {
        statement: AstStatement::If(condition, then_body, else_body),
        origin,
        comment,
    });
    prefix.extend(suffix);
    Some(prefix)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LoopBranchRole {
    Body,
    Exit,
    Empty,
}

fn lower_loop_region(
    ast: &mut Ast,
    function_id: AstFunctionId,
    variables: &ArcAstVariableMap,
    control_block_id: usize,
    body_region: &StructuredRegion,
    statements_by_block: &mut HashMap<usize, Vec<WrappedAstStatement>>,
    ir: &IrFunction,
    is_do_while: bool,
) -> Option<Vec<WrappedAstStatement>> {
    let control_block = statements_by_block
        .remove(&control_block_id)
        .unwrap_or_default();
    let (mut prefix, if_statement, suffix) = extract_pure_if(control_block)?;
    let mut body_block_ids = collect_region_block_ids(body_region);
    body_block_ids.remove(&control_block_id);

    let WrappedAstStatement {
        statement,
        origin,
        comment,
    } = if_statement;
    let AstStatement::If(condition, true_branch, false_branch) = statement else {
        return None;
    };

    let true_role = classify_loop_branch(&true_branch, &body_block_ids, ir)?;
    let false_role =
        classify_loop_branch(false_branch.as_deref().unwrap_or(&[]), &body_block_ids, ir)?;
    let body_on_true = determine_body_on_true(true_role, false_role)?;
    let condition = if body_on_true {
        condition
    } else {
        negate_condition(condition)
    };

    if is_do_while {
        let mut body = lower_region(
            ast,
            function_id,
            variables,
            body_region,
            statements_by_block,
            ir,
        )?;
        body.extend(prefix);
        let loop_statement = WrappedAstStatement {
            statement: AstStatement::DoWhile(condition, body),
            origin,
            comment,
        };
        let mut lowered = vec![loop_statement];
        lowered.extend(suffix);
        Some(lowered)
    } else {
        let body = lower_region(
            ast,
            function_id,
            variables,
            body_region,
            statements_by_block,
            ir,
        )?;
        let loop_statement = WrappedAstStatement {
            statement: AstStatement::While(condition, body),
            origin,
            comment,
        };
        prefix.push(loop_statement);
        prefix.extend(suffix);
        Some(prefix)
    }
}

fn lower_switch_region(
    ast: &mut Ast,
    function_id: AstFunctionId,
    variables: &ArcAstVariableMap,
    head_block: usize,
    case_regions: &[(Vec<i64>, StructuredRegion)],
    default_region: Option<&StructuredRegion>,
    statements_by_block: &mut HashMap<usize, Vec<WrappedAstStatement>>,
    ir: &IrFunction,
) -> Option<Vec<WrappedAstStatement>> {
    let control_block = statements_by_block.remove(&head_block).unwrap_or_default();
    let (mut prefix, if_statement, suffix) = extract_pure_if(control_block)?;
    let WrappedAstStatement {
        statement,
        origin,
        comment,
    } = if_statement;
    let AstStatement::If(condition, _, _) = statement else {
        return None;
    };
    let selector = extract_switch_selector(condition)?;

    let mut lowered_cases = Vec::new();
    for (labels, region) in case_regions {
        if labels.is_empty() {
            return None;
        }
        let body = lower_region(ast, function_id, variables, region, statements_by_block, ir)?;
        for label in labels {
            lowered_cases.push((AstLiteral::Int(*label), body.clone()));
        }
    }
    let default_body = match default_region {
        Some(region) => Some(lower_region(
            ast,
            function_id,
            variables,
            region,
            statements_by_block,
            ir,
        )?),
        None => None,
    };

    prefix.push(WrappedAstStatement {
        statement: AstStatement::Switch(selector, lowered_cases, default_body),
        origin,
        comment,
    });
    prefix.extend(suffix);
    Some(prefix)
}

fn collect_region_block_ids(region: &StructuredRegion) -> HashSet<usize> {
    let mut block_ids = HashSet::new();
    collect_region_block_ids_into(region, &mut block_ids);
    block_ids
}

fn collect_region_block_ids_into(region: &StructuredRegion, block_ids: &mut HashSet<usize>) {
    match region {
        StructuredRegion::Sequence(regions) => {
            for region in regions {
                collect_region_block_ids_into(region, block_ids);
            }
        }
        StructuredRegion::IfThenElse {
            head_block,
            then_region,
            else_region,
        } => {
            block_ids.insert(*head_block);
            collect_region_block_ids_into(then_region, block_ids);
            if let Some(region) = else_region.as_deref() {
                collect_region_block_ids_into(region, block_ids);
            }
        }
        StructuredRegion::Switch {
            head_block,
            cases,
            default,
        } => {
            block_ids.insert(*head_block);
            for (_, region) in cases {
                collect_region_block_ids_into(region, block_ids);
            }
            if let Some(region) = default.as_deref() {
                collect_region_block_ids_into(region, block_ids);
            }
        }
        StructuredRegion::While { header_block, body } => {
            block_ids.insert(*header_block);
            collect_region_block_ids_into(body, block_ids);
        }
        StructuredRegion::DoWhile { body, latch_block } => {
            block_ids.insert(*latch_block);
            collect_region_block_ids_into(body, block_ids);
        }
        StructuredRegion::Block(block_id) => {
            block_ids.insert(*block_id);
        }
        StructuredRegion::Goto(_)
        | StructuredRegion::Label(_)
        | StructuredRegion::Break
        | StructuredRegion::Continue => {}
    }
}

fn classify_loop_branch(
    branch: &[WrappedAstStatement],
    body_block_ids: &HashSet<usize>,
    ir: &IrFunction,
) -> Option<LoopBranchRole> {
    let mut saw_body = false;
    let mut saw_exit = false;

    for statement in branch {
        match &statement.statement {
            AstStatement::Comment(_) | AstStatement::Empty => continue,
            AstStatement::Continue => saw_body = true,
            AstStatement::Break => saw_exit = true,
            _ => {
                let block_ids = origin_block_ids(&statement.origin, ir)?;
                if block_ids.is_empty() {
                    return None;
                }
                let in_body = block_ids
                    .iter()
                    .any(|block_id| body_block_ids.contains(block_id));
                let outside_body = block_ids
                    .iter()
                    .any(|block_id| !body_block_ids.contains(block_id));
                match (in_body, outside_body) {
                    (true, false) => saw_body = true,
                    (false, true) => saw_exit = true,
                    _ => return None,
                }
            }
        }

        if saw_body && saw_exit {
            return None;
        }
    }

    match (saw_body, saw_exit) {
        (true, false) => Some(LoopBranchRole::Body),
        (false, true) => Some(LoopBranchRole::Exit),
        (false, false) => Some(LoopBranchRole::Empty),
        (true, true) => None,
    }
}

fn origin_block_ids(origin: &AstStatementOrigin, ir: &IrFunction) -> Option<HashSet<usize>> {
    let mut block_ids = HashSet::new();
    collect_origin_block_ids(origin, ir, &mut block_ids)?;
    Some(block_ids)
}

fn collect_origin_block_ids(
    origin: &AstStatementOrigin,
    ir: &IrFunction,
    block_ids: &mut HashSet<usize>,
) -> Option<()> {
    match origin {
        AstStatementOrigin::Ir(descriptor) => {
            let ir_index = descriptor.descriptor().ir_index() as usize;
            block_ids.insert(*ir.get_ir_block_ids().get(ir_index)?);
            Some(())
        }
        AstStatementOrigin::Combination(origins) => {
            for origin in origins {
                collect_origin_block_ids(origin, ir, block_ids)?;
            }
            Some(())
        }
        AstStatementOrigin::Unknown
        | AstStatementOrigin::UserInput
        | AstStatementOrigin::PreDefined => None,
    }
}

fn determine_body_on_true(true_role: LoopBranchRole, false_role: LoopBranchRole) -> Option<bool> {
    match (true_role, false_role) {
        (LoopBranchRole::Body, LoopBranchRole::Exit)
        | (LoopBranchRole::Body, LoopBranchRole::Empty)
        | (LoopBranchRole::Empty, LoopBranchRole::Exit) => Some(true),
        (LoopBranchRole::Exit, LoopBranchRole::Body)
        | (LoopBranchRole::Empty, LoopBranchRole::Body)
        | (LoopBranchRole::Exit, LoopBranchRole::Empty) => Some(false),
        _ => None,
    }
}

fn negate_condition(condition: Wrapped<AstExpression>) -> Wrapped<AstExpression> {
    Wrapped {
        item: AstExpression::UnaryOp(AstUnaryOperator::Not, Box::new(condition)),
        origin: AstValueOrigin::Unknown,
        comment: None,
    }
}

fn extract_switch_selector(condition: Wrapped<AstExpression>) -> Option<Wrapped<AstExpression>> {
    let Wrapped {
        item,
        origin,
        comment,
    } = condition;
    let AstExpression::BinaryOp(operator, left, right) = item else {
        return None;
    };
    match (operator, left.item, right.item) {
        (AstBinaryOperator::Equal, selector, AstExpression::Literal(_))
        | (AstBinaryOperator::NotEqual, selector, AstExpression::Literal(_)) => Some(Wrapped {
            item: selector,
            origin,
            comment,
        }),
        (AstBinaryOperator::Equal, AstExpression::Literal(_), selector)
        | (AstBinaryOperator::NotEqual, AstExpression::Literal(_), selector) => Some(Wrapped {
            item: selector,
            origin,
            comment,
        }),
        _ => None,
    }
}

fn extract_pure_if(
    statements: Vec<WrappedAstStatement>,
) -> Option<(
    Vec<WrappedAstStatement>,
    WrappedAstStatement,
    Vec<WrappedAstStatement>,
)> {
    let meaningful_indices: Vec<usize> = statements
        .iter()
        .enumerate()
        .filter_map(|(index, statement)| is_meaningful(statement).then_some(index))
        .collect();
    if meaningful_indices.len() != 1 {
        return None;
    }
    let if_index = meaningful_indices[0];
    let candidate = &statements[if_index];
    let AstStatement::If(_, then_body, else_body) = &candidate.statement else {
        return None;
    };
    if !is_simple_control_stub_list(then_body)
        || !else_body
            .as_ref()
            .is_none_or(|branch| is_simple_control_stub_list(branch))
    {
        return None;
    }

    let mut prefix = Vec::new();
    let mut suffix = Vec::new();
    let mut extracted_if = None;
    for (index, statement) in statements.into_iter().enumerate() {
        if index < if_index {
            prefix.push(statement);
        } else if index == if_index {
            extracted_if = Some(statement);
        } else {
            suffix.push(statement);
        }
    }

    Some((prefix, extracted_if?, suffix))
}

fn extract_if_for_structuring(
    statements: Vec<WrappedAstStatement>,
) -> Option<(
    Vec<WrappedAstStatement>,
    WrappedAstStatement,
    Vec<WrappedAstStatement>,
)> {
    if let Some(extracted) = extract_pure_if(statements.clone()) {
        return Some(extracted);
    }

    let mut if_index = None;
    for (index, statement) in statements.iter().enumerate() {
        if !is_meaningful(statement) {
            continue;
        }
        if matches!(statement.statement, AstStatement::If(_, _, _)) {
            if if_index.replace(index).is_some() {
                return None;
            }
        }
    }
    let if_index = if_index?;
    let candidate = &statements[if_index];
    let AstStatement::If(_, then_body, else_body) = &candidate.statement else {
        return None;
    };
    if !is_simple_control_stub_list(then_body)
        || !else_body
            .as_ref()
            .is_none_or(|branch| is_simple_control_stub_list(branch))
    {
        return None;
    }

    if statements
        .iter()
        .enumerate()
        .any(|(index, statement)| index > if_index && is_meaningful(statement))
    {
        return None;
    }
    if statements
        .iter()
        .enumerate()
        .any(|(index, statement)| index < if_index && !is_predicate_setup_statement(statement))
    {
        return None;
    }

    let mut prefix = Vec::new();
    let mut suffix = Vec::new();
    let mut extracted_if = None;
    for (index, statement) in statements.into_iter().enumerate() {
        if index < if_index {
            prefix.push(statement);
        } else if index == if_index {
            extracted_if = Some(statement);
        } else {
            suffix.push(statement);
        }
    }

    Some((prefix, extracted_if?, suffix))
}

fn is_predicate_setup_statement(statement: &WrappedAstStatement) -> bool {
    match &statement.statement {
        AstStatement::Declaration(_, Some(init)) => {
            super::opt_utils::is_pure_expression(&init.item)
        }
        AstStatement::Assignment(lhs, rhs) => {
            matches!(lhs.item, AstExpression::Variable(_, _))
                && super::opt_utils::is_pure_expression(&rhs.item)
        }
        AstStatement::Comment(_) | AstStatement::Empty => true,
        _ => false,
    }
}

fn abstract_condition_if_needed(
    ast: &mut Ast,
    function_id: AstFunctionId,
    variables: &ArcAstVariableMap,
    prefix: &mut Vec<WrappedAstStatement>,
    if_statement: &mut WrappedAstStatement,
) -> Option<()> {
    let AstStatement::If(condition, _, _) = &mut if_statement.statement else {
        return None;
    };
    if !needs_predicate_abstraction(&condition.item) {
        return Some(());
    }
    if !super::opt_utils::is_pure_expression(&condition.item) {
        return None;
    }

    let predicate_variable_id = ast.new_variable_id(&function_id);
    let predicate_variable = AstVariable {
        name: None,
        id: predicate_variable_id,
        var_type: AstValueType::Bool,
        const_value: None,
        data_access_ir: None,
    };
    variables
        .write()
        .unwrap()
        .insert(predicate_variable_id, predicate_variable.clone());

    let predicate_initializer = condition.clone();
    prefix.push(WrappedAstStatement {
        statement: AstStatement::Declaration(predicate_variable, Some(predicate_initializer)),
        origin: AstStatementOrigin::Combination(vec![if_statement.origin.clone()]),
        comment: None,
    });
    *condition = Wrapped {
        item: AstExpression::Variable(variables.clone(), predicate_variable_id),
        origin: AstValueOrigin::Combination(vec![condition.origin.clone()]),
        comment: None,
    };
    Some(())
}

fn needs_predicate_abstraction(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Variable(_, _)
        | AstExpression::Literal(AstLiteral::Bool(_))
        | AstExpression::Literal(AstLiteral::Int(_))
        | AstExpression::Literal(AstLiteral::UInt(_)) => false,
        AstExpression::UnaryOp(AstUnaryOperator::Not, inner) => {
            needs_predicate_abstraction(&inner.item)
        }
        _ => true,
    }
}

fn is_simple_control_stub_list(statements: &[WrappedAstStatement]) -> bool {
    statements.iter().all(|statement| {
        matches!(
            statement.statement,
            AstStatement::Goto(_)
                | AstStatement::Break
                | AstStatement::Continue
                | AstStatement::Comment(_)
                | AstStatement::Empty
        )
    })
}

fn is_meaningful(statement: &WrappedAstStatement) -> bool {
    !matches!(
        statement.statement,
        AstStatement::Comment(_) | AstStatement::Empty
    )
}

fn synthetic_statement(statement: AstStatement) -> WrappedAstStatement {
    WrappedAstStatement {
        statement,
        origin: AstStatementOrigin::Unknown,
        comment: None,
    }
}

fn synthetic_label(block_id: usize) -> String {
    format!("__structured_block_{block_id}")
}
