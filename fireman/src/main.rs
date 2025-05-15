use fireball::{core::Fire, Fireball};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct Setting {
    path: String,
    output: Option<String>,
}

fn parse_arg() -> clap::ArgMatches {
    use clap::*;
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
            Arg::new("json sample")
                .long("jsonsample")
                .action(ArgAction::SetTrue)
                .help("Print json sample"),
            Arg::new("input path")
                .short('i')
                .long("path")
                .value_name("TARGET PATH")
                .action(ArgAction::Set)
                .help("Program wants to decompile")
                .required_unless_present_any(["tui", "json", "json sample"]),
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
        todo!("tui mode, reads inputs or json too")
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
        for ir in ir_block.ir().iter() {
            println!();
            println!("{:?}", ir.instruction);
            ir.statements.as_ref().unwrap().iter().for_each(|x| {
                println!("{:?}", x);
            });
        }
        println!();
        println!();
        queue.extend(now.get_connected_to().iter().filter_map(|x| x.to()));
    }
}
