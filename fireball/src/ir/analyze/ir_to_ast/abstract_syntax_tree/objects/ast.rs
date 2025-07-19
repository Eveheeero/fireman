use crate::{
    ir::{
        analyze::{
            IrFunction,
            ir_to_ast::{
                abstract_syntax_tree::objects::*,
                ws,
            },
        },
        utils::IrStatementDescriptor,
    },
    prelude::*,
    utils::version_map::VersionMap,
};
use hashbrown::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Ast {
    pub function_versions: HashMap<AstFunctionId, AstFunctionVersion>,
    pub functions: ArcAstFunctionMap,
    pub last_variable_id: HashMap<AstFunctionId, u32>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            function_versions: HashMap::new(),
            functions: Arc::new(RwLock::new(HashMap::new())),
            last_variable_id: HashMap::new(),
        }
    }

    /// 1. generate default function
    /// 2. set ast to pointing that version
    pub fn generate_default_function(&mut self, data: Arc<IrFunction>) -> AstFunctionId {
        let start_address = data.get_ir().first().map(|x| &x.address).unwrap();
        let id = AstFunctionId {
            address: start_address.get_virtual_address(),
        };
        let mut body = Vec::new();
        for (ir_index, (ir, instruction)) in data
            .get_ir()
            .iter()
            .zip(data.get_instructions().iter())
            .enumerate()
        {
            let ir_index = ir_index as u32;
            if let Some(stmts) = ir.statements {
                for (stmt_index, stmt) in stmts.iter().enumerate() {
                    let stmt_index = stmt_index as u8;
                    let stmt_position = AstDescriptor::new(
                        data.clone(),
                        IrStatementDescriptor::new(ir_index, Some(stmt_index)),
                    );
                    body.push(ws(AstStatement::Ir(Box::new(stmt.clone())), stmt_position));
                }
            } else {
                body.push(ws(
                    AstStatement::Assembly(instruction.inner.to_string()),
                    AstDescriptor::new(data.clone(), IrStatementDescriptor::new(ir_index, None)),
                ));
            }
        }
        let func = AstFunction {
            name: None,
            id,
            ir: data,
            return_type: AstValueType::Void,
            parameters: Vec::new(),
            variables: Arc::new(RwLock::new(HashMap::new())),
            body,

            processed_optimizations: Vec::new(),
        };
        self.functions
            .write()
            .unwrap()
            .insert(id, VersionMap::new(AstFunctionVersion(1), func));
        self.function_versions.insert(id, AstFunctionVersion(1));
        id
    }
    /// clone function and get cloned function version
    pub fn clone_function(
        &mut self,
        id: &AstFunctionId,
        from_version: &AstFunctionVersion,
    ) -> Option<AstFunctionVersion> {
        let mut functions = self.functions.write().unwrap();
        let function = functions
            .get(id)
            .and_then(|x| x.get(from_version))
            .cloned()?;
        let version_map = functions.get_mut(&function.id).unwrap();
        let new_version = AstFunctionVersion(version_map.last_version().0 + 1);

        self.function_versions.insert(function.id, new_version);
        version_map.insert(new_version, function).unwrap();
        Some(new_version)
    }
    pub fn new_variable_id(&mut self, current_function: &AstFunctionId) -> AstVariableId {
        let last_index = self.last_variable_id.entry(*current_function).or_insert(0);
        *last_index += 1;
        AstVariableId {
            index: *last_index,
            parent: Some(*current_function),
        }
    }
    pub fn get_variables(
        &self,
        function_id: &AstFunctionId,
        _function_version: &AstFunctionVersion,
    ) -> Result<ArcAstVariableMap, DecompileError> {
        if let Some(version_map) = self.functions.read().unwrap().get(function_id) {
            // get any version of function because all function with same id has same variable map
            let func = version_map.get_last_version();
            Ok(func.variables.clone())
        } else {
            error!(
                "Tried to get variables from a non-existing function: {:?}",
                function_id
            );
            Err(DecompileError::Unknown(Some(
                "Tried to get variables from a non-existing function".to_string(),
            )))
        }
    }
}
