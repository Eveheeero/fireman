#[cfg(test)]
pub(crate) mod test_utils {
    use crate::{
        abstract_syntax_tree::*, ir::analyze::IrFunction, pattern_matching::AstPattern,
        utils::version_map::VersionMap,
    };
    use hashbrown::HashMap;
    use std::{
        path::PathBuf,
        sync::{Arc, RwLock},
    };

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

    fn pattern_file_path(relative_path: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("fireball manifest should live under the workspace root")
            .join("patterns")
            .join(relative_path)
    }

    fn file_backed_pattern(relative_path: &str) -> AstPattern {
        let path = pattern_file_path(relative_path);
        assert!(
            path.is_file(),
            "expected pattern file to exist: {}",
            path.display()
        );
        AstPattern::from_file(path.to_string_lossy().to_string())
    }

    pub fn run_parity(
        relative_path: &str,
        body: Vec<WrappedAstStatement>,
        vm: Arc<RwLock<HashMap<AstVariableId, AstVariable>>>,
        config_fn: impl Fn(AstOptimizationConfig) -> AstOptimizationConfig,
    ) -> (String, String) {
        let ast_fb = build_ast(body.clone(), vm.clone());
        let ast_embed = build_ast(body, vm);

        let fb_pattern = file_backed_pattern(relative_path);
        let fb_base = AstOptimizationConfig::NONE
            .constant_folding(true)
            .pattern_matching_enabled(true)
            .pattern_matching(vec![fb_pattern])
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

    pub fn run_direct_embedded_pass<F>(body: Vec<WrappedAstStatement>, pass: F) -> String
    where
        F: FnOnce(
            &mut Ast,
            AstFunctionId,
            AstFunctionVersion,
        ) -> Result<(), crate::prelude::DecompileError>,
    {
        let vm = Arc::new(RwLock::new(HashMap::new()));
        let fid = AstFunctionId { address: 0x9000 };
        let version = AstFunctionVersion(1);
        let mut ast = build_ast(body, vm);
        pass(&mut ast, fid, version).unwrap();
        ast.print(Some(AstPrintConfig::NONE))
    }

    pub fn assert_before_ir_suppression<F>(
        relative_path: &str,
        trigger: &str,
        retained: &str,
        pass: F,
    ) where
        F: FnOnce(
            &mut Ast,
            AstFunctionId,
            AstFunctionVersion,
        ) -> Result<(), crate::prelude::DecompileError>,
    {
        let body = vec![
            wrap_statement(AstStatement::Comment("keep-comment".to_string())),
            wrap_statement(AstStatement::Assembly(trigger.to_string())),
            wrap_statement(AstStatement::Assembly(retained.to_string())),
        ];
        let printed = run_direct_embedded_pass(body.clone(), pass);
        let fb_pattern = file_backed_pattern(relative_path);
        let ast = build_ast(body, Arc::new(RwLock::new(HashMap::new())));
        let fb_printed = ast
            .optimize(Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![fb_pattern])
                    .use_embedded_passes(false),
            ))
            .expect("file-backed suppression pattern should optimize successfully")
            .print(Some(AstPrintConfig::NONE));

        assert_eq!(
            fb_printed, printed,
            "embedded pass and `{relative_path}` should produce identical output"
        );

        assert!(
            !printed.contains(trigger),
            "suppressed assembly should be removed, but `{trigger}` remained:\n{printed}"
        );
        assert!(
            printed.contains(retained),
            "non-matching assembly should be preserved, missing `{retained}`:\n{printed}"
        );
        assert!(
            printed.contains("keep-comment"),
            "non-assembly statements should be preserved:\n{printed}"
        );
    }
}
