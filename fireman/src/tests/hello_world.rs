use fireball::{
    abstract_syntax_tree::AstPrintConfig,
    core::{Fire, FireRaw},
    ir::analyze::generate_ast_with_pre_defined_symbols,
    pe::Pe,
    utils::test_log_subscriber_with_file,
};
use std::path::Path;
use tracing::Dispatch;

fn get_binary() -> &'static [u8] {
    include_bytes!("../../../fireball/tests/resources/hello_world.exe")
}

#[test]
fn hello_world() {
    let subscriber = test_log_subscriber_with_file("logs/fireman_hello_world.log");

    tracing::dispatcher::with_default(&Dispatch::new(subscriber), || {
        let binary = get_binary();

        let pe = Pe::from_binary(binary.to_vec()).unwrap();
        let result = pe.decompile_all().unwrap();
        println!("{}", &result);
        let file = Path::new("logs/hello_world_result.log");
        std::fs::create_dir_all(file.parent().unwrap()).unwrap();
        std::fs::write(file, &result).unwrap();
        assert!(true);
    });
}

#[test]
fn hello_world_all_print_config_trace() {
    let subscriber = test_log_subscriber_with_file("logs/fireman_hello_world_all_print_config.log");

    tracing::dispatcher::with_default(&Dispatch::new(subscriber), || {
        let binary = get_binary();

        let pe = Pe::from_binary(binary.to_vec()).unwrap();
        let ast = generate_ast_with_pre_defined_symbols(
            pe.analyze_all().unwrap(),
            pe.get_pre_defined_symbol_map(),
        )
        .unwrap()
        .optimize(None)
        .unwrap();
        let result = ast.print(Some(AstPrintConfig::ALL));
        println!("{}", &result);

        let file = Path::new("logs/hello_world_result_all_true.log");
        std::fs::create_dir_all(file.parent().unwrap()).unwrap();
        std::fs::write(file, &result).unwrap();

        assert!(
            result.contains("// 0x"),
            "print_instruction should be enabled"
        );
        assert!(
            result.contains("/* call o1 */"),
            "print_ir should be enabled"
        );
        assert!(
            result.contains("/* param "),
            "parameter_usage_comment should be emitted when ALL config is used"
        );
        assert!(
            result.contains("/* range "),
            "variable_usage_comment should be emitted when ALL config is used"
        );
        assert!(
            result.contains("access r:"),
            "variable usage trace should include read/write summary"
        );
        let has_main_definition = ["void main(", "int main(", "unknown_t main("]
            .iter()
            .any(|pattern| result.contains(pattern));
        assert!(
            has_main_definition,
            "CRT startup special-case should expose real main function name"
        );
        assert!(
            result.contains("__tmainCRTStartup"),
            "CRT startup function should be recognized in special-case output"
        );

        let startup_start = result
            .find("void __tmainCRTStartup(")
            .expect("CRT startup function definition must exist");
        let startup_suffix = &result[startup_start..];
        let main_call_pos = startup_suffix
            .find("\n    main(")
            .expect("CRT startup should call real main");
        let termination_call_pos = startup_suffix
            .find("\n    ext_msvcrt_dll___cexit(")
            .or_else(|| startup_suffix.find("\n    ext_msvcrt_dll__exit("))
            .expect("CRT startup should contain termination call");
        assert!(
            main_call_pos < termination_call_pos,
            "real main call should happen before termination calls in CRT startup"
        );
    });
}
