use super::AstPattern;

const EXAMPLE_FB: &str = "placeholder. use include_str to fb file";

pub(super) fn predefined_patterns() -> Vec<AstPattern> {
    vec![
        // AstPattern::from_predefined_include("placeholder.fb", EXAMPLE_FB),
    ]
}

pub(super) fn predefined_pattern(name: &str) -> Option<AstPattern> {
    // Accept both full paths ("patterns/.../foo.fb") and bare names ("foo.fb").
    let short_name = name.rsplit('/').next().unwrap_or(name);
    match short_name {
        // "placeholder.fb" => Some(AstPattern::from_predefined_include(
        //     "placeholder.fb",
        //     EXAMPLE_FB,
        // )),
        _ => None,
    }
}
