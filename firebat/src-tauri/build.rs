fn main() {
    tauri_build::build();
    let dir_contents = std::fs::read_dir("../../bindings").unwrap();
    let mut result = String::new();
    for entry in dir_contents {
        let entry = entry.unwrap();
        let path = entry.path();
        let content = std::fs::read_to_string(&path).unwrap();
        result.push_str(&content);
        result.push_str("\n");
    }
    std::fs::write("../../firebat/src/bindings.ts", result).unwrap();
}
