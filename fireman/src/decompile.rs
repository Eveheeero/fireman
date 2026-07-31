use crate::{DecompileArgs, JsonPreset};
use fireball::{core::FireRaw, pattern_matching::AstPattern};

pub fn decompile(args: DecompileArgs) {
    let DecompileArgs {
        input,
        output,
        custom_script,
        json: json_preset_path,
    } = args;
    let json_preset: JsonPreset = if let Some(json_preset_path) = json_preset_path
        && let Ok(json_preset) = std::fs::read_to_string(&json_preset_path)
    {
        match serde_json::from_str(&json_preset) {
            Ok(json_preset) => json_preset,
            Err(e) => {
                eprintln!("Error parsing JSON preset {}: {}", json_preset_path, e);
                Default::default()
            }
        }
    } else {
        Default::default()
    };
    let mut optimization_config = json_preset
        .optimization_config
        .to_fireball_optimization_config();
    for path in custom_script
        .into_iter()
        .chain(json_preset.custom_script.into_iter())
    {
        let content = std::fs::read_to_string(&path);
        match content {
            Ok(content) => {
                optimization_config
                    .pattern_matching
                    .push(AstPattern::new(path, content));
            }
            Err(e) => {
                eprintln!("Error reading file {}: {}", path, e);
            }
        }
    }
    let print_config = json_preset.print_config.to_fireball_print_config();

    let fireball = fireball::Fireball::from_path(input.to_str().unwrap()).unwrap();
    let blocks = fireball.analyze_all().unwrap();
    let defined = fireball.get_defined();
    let result = fireball::ir::analyze::generate_ast_with_pre_defined_symbols(blocks, defined)
        .unwrap()
        .optimize(Some(optimization_config))
        .unwrap()
        .print(Some(print_config));
    if let Some(out) = output {
        std::fs::write(out, result).unwrap();
    } else {
        println!("{}", result);
    }
}
