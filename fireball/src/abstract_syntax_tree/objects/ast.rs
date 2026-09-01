use crate::{
    abstract_syntax_tree::objects::*, core::PreDefinedOffsets, ir::analyze::IrFunction, prelude::*,
};
use hashbrown::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Ast {
    pub functions: AstFunctionMap,
    pub last_variable_id: HashMap<AstFunctionId, u32>,
    pub pre_defined_symbols: HashMap<u64, String>,
    pub comments: HashMap<AstNodeId, String>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            last_variable_id: HashMap::new(),
            pre_defined_symbols: HashMap::new(),
            comments: HashMap::new(),
        }
    }

    pub fn set_pre_defined_symbols(&mut self, symbols: Arc<PreDefinedOffsets>) {
        let reader = symbols.get_reader();
        let mut entries: Vec<_> = reader
            .iter()
            .map(|item| (item.address.get_virtual_address(), item.name.clone()))
            .collect();
        entries.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        let mut map: HashMap<u64, String> = HashMap::new();
        for (address, name) in entries {
            map.entry(address).or_insert(name);
        }
        self.pre_defined_symbols = map;
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
                for stmt in stmts.iter() {
                    body.push(Wrapped {
                        id: AstNodeId::new(),
                        item: AstStatement::Ir(Box::new((Some(ir_index), stmt.clone()))),
                    });
                }
            } else {
                body.push(Wrapped {
                    id: AstNodeId::new(),
                    item: AstStatement::Assembly(instruction.inner.to_string()),
                });
            }
        }
        let func = AstFunction {
            name: None,
            id,
            origin_ir: data,
            return_type: AstValueType::Void,
            parameters: Vec::new(),
            variables: Arc::new(RwLock::new(HashMap::new())),
            body,

            processed_optimizations: Vec::new(),
        };
        self.functions.insert(id, func);
        id
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
    ) -> Result<ArcAstVariableMap, DecompileError> {
        if let Some(function) = self.functions.get(function_id) {
            Ok(function.variables.clone())
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
    pub fn shrink(&mut self) {
        self.last_variable_id.shrink_to_fit();
        self.pre_defined_symbols.shrink_to_fit();
    }
    pub fn get_comment(&self, id: &AstNodeId) -> Option<&String> {
        self.comments.get(id)
    }
    pub fn set_comment(&mut self, id: &AstNodeId, comment: String) {
        self.comments.insert(*id, comment);
    }
}
