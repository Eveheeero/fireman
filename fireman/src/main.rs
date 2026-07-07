mod decompile;
mod print_json_sample;
mod tui;

use clap::{Arg, ArgAction, ArgMatches, Command};
use fireball::pattern_matching::AstPattern;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

struct ResolvedArgs {
    input: Option<String>,
    output: Option<String>,
    is_tui: bool,
    print_json_sample: bool,
    custom_script: Vec<String>,
    json: Option<String>,
}

impl ResolvedArgs {
    fn to_print_json_sample_args(self) -> Result<PrintJsonSampleArgs, String> {
        Ok(PrintJsonSampleArgs {
            output: self.output.map(|path| PathBuf::from(path)),
        })
    }
    fn to_tui_args(self) -> Result<TuiArgs, String> {
        Ok(TuiArgs {
            input: self.input,
            custom_script: self.custom_script,
            json: self.json,
        })
    }
    fn to_decompile_args(self) -> Result<DecompileArgs, String> {
        let args = DecompileArgs {
            input: self
                .input
                .map(|path| PathBuf::from(path))
                .expect("Input path doesn't given."),
            output: self.output.map(|path| PathBuf::from(path)),
            custom_script: self.custom_script,
            json: self.json,
        };
        // validate
        if !args.input.is_file() {
            return Err("Input file does not exist".to_string());
        }
        Ok(args)
    }
}

struct PrintJsonSampleArgs {
    output: Option<PathBuf>,
}

struct TuiArgs {
    input: Option<String>,      // if invalid path, handle in tui
    custom_script: Vec<String>, // if invalid path, ignore
    json: Option<String>,       // if invalid path, ignore
}

struct DecompileArgs {
    input: PathBuf,
    output: Option<PathBuf>,
    custom_script: Vec<String>, // if invalid path, ignore
    json: Option<String>,       // if invalid path, ignore
}

fn main() {
    let args = parse_arg();
    let args = resolve_args(args);

    if args.print_json_sample {
        let args = args.to_print_json_sample_args().unwrap();
        print_json_sample::print_json_sample(args);
        return;
    } else if args.is_tui {
        let args = args.to_tui_args().unwrap();
        tui::tui(args);
        return;
    }

    let args = args.to_decompile_args().unwrap();
    decompile::decompile(args);
}

fn parse_arg() -> ArgMatches {
    Command::new("fireman")
        .about("Fireman decompiler CLI")
        .author("Eveheeero, xhve00000@mail.com")
        .version("0.0.0")
        .args([
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
                .action(ArgAction::SetTrue)
                .help("Print json sample"),
            Arg::new("script")
                .long("script")
                .value_name("PATH")
                .action(ArgAction::Append)
                .help("Enable a script file (.fb)"),
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("PATH")
                .action(ArgAction::Set)
                .required_unless_present_any(["tui", "json", "jsonsample"])
                .help("Binary to decompile"),
            Arg::new("output")
                .short('o')
                .long("out")
                .value_name("PATH")
                .action(ArgAction::Set)
                .help("Write the printed AST to PATH"),
        ])
        .get_matches()
}

fn resolve_args(args: ArgMatches) -> ResolvedArgs {
    ResolvedArgs {
        is_tui: args.get_one::<bool>("tui").copied().unwrap_or(false),
        print_json_sample: args.get_one::<bool>("jsonsample").copied().unwrap_or(false),
        custom_script: args
            .get_many::<String>("script")
            .unwrap_or_default()
            .cloned()
            .collect(),
        input: args.get_one::<String>("input").cloned(),
        output: args.get_one::<String>("output").cloned(),
        json: args.get_one::<String>("json").cloned(),
    }
}

#[derive(Serialize, Deserialize, Default)]
struct JsonPreset {
    custom_script: Vec<String>,
    optimization_config: JsonPresetOptimizationConfig,
    print_config: JsonPresetPrintConfig,
}

/// see [fireball::abstract_syntax_tree::AstOptimizationConfig]
#[derive(Serialize, Deserialize)]
struct JsonPresetOptimizationConfig {
    ir_analyzation: bool,
    parameter_analyzation: bool,
    call_argument_analyzation: bool,
    constant_folding: bool,
    control_flow_cleanup: bool,
    collapse_unused_variable: bool,
    dead_store_elimination: bool,
    pattern_matching_enabled: bool,
    pattern_matching: Vec<String>, // if invalid name, ignore
    loop_analyzation: bool,
    copy_propagation: bool,
    expression_inlining: bool,
    operator_canonicalization: bool,
    magic_division_recovery: bool,
    identity_simplification: bool,
    bit_trick_recognition: bool,
    cast_minimization: bool,
    ternary_recovery: bool,
    boolean_recovery: bool,
    assertion_recovery: bool,
    do_while_recovery: bool,
    clamp_recovery: bool,
    loop_cleanup: bool,
    if_conversion_reversal: bool,
    switch_reconstruction: bool,
    lifetime_scoping: bool,
    signedness_inference: bool,
    name_recovery: bool,
    early_return_normalization: bool,
    anti_debug_ast_suppression: bool,
    logging_suppression: bool,
    static_guard_suppression: bool,
    security_scaffold_suppression: bool,
    max_pass_iterations: usize,
    use_embedded_passes: bool,
}

impl JsonPresetOptimizationConfig {
    fn to_fireball_optimization_config(
        self,
    ) -> fireball::abstract_syntax_tree::AstOptimizationConfig {
        fireball::abstract_syntax_tree::AstOptimizationConfig {
            ir_analyzation: self.ir_analyzation,
            parameter_analyzation: self.parameter_analyzation,
            call_argument_analyzation: self.call_argument_analyzation,
            constant_folding: self.constant_folding,
            control_flow_cleanup: self.control_flow_cleanup,
            collapse_unused_variable: self.collapse_unused_variable,
            dead_store_elimination: self.dead_store_elimination,
            pattern_matching_enabled: self.pattern_matching_enabled,
            pattern_matching: self
                .pattern_matching
                .iter()
                .filter_map(|x| AstPattern::predefined_pattern(&x))
                .collect(),
            loop_analyzation: self.loop_analyzation,
            copy_propagation: self.copy_propagation,
            expression_inlining: self.expression_inlining,
            operator_canonicalization: self.operator_canonicalization,
            magic_division_recovery: self.magic_division_recovery,
            identity_simplification: self.identity_simplification,
            bit_trick_recognition: self.bit_trick_recognition,
            cast_minimization: self.cast_minimization,
            ternary_recovery: self.ternary_recovery,
            boolean_recovery: self.boolean_recovery,
            assertion_recovery: self.assertion_recovery,
            do_while_recovery: self.do_while_recovery,
            clamp_recovery: self.clamp_recovery,
            loop_cleanup: self.loop_cleanup,
            if_conversion_reversal: self.if_conversion_reversal,
            switch_reconstruction: self.switch_reconstruction,
            lifetime_scoping: self.lifetime_scoping,
            signedness_inference: self.signedness_inference,
            name_recovery: self.name_recovery,
            early_return_normalization: self.early_return_normalization,
            anti_debug_ast_suppression: self.anti_debug_ast_suppression,
            logging_suppression: self.logging_suppression,
            static_guard_suppression: self.static_guard_suppression,
            security_scaffold_suppression: self.security_scaffold_suppression,
            max_pass_iterations: self.max_pass_iterations,
            use_embedded_passes: self.use_embedded_passes,
        }
    }
    fn from_fireball_optimization_config(
        o: fireball::abstract_syntax_tree::AstOptimizationConfig,
    ) -> Self {
        Self {
            ir_analyzation: o.ir_analyzation,
            parameter_analyzation: o.parameter_analyzation,
            call_argument_analyzation: o.call_argument_analyzation,
            constant_folding: o.constant_folding,
            control_flow_cleanup: o.control_flow_cleanup,
            collapse_unused_variable: o.collapse_unused_variable,
            dead_store_elimination: o.dead_store_elimination,
            pattern_matching_enabled: o.pattern_matching_enabled,
            pattern_matching: o
                .pattern_matching
                .iter()
                .map(|x| x.name().to_string())
                .collect(),
            loop_analyzation: o.loop_analyzation,
            copy_propagation: o.copy_propagation,
            expression_inlining: o.expression_inlining,
            operator_canonicalization: o.operator_canonicalization,
            magic_division_recovery: o.magic_division_recovery,
            identity_simplification: o.identity_simplification,
            bit_trick_recognition: o.bit_trick_recognition,
            cast_minimization: o.cast_minimization,
            ternary_recovery: o.ternary_recovery,
            boolean_recovery: o.boolean_recovery,
            assertion_recovery: o.assertion_recovery,
            do_while_recovery: o.do_while_recovery,
            clamp_recovery: o.clamp_recovery,
            loop_cleanup: o.loop_cleanup,
            if_conversion_reversal: o.if_conversion_reversal,
            switch_reconstruction: o.switch_reconstruction,
            lifetime_scoping: o.lifetime_scoping,
            signedness_inference: o.signedness_inference,
            name_recovery: o.name_recovery,
            early_return_normalization: o.early_return_normalization,
            anti_debug_ast_suppression: o.anti_debug_ast_suppression,
            logging_suppression: o.logging_suppression,
            static_guard_suppression: o.static_guard_suppression,
            security_scaffold_suppression: o.security_scaffold_suppression,
            max_pass_iterations: o.max_pass_iterations,
            use_embedded_passes: o.use_embedded_passes,
        }
    }
}

impl Default for JsonPresetOptimizationConfig {
    fn default() -> Self {
        Self::from_fireball_optimization_config(
            fireball::abstract_syntax_tree::AstOptimizationConfig::default(),
        )
    }
}

/// see [fireball::abstract_syntax_tree::AstPrintConfig]
#[derive(Serialize, Deserialize)]
struct JsonPresetPrintConfig {
    print_instruction: bool,
    print_ir: bool,
    print_empty_statement: bool,
    replace_constant: bool,
    parameter_usage_comment: bool,
    variable_usage_comment: bool,
    hide_unused_declarations: bool,
}

impl JsonPresetPrintConfig {
    fn to_fireball_print_config(self) -> fireball::abstract_syntax_tree::AstPrintConfig {
        fireball::abstract_syntax_tree::AstPrintConfig {
            print_instruction: self.print_instruction,
            print_ir: self.print_ir,
            print_empty_statement: self.print_empty_statement,
            replace_constant: self.replace_constant,
            parameter_usage_comment: self.parameter_usage_comment,
            variable_usage_comment: self.variable_usage_comment,
            hide_unused_declarations: self.hide_unused_declarations,
        }
    }
    fn from_fireball_print_config(o: fireball::abstract_syntax_tree::AstPrintConfig) -> Self {
        Self {
            print_instruction: o.print_instruction,
            print_ir: o.print_ir,
            print_empty_statement: o.print_empty_statement,
            replace_constant: o.replace_constant,
            parameter_usage_comment: o.parameter_usage_comment,
            variable_usage_comment: o.variable_usage_comment,
            hide_unused_declarations: o.hide_unused_declarations,
        }
    }
}

impl Default for JsonPresetPrintConfig {
    fn default() -> Self {
        Self::from_fireball_print_config(fireball::abstract_syntax_tree::AstPrintConfig::default())
    }
}
