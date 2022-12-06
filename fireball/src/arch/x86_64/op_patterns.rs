use regex::Regex;

lazy_static::lazy_static! {
  pub static ref PATTERNS: Vec<Regex> = generate_regex_pattern();
}

#[rustfmt::skip]
fn generate_regex_pattern() -> Vec<Regex> {
    // https://github.com/google/re2/wiki/Syntax
    // https://docs.rs/regex/latest/regex/
    vec![
        Regex::new(r"^0x(?P<address>[0-9a-fA-F]+)$").unwrap(), // https://regex101.com/r/l6QWI9/1
        Regex::new(r"^\wword ptr \[\w?ip (?P<operator>[+-]) 0x(?P<relative_address>[0-9a-fA-F]+)]$").unwrap(), // https://regex101.com/r/l6QWI9/4
        Regex::new(r"^(?P<to>\w{2,3}), \[(?P<base>\w{2,3}) (?P<operator>[+-]) (?P<other>.+)]$").unwrap(), // https://regex101.com/r/l6QWI9/3
    ]
}

#[cfg(test)]
mod tests {
    use super::PATTERNS;
    #[test]
    #[rustfmt::skip]
    fn test_regex() {
        assert!(PATTERNS[0].is_match("0xabcdef"));
        assert!(PATTERNS[1].is_match("dword ptr [eip + 0xabcdef]"));
        assert!(PATTERNS[1].is_match("qword ptr [rip - 0xabcdef]"));
        assert!(PATTERNS[2].is_match("rax, [rbx + 0xabcdef]"));
    }
}
