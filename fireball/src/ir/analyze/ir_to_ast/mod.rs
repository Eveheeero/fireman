pub mod abstract_syntax_tree;
mod convert;

use crate::{
    core::Block,
    ir::{
        analyze::{
            ControlFlowGraphAnalyzer, DataType, IrFunction,
            ir_function::generate_ir_function,
            ir_to_ast::abstract_syntax_tree::{
                Ast, AstDescriptor, AstStatement, AstValue, AstValueType, AstVariable,
                AstVariableId, PrintWithConfig, Wrapped,
            },
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

/// Generate AST from targets
pub fn generate_ast(targets: impl IntoIterator<Item = Arc<Block>>) -> Result<Ast, DecompileError> {
    let mut ast = Ast::new();
    let mut cfg_analyzer = ControlFlowGraphAnalyzer::new();
    cfg_analyzer.add_targets(targets);
    let cfgs = cfg_analyzer.analyze();
    for cfg in cfgs.into_iter() {
        let merged = generate_ir_function(&cfg.get_blocks());
        generate_ast_function(&mut ast, merged)?;
    }
    Ok(ast)
}

/// Generate AST function and add it to AST
///
/// ### Arguments
/// * `ast: &mut Ast` - The AST to which the function will be added.
/// * `data: IrFunction` - The merged IR data containing the function's instructions and variables.
pub fn generate_ast_function(ast: &mut Ast, data: IrFunction) -> Result<(), DecompileError> {
    let data = Arc::new(data);
    let func_id = ast.generate_default_function(data.clone());
    let function_version = *ast.function_versions.get(&func_id).unwrap();

    let mut locals = HashMap::new();
    let mut var_map: HashMap<Aos<IrData>, AstVariableId> = HashMap::new();
    for var in data.get_variables().iter() {
        let var_id = ast.new_variable_id(&func_id);
        let c_type = match var.data_type {
            DataType::Unknown => AstValueType::Unknown,
            DataType::Bool => AstValueType::Bool,
            DataType::Int => AstValueType::Int,
            DataType::Float => AstValueType::Double,
            DataType::StringPointer => AstValueType::Pointer(Box::new(AstValueType::Char)),
            DataType::Char => AstValueType::Char,
            DataType::Address => AstValueType::Pointer(Box::new(AstValueType::Void)),
        };
        let mut const_value: Option<Wrapped<AstValue>> = None;
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
                    trace!(
                        "Constant value found in {}: {}",
                        position,
                        c.to_string_with_config(None)
                    );
                    if const_value.is_some() && const_value.as_ref().unwrap() != &c {
                        warn!(
                            "Constant value mismatch in position {}: {} != {}",
                            position,
                            const_value.unwrap().to_string_with_config(None),
                            c.to_string_with_config(None)
                        );
                    }
                    const_value = Some(c);
                }
            }
        }
        locals.insert(
            var_id,
            AstVariable {
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
        .get_mut(&function_version)
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
        if let Some(stmts) = ir.statements {
            let instruction_args = instruction.inner.arguments.as_ref();
            for (stmt_index, stmt) in stmts.iter().enumerate() {
                let stmt_index = stmt_index as u8;
                let stmt_position = AstDescriptor::new(
                    data.clone(),
                    IrStatementDescriptor::new(ir_index, Some(stmt_index)),
                );
                func_body.push(convert_stmt(
                    ast,
                    func_id,
                    function_version,
                    stmt,
                    &stmt_position,
                    None,
                    &var_map,
                    instruction_args,
                )?);
            }
        } else {
            func_body.push(ws(
                AstStatement::Assembly(instruction.inner.to_string()),
                AstDescriptor::new(data.clone(), IrStatementDescriptor::new(ir_index, None)),
            ));
        }
    }
    ast.functions
        .write()
        .unwrap()
        .get_mut(&func_id)
        .unwrap()
        .get_mut(&function_version)
        .unwrap()
        .body = func_body;

    Ok(())
}
