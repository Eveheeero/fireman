//! Module for converting analyzed loops into C loop structures

use crate::{
    core::Block,
    ir::{
        analyze::{
            MergedIr,
            ir_to_c::c_abstract_syntax_tree::{
                AstDescriptor, CAst, Expression, FunctionId, Statement, VariableId, Wrapped,
                WrappedStatement,
            },
            loop_analysis::{AnalyzedLoop, LoopPattern},
        },
        data::IrData,
        statements::IrStatement,
        utils::IrStatementDescriptor,
    },
    prelude::*,
    utils::Aos,
};
use hashbrown::HashMap;
use std::sync::Arc;

// Helper function to wrap statement
fn ws(statement: Statement, from: AstDescriptor) -> WrappedStatement {
    WrappedStatement {
        statement,
        from: Some(from),
        comment: None,
    }
}

// Helper function to wrap expression data without origin
fn wdn<T>(item: T) -> Wrapped<T> {
    Wrapped {
        item,
        origin_expr: None,
        comment: None,
    }
}

// Helper function to wrap expression data with origin
fn wd<T>(item: T, origin_expr: &Aos<IrData>) -> Wrapped<T> {
    Wrapped {
        item,
        origin_expr: Some(origin_expr.clone()),
        comment: None,
    }
}

/// Convert analyzed loops into C loop statements
pub fn convert_loops_to_c(
    ast: &mut CAst,
    function_id: FunctionId,
    analyzed_loops: &[AnalyzedLoop],
    merged_ir: &Arc<MergedIr>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Result<Vec<WrappedStatement>, DecompileError> {
    let mut result = Vec::new();

    for loop_info in analyzed_loops {
        match convert_single_loop(ast, function_id, loop_info, merged_ir, var_map)? {
            Some(stmt) => result.push(stmt),
            None => {
                // Failed to convert to structured loop, will use goto-based approach
                warn!(
                    "Failed to convert loop at {:?} to structured form",
                    loop_info.loop_info.loop_from.get_start_address()
                );
            }
        }
    }

    Ok(result)
}

/// Convert a single analyzed loop into a C loop statement
fn convert_single_loop(
    ast: &mut CAst,
    function_id: FunctionId,
    loop_info: &AnalyzedLoop,
    merged_ir: &Arc<MergedIr>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Result<Option<WrappedStatement>, DecompileError> {
    match &loop_info.pattern {
        LoopPattern::WhileLoop {
            condition_block,
            body_blocks,
        } => convert_while_loop(
            ast,
            function_id,
            condition_block,
            body_blocks,
            merged_ir,
            var_map,
        ),
        LoopPattern::DoWhileLoop {
            body_blocks,
            condition_block,
        } => convert_do_while_loop(
            ast,
            function_id,
            body_blocks,
            condition_block,
            merged_ir,
            var_map,
        ),
        LoopPattern::ForLoop {
            init_block,
            condition_block,
            increment_block,
            body_blocks,
        } => convert_for_loop(
            ast,
            function_id,
            init_block.as_ref(),
            condition_block,
            increment_block.as_ref(),
            body_blocks,
            merged_ir,
            var_map,
        ),
        LoopPattern::RangeBasedLoop { .. } => {
            // Range-based loops would need more complex transformation
            Ok(None)
        }
        LoopPattern::GenericLoop { .. } => {
            // Generic loops cannot be easily converted to structured form
            Ok(None)
        }
    }
}

/// Convert a while loop pattern
fn convert_while_loop(
    ast: &mut CAst,
    function_id: FunctionId,
    condition_block: &Arc<Block>,
    body_blocks: &[Arc<Block>],
    merged_ir: &Arc<MergedIr>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Result<Option<WrappedStatement>, DecompileError> {
    // Find the condition expression in the condition block
    let condition_expr = find_condition_expression(condition_block, merged_ir)?;

    if let Some(cond) = condition_expr {
        // Convert condition expression to a simple form
        let cond_expr = convert_irdata_to_expr(ast, function_id, &cond, var_map)?;

        // Convert body statements
        let body_stmts =
            convert_blocks_to_statements(ast, function_id, body_blocks, merged_ir, var_map)?;

        // Create while statement
        let while_stmt = Statement::While(cond_expr, body_stmts);
        let from = create_ast_descriptor(
            merged_ir.clone(),
            condition_block.get_start_address().get_virtual_address(),
        );

        Ok(Some(ws(while_stmt, from)))
    } else {
        Ok(None)
    }
}

/// Convert a do-while loop pattern
fn convert_do_while_loop(
    ast: &mut CAst,
    function_id: FunctionId,
    body_blocks: &[Arc<Block>],
    condition_block: &Arc<Block>,
    merged_ir: &Arc<MergedIr>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Result<Option<WrappedStatement>, DecompileError> {
    // For do-while, we need to transform it into a while loop with an initial execution
    // Since C AST doesn't have do-while, we'll use while(true) with a break condition

    // Convert body statements
    let mut body_stmts =
        convert_blocks_to_statements(ast, function_id, body_blocks, merged_ir, var_map)?;

    // Find and add the condition check at the end
    if let Some(cond) = find_condition_expression(condition_block, merged_ir)? {
        let cond_expr = convert_irdata_to_expr(ast, function_id, &cond, var_map)?;

        // Add a break statement if condition is false
        let break_condition = wdn(Expression::UnaryOp(
            crate::ir::analyze::ir_to_c::c_abstract_syntax_tree::UnaryOperator::Not,
            Box::new(cond_expr),
        ));

        let default_desc = create_ast_descriptor(merged_ir.clone(), 0);
        let break_stmt = Statement::If(
            break_condition,
            vec![ws(
                Statement::Comment("break".to_string()),
                default_desc.clone(),
            )],
            None,
        );

        body_stmts.push(ws(break_stmt, default_desc));
    }

    // Create while(true) loop
    let true_expr = wdn(Expression::Literal(
        crate::ir::analyze::ir_to_c::c_abstract_syntax_tree::Literal::Bool(true),
    ));

    let while_stmt = Statement::While(true_expr, body_stmts);
    let from = create_ast_descriptor(
        merged_ir.clone(),
        body_blocks[0].get_start_address().get_virtual_address(),
    );

    Ok(Some(ws(while_stmt, from)))
}

/// Convert a for loop pattern
fn convert_for_loop(
    ast: &mut CAst,
    function_id: FunctionId,
    _init_block: Option<&Arc<Block>>,
    condition_block: &Arc<Block>,
    _increment_block: Option<&Arc<Block>>,
    body_blocks: &[Arc<Block>],
    merged_ir: &Arc<MergedIr>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Result<Option<WrappedStatement>, DecompileError> {
    // For now, convert for loops as while loops
    // Full implementation would extract init and increment statements
    convert_while_loop(
        ast,
        function_id,
        condition_block,
        body_blocks,
        merged_ir,
        var_map,
    )
}

/// Find the condition expression in a condition block
fn find_condition_expression(
    block: &Arc<Block>,
    merged_ir: &Arc<MergedIr>,
) -> Result<Option<Aos<IrData>>, DecompileError> {
    // Look for a conditional jump in the block's IR statements
    // This is a simplified implementation

    // Find the IR statements for this block
    for (idx, ir) in merged_ir.get_ir().iter().enumerate() {
        if block.contains(&ir.address) {
            if let Some(stmts) = &ir.statements {
                for stmt in stmts.iter() {
                    // Look for condition statements
                    if let IrStatement::Condition { condition, .. } = stmt {
                        let resolved = crate::ir::analyze::variables::resolve_operand(
                            condition,
                            &merged_ir.get_instructions()[idx].inner.arguments,
                        );
                        return Ok(Some(resolved));
                    }
                }
            }
        }
    }

    Ok(None)
}

/// Convert blocks to C statements
fn convert_blocks_to_statements(
    ast: &mut CAst,
    function_id: FunctionId,
    blocks: &[Arc<Block>],
    merged_ir: &Arc<MergedIr>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Result<Vec<WrappedStatement>, DecompileError> {
    let mut statements = Vec::new();

    for block in blocks {
        // Find all IR statements in this block
        for (idx, ir) in merged_ir.get_ir().iter().enumerate() {
            if block.contains(&ir.address) {
                if let Some(stmts) = &ir.statements {
                    for (stmt_idx, stmt) in stmts.iter().enumerate() {
                        let desc = create_ast_descriptor_with_index(
                            merged_ir.clone(),
                            idx as u32,
                            Some(stmt_idx as u8),
                        );

                        // Convert statement to a simple form
                        let converted = convert_ir_statement_to_c(
                            ast,
                            function_id,
                            stmt,
                            desc,
                            var_map,
                            &merged_ir.get_instructions()[idx].inner.arguments,
                        )?;

                        if let Some(c_stmt) = converted {
                            statements.push(c_stmt);
                        }
                    }
                }
            }
        }
    }

    Ok(statements)
}

// Helper function to create AstDescriptor
fn create_ast_descriptor(merged_ir: Arc<MergedIr>, address: u64) -> AstDescriptor {
    // Find the IR index for this address
    for (idx, ir) in merged_ir.get_ir().iter().enumerate() {
        if ir.address.get_virtual_address() == address {
            return AstDescriptor::new(merged_ir, IrStatementDescriptor::new(idx as u32, None));
        }
    }
    // Default if not found
    AstDescriptor::new(merged_ir, IrStatementDescriptor::new(0, None))
}

// Helper function to create AstDescriptor with specific indices
fn create_ast_descriptor_with_index(
    merged_ir: Arc<MergedIr>,
    ir_index: u32,
    stmt_index: Option<u8>,
) -> AstDescriptor {
    AstDescriptor::new(merged_ir, IrStatementDescriptor::new(ir_index, stmt_index))
}

// Simple IR data to expression converter
fn convert_irdata_to_expr(
    ast: &mut CAst,
    function_id: FunctionId,
    data: &Aos<IrData>,
    var_map: &HashMap<Aos<IrData>, VariableId>,
) -> Result<Wrapped<Expression>, DecompileError> {
    // Check if it's a variable
    if let Some(&vid) = var_map.get(data) {
        let vars = ast.get_variables(&function_id)?;
        return Ok(wd(Expression::Variable(vars, vid), data));
    }

    // Handle other cases
    match data.as_ref() {
        IrData::Constant(c) => Ok(wd(
            Expression::Literal(
                crate::ir::analyze::ir_to_c::c_abstract_syntax_tree::Literal::Int(*c as i64),
            ),
            data,
        )),
        IrData::Dereference(inner) => {
            let inner_expr = convert_irdata_to_expr(ast, function_id, inner, var_map)?;
            Ok(wd(Expression::Deref(Box::new(inner_expr)), data))
        }
        _ => {
            // For complex expressions, use a placeholder
            Ok(wd(Expression::Unknown, data))
        }
    }
}

// Simple IR statement to C statement converter
fn convert_ir_statement_to_c(
    ast: &mut CAst,
    function_id: FunctionId,
    stmt: &IrStatement,
    desc: AstDescriptor,
    var_map: &HashMap<Aos<IrData>, VariableId>,
    instruction_args: &[iceball::Argument],
) -> Result<Option<WrappedStatement>, DecompileError> {
    match stmt {
        IrStatement::Assignment { from, to, .. } => {
            let from_resolved =
                crate::ir::analyze::variables::resolve_operand(from, instruction_args);
            let to_resolved = crate::ir::analyze::variables::resolve_operand(to, instruction_args);

            let from_expr = convert_irdata_to_expr(ast, function_id, &from_resolved, var_map)?;
            let to_expr = convert_irdata_to_expr(ast, function_id, &to_resolved, var_map)?;

            Ok(Some(ws(Statement::Assignment(to_expr, from_expr), desc)))
        }
        IrStatement::Condition {
            condition,
            true_branch: _,
            false_branch,
        } => {
            let cond_resolved =
                crate::ir::analyze::variables::resolve_operand(condition, instruction_args);
            let cond_expr = convert_irdata_to_expr(ast, function_id, &cond_resolved, var_map)?;

            // For simplicity, we'll convert to an if statement with empty bodies
            let then_stmts = Vec::new();
            let else_stmts = if false_branch.is_empty() {
                None
            } else {
                Some(Vec::new())
            };

            Ok(Some(ws(
                Statement::If(cond_expr, then_stmts, else_stmts),
                desc,
            )))
        }
        IrStatement::Jump { .. } | IrStatement::JumpByCall { .. } => {
            // Skip jump statements in loop bodies as they're handled by loop structure
            Ok(None)
        }
        _ => {
            // Other statements become comments for now
            Ok(Some(ws(
                Statement::Comment(format!("IR: {:?}", stmt)),
                desc,
            )))
        }
    }
}
