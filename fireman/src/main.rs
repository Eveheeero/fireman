use clap::{Arg, ArgAction, ArgMatches, Command};
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
            input: self.input.map(|path| PathBuf::from(path)),
            output: self.output.map(|path| PathBuf::from(path)),
            custom_script: self.custom_script,
            json: self.json,
        };
        // validate
        if let Some(input) = &args.input && !input.is_file() {
            return Err("Input file does not exist".to_string());
        }
        Ok(args)
    }
}

struct PrintJsonSampleArgs {
    output: Option<PathBuf>,
}

struct TuiArgs {
    input: Option<String>,      // if valid path, handle in tui
    custom_script: Vec<String>, // if valid path, ignore
    json: Option<String>,       // if valid path, ignore
}

struct DecompileArgs {
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    custom_script: Vec<String>, // if valid path, ignore
    json: Option<String>,       // if valid path, ignore
}

fn main() -> Result<(), std::io::Error> {
    let args = parse_arg();
    let args = resolve_args(args);

    if args.print_json_sample {
        let args = args.to_print_json_sample_args();
        let args = match args {
            Ok(args) => args,
            Err(msg) => panic!("{}",msg),
        };
        todo!("open json sample function");
        return Ok(());
    } else if args.is_tui {
        let args = args.to_tui_args();
        let args = match args {
            Ok(args) => args,
            Err(msg) => panic!("{}",msg),
        };
        todo!("open tui session with args");
        return Ok(());
    }

    let args =args.to_decompile_args();
    let args = match args {
        Ok(args) => args,
        Err(msg) => panic!("{}",msg),
    };
    todo!();
    Ok(())
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
