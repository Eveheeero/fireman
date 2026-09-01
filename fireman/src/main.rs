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

#[derive(Serialize, Deserialize)]
struct JsonPreset {
    custom_script: Vec<String>,
    optimizations: Vec<JsonPresetOptimizationConfig>,
    print_config: JsonPresetPrintConfig,
}

impl Default for JsonPreset {
    fn default() -> Self {
        Self {
            custom_script: [].into(),
            optimizations: fireball::abstract_syntax_tree::AstOptimizationKind::all()
                .into_iter()
                .map(crate::JsonPresetOptimizationConfig::from_fireball_optimization)
                .collect(),
            print_config: JsonPresetPrintConfig::default(),
        }
    }
}

/// see [fireball::abstract_syntax_tree::AstOptimizationKind]
#[derive(Serialize, Deserialize)]
enum JsonPresetOptimizationConfig {
    IrAnalyzation,
    ParameterAnalyzation,
    ConstantFolding,
    CollapseUnusedVariables,
    OptimizationLoop(Vec<JsonPresetOptimizationConfig>, u8),
    PatternMatching(String),
}

impl JsonPresetOptimizationConfig {
    fn to_fireball_optimization(self) -> fireball::abstract_syntax_tree::AstOptimizationKind {
        match self {
            JsonPresetOptimizationConfig::IrAnalyzation => {
                fireball::abstract_syntax_tree::AstOptimizationKind::IrAnalyzation
            }
            JsonPresetOptimizationConfig::ParameterAnalyzation => {
                fireball::abstract_syntax_tree::AstOptimizationKind::ParameterAnalyzation
            }
            JsonPresetOptimizationConfig::ConstantFolding => {
                fireball::abstract_syntax_tree::AstOptimizationKind::ConstantFolding
            }
            JsonPresetOptimizationConfig::CollapseUnusedVariables => {
                fireball::abstract_syntax_tree::AstOptimizationKind::CollapseUnusedVariables
            }
            JsonPresetOptimizationConfig::OptimizationLoop(optimizations, loop_count) => {
                fireball::abstract_syntax_tree::AstOptimizationKind::OptimizationLoop(
                    optimizations
                        .into_iter()
                        .map(JsonPresetOptimizationConfig::to_fireball_optimization)
                        .collect(),
                    loop_count,
                )
            }
            JsonPresetOptimizationConfig::PatternMatching(pattern) => {
                let pattern = if let Some(pattern) = AstPattern::predefined_pattern(&pattern) {
                    pattern
                } else {
                    AstPattern::from_file(pattern)
                };
                fireball::abstract_syntax_tree::AstOptimizationKind::PatternMatching(Box::new(
                    pattern,
                ))
            }
        }
    }
    fn from_fireball_optimization(o: fireball::abstract_syntax_tree::AstOptimizationKind) -> Self {
        match o {
            fireball::abstract_syntax_tree::AstOptimizationKind::IrAnalyzation => {
                Self::IrAnalyzation
            }
            fireball::abstract_syntax_tree::AstOptimizationKind::ParameterAnalyzation => {
                Self::ParameterAnalyzation
            }
            fireball::abstract_syntax_tree::AstOptimizationKind::ConstantFolding => {
                Self::ConstantFolding
            }
            fireball::abstract_syntax_tree::AstOptimizationKind::CollapseUnusedVariables => {
                Self::CollapseUnusedVariables
            }
            fireball::abstract_syntax_tree::AstOptimizationKind::OptimizationLoop(
                optimizations,
                loop_count,
            ) => Self::OptimizationLoop(
                optimizations
                    .into_iter()
                    .map(Self::from_fireball_optimization)
                    .collect(),
                loop_count,
            ),
            fireball::abstract_syntax_tree::AstOptimizationKind::PatternMatching(pattern) => {
                Self::PatternMatching(pattern.name().to_string())
            }
        }
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
