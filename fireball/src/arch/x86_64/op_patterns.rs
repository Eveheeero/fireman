use regex::Regex;

lazy_static::lazy_static! {
  pub static ref PATTERNS: Vec<Regex> = generate_regex_pattern();
  pub static ref OTHERS: Vec<Regex> = generate_other_pattern();
}

#[rustfmt::skip]
fn generate_regex_pattern() -> Vec<Regex> {
    // https://github.com/google/re2/wiki/Syntax
    // https://docs.rs/regex/latest/regex/
    vec![
        Regex::new(r"^0x(?P<address>[0-9a-fA-F]+)$").unwrap(), // https://regex101.com/r/l6QWI9/1
        Regex::new(r"^\wword ptr \[\w?ip (?P<operator>[+-]) 0x(?P<relative_address>[0-9a-fA-F]+)]$").unwrap(), // https://regex101.com/r/l6QWI9/4
        Regex::new(r"^[a-zA-Z]{2,3}$").unwrap(), // https://regex101.com/r/l6QWI9/5
    ]
}

#[rustfmt::skip]
fn generate_other_pattern() -> Vec<Regex> {
    // https://github.com/google/re2/wiki/Syntax
    // https://docs.rs/regex/latest/regex/
    vec![
        // lea rax, [rbx + 0xabcdef]
        Regex::new(r"^(?P<to>\w{2,3}), \[(?P<base>\w{2,3}) (?P<operator>[+-]) 0x(?P<relative_address>[0-9a-fA-F]+)]$").unwrap(), // https://regex101.com/r/l6QWI9/3
        // rip
        Regex::new(r"^.?ip").unwrap(),
        // mov rax, qword ptr [rip - 0xabcdef]
        Regex::new(r"^(?P<to>\w{2,3}), \wword ptr \[(?P<base>\w{2,3}) (?P<operator>[+-]) 0x(?P<relative_address>[0-9a-fA-F]+)]$").unwrap(),
        // mov rax, qword ptr [rax + rdx*4]
        Regex::new(r"^(?P<to>\w{2,3}), \wword ptr \[(?P<base>\w{2,3}) (?P<operator>[+-]) (?P<other>\w{2,3})\*(?P<mul>\d+)]$").unwrap(),
        // mov qword ptr [rax], rax
        Regex::new(r"^?word ptr \[(?P<base>\w{2,3})\], (?P<to>\w{2,3})$").unwrap(),
        // rsp
        Regex::new(r"^.?sp").unwrap(),
        // rbp
        Regex::new(r"^.?bp").unwrap(),
        // mov eax, dword ptr [rbp - 4]
        Regex::new(r"^(?P<to>\w{2,3}), \wword ptr \[(?P<base>\w{2,3}) (?P<operator>[+-]) (?P<other>\d+)]$").unwrap(),
        // add eax, 10
        Regex::new(r"^(?P<to>\w{2,3}), (?P<other>\d+)$").unwrap(),
        // add eax, ebp
        Regex::new(r"^(?P<to>\w{2,3}), (?P<other>\w{2,3})$").unwrap(),
        // eax, dword ptr [rdx + rax]
        Regex::new(r"^(?P<to>\w{2,3}), \wword ptr \[(?P<base>\w{2,3}) (?P<operator>[+-]) (?P<other>\w{2,3})]$").unwrap(),
        // rax, qword ptr [rax]
        Regex::new(r"^(?P<to>\w{2,3}), \wword ptr \[(?P<base>\w{2,3})]$").unwrap(),
    ]
}

#[cfg(test)]
mod tests {
    use super::{OTHERS, PATTERNS};

    #[test]
    #[rustfmt::skip]
    fn test_regex() {
        assert!(PATTERNS[0].is_match("0xabcdef"));
        assert!(PATTERNS[1].is_match("dword ptr [eip + 0xabcdef]"));
        assert!(PATTERNS[1].is_match("qword ptr [rip - 0xabcdef]"));
        assert!(PATTERNS[2].is_match("eax"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_regex_others() {
        assert!(OTHERS[0].is_match("rax, [rbx + 0xabcdef]"));
        assert!(OTHERS[1].is_match("rip, [rbx + 0xabcdef]"));
        assert!(OTHERS[2].is_match("rax, qword ptr [rip - 0xabcdef]"));
        assert!(OTHERS[3].is_match("rax, qword ptr [rax + rdx*4]"));
        assert!(OTHERS[4].is_match("qword ptr [rax], rax"));
        assert!(OTHERS[5].is_match("rsp"));
        assert!(OTHERS[6].is_match("rbp"));
        assert!(OTHERS[7].is_match("eax, dword ptr [rbp - 4]"));
        assert!(OTHERS[8].is_match("eax, 4000"));
        assert!(OTHERS[9].is_match("eax, ebp"));
        assert!(OTHERS[10].is_match("eax, dword ptr [rdx + rax]"));
        assert!(OTHERS[11].is_match("rax, qword ptr [rax]"));
    }
}
