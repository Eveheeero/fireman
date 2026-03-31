pub mod core;
mod license;
pub mod model;
pub mod prelude;
#[cfg(test)]
mod tests;
mod tui;
pub mod utils;
pub mod worker;

use crate::model::{
    OptimizationScriptPreset, OptimizationSettings, OptimizationStore, build_optimization_config,
};
use clap::{Arg, ArgAction, ArgMatches, Command, builder::BoolishValueParser};
use fireball::{
    Fireball,
    abstract_syntax_tree::AstPrintConfig,
    core::{Block, FireRaw},
    ir::analyze::generate_ast,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashSet, VecDeque},
    fs,
    path::Path,
    sync::Arc,
};

const OPTIMIZATION_ARG_IDS: [&str; 33] = [
    "ir-analyzation",
    "parameter-analyzation",
    "call-argument-analyzation",
    "constant-folding",
    "control-flow-cleanup",
    "collapse-unused-variable",
    "dead-store-elimination",
    "pattern-matching-enabled",
    "loop-analyzation",
    "copy-propagation",
    "expression-inlining",
    "ternary-recovery",
    "boolean-recovery",
    "switch-reconstruction",
    "lifetime-scoping",
    "signedness-inference",
    "name-recovery",
    "early-return-normalization",
    "use-embedded-passes",
    "operator-canonicalization",
    "magic-division-recovery",
    "identity-simplification",
    "bit-trick-recognition",
    "cast-minimization",
    "assertion-recovery",
    "do-while-recovery",
    "clamp-recovery",
    "loop-cleanup",
    "if-conversion-reversal",
    "anti-debug-ast-suppression",
    "logging-suppression",
    "static-guard-suppression",
    "security-scaffold-suppression",
];

#[derive(Clone, Debug, Serialize, Deserialize)]
struct FiremanConfig {
    path: String,
    output: Option<String>,
    print: PrintConfig,
    optimization_store: OptimizationStore,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PrintConfig {
    print_instruction: bool,
    print_ir: bool,
    print_empty_statement: bool,
    replace_constant: bool,
    parameter_usage_comment: bool,
    variable_usage_comment: bool,
    hide_unused_declarations: bool,
}

#[derive(Clone, Debug)]
struct ResolvedConfig {
    input_path: Option<String>,
    output_path: Option<String>,
    print_config: PrintConfig,
    optimization_store: OptimizationStore,
}

impl Default for PrintConfig {
    fn default() -> Self {
        Self::from_ast_config(AstPrintConfig::DEFAULT)
    }
}

impl PrintConfig {
    fn from_ast_config(config: AstPrintConfig) -> Self {
        Self {
            print_instruction: config.print_instruction,
            print_ir: config.print_ir,
            print_empty_statement: config.print_empty_statement,
            replace_constant: config.replace_constant,
            parameter_usage_comment: config.parameter_usage_comment,
            variable_usage_comment: config.variable_usage_comment,
            hide_unused_declarations: config.hide_unused_declarations,
        }
    }

    fn into_ast_print_config(self) -> AstPrintConfig {
        AstPrintConfig {
            print_instruction: self.print_instruction,
            print_ir: self.print_ir,
            print_empty_statement: self.print_empty_statement,
            replace_constant: self.replace_constant,
            parameter_usage_comment: self.parameter_usage_comment,
            variable_usage_comment: self.variable_usage_comment,
            hide_unused_declarations: self.hide_unused_declarations,
        }
    }
}

impl FiremanConfig {
    fn sample() -> Self {
        let script_preset = OptimizationScriptPreset {
            name: "sample-script".to_owned(),
            path: "./path/to/sample.fb".to_owned(),
            enabled: true,
            applied_enabled: false,
        };
        let mut sample_store = OptimizationStore {
            script_presets: vec![script_preset],
            editor_buffer: "// sample inline script".to_owned(),
            editor_path: Some("./path/to/editor.fb".to_owned()),
            applied_buffer_script: Some("// applied buffer script".to_owned()),
            ..Default::default()
        };
        sample_store.draft_settings = OptimizationSettings::default();
        sample_store.applied_settings = OptimizationSettings::default();
        Self {
            path: "./path/to/target.exe".to_owned(),
            output: Some("./path/to/output.txt".to_owned()),
            print: PrintConfig::from_ast_config(AstPrintConfig::ALL),
            optimization_store: sample_store,
        }
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = parse_arg();
    if args.get_one::<bool>("license").copied().unwrap_or(false) {
        print!("{}", license::format_license_text("Fireman decompiler"));
        return Ok(());
    }
    if let Some(json_sample) = args.get_one::<String>("jsonsample") {
        save_json_sample(Path::new(json_sample))?;
        return Ok(());
    }

    let (resolved_config, has_json_source) = resolve_config(&args)?;

    if args.get_one::<bool>("tui").copied().unwrap_or(false) {
        tui::main(build_tui_startup_config(
            &args,
            resolved_config,
            has_json_source,
        ));
        return Ok(());
    }

    let input_path = resolved_config
        .input_path
        .ok_or_else(|| "target path is required".to_owned())?;
    let output_path = resolved_config.output_path;
    let print_config = resolved_config.print_config;
    let optimization_store = resolved_config.optimization_store;
    let script_paths = collect_applied_script_paths(&optimization_store);
    let buffer_script = optimization_store.applied_buffer_script.clone();

    let fireball = Fireball::from_path(&input_path).map_err(|error| error.to_string())?;
    let blocks = match fireball.analyze_from_entry() {
        Ok(entry) => collect_blocks(&fireball, entry)?,
        Err(fireball::DecompileError::NoEntryPoint) => {
            eprintln!("No entry point found — analyzing all exported functions");
            fireball.analyze_all().map_err(|error| error.to_string())?
        }
        Err(error) => return Err(error.to_string()),
    };
    let optimization_config = build_optimization_config(
        &optimization_store.applied_settings,
        &script_paths,
        buffer_script.as_deref(),
    )?;
    let ast = generate_ast(blocks).map_err(|error| error.to_string())?;
    let optimized = ast
        .optimize(Some(optimization_config))
        .map_err(|error| error.to_string())?;
    let printed = optimized.print(Some(print_config.into_ast_print_config()));

    if let Some(path) = output_path.as_deref() {
        if let Some(parent) = Path::new(path)
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        fs::write(path, &printed).map_err(|error| error.to_string())?;
    }

    println!("{printed}");
    Ok(())
}

fn resolve_config(args: &ArgMatches) -> Result<(ResolvedConfig, bool), String> {
    let json_config = args
        .get_one::<String>("json")
        .map(|path| load_json_config(Path::new(path)))
        .transpose()?;
    let has_json_source = json_config.is_some();

    let input_path = args
        .get_one::<String>("input-path")
        .cloned()
        .or_else(|| json_config.as_ref().map(|config| config.path.clone()));
    let output_path = args.get_one::<String>("output-path").cloned().or_else(|| {
        json_config
            .as_ref()
            .and_then(|config| config.output.clone())
    });

    let mut print_config = json_config
        .as_ref()
        .map(|config| config.print.clone())
        .unwrap_or_default();
    apply_print_overrides(args, &mut print_config);

    let mut optimization_store = json_config
        .as_ref()
        .map(|config| config.optimization_store.clone())
        .unwrap_or_else(Default::default);
    apply_optimization_overrides(args, &mut optimization_store.draft_settings);
    apply_optimization_overrides(args, &mut optimization_store.applied_settings);
    apply_script_overrides(args, &mut optimization_store);

    Ok((
        ResolvedConfig {
            input_path,
            output_path,
            print_config,
            optimization_store,
        },
        has_json_source,
    ))
}

fn build_tui_startup_config(
    args: &ArgMatches,
    resolved_config: ResolvedConfig,
    has_json_source: bool,
) -> Option<tui::StartupConfig> {
    let has_input_path = resolved_config.input_path.is_some();
    let has_optimization_store = has_json_source || has_tui_optimization_overrides(args);

    if !has_input_path && !has_optimization_store {
        return None;
    }

    Some(tui::StartupConfig {
        input_path: resolved_config.input_path,
        optimization_store: has_optimization_store.then_some(resolved_config.optimization_store),
    })
}

fn parse_arg() -> ArgMatches {
    Command::new("fireman")
        .about("Fireman decompiler CLI")
        .author("Eveheeero, xhve00000@gmail.com")
        .version(env!("CARGO_PKG_VERSION"))
        .args([
            Arg::new("license")
                .long("license")
                .action(ArgAction::SetTrue)
                .help("Print license information and exit"),
            Arg::new("tui")
                .long("tui")
                .action(ArgAction::SetTrue)
                .help("Run in TUI mode"),
            Arg::new("json")
                .short('j')
                .long("json")
                .value_name("PATH")
                .action(ArgAction::Set)
                .help("Load configuration from JSON"),
            Arg::new("jsonsample")
                .long("jsonsample")
                .value_name("PATH")
                .action(ArgAction::Set)
                .help("Emit a configuration sample to PATH"),
            Arg::new("script")
                .long("script")
                .value_name("PATH")
                .action(ArgAction::Append)
                .help("Enable a script file (.fb)"),
            Arg::new("script-buffer")
                .long("script-buffer")
                .value_name("SCRIPT")
                .action(ArgAction::Set)
                .help("Apply an inline optimization script"),
            Arg::new("input-path")
                .short('i')
                .long("path")
                .value_name("TARGET")
                .action(ArgAction::Set)
                .required_unless_present_any(["tui", "json", "jsonsample", "license"])
                .help("Binary to decompile"),
            Arg::new("output-path")
                .short('o')
                .long("out")
                .value_name("PATH")
                .action(ArgAction::Set)
                .help("Write the printed AST to PATH"),
            Arg::new("max-pass-iterations")
                .long("max-pass-iterations")
                .value_name("NUMBER")
                .value_parser(clap::value_parser!(usize))
                .action(ArgAction::Set)
                .help("Limit optimization passes"),
        ])
        .args(&print_args())
        .args(&optimization_args())
        .get_matches()
}

fn print_args() -> [Arg; 7] {
    [
        Arg::new("print-instruction")
            .long("print-instruction")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Print instruction comments"),
        Arg::new("print-ir")
            .long("print-ir")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Include IR statements"),
        Arg::new("print-empty-statement")
            .long("print-empty-statement")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Print empty statements"),
        Arg::new("replace-constant")
            .long("replace-constant")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Replace literal constants with inferred values"),
        Arg::new("parameter-usage-comment")
            .long("parameter-usage-comment")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Emit parameter usage hints"),
        Arg::new("variable-usage-comment")
            .long("variable-usage-comment")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Emit variable usage hints"),
        Arg::new("hide-unused-declarations")
            .long("hide-unused-declarations")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Hide declarations not used in AST"),
    ]
}

fn optimization_args() -> [Arg; 33] {
    [
        Arg::new("ir-analyzation")
            .long("ir-analyzation")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle IR analyzation"),
        Arg::new("parameter-analyzation")
            .long("parameter-analyzation")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle parameter analyzation"),
        Arg::new("call-argument-analyzation")
            .long("call-argument-analyzation")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle call argument analyzation"),
        Arg::new("constant-folding")
            .long("constant-folding")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle constant folding"),
        Arg::new("control-flow-cleanup")
            .long("control-flow-cleanup")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle control flow cleanup"),
        Arg::new("collapse-unused-variable")
            .long("collapse-unused-variable")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Collapse unused variables"),
        Arg::new("dead-store-elimination")
            .long("dead-store-elimination")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle dead store elimination"),
        Arg::new("pattern-matching-enabled")
            .long("pattern-matching-enabled")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Enable pattern matching"),
        Arg::new("loop-analyzation")
            .long("loop-analyzation")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle loop analyzation"),
        Arg::new("copy-propagation")
            .long("copy-propagation")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle copy propagation"),
        Arg::new("expression-inlining")
            .long("expression-inlining")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle expression inlining"),
        Arg::new("ternary-recovery")
            .long("ternary-recovery")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle ternary recovery"),
        Arg::new("boolean-recovery")
            .long("boolean-recovery")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle boolean recovery"),
        Arg::new("switch-reconstruction")
            .long("switch-reconstruction")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle switch reconstruction"),
        Arg::new("lifetime-scoping")
            .long("lifetime-scoping")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle lifetime scoping"),
        Arg::new("signedness-inference")
            .long("signedness-inference")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle signedness inference"),
        Arg::new("name-recovery")
            .long("name-recovery")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle name recovery"),
        Arg::new("early-return-normalization")
            .long("early-return-normalization")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle early return normalization"),
        Arg::new("use-embedded-passes")
            .long("use-embedded-passes")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle embedded passes"),
        Arg::new("operator-canonicalization")
            .long("operator-canonicalization")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle operator canonicalization"),
        Arg::new("magic-division-recovery")
            .long("magic-division-recovery")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle magic division recovery"),
        Arg::new("identity-simplification")
            .long("identity-simplification")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle identity simplification"),
        Arg::new("bit-trick-recognition")
            .long("bit-trick-recognition")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle bit trick recognition"),
        Arg::new("cast-minimization")
            .long("cast-minimization")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle cast minimization"),
        Arg::new("assertion-recovery")
            .long("assertion-recovery")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle assertion recovery"),
        Arg::new("do-while-recovery")
            .long("do-while-recovery")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle do-while recovery"),
        Arg::new("clamp-recovery")
            .long("clamp-recovery")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle clamp recovery"),
        Arg::new("loop-cleanup")
            .long("loop-cleanup")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle loop cleanup"),
        Arg::new("if-conversion-reversal")
            .long("if-conversion-reversal")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle if-conversion reversal"),
        Arg::new("anti-debug-ast-suppression")
            .long("anti-debug-ast-suppression")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle anti-debug AST suppression"),
        Arg::new("logging-suppression")
            .long("logging-suppression")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle logging suppression"),
        Arg::new("static-guard-suppression")
            .long("static-guard-suppression")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle static guard suppression"),
        Arg::new("security-scaffold-suppression")
            .long("security-scaffold-suppression")
            .value_name("BOOL")
            .value_parser(BoolishValueParser::new())
            .action(ArgAction::Set)
            .help("Toggle security scaffold suppression"),
    ]
}

fn apply_print_overrides(matches: &ArgMatches, config: &mut PrintConfig) {
    override_bool(matches, "print-instruction", |value| {
        config.print_instruction = value
    });
    override_bool(matches, "print-ir", |value| config.print_ir = value);
    override_bool(matches, "print-empty-statement", |value| {
        config.print_empty_statement = value
    });
    override_bool(matches, "replace-constant", |value| {
        config.replace_constant = value
    });
    override_bool(matches, "parameter-usage-comment", |value| {
        config.parameter_usage_comment = value
    });
    override_bool(matches, "variable-usage-comment", |value| {
        config.variable_usage_comment = value
    });
    override_bool(matches, "hide-unused-declarations", |value| {
        config.hide_unused_declarations = value
    });
}

fn apply_optimization_overrides(matches: &ArgMatches, settings: &mut OptimizationSettings) {
    macro_rules! opt_field {
        ($id:expr, $field:ident) => {
            override_bool(matches, $id, |value| settings.$field = value);
        };
    }

    opt_field!("ir-analyzation", ir_analyzation);
    opt_field!("parameter-analyzation", parameter_analyzation);
    opt_field!("call-argument-analyzation", call_argument_analyzation);
    opt_field!("constant-folding", constant_folding);
    opt_field!("control-flow-cleanup", control_flow_cleanup);
    opt_field!("collapse-unused-variable", collapse_unused_varaible);
    opt_field!("dead-store-elimination", dead_store_elimination);
    opt_field!("pattern-matching-enabled", pattern_matching_enabled);
    opt_field!("loop-analyzation", loop_analyzation);
    opt_field!("copy-propagation", copy_propagation);
    opt_field!("expression-inlining", expression_inlining);
    opt_field!("ternary-recovery", ternary_recovery);
    opt_field!("boolean-recovery", boolean_recovery);
    opt_field!("switch-reconstruction", switch_reconstruction);
    opt_field!("lifetime-scoping", lifetime_scoping);
    opt_field!("signedness-inference", signedness_inference);
    opt_field!("name-recovery", name_recovery);
    opt_field!("early-return-normalization", early_return_normalization);
    opt_field!("use-embedded-passes", use_embedded_passes);
    opt_field!("operator-canonicalization", operator_canonicalization);
    opt_field!("magic-division-recovery", magic_division_recovery);
    opt_field!("identity-simplification", identity_simplification);
    opt_field!("bit-trick-recognition", bit_trick_recognition);
    opt_field!("cast-minimization", cast_minimization);
    opt_field!("assertion-recovery", assertion_recovery);
    opt_field!("do-while-recovery", do_while_recovery);
    opt_field!("clamp-recovery", clamp_recovery);
    opt_field!("loop-cleanup", loop_cleanup);
    opt_field!("if-conversion-reversal", if_conversion_reversal);
    opt_field!("anti-debug-ast-suppression", anti_debug_ast_suppression);
    opt_field!("logging-suppression", logging_suppression);
    opt_field!("static-guard-suppression", static_guard_suppression);
    opt_field!("security-scaffold-suppression", security_scaffold_suppression);

    if let Some(value) = matches.get_one::<usize>("max-pass-iterations") {
        settings.max_pass_iterations = *value;
    }
}

fn apply_script_overrides(matches: &ArgMatches, store: &mut OptimizationStore) {
    if let Some(scripts) = matches.get_many::<String>("script") {
        for script_path in scripts {
            upsert_script_preset(store, script_path);
        }
    }

    if let Some(buffer_script) = matches.get_one::<String>("script-buffer") {
        store.editor_buffer = buffer_script.clone();
        store.editor_path = None;
        store.applied_buffer_script = Some(buffer_script.clone());
    }
}

fn upsert_script_preset(store: &mut OptimizationStore, script_path: &str) {
    let script_name = Path::new(script_path)
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or(script_path)
        .to_owned();

    if let Some(existing) = store
        .script_presets
        .iter_mut()
        .find(|preset| preset.path == script_path)
    {
        existing.name = script_name;
        existing.enabled = true;
        existing.applied_enabled = true;
        return;
    }

    store.script_presets.push(OptimizationScriptPreset {
        name: script_name,
        path: script_path.to_owned(),
        enabled: true,
        applied_enabled: true,
    });
}

fn collect_applied_script_paths(store: &OptimizationStore) -> Vec<String> {
    store
        .script_presets
        .iter()
        .filter(|preset| preset.applied_enabled)
        .map(|preset| preset.path.clone())
        .collect()
}

fn has_tui_optimization_overrides(matches: &ArgMatches) -> bool {
    matches.get_many::<String>("script").is_some()
        || matches.get_one::<String>("script-buffer").is_some()
        || matches.get_one::<usize>("max-pass-iterations").is_some()
        || OPTIMIZATION_ARG_IDS
            .iter()
            .any(|id| matches.get_one::<bool>(id).is_some())
}

fn collect_blocks(fireball: &Fireball, entry: Arc<Block>) -> Result<Vec<Arc<Block>>, String> {
    let mut blocks = Vec::new();
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let entry_address = entry.get_start_address().get_virtual_address();
    seen.insert(entry_address);
    queue.push_back(entry);

    while let Some(block) = queue.pop_front() {
        blocks.push(block.clone());
        for relation in block.get_connected_to().iter() {
            if let Some(address) = relation.to() {
                let next_address = address.get_virtual_address();
                if !seen.insert(next_address) {
                    continue;
                }
                let next_block = fireball
                    .analyze_block(&address)
                    .map_err(|error| error.to_string())?;
                queue.push_back(next_block);
            }
        }
    }

    Ok(blocks)
}

fn load_json_config(path: &Path) -> Result<FiremanConfig, String> {
    let json = fs::read_to_string(path).map_err(|error| error.to_string())?;
    serde_json::from_str(&json).map_err(|error| error.to_string())
}

fn save_json_sample(path: &Path) -> Result<(), String> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let json = serde_json::to_string_pretty(&FiremanConfig::sample())
        .map_err(|error| error.to_string())?;
    fs::write(path, json).map_err(|error| error.to_string())
}

fn override_bool(matches: &ArgMatches, id: &str, setter: impl FnOnce(bool)) {
    if let Some(value) = matches.get_one::<bool>(id) {
        setter(*value);
    }
}
