use crate::{DecompileArgs, JsonPreset, JsonPresetOptimizationConfig};
use fireball::{
    abstract_syntax_tree::AstOptimizationKind, core::FireRaw, pattern_matching::AstPattern,
};

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
    let mut optimizations: Vec<_> = json_preset
        .optimizations
        .into_iter()
        .map(JsonPresetOptimizationConfig::to_fireball_optimization)
        .collect();
    for path in custom_script
        .into_iter()
        .chain(json_preset.custom_script.into_iter())
    {
        let content = std::fs::read_to_string(&path);
        match content {
            Ok(content) => {
                optimizations.push(AstOptimizationKind::PatternMatching(Box::new(
                    AstPattern::new(path, content),
                )));
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
    let mut ast =
        fireball::ir::analyze::generate_ast_with_pre_defined_symbols(blocks, defined).unwrap();
    ast.optimize(Some(&optimizations)).unwrap();
    let result = ast.print(Some(print_config));
    if let Some(out) = output {
        std::fs::write(out, result).unwrap();
    } else {
        println!("{}", result);
    }
}
