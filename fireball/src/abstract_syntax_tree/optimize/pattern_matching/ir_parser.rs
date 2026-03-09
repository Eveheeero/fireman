use super::{AstPatternAstMatcher, AstPatternIrMatcher};
use crate::{
    ir::{
        analyze::datatype::DataType,
        data::{IrAccessSize, IrData, IrDataOperation, IrIntrinsic},
        operator::{IrBinaryOperator, IrUnaryOperator},
        statements::{IrStatement, IrStatementSpecial},
    },
    utils::Aos,
};
use std::{num::NonZeroU8, panic::AssertUnwindSafe};
use tracing::debug;

pub(super) fn compile_ast_matcher(text: &str) -> AstPatternAstMatcher {
    if let Some(content) = text.strip_prefix("comment ") {
        let normalized = normalize_comment_bytes(content);
        if normalized.is_empty() {
            return AstPatternAstMatcher::Unsupported;
        }
        return AstPatternAstMatcher::CommentContains(normalized.into_boxed_slice());
    }

    let strict = normalize_for_wildcard_match(text);
    if strict == "empty" {
        return AstPatternAstMatcher::Empty;
    }
    if strict == "undefined" {
        return AstPatternAstMatcher::Undefined;
    }
    if strict == "return" {
        return AstPatternAstMatcher::ReturnAny;
    }
    if strict == "block(...)" {
        return AstPatternAstMatcher::BlockAny;
    }
    if strict == "block([])" {
        return AstPatternAstMatcher::BlockEmpty;
    }
    if strict == "some([])" {
        return AstPatternAstMatcher::SomeEmpty;
    }
    if normalize_for_wildcard_match_relaxed(text) == "if..." {
        return AstPatternAstMatcher::IfAny;
    }
    if let Some(ir_text) = text.strip_prefix("ir ") {
        if let Some(statement) = parse_ir_statement(ir_text.trim()) {
            return AstPatternAstMatcher::IrExact(Box::new(statement));
        }
    }
    if let Some(asm_text) = text.strip_prefix("asm ") {
        if let Some(statement) = parse_asm_statement(asm_text.trim()) {
            return AstPatternAstMatcher::IrExact(Box::new(statement));
        }
    }
    if let Some(label) = parse_label_match(text) {
        return AstPatternAstMatcher::Label(label);
    }
    AstPatternAstMatcher::Unsupported
}

pub(super) fn parse_label_match(text: &str) -> Option<Option<Box<[u8]>>> {
    let strict = normalize_for_wildcard_match(text);
    if let Some(inner) = strict
        .strip_prefix("label(")
        .and_then(|rest| rest.strip_suffix(")"))
    {
        if inner.is_empty() {
            return Some(None);
        }
        let normalized = normalize_for_match(inner);
        return Some(Some(normalized.into_bytes().into_boxed_slice()));
    }
    None
}

pub(super) fn compile_ir_matcher(text: &str) -> AstPatternIrMatcher {
    let strict = normalize_for_wildcard_match(text);
    if strict == "..." {
        return AstPatternIrMatcher::Any;
    }
    if normalize_for_wildcard_match_relaxed(text) == "if..." {
        return AstPatternIrMatcher::IfAny;
    }
    parse_ir_statement(text)
        .map(AstPatternIrMatcher::Exact)
        .unwrap_or(AstPatternIrMatcher::Unsupported)
}

pub(super) fn normalize_comment_bytes(text: &str) -> Vec<u8> {
    normalize_for_match(text).into_bytes()
}

pub(super) fn normalized_comment_contains(comment: &str, expected_normalized: &[u8]) -> bool {
    if expected_normalized.is_empty() {
        return false;
    }
    let normalized = normalize_comment_bytes(comment);
    if normalized.len() < expected_normalized.len() {
        return false;
    }
    normalized
        .windows(expected_normalized.len())
        .any(|window| window == expected_normalized)
}

pub(super) fn normalize_for_match(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

pub(super) fn normalize_for_wildcard_match(text: &str) -> String {
    text.chars()
        .filter(|ch| !ch.is_whitespace())
        .map(|ch| ch.to_ascii_lowercase())
        .collect()
}

pub(super) fn normalize_for_wildcard_match_relaxed(text: &str) -> String {
    text.chars()
        .filter(|ch| {
            !ch.is_whitespace() && !matches!(ch, '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';')
        })
        .map(|ch| ch.to_ascii_lowercase())
        .collect()
}

pub(super) fn parse_asm_statement(text: &str) -> Option<IrStatement> {
    let mut raw = text.trim();
    if raw.is_empty() {
        return None;
    }
    raw = raw.strip_prefix("asm ").unwrap_or(raw).trim();
    raw = raw.split(';').next().unwrap_or(raw).trim();
    if raw.is_empty() {
        return None;
    }

    let mut parts = raw.splitn(2, |ch: char| ch.is_whitespace());
    let mnemonic = parts.next()?.trim();
    let operands = parts.next().unwrap_or_default().trim();

    let statement = iceball::parse_statement(iceball::Architecture::X64, mnemonic).ok()?;
    let arguments = parse_asm_arguments(operands)?;
    let instruction = crate::core::Instruction {
        address: 0,
        inner: iceball::Instruction {
            statement: Ok(statement),
            arguments: arguments.into_boxed_slice(),
            bytes: None,
        },
    };
    let statements = crate::arch::x86_64::instruction_analyze::create_ir_statement(&instruction)?;
    statements.first().cloned()
}

pub(super) fn parse_asm_arguments(text: &str) -> Option<Vec<iceball::Argument>> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Some(Vec::new());
    }

    let mut arguments = Vec::new();
    for operand in split_top_level(trimmed, ',') {
        let operand = operand.trim();
        if operand.is_empty() {
            continue;
        }
        arguments.push(parse_asm_argument_lossy(operand)?);
    }
    Some(arguments)
}

pub(super) fn parse_asm_argument_lossy(op: &str) -> Option<iceball::Argument> {
    if let Ok(value) = op.parse::<u64>() {
        return Some(iceball::Argument::Constant(value));
    }

    if let Some(arg) = parse_asm_argument_safe(op) {
        return Some(arg);
    }

    let lowered = op.to_ascii_lowercase();
    let stripped = [
        "byte ptr ",
        "word ptr ",
        "dword ptr ",
        "qword ptr ",
        "xmmword ptr ",
        "ymmword ptr ",
        "zmmword ptr ",
        "ptr ",
    ]
    .iter()
    .find_map(|prefix| lowered.strip_prefix(prefix).map(str::trim));

    stripped.and_then(parse_asm_argument_safe)
}

pub(super) fn parse_asm_argument_safe(op: &str) -> Option<iceball::Argument> {
    fn try_parse(op: &str) -> Option<iceball::Argument> {
        std::panic::catch_unwind(|| iceball::parse_argument(iceball::Architecture::X64, op))
            .ok()
            .and_then(Result::ok)
    }

    try_parse(op)
        .or_else(|| try_parse(&op.to_ascii_uppercase()))
        .or_else(|| try_parse(&op.to_ascii_lowercase()))
}

pub(super) fn parse_ir_statement(replacement: &str) -> Option<IrStatement> {
    let text = replacement.trim();
    if text.is_empty() {
        return None;
    }

    let normalized = normalize_for_match(text);
    if normalized == "undefined" {
        return Some(IrStatement::Undefined);
    }
    if normalized == "halt" {
        return Some(IrStatement::Halt);
    }

    if let Some(rest) = text.strip_prefix("exception ") {
        return Some(IrStatement::Exception(leak_static_str(rest.trim())));
    }
    if let Some(rest) = text.strip_prefix("jmp ") {
        return Some(IrStatement::Jump {
            target: parse_ir_data(rest),
        });
    }
    if let Some(rest) = text.strip_prefix("call ") {
        return Some(IrStatement::JumpByCall {
            target: parse_ir_data(rest),
        });
    }
    if text.starts_with("if ") {
        return parse_ir_condition_statement(text);
    }
    if let Some(rest) = text.strip_prefix("type ") {
        return parse_ir_type_special(rest).map(|special| {
            IrStatement::Special(IrStatementSpecial::TypeSpecified {
                location: special.location,
                size: special.size,
                data_type: special.data_type,
            })
        });
    }
    if let Some(rest) = text.strip_prefix("calc_flags ") {
        return parse_ir_calc_flags(rest);
    }
    if let Some(rest) = text.strip_prefix("assert ") {
        return Some(IrStatement::Special(IrStatementSpecial::Assertion {
            condition: parse_ir_data(strip_wrapping_parens(rest.trim())),
        }));
    }
    if let Some(assignment) = parse_ir_assignment_statement(text) {
        return Some(assignment);
    }

    debug!("Could not parse IR statement pattern: {}", replacement);
    None
}

#[derive(Debug)]
pub(super) struct AstPatternParsedTypeSpecial {
    pub(super) location: Aos<IrData>,
    pub(super) size: IrAccessSize,
    pub(super) data_type: DataType,
}

pub(super) fn parse_ir_assignment_statement(text: &str) -> Option<IrStatement> {
    let (to_text, rhs) = text.split_once(" = ")?;
    let rhs = rhs.trim();
    if !rhs.starts_with('(') {
        return None;
    }
    let close = find_matching_delimiter(rhs, 0, '(', ')')?;
    let size_text = &rhs[1..close];
    let from_text = rhs[close + 1..].trim();
    Some(IrStatement::Assignment {
        from: parse_ir_data(from_text),
        to: parse_ir_data(to_text.trim()),
        size: parse_ir_access_size(size_text.trim()),
    })
}

pub(super) fn parse_ir_type_special(text: &str) -> Option<AstPatternParsedTypeSpecial> {
    let (location_text, rhs) = text.split_once(" = ")?;
    let rhs = rhs.trim();
    let data_type_tokens = [
        ("*c", DataType::StringPointer),
        ("u", DataType::Unknown),
        ("b", DataType::Bool),
        ("i", DataType::Int),
        ("f32", DataType::Float32),
        ("f64", DataType::Float64),
        ("f80", DataType::Float80),
        ("f", DataType::Float64),
        ("c", DataType::Char),
        ("*", DataType::Address),
    ];

    for (token, data_type) in data_type_tokens {
        if let Some(size_text) = rhs.strip_suffix(token) {
            return Some(AstPatternParsedTypeSpecial {
                location: parse_ir_data(location_text.trim()),
                size: parse_ir_access_size(size_text.trim()),
                data_type,
            });
        }
    }
    None
}

pub(super) fn parse_ir_calc_flags(text: &str) -> Option<IrStatement> {
    let trimmed = text.trim();
    let open_list = trimmed.find('[')?;
    let close_list = find_matching_delimiter(trimmed, open_list, '[', ']')?;
    let flags_text = &trimmed[open_list + 1..close_list];
    let after_flags = trimmed[close_list + 1..].trim();
    let operation_text = strip_wrapping_parens(after_flags);

    let flags = split_top_level(flags_text, ',')
        .into_iter()
        .map(parse_ir_data)
        .collect::<Vec<_>>();

    Some(IrStatement::Special(
        IrStatementSpecial::CalcFlagsAutomatically {
            operation: parse_ir_data(operation_text),
            size: IrAccessSize::ArchitectureSize,
            flags,
        },
    ))
}

pub(super) fn parse_ir_condition_statement(text: &str) -> Option<IrStatement> {
    let after_if = text.strip_prefix("if ")?.trim_start();
    let open_true = after_if.find('{')?;
    let condition_text = after_if[..open_true].trim();
    let (true_block, rest) = parse_braced_block(&after_if[open_true..])?;
    let rest = rest.trim_start();

    let false_branch = if let Some(else_part) = rest.strip_prefix("else") {
        let (false_block, _) = parse_braced_block(else_part.trim_start())?;
        parse_ir_statement_sequence(&false_block)
    } else {
        Vec::new()
    };

    Some(IrStatement::Condition {
        condition: parse_ir_data(condition_text),
        true_branch: parse_ir_statement_sequence(&true_block).into_boxed_slice(),
        false_branch: false_branch.into_boxed_slice(),
    })
}

pub(super) fn parse_ir_statement_sequence(text: &str) -> Vec<IrStatement> {
    split_top_level(text, ';')
        .into_iter()
        .filter_map(|segment| {
            let segment = segment.trim();
            if segment.is_empty() {
                return None;
            }
            Some(parse_ir_statement(segment).unwrap_or(IrStatement::Undefined))
        })
        .collect()
}

pub(super) fn parse_ir_access_size(text: &str) -> IrAccessSize {
    let trimmed = text.trim();
    if trimmed.eq_ignore_ascii_case("arch_len") {
        return IrAccessSize::ArchitectureSize;
    }
    if trimmed.eq_ignore_ascii_case("unlimited") {
        return IrAccessSize::Unlimited;
    }
    if let Some(inner) = trimmed
        .strip_prefix("sizeof(")
        .and_then(|value| value.strip_suffix(')'))
    {
        return IrAccessSize::RelativeWith(parse_ir_data(inner.trim()));
    }
    if let Some(inner) = trimmed.strip_suffix("bit") {
        return IrAccessSize::ResultOfBit(parse_ir_data(inner.trim()));
    }
    if let Some(inner) = trimmed.strip_suffix("byte") {
        return IrAccessSize::ResultOfByte(parse_ir_data(inner.trim()));
    }
    IrAccessSize::ArchitectureSize
}

pub(super) fn parse_ir_data(text: &str) -> Aos<IrData> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return ir_unknown_data();
    }

    if let Some(inner) = parse_outer_wrapped(trimmed, '(', ')') {
        return parse_ir_data(inner);
    }

    if let Some(intrinsic) = parse_ir_intrinsic(trimmed) {
        return IrData::Intrinsic(intrinsic).into();
    }
    if let Some(operation) = parse_ir_operation(trimmed) {
        return IrData::Operation(operation).into();
    }
    if let Some(constant) = parse_ir_constant(trimmed) {
        return IrData::Constant(constant).into();
    }
    if let Some(operand) = parse_ir_operand(trimmed) {
        return IrData::Operand(operand).into();
    }
    if let Some(register) = try_parse_register(trimmed) {
        return register;
    }

    ir_unknown_data()
}

pub(super) fn parse_ir_operation(text: &str) -> Option<IrDataOperation> {
    let unary_ops = [
        ("sign_extend ", IrUnaryOperator::SignExtend),
        ("zero_extend ", IrUnaryOperator::ZeroExtend),
        ("! ", IrUnaryOperator::Not),
        ("- ", IrUnaryOperator::Negation),
    ];
    for (prefix, operator) in unary_ops {
        if let Some(arg_text) = text.strip_prefix(prefix) {
            return Some(IrDataOperation::Unary {
                operator,
                arg: parse_ir_data(arg_text),
            });
        }
    }

    if let Some((arg1, operator, arg2)) = parse_ir_binary_operation_parts(text) {
        return Some(IrDataOperation::Binary {
            operator,
            arg1: parse_ir_data(arg1),
            arg2: parse_ir_data(arg2),
        });
    }
    None
}

pub(super) fn parse_ir_binary_operation_parts(
    text: &str,
) -> Option<(&str, IrBinaryOperator, &str)> {
    if let Some((left, right)) = split_once_top_level(text, " == ") {
        let (size, rhs) = parse_operator_sized_rhs(right)?;
        return Some((left, IrBinaryOperator::Equal(size), rhs));
    }
    if let Some((left, right)) = split_once_top_level(text, " <= ") {
        let (size, rhs) = parse_operator_sized_rhs(right)?;
        return Some((left, IrBinaryOperator::SignedLessOrEqual(size), rhs));
    }
    if let Some((left, right)) = split_once_top_level(text, " < ") {
        let (size, rhs) = parse_operator_sized_rhs(right)?;
        return Some((left, IrBinaryOperator::SignedLess(size), rhs));
    }

    let binary_tokens = [
        (" << ", IrBinaryOperator::Shl),
        (" >> ", IrBinaryOperator::Shr),
        (" sar ", IrBinaryOperator::Sar),
        (" div ", IrBinaryOperator::UnsignedDiv),
        (" rem ", IrBinaryOperator::UnsignedRem),
        (" & ", IrBinaryOperator::And),
        (" | ", IrBinaryOperator::Or),
        (" ^ ", IrBinaryOperator::Xor),
        (" + ", IrBinaryOperator::Add),
        (" - ", IrBinaryOperator::Sub),
        (" * ", IrBinaryOperator::Mul),
        (" / ", IrBinaryOperator::SignedDiv),
        (" % ", IrBinaryOperator::SignedRem),
    ];
    for (token, operator) in binary_tokens {
        if let Some((left, right)) = split_once_top_level(text, token) {
            return Some((left, operator, right));
        }
    }
    None
}

pub(super) fn parse_operator_sized_rhs(text: &str) -> Option<(IrAccessSize, &str)> {
    let rhs = text.trim_start();
    if !rhs.starts_with('(') {
        return None;
    }
    let close = find_matching_delimiter(rhs, 0, '(', ')')?;
    let size = parse_ir_access_size(&rhs[1..close]);
    Some((size, rhs[close + 1..].trim_start()))
}

pub(super) fn parse_ir_intrinsic(text: &str) -> Option<IrIntrinsic> {
    if text.eq_ignore_ascii_case("unknown") {
        return Some(IrIntrinsic::Unknown);
    }
    if text.eq_ignore_ascii_case("undefined") {
        return Some(IrIntrinsic::Undefined);
    }
    if text.eq_ignore_ascii_case("arch_byte_size") {
        return Some(IrIntrinsic::ArchitectureByteSize);
    }
    if text.eq_ignore_ascii_case("arch_bit_size") {
        return Some(IrIntrinsic::ArchitectureBitSize);
    }
    if text.eq_ignore_ascii_case("arch_bit_per_byte") {
        return Some(IrIntrinsic::ArchitectureBitPerByte);
    }
    if text.eq_ignore_ascii_case("instruction_byte_size") {
        return Some(IrIntrinsic::InstructionByteSize);
    }

    if let Some(size_text) = parse_function_arg(text, "signed_max") {
        return Some(IrIntrinsic::SignedMax(parse_ir_access_size(size_text)));
    }
    if let Some(size_text) = parse_function_arg(text, "signed_min") {
        return Some(IrIntrinsic::SignedMin(parse_ir_access_size(size_text)));
    }
    if let Some(size_text) = parse_function_arg(text, "unsigned_max") {
        return Some(IrIntrinsic::UnsignedMax(parse_ir_access_size(size_text)));
    }
    if let Some(size_text) = parse_function_arg(text, "unsigned_min") {
        return Some(IrIntrinsic::UnsignedMin(parse_ir_access_size(size_text)));
    }
    if let Some(size_text) = parse_function_arg(text, "bit_ones") {
        return Some(IrIntrinsic::BitOnes(parse_ir_access_size(size_text)));
    }
    if let Some(size_text) = parse_function_arg(text, "bit_zeros") {
        return Some(IrIntrinsic::BitZeros(parse_ir_access_size(size_text)));
    }
    if let Some(data_text) = parse_function_arg(text, "byte_size_of") {
        return Some(IrIntrinsic::ByteSizeOf(parse_ir_data(data_text)));
    }
    if let Some(data_text) = parse_function_arg(text, "bit_size_of") {
        return Some(IrIntrinsic::BitSizeOf(parse_ir_data(data_text)));
    }
    if let Some(arg_text) = parse_function_arg(text, "sized") {
        let args = split_top_level(arg_text, ',');
        if args.len() == 2 {
            return Some(IrIntrinsic::Sized(
                parse_ir_data(args[0]),
                parse_ir_access_size(args[1]),
            ));
        }
    }
    if let Some(value_text) = parse_function_arg(text, "operand_exists") {
        if let Ok(raw) = value_text.trim().parse::<u8>() {
            if let Some(value) = NonZeroU8::new(raw) {
                return Some(IrIntrinsic::OperandExists(value));
            }
        }
    }
    if let Some(value_text) = parse_function_arg(text, "arch_byte_size_condition") {
        return Some(IrIntrinsic::ArchitectureByteSizeCondition(
            parse_num_condition(value_text),
        ));
    }

    None
}

pub(super) fn parse_num_condition(text: &str) -> crate::ir::data::NumCondition {
    let normalized = normalize_for_match(text);
    let parse_u16 = |value: &str| value.trim().parse::<u16>().ok();

    for (token, make) in [
        (
            " not in ",
            crate::ir::data::NumCondition::ExcludesRange as fn(u16, u16) -> _,
        ),
        (
            " in ",
            crate::ir::data::NumCondition::RangeInclusive as fn(u16, u16) -> _,
        ),
    ] {
        if let Some((_, rhs)) = normalized.split_once(token) {
            let rhs = rhs.trim();
            if let Some(inner) = rhs.strip_prefix('[').and_then(|v| v.strip_suffix(']')) {
                if let Some((a, b)) = inner.split_once("..") {
                    if let (Some(a), Some(b)) = (parse_u16(a), parse_u16(b)) {
                        return make(a, b);
                    }
                }
            }
        }
    }

    let comparisons = [
        (
            ">=",
            crate::ir::data::NumCondition::HigherOrEqual as fn(u16) -> _,
        ),
        (
            "<=",
            crate::ir::data::NumCondition::LowerOrEqual as fn(u16) -> _,
        ),
        (
            "!=",
            crate::ir::data::NumCondition::NotEqual as fn(u16) -> _,
        ),
        ("==", crate::ir::data::NumCondition::Equal as fn(u16) -> _),
        (">", crate::ir::data::NumCondition::Higher as fn(u16) -> _),
        ("<", crate::ir::data::NumCondition::Lower as fn(u16) -> _),
    ];
    for (token, make) in comparisons {
        if let Some((_, rhs)) = normalized.split_once(token) {
            if let Some(value) = parse_u16(rhs) {
                return make(value);
            }
        }
    }
    crate::ir::data::NumCondition::Equal(0)
}

pub(super) fn parse_ir_constant(text: &str) -> Option<usize> {
    let trimmed = text.trim();
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        return usize::from_str_radix(hex, 16).ok();
    }
    trimmed.parse::<usize>().ok()
}

pub(super) fn parse_ir_operand(text: &str) -> Option<NonZeroU8> {
    let trimmed = text.trim();
    let raw = trimmed.strip_prefix('o')?.parse::<u8>().ok()?;
    NonZeroU8::new(raw)
}

pub(super) fn try_parse_register(text: &str) -> Option<Aos<IrData>> {
    let candidate = text.trim();
    if candidate.is_empty() {
        return None;
    }
    if !candidate
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    {
        return None;
    }
    std::panic::catch_unwind(AssertUnwindSafe(|| {
        crate::arch::x86_64::str_to_x64_register(candidate)
    }))
    .ok()
}

pub(super) fn parse_function_arg<'a>(text: &'a str, name: &str) -> Option<&'a str> {
    let prefix = format!("{name}(");
    let candidate = text.trim();
    if !candidate.starts_with(&prefix) || !candidate.ends_with(')') {
        return None;
    }
    let inner = &candidate[prefix.len()..candidate.len() - 1];
    Some(inner.trim())
}

pub(super) fn split_once_top_level<'a>(text: &'a str, token: &str) -> Option<(&'a str, &'a str)> {
    let mut paren = 0usize;
    let mut bracket = 0usize;
    let mut brace = 0usize;
    for (idx, ch) in text.char_indices() {
        match ch {
            '(' => paren += 1,
            ')' => paren = paren.saturating_sub(1),
            '[' => bracket += 1,
            ']' => bracket = bracket.saturating_sub(1),
            '{' => brace += 1,
            '}' => brace = brace.saturating_sub(1),
            _ => {}
        }
        if paren == 0 && bracket == 0 && brace == 0 && text[idx..].starts_with(token) {
            let left = text[..idx].trim_end();
            let right = text[idx + token.len()..].trim_start();
            return Some((left, right));
        }
    }
    None
}

pub(super) fn split_top_level(text: &str, delimiter: char) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0usize;
    let mut paren = 0usize;
    let mut bracket = 0usize;
    let mut brace = 0usize;
    for (idx, ch) in text.char_indices() {
        match ch {
            '(' => paren += 1,
            ')' => paren = paren.saturating_sub(1),
            '[' => bracket += 1,
            ']' => bracket = bracket.saturating_sub(1),
            '{' => brace += 1,
            '}' => brace = brace.saturating_sub(1),
            _ => {}
        }
        if ch == delimiter && paren == 0 && bracket == 0 && brace == 0 {
            out.push(text[start..idx].trim());
            start = idx + ch.len_utf8();
        }
    }
    out.push(text[start..].trim());
    out
}

pub(super) fn parse_braced_block(text: &str) -> Option<(String, &str)> {
    let trimmed = text.trim_start();
    if !trimmed.starts_with('{') {
        return None;
    }
    let close = find_matching_delimiter(trimmed, 0, '{', '}')?;
    let inner = trimmed[1..close].to_string();
    let rest = &trimmed[close + 1..];
    Some((inner, rest))
}

pub(super) fn find_matching_delimiter(
    text: &str,
    open_index: usize,
    open_delim: char,
    close_delim: char,
) -> Option<usize> {
    let mut depth = 0usize;
    for (idx, ch) in text.char_indices().skip(open_index) {
        if ch == open_delim {
            depth += 1;
        } else if ch == close_delim {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(idx);
            }
        }
    }
    None
}

pub(super) fn parse_outer_wrapped(text: &str, open: char, close: char) -> Option<&str> {
    if !text.starts_with(open) || !text.ends_with(close) {
        return None;
    }
    let close_idx = find_matching_delimiter(text, 0, open, close)?;
    if close_idx + close.len_utf8() != text.len() {
        return None;
    }
    Some(text[open.len_utf8()..close_idx].trim())
}

pub(super) fn strip_wrapping_parens(text: &str) -> &str {
    parse_outer_wrapped(text, '(', ')').unwrap_or(text).trim()
}

pub(super) fn ir_unknown_data() -> Aos<IrData> {
    IrData::Intrinsic(IrIntrinsic::Unknown).into()
}

pub(super) fn leak_static_str(text: &str) -> &'static str {
    Box::leak(text.to_string().into_boxed_str())
}
