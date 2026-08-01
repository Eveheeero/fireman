mod decompile;
mod print_json_sample;
mod tui;

use clap::{Arg, ArgAction, ArgMatches, Command};
use fireball::pattern_matching::AstPattern;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    constant_folding: bool,
    collapse_unused_variable: bool,
    pattern_matching: Vec<String>, // if invalid name, ignore
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
            constant_folding: self.constant_folding,
            collapse_unused_variable: self.collapse_unused_variable,
            pattern_matching: self
                .pattern_matching
                .iter()
                .filter_map(|x| AstPattern::predefined_pattern(&x))
                .collect(),
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
            constant_folding: o.constant_folding,
            collapse_unused_variable: o.collapse_unused_variable,
            pattern_matching: o
                .pattern_matching
                .iter()
                .map(|x| x.name().to_string())
                .collect(),
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
    print_empty_statement: bool,
    replace_constant: bool,
    parameter_usage_comment: bool,
    variable_usage_comment: bool,
    hide_unused_declarations: bool,
}

impl JsonPresetPrintConfig {
    fn to_fireball_print_config(self) -> fireball::abstract_syntax_tree::AstPrintConfig {
        fireball::abstract_syntax_tree::AstPrintConfig {
            print_empty_statement: self.print_empty_statement,
            replace_constant: self.replace_constant,
            parameter_usage_comment: self.parameter_usage_comment,
            variable_usage_comment: self.variable_usage_comment,
            hide_unused_declarations: self.hide_unused_declarations,
        }
    }
    fn from_fireball_print_config(o: fireball::abstract_syntax_tree::AstPrintConfig) -> Self {
        Self {
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
