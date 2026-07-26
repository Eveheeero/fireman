use crate::{JsonPreset, PrintJsonSampleArgs};

pub fn print_json_sample(args: PrintJsonSampleArgs) {
    let mut default_json_preset = JsonPreset::default();
    default_json_preset
        .custom_script
        .push("/path/to/script1.fb".to_string());
    default_json_preset
        .custom_script
        .push("/path/to/script2.fb".to_string());
    let default_json_preset = serde_json::to_string_pretty(&default_json_preset).unwrap();

    if let Some(path) = args.output {
        std::fs::write(path, default_json_preset).unwrap();
    } else {
        println!("{}", default_json_preset);
    }
}
