pub mod prelude;
#[cfg(test)]
mod tests;
mod tui;
pub mod utils;

use clap::{Arg, ArgAction, Command};
use fireball::{Fireball, core::FireRaw};

#[allow(dead_code)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct Setting {
    path: String,
    output: Option<String>,
}

fn parse_arg() -> clap::ArgMatches {
    Command::new("fireman")
        .author("Eveheeero, xhve00000@mail.com")
        .version("0.0.0")
        .about("Fireman decopiler CLI")
        .args([
            Arg::new("tui")
                .long("tui")
                .action(ArgAction::SetTrue)
                .help("Run decompiler as TUI mode"),
            Arg::new("json")
                .short('j')
                .long("json")
                .value_name("PATH")
                .action(ArgAction::Set)
                .help("JSON config path"),
            Arg::new("jsonsample")
                .long("jsonsample")
                .action(ArgAction::SetTrue)
                .help("Print json sample"),
            Arg::new("opt")
                .long("opt")
                .value_name("KEY=VALUE")
                .action(ArgAction::Append)
                .help("Override optimization setting, e.g. ir_analyzation=false"),
            Arg::new("script")
                .long("script")
                .value_name("PATH")
                .action(ArgAction::Append)
                .help("Enable an optimization script (.fb)"),
            Arg::new("script-buffer")
                .long("script-buffer")
                .value_name("PATH")
                .action(ArgAction::Set)
                .help("Load optimization script buffer from file"),
            Arg::new("input path")
                .short('i')
                .long("path")
                .value_name("TARGET PATH")
                .action(ArgAction::Set)
                .help("Program wants to decompile")
                .required_unless_present_any(["tui", "json", "jsonsample"]),
            Arg::new("output path")
                .short('o')
                .long("out")
                .value_name("OUTPUT PATH")
                .action(ArgAction::Set)
                .help("Output Path"),
        ])
        .get_matches()
}

fn main() {
    let args = parse_arg();
    let tui = args.get_one::<bool>("tui").unwrap();
    let input = args.get_one::<String>("input path");
    let json = args.get_one::<String>("json");
    let json_sample = args.get_one::<bool>("json sample").unwrap();

    if *json_sample {
        todo!()
    }

    if let Some(json) = json {
        todo!("json preset mode {}", json)
    }

    if *tui {
        // tui mode, reads inputs or json too
        tui::main();
        return;
    }

    let input = input.unwrap();
    let fire = Fireball::from_path(input).unwrap();
    let result = fire.analyze_from_entry().unwrap();
    let mut walked = Vec::new();
    let mut queue: Vec<_> = [result.get_start_address().clone()].into();
    while let Some(address) = queue.pop() {
        if walked.contains(&address) {
            continue;
        }
        println!(
            "Ir Block starts from {:#010x}",
            address.get_virtual_address()
        );
        let now = fire.analyze_block(&address).unwrap();
        walked.push(address);
        let reader = now.get_ir();
        let ir_block = reader.as_ref().unwrap();
        println!();
        for (ir, instruction) in ir_block.ir().iter().zip(ir_block.instructions().iter()) {
            println!();
            println!("{}", instruction);
            ir.statements.as_ref().unwrap().iter().for_each(|x| {
                println!("{}", x);
            });
        }
        println!();
        println!();
        queue.extend(now.get_connected_to().iter().filter_map(|x| x.to()));
    }
}
