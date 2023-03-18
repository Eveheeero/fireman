use once_cell::sync::Lazy;
use regex::Regex;

/// jcc나 call등의 명령을 통해 나타낼 수 있는 주소 패턴들을 정의한다.
/// 해당 패턴은 op_parse의 함수들과 매칭된다. (첫번째 정규식은 op_parse의 첫번째 함수와 매칭되는 등...)
/// 해당 패턴에 일치하면 op_parse의 함수를 통해 점프하는 대상 주소를 파싱한다.
pub static JMP_TARGET_INST_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    // https://github.com/google/re2/wiki/Syntax
    // https://docs.rs/regex/latest/regex/

    vec![
        Regex::new(r"^0x(?P<address>[0-9a-fA-F]+)$").unwrap(), // https://regex101.com/r/l6QWI9/1
        Regex::new(
            r"^\wword ptr \[\w?ip (?P<operator>[+-]) 0x(?P<relative_address>[0-9a-fA-F]+)]$",
        )
        .unwrap(), // https://regex101.com/r/l6QWI9/4
        Regex::new(r"^[a-zA-Z]{2,3}$").unwrap(),               // https://regex101.com/r/l6QWI9/5
    ]
});

/// jcc나 call등의 명령을 제외한 나머지 명령어의 패턴들을 정의한다.
pub static NOT_JMP_TARGET_INST_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
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
});

#[cfg(test)]
mod tests {
    use super::{JMP_TARGET_INST_PATTERNS, NOT_JMP_TARGET_INST_PATTERNS};

    #[test]
    #[rustfmt::skip]
    fn test_regex() {
        assert!(NOT_JMP_TARGET_INST_PATTERNS[0].is_match("0xabcdef"));
        assert!(NOT_JMP_TARGET_INST_PATTERNS[1].is_match("dword ptr [eip + 0xabcdef]"));
        assert!(NOT_JMP_TARGET_INST_PATTERNS[1].is_match("qword ptr [rip - 0xabcdef]"));
        assert!(NOT_JMP_TARGET_INST_PATTERNS[2].is_match("eax"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_regex_others() {
        assert!(JMP_TARGET_INST_PATTERNS[0].is_match("rax, [rbx + 0xabcdef]"));
        assert!(JMP_TARGET_INST_PATTERNS[1].is_match("rip, [rbx + 0xabcdef]"));
        assert!(JMP_TARGET_INST_PATTERNS[2].is_match("rax, qword ptr [rip - 0xabcdef]"));
        assert!(JMP_TARGET_INST_PATTERNS[3].is_match("rax, qword ptr [rax + rdx*4]"));
        assert!(JMP_TARGET_INST_PATTERNS[4].is_match("qword ptr [rax], rax"));
        assert!(JMP_TARGET_INST_PATTERNS[5].is_match("rsp"));
        assert!(JMP_TARGET_INST_PATTERNS[6].is_match("rbp"));
        assert!(JMP_TARGET_INST_PATTERNS[7].is_match("eax, dword ptr [rbp - 4]"));
        assert!(JMP_TARGET_INST_PATTERNS[8].is_match("eax, 4000"));
        assert!(JMP_TARGET_INST_PATTERNS[9].is_match("eax, ebp"));
        assert!(JMP_TARGET_INST_PATTERNS[10].is_match("eax, dword ptr [rdx + rax]"));
        assert!(JMP_TARGET_INST_PATTERNS[11].is_match("rax, qword ptr [rax]"));
    }
}
