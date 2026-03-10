#[cfg(test)]
pub(crate) mod test_utils {
    use crate::{abstract_syntax_tree::*, ir::analyze::IrFunction, utils::version_map::VersionMap};
    use hashbrown::HashMap;
    use std::sync::{Arc, RwLock};

    pub fn wrap_expression(item: AstExpression) -> Wrapped<AstExpression> {
        Wrapped {
            item,
            origin: AstValueOrigin::Unknown,
            comment: None,
        }
    }

    pub fn wrap_statement(statement: AstStatement) -> WrappedAstStatement {
        WrappedAstStatement {
            statement,
            origin: AstStatementOrigin::Unknown,
            comment: None,
        }
    }

    pub fn make_var_map(
        function_id: AstFunctionId,
        names: &[&str],
    ) -> (
        Vec<AstVariableId>,
        Arc<RwLock<HashMap<AstVariableId, AstVariable>>>,
    ) {
        let mut ids = Vec::new();
        let mut map = HashMap::new();
        for (i, name) in names.iter().enumerate() {
            let id = AstVariableId {
                index: (i + 1) as u32,
                parent: Some(function_id),
            };
            ids.push(id);
            map.insert(
                id,
                AstVariable {
                    name: Some(name.to_string()),
                    id,
                    var_type: AstValueType::Int,
                    const_value: None,
                    data_access_ir: None,
                },
            );
        }
        (ids, Arc::new(RwLock::new(map)))
    }

    fn build_test_function(
        function_id: AstFunctionId,
        body: Vec<WrappedAstStatement>,
        variables: Arc<RwLock<HashMap<AstVariableId, AstVariable>>>,
    ) -> AstFunction {
        let instructions: Arc<[crate::core::Instruction]> = Vec::new().into();
        let ir = Arc::new(IrFunction::new(instructions, Vec::new(), Vec::new()));
        AstFunction {
            name: Some("test_fn".to_string()),
            id: function_id,
            ir,
            return_type: AstValueType::Int,
            parameters: Vec::new(),
            variables,
            body,
            processed_optimizations: Vec::new(),
        }
    }

    fn build_ast(
        body: Vec<WrappedAstStatement>,
        vm: Arc<RwLock<HashMap<AstVariableId, AstVariable>>>,
    ) -> Ast {
        let fid = AstFunctionId { address: 0x9000 };
        let version = AstFunctionVersion(1);
        let function = build_test_function(fid, body, vm);
        let mut functions = HashMap::new();
        functions.insert(fid, VersionMap::new(version, function));
        Ast {
            function_versions: HashMap::from([(fid, version)]),
            functions: Arc::new(RwLock::new(functions)),
            last_variable_id: HashMap::new(),
            pre_defined_symbols: HashMap::new(),
        }
    }

    pub fn run_parity(
        body: Vec<WrappedAstStatement>,
        vm: Arc<RwLock<HashMap<AstVariableId, AstVariable>>>,
        config_fn: impl Fn(AstOptimizationConfig) -> AstOptimizationConfig,
    ) -> (String, String) {
        let ast_fb = build_ast(body.clone(), vm.clone());
        let ast_embed = build_ast(body, vm);

        let fb_base = AstOptimizationConfig::NONE
            .constant_folding(true)
            .pattern_matching_enabled(true)
            .use_embedded_passes(false);
        let embed_base = AstOptimizationConfig::NONE
            .constant_folding(true)
            .use_embedded_passes(true);

        let fb_config = config_fn(fb_base);
        let embed_config = config_fn(embed_base);

        let fb_result = ast_fb.optimize(Some(fb_config)).unwrap();
        let embed_result = ast_embed.optimize(Some(embed_config)).unwrap();

        let print_cfg = Some(AstPrintConfig::NONE);
        (fb_result.print(print_cfg), embed_result.print(print_cfg))
    }
}
