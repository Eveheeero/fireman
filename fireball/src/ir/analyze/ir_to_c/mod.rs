pub mod c_abstract_syntax_tree;
mod convert;

use crate::{
    core::Block,
    ir::{
        analyze::{
            ir_block_merger::merge_blocks,
            ir_to_c::c_abstract_syntax_tree::{CAst, CType, Statement, Variable, VariableId},
            ControlFlowGraphAnalyzer, DataType, MergedIr,
        },
        data::IrData,
        utils::IrStatementDescriptor,
    },
    prelude::*,
    utils::Aos,
};
use convert::*;
use hashbrown::HashMap;
use std::sync::{Arc, RwLock};

/// Generate C AST from targets
pub fn generate_c_ast(
    targets: impl IntoIterator<Item = Arc<Block>>,
) -> Result<CAst, DecompileError> {
    let mut ast = CAst::new();
    let mut cfg_analyzer = ControlFlowGraphAnalyzer::new();
    cfg_analyzer.add_targets(targets);
    let cfgs = cfg_analyzer.analyze();
    for cfg in cfgs.into_iter() {
        let merged = merge_blocks(&cfg.get_blocks());
        generate_c_ast_function(&mut ast, &merged)?;
    }
    Ok(ast)
}

/// Generate C function and add it to AST
///
/// ```rust, ignore
/// let mut ast = fireball::ir::analyze::ir_to_c::c_abstract_syntax_tree::CAst::new();
/// let merged = fireball::ir::analyze::ir_block_merger::merge_blocks(want_to_merge);
/// generate_c_function(&mut ast, &merged);
/// ```
///
/// ### Arguments
/// * `ast: &mut CAst` - The C AST to which the function will be added.
/// * `data: &MergedIr` - The merged IR data containing the function's instructions and variables.
pub fn generate_c_ast_function(ast: &mut CAst, data: &MergedIr) -> Result<(), DecompileError> {
    let func_id = ast.generate_default_function(data.get_ir().first().map(|x| &x.address).unwrap());

    let mut locals = HashMap::new();
    let mut var_map: HashMap<Aos<IrData>, VariableId> = HashMap::new();
    for var in data.get_variables().iter() {
        let var_id = ast.new_variable_id(&func_id);
        let c_type = match var.data_type {
            DataType::Unknown => CType::Unknown,
            DataType::Int => CType::Int,
            DataType::Float => CType::Double,
            DataType::StringPointer => CType::Pointer(Box::new(CType::Char)),
            DataType::Char => CType::Char,
            DataType::Address => CType::Pointer(Box::new(CType::Void)),
        };
        let mut const_value = None;
        for (position, accesses) in var.get_data_accesses().iter() {
            let instruction_arg_size = data.get_instructions()[position.ir_index() as usize]
                .inner
                .arguments
                .len() as u8;
            let position = &data.get_ir()[position.ir_index() as usize].address;
            for da in accesses.iter() {
                var_map.insert(da.location().clone(), var_id);
                // Resolve constant value
                if let Some(c) = resolve_constant(
                    position,
                    instruction_arg_size,
                    &da.location(),
                    &da.location(),
                )? {
                    trace!("Constant value found in {}: {}", position, c);
                    if const_value.is_some() && const_value.as_ref().unwrap() != &c {
                        warn!(
                            "Constant value mismatch in position {}: {} != {}",
                            position,
                            const_value.unwrap(),
                            c
                        );
                    }
                    const_value = Some(c);
                }
            }
        }
        locals.insert(
            var_id,
            Variable {
                name: var_id.get_default_name(),
                id: var_id,
                var_type: c_type,
                const_value,
            },
        );
    }
    ast.functions
        .write()
        .unwrap()
        .get_mut(&func_id)
        .unwrap()
        .variables = Arc::new(RwLock::new(locals));

    let mut func_body = Vec::new();
    for (ir_index, (ir, instruction)) in data
        .get_ir()
        .iter()
        .zip(data.get_instructions().iter())
        .enumerate()
    {
        let ir_index = ir_index as u32;
        func_body.push(ws(
            Statement::Comment(instruction.to_string()),
            IrStatementDescriptor::new(ir_index, None),
        ));
        if let Some(stmts) = ir.statements {
            let instruction_args = instruction.inner.arguments.as_ref();
            for (stmt_index, stmt) in stmts.iter().enumerate() {
                let stmt_index = stmt_index as u8;
                let stmt_position = IrStatementDescriptor::new(ir_index, Some(stmt_index));
                func_body.push(ws(Statement::Comment(stmt.to_string()), stmt_position));
                func_body.push(convert_stmt(
                    ast,
                    func_id,
                    stmt,
                    &stmt_position,
                    None,
                    &var_map,
                    instruction_args,
                )?);
            }
        } else {
            func_body.push(ws(
                Statement::Assembly(instruction.inner.to_string()),
                IrStatementDescriptor::new(ir_index, None),
            ));
        }
    }
    ast.functions
        .write()
        .unwrap()
        .get_mut(&func_id)
        .unwrap()
        .body = func_body;

    Ok(())
}
