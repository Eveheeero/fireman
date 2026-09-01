//! Convert IR statements into high-level AST representation.

mod convert;

use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstOptimizationKind, AstStatement, AstValue, AstValueType, AstVariable,
        AstVariableId, PrintWithConfig, Wrapped,
        optimize::ir_analyzation::convert::{convert_stmt, resolve_constant},
    },
    ir::{analyze::DataType, data::IrData},
    prelude::{DecompileError, *},
    utils::Aos,
};
use hashbrown::HashMap;
use std::sync::{Arc, RwLock};

/// Generate Ast function body with given ir function
pub(super) fn analyze_ir_function(
    ast: &mut Ast,
    function_id: AstFunctionId,
) -> Result<(), DecompileError> {
    let ir_function;
    let mut body;
    {
        let function = ast.functions.get_mut(&function_id).unwrap();

        // if analyzed, pass
        if function
            .processed_optimizations
            .contains(&AstOptimizationKind::IrAnalyzation)
        {
            return Ok(());
        }

        body = std::mem::take(&mut function.body);
        ir_function = function.origin_ir.clone();
    }

    let mut locals = HashMap::new();
    let mut var_map: HashMap<Aos<IrData>, AstVariableId> = HashMap::new();
    for var in ir_function.get_variables().iter() {
        let var_id = ast.new_variable_id(&function_id);
        let mut c_type = match var.data_type {
            DataType::Unknown => AstValueType::Unknown,
            DataType::Bool => AstValueType::Bool,
            DataType::Int => AstValueType::Int,
            DataType::Float32 => AstValueType::Float,
            DataType::Float64 | DataType::Float80 => AstValueType::Double,
            DataType::StringPointer => AstValueType::Pointer(Box::new(AstValueType::Char)),
            DataType::Char => AstValueType::Char,
            DataType::Address => AstValueType::Pointer(Box::new(AstValueType::Void)),
        };
        let mut const_value: Option<Wrapped<AstValue>> = None;
        let mut accesses_by_position: Vec<_> = var.get_data_accesses().iter().collect();
        accesses_by_position.sort_unstable_by_key(|(position, _)| position.to_u64());
        for (position, accesses) in accesses_by_position {
            let instruction_arg_size = ir_function.get_instructions()[position.ir_index() as usize]
                .inner
                .arguments
                .len() as u8;
            let instruction_byte_size = ir_function.get_instructions()
                [position.ir_index() as usize]
                .inner
                .bytes
                .as_ref()
                .map(|x| x.len() as u8)
                .unwrap_or(0);
            let position = &ir_function.get_ir()[position.ir_index() as usize].address;
            for da in accesses.iter() {
                var_map.insert(da.location().clone(), var_id);
                // Resolve constant value
                if let Some(c) = resolve_constant(
                    position,
                    instruction_arg_size,
                    instruction_byte_size,
                    &da.location(),
                    &da.location(),
                )? {
                    trace!(
                        "Constant value found in {}: {}",
                        position,
                        c.to_string_with_config(ast, None)
                    );
                    if c_type == AstValueType::Unknown {
                        c_type = match &c.item {
                            AstValue::Void => AstValueType::Void,
                            AstValue::Unknown => AstValueType::Unknown,
                            AstValue::Undefined => AstValueType::Unknown,
                            AstValue::Max => AstValueType::Int,
                            AstValue::Min => AstValueType::Int,
                            AstValue::Num(_) => AstValueType::Int,
                            AstValue::Char(_) => AstValueType::Char,
                            AstValue::Double(_) => AstValueType::Double,
                            AstValue::Bool(_) => AstValueType::Bool,
                            AstValue::Pointer(_) | AstValue::Array(_) => {
                                AstValueType::Pointer(Box::new(AstValueType::Void))
                            }
                        };
                        debug!(
                            "Constant value found in {}({}) but datatype not set. init datatype to {}",
                            position,
                            c.to_string_with_config(ast, None),
                            c_type.to_string_with_config(ast, None)
                        );
                    }
                    if const_value.is_some() && const_value.as_ref().unwrap() != &c {
                        warn!(
                            "Constant value mismatch in position {}: {} != {}",
                            position,
                            const_value
                                .as_ref()
                                .unwrap()
                                .to_string_with_config(ast, None),
                            c.to_string_with_config(ast, None)
                        );
                        debug_assert!(
                            false,
                            "Constant value mismatch in position {}: {} != {}",
                            position,
                            const_value.unwrap().to_string_with_config(ast, None),
                            c.to_string_with_config(ast, None)
                        );
                    }
                    const_value = Some(c);
                }
            }
        }
        locals.insert(
            var_id,
            AstVariable {
                name: None,
                id: var_id,
                var_type: c_type,
                const_value,
                data_access_ir: Some(var.get_data_accesses().clone()),
            },
        );
    }
    ast.functions.get_mut(&function_id).unwrap().variables = Arc::new(RwLock::new(locals));

    let map = ir_function.get_instructions().as_ref();
    for ws in &mut body {
        // skip if not analyzable
        let AstStatement::Ir(stmt) = &ws.item else {
            continue;
        };
        // skip if ir not generated by instruction
        let Some(ir_index) = &stmt.0 else {
            continue;
        };

        let instruction = &map[usize::try_from(*ir_index).unwrap()];
        let instruction_args = &instruction.inner.arguments;
        /* analyze and turn into ast */
        let stmt = convert_stmt(ast, function_id, &stmt.1, &var_map, instruction_args)?;
        *ws = stmt;
    }

    {
        let function = ast.functions.get_mut(&function_id).unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(AstOptimizationKind::IrAnalyzation);
    }
    Ok(())
}
