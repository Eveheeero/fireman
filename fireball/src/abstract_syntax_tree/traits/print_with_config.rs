use super::*;
use std::collections::HashSet;

fn collect_statement_ir_origins<'a>(
    origin: &'a AstStatementOrigin,
    out: &mut Vec<&'a AstDescriptor>,
) {
    match origin {
        AstStatementOrigin::Ir(descriptor) => out.push(descriptor),
        AstStatementOrigin::Combination(origins) => {
            for origin in origins {
                collect_statement_ir_origins(origin, out);
            }
        }
        _ => {}
    }
}

fn descriptor_source_key(descriptor: &AstDescriptor) -> usize {
    std::sync::Arc::as_ptr(descriptor.ir()) as usize
}

fn wrapped_statement_with_origin(stmt: &WrappedAstStatement, config: AstPrintConfig) -> String {
    let statement_text = stmt.to_string_with_config(Some(config));
    if statement_text.is_empty() {
        return statement_text;
    }

    let mut origins = Vec::new();
    collect_statement_ir_origins(&stmt.origin, &mut origins);

    let mut origin_lines = Vec::new();
    let mut printed_instruction = HashSet::new();
    let mut printed_ir = HashSet::new();

    for descriptor in origins {
        let source_key = descriptor_source_key(descriptor);
        if config.print_instruction {
            let ir_key = (source_key, descriptor.descriptor().ir_index());
            if printed_instruction.insert(ir_key)
                && let Some(instruction) = descriptor
                    .ir()
                    .get_instructions()
                    .get(descriptor.descriptor().ir_index() as usize)
            {
                origin_lines.push(format!("// {}", instruction));
            }
        }

        if config.print_ir
            && let Some(statement_index) = descriptor.descriptor().statement_index()
        {
            let descriptor_key = (source_key, *descriptor.descriptor());
            if printed_ir.insert(descriptor_key)
                && let Some(ir) = descriptor
                    .ir()
                    .get_ir()
                    .get(descriptor.descriptor().ir_index() as usize)
                && let Some(statements) = ir.statements.as_ref()
                && let Some(ir_stmt) = statements.get(*statement_index as usize)
            {
                origin_lines.push(format!("/* {} */", ir_stmt));
            }
        }
    }

    if origin_lines.is_empty() {
        statement_text
    } else {
        format!("{}\n{}", origin_lines.join("\n"), statement_text)
    }
}

fn statement_body(stmts: &[WrappedAstStatement], config: AstPrintConfig) -> Vec<String> {
    stmts
        .iter()
        .map(|stmt| wrapped_statement_with_origin(stmt, config))
        .filter(|stmt| !stmt.is_empty())
        .collect()
}

fn indent_multiline(text: &str, indent: &str) -> String {
    text.lines()
        .map(|line| format!("{indent}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn write_multiline_block_from_body(
    f: &mut impl std::fmt::Write,
    stmts: &[String],
) -> std::fmt::Result {
    write!(f, "{{\n")?;
    for stmt in stmts {
        write!(f, "{}\n", indent_multiline(stmt, "    "))?;
    }
    write!(f, "}}")
}

fn write_block_with_style(
    f: &mut impl std::fmt::Write,
    stmts: &[String],
    multiline: bool,
) -> std::fmt::Result {
    if multiline {
        write_multiline_block_from_body(f, stmts)
    } else if stmts.is_empty() {
        write!(f, "{{ }}")
    } else {
        write!(f, "{{ {} }}", stmts.join(" "))
    }
}

fn write_inline_block(
    f: &mut impl std::fmt::Write,
    stmts: &[WrappedAstStatement],
    config: AstPrintConfig,
) -> std::fmt::Result {
    let body = statement_body(stmts, config);
    if body.is_empty() {
        write!(f, "{{ }}")
    } else if body.len() > 1 || body.iter().any(|stmt| stmt.contains('\n')) {
        write_multiline_block_from_body(f, &body)
    } else {
        write!(f, "{{ {} }}", body.join(" "))
    }
}

fn trim_trailing_semicolon(text: &str) -> &str {
    let trimmed = text.trim_end();
    trimmed
        .strip_suffix(';')
        .map(str::trim_end)
        .unwrap_or(trimmed)
}

fn render_for_header_statement(stmt: &WrappedAstStatement, config: AstPrintConfig) -> String {
    let rendered = stmt.statement.to_string_with_config(Some(config));
    trim_trailing_semicolon(&rendered).to_string()
}

fn binary_operator_precedence(op: &AstBinaryOperator) -> u8 {
    match op {
        AstBinaryOperator::LogicOr => 1,
        AstBinaryOperator::LogicAnd => 2,
        AstBinaryOperator::BitOr => 3,
        AstBinaryOperator::BitXor => 4,
        AstBinaryOperator::BitAnd => 5,
        AstBinaryOperator::Equal | AstBinaryOperator::NotEqual => 6,
        AstBinaryOperator::Less
        | AstBinaryOperator::LessEqual
        | AstBinaryOperator::Greater
        | AstBinaryOperator::GreaterEqual => 7,
        AstBinaryOperator::LeftShift | AstBinaryOperator::RightShift => 8,
        AstBinaryOperator::Add | AstBinaryOperator::Sub => 9,
        AstBinaryOperator::Mul | AstBinaryOperator::Div | AstBinaryOperator::Mod => 10,
    }
}

fn render_binary_operand(
    expr: &Wrapped<AstExpression>,
    parent_op: &AstBinaryOperator,
    is_right_operand: bool,
    config: AstPrintConfig,
) -> String {
    let rendered = expr.to_string_with_config(Some(config));
    if let AstExpression::BinaryOp(child_op, _, _) = expr.as_ref() {
        let parent_precedence = binary_operator_precedence(parent_op);
        let child_precedence = binary_operator_precedence(child_op);
        let needs_parentheses = child_precedence < parent_precedence
            || (is_right_operand && child_precedence == parent_precedence);
        if needs_parentheses {
            format!("({rendered})")
        } else {
            rendered
        }
    } else {
        rendered
    }
}

fn render_prefixed_operand(expr: &Wrapped<AstExpression>, config: AstPrintConfig) -> String {
    let rendered = expr.to_string_with_config(Some(config));
    if matches!(expr.as_ref(), AstExpression::BinaryOp(_, _, _)) {
        format!("({rendered})")
    } else {
        rendered
    }
}

impl PrintWithConfig for AstValueType {
    fn to_string_with_config(&self, option: Option<AstPrintConfig>) -> String {
        let mut output = String::new();
        self.print(&mut output, option).unwrap();
        output
    }
    fn print(
        &self,
        f: &mut impl std::fmt::Write,
        config: Option<AstPrintConfig>,
    ) -> std::fmt::Result {
        let config = config.unwrap_or_default();
        match self {
            AstValueType::Void => write!(f, "void"),
            AstValueType::Unknown => write!(f, "unknown_t"),
            AstValueType::Int => write!(f, "int"),
            AstValueType::Int8 => write!(f, "int8_t"),
            AstValueType::Int16 => write!(f, "int16_t"),
            AstValueType::Int32 => write!(f, "int32_t"),
            AstValueType::Int64 => write!(f, "int64_t"),
            AstValueType::UInt => write!(f, "uint"),
            AstValueType::UInt8 => write!(f, "uint8_t"),
            AstValueType::UInt16 => write!(f, "uint16_t"),
            AstValueType::UInt32 => write!(f, "uint32_t"),
            AstValueType::UInt64 => write!(f, "uint64_t"),
            AstValueType::Char => write!(f, "char"),
            AstValueType::Float => write!(f, "float"),
            AstValueType::Double => write!(f, "double"),
            AstValueType::Bool => write!(f, "bool"),
            AstValueType::Pointer(t) => write!(f, "{}*", t.to_string_with_config(Some(config))),
            AstValueType::Array(t, size) => {
                write!(f, "{}[{}]", t.to_string_with_config(Some(config)), size)
            }
            AstValueType::Struct(name, _) => write!(f, "struct {}", name),
            AstValueType::Union(name, _) => write!(f, "union {}", name),
        }
    }
}

impl PrintWithConfig for AstStatement {
    fn to_string_with_config(&self, option: Option<AstPrintConfig>) -> String {
        let mut output = String::new();
        self.print(&mut output, option).unwrap();
        output
    }
    fn print(
        &self,
        f: &mut impl std::fmt::Write,
        config: Option<AstPrintConfig>,
    ) -> std::fmt::Result {
        let config = config.unwrap_or_default();
        match self {
            AstStatement::Declaration(var, None) => write!(
                f,
                "{} {};",
                var.var_type.to_string_with_config(Some(config)),
                var.name()
            ),
            AstStatement::Declaration(var, Some(expr)) => {
                write!(
                    f,
                    "{} {} = {};",
                    var.var_type.to_string_with_config(Some(config)),
                    var.name(),
                    expr.to_string_with_config(Some(config))
                )
            }
            AstStatement::Assignment(left, right) => write!(
                f,
                "{} = {};",
                left.to_string_with_config(Some(config)),
                right.to_string_with_config(Some(config))
            ),
            AstStatement::If(cond, then_body, else_body) => {
                let then_stmts = statement_body(then_body, config);
                let else_stmts = else_body.as_ref().map(|body| statement_body(body, config));
                if then_stmts.is_empty() && !config.print_empty_statement {
                    if else_stmts.as_ref().is_none_or(|body| body.is_empty()) {
                        return Ok(());
                    }
                }

                write!(f, "if ({}) ", cond.to_string_with_config(Some(config)))?;
                let then_multiline =
                    then_stmts.len() > 1 || then_stmts.iter().any(|stmt| stmt.contains('\n'));
                write_block_with_style(f, &then_stmts, then_multiline)?;
                if let Some(else_stmts) = else_stmts {
                    write!(f, " else ")?;
                    let else_multiline =
                        else_stmts.len() > 1 || else_stmts.iter().any(|stmt| stmt.contains('\n'));
                    write_block_with_style(f, &else_stmts, else_multiline)?;
                }
                Ok(())
            }
            AstStatement::While(cond, body) => {
                if body.is_empty() && !config.print_empty_statement {
                    return Ok(());
                }

                write!(f, "while ({}) ", cond.to_string_with_config(Some(config)))?;
                write_inline_block(f, body, config)
            }
            AstStatement::For(init, cond, update, body) => {
                if body.is_empty() && !config.print_empty_statement {
                    return Ok(());
                }

                let init_text = render_for_header_statement(init.as_ref(), config);
                let cond_text = cond.to_string_with_config(Some(config));
                let update_text = render_for_header_statement(update.as_ref(), config);
                write!(f, "for ({}; {}; {}) ", init_text, cond_text, update_text)?;
                write_inline_block(f, body, config)
            }
            AstStatement::Return(expr) => {
                if let Some(expr) = expr {
                    write!(f, "return {};", expr.to_string_with_config(Some(config)))
                } else {
                    write!(f, "return;")
                }
            }
            AstStatement::Call(call) => match call {
                AstCall::Variable {
                    var_map,
                    var_id,
                    args,
                    ..
                } => {
                    let var_map = var_map.read().unwrap();
                    let var = var_map.get(var_id).unwrap();
                    write!(f, "{}(", var.name())?;
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", arg.to_string_with_config(Some(config)))?;
                    }
                    write!(f, ");")
                }
                AstCall::Function { target, args } => {
                    write!(f, "{}(", target.get_default_name())?;
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", arg.to_string_with_config(Some(config)))?;
                    }
                    write!(f, ");")
                }
                AstCall::Builtin(func, arg) => {
                    let name: &str = match func {
                        AstBuiltinFunction::Print => "print",
                        AstBuiltinFunction::ArchBitPerByte => "ARCH_BIT_PER_BYTE",
                        AstBuiltinFunction::InstructionByteSize => "INSTRUCTION_BYTE_SIZE",
                        AstBuiltinFunction::ByteSizeOf => "byte_size_of",
                        AstBuiltinFunction::BitSizeOf => "bit_size_of",
                        AstBuiltinFunction::Sized => "sized",
                        AstBuiltinFunction::OperandExists => "operand_exists",
                        AstBuiltinFunction::SignedMax => "signed_max",
                        AstBuiltinFunction::SignedMin => "signed_min",
                        AstBuiltinFunction::UnsignedMax => "unsigned_max",
                        AstBuiltinFunction::UnsignedMin => "unsigned_min",
                        AstBuiltinFunction::BitOnes => "bit_ones",
                        AstBuiltinFunction::BitZeros => "bit_zeros",
                    };

                    let args: Vec<&Wrapped<AstExpression>> = match arg.as_ref() {
                        AstBuiltinFunctionArgument::None => Vec::new(),
                        AstBuiltinFunctionArgument::Print(args) => args.iter().collect(),
                        AstBuiltinFunctionArgument::ByteSizeOf(e)
                        | AstBuiltinFunctionArgument::BitSizeOf(e)
                        | AstBuiltinFunctionArgument::OperandExists(e)
                        | AstBuiltinFunctionArgument::SignedMax(e)
                        | AstBuiltinFunctionArgument::SignedMin(e)
                        | AstBuiltinFunctionArgument::UnsignedMax(e)
                        | AstBuiltinFunctionArgument::UnsignedMin(e)
                        | AstBuiltinFunctionArgument::BitOnes(e)
                        | AstBuiltinFunctionArgument::BitZeros(e) => vec![e],
                        AstBuiltinFunctionArgument::Sized(e1, e2) => vec![e1, e2],
                    };

                    write!(f, "{}(", name)?;
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", arg.to_string_with_config(Some(config)))?;
                    }
                    write!(f, ");")
                }
                AstCall::Unknown(name, args) => {
                    write!(f, "{}(", name)?;
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", arg.to_string_with_config(Some(config)))?;
                    }
                    write!(f, ");")
                }
            },
            AstStatement::Switch(discrim, cases, default) => {
                write!(
                    f,
                    "switch ({}) ",
                    discrim.to_string_with_config(Some(config))
                )?;
                write!(f, "{{\n")?;
                for (lit, case_body) in cases {
                    let body_strs = statement_body(case_body, config);
                    write!(f, "    case {}:\n", lit.to_string_with_config(Some(config)))?;
                    for s in &body_strs {
                        write!(f, "{}\n", indent_multiline(s, "        "))?;
                    }
                }
                if let Some(default_body) = default {
                    let body_strs = statement_body(default_body, config);
                    write!(f, "    default:\n")?;
                    for s in &body_strs {
                        write!(f, "{}\n", indent_multiline(s, "        "))?;
                    }
                }
                write!(f, "}}")
            }
            AstStatement::Label(name) => write!(f, "{}:", name),
            AstStatement::Goto(name) => {
                write!(f, "goto {};", name.to_string_with_config(Some(config)))
            }
            AstStatement::Block(stmts) => {
                if stmts.is_empty() && !config.print_empty_statement {
                    return Ok(());
                }

                write_inline_block(f, stmts, config)
            }
            AstStatement::Empty => {
                if config.print_empty_statement {
                    write!(f, ";")
                } else {
                    Ok(())
                }
            }
            AstStatement::Undefined => write!(f, "<UNDEFINED BEHAVIOR>"),
            AstStatement::Exception(e) => write!(f, "<EXCEPTION: {e}>"),
            AstStatement::Assembly(code) => write!(f, "<ASSEMBLY: {code}>"),
            AstStatement::Comment(comment) => write!(f, "/* {} */", comment),
            AstStatement::Ir(ir) => write!(f, "<IR: {ir}>"),
        }
    }
}

impl PrintWithConfig for AstExpression {
    fn to_string_with_config(&self, option: Option<AstPrintConfig>) -> String {
        let mut output = String::new();
        self.print(&mut output, option).unwrap();
        output
    }
    fn print(
        &self,
        f: &mut impl std::fmt::Write,
        config: Option<AstPrintConfig>,
    ) -> std::fmt::Result {
        let config = config.unwrap_or_default();
        match self {
            AstExpression::Literal(lit) => write!(f, "{}", lit.to_string_with_config(Some(config))),
            AstExpression::Variable(var_map, id) => {
                let var_map = var_map.read().unwrap();
                let var = var_map.get(id).unwrap();
                if config.replace_constant
                    && let Some(const_value) = &var.const_value
                {
                    write!(f, "{}", const_value.to_string_with_config(Some(config)))
                } else {
                    write!(f, "{}", var.name())
                }
            }
            AstExpression::UnaryOp(op, expr) => {
                let expr = render_prefixed_operand(expr, config);
                write!(f, "{}{}", op.to_string_with_config(Some(config)), expr)
            }
            AstExpression::BinaryOp(op, left, right) => {
                let left_text = render_binary_operand(left, op, false, config);
                let right_text = render_binary_operand(right, op, true, config);
                write!(
                    f,
                    "{} {} {}",
                    left_text,
                    op.to_string_with_config(Some(config)),
                    right_text
                )
            }
            AstExpression::Call(call) => match call {
                AstCall::Variable {
                    var_map,
                    var_id,
                    args,
                    ..
                } => {
                    let var_map = var_map.read().unwrap();
                    let var = var_map.get(var_id).unwrap();
                    write!(f, "{}(", var.name())?;
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", arg.to_string_with_config(Some(config)))?;
                    }
                    write!(f, ")")
                }
                AstCall::Function { target, args } => {
                    write!(f, "{}(", target.get_default_name())?;
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", arg.to_string_with_config(Some(config)))?;
                    }
                    write!(f, ")")
                }
                AstCall::Builtin(func, arg) => {
                    let name: &str = match func {
                        AstBuiltinFunction::Print => "print",
                        AstBuiltinFunction::ArchBitPerByte => "ARCH_BIT_PER_BYTE",
                        AstBuiltinFunction::InstructionByteSize => "INSTRUCTION_BYTE_SIZE",
                        AstBuiltinFunction::ByteSizeOf => "byte_size_of",
                        AstBuiltinFunction::BitSizeOf => "bit_size_of",
                        AstBuiltinFunction::Sized => "sized",
                        AstBuiltinFunction::OperandExists => "operand_exists",
                        AstBuiltinFunction::SignedMax => "signed_max",
                        AstBuiltinFunction::SignedMin => "signed_min",
                        AstBuiltinFunction::UnsignedMax => "unsigned_max",
                        AstBuiltinFunction::UnsignedMin => "unsigned_min",
                        AstBuiltinFunction::BitOnes => "bit_ones",
                        AstBuiltinFunction::BitZeros => "bit_zeros",
                    };

                    let args: Vec<&Wrapped<AstExpression>> = match arg.as_ref() {
                        AstBuiltinFunctionArgument::None => Vec::new(),
                        AstBuiltinFunctionArgument::Print(args) => args.iter().collect(),
                        AstBuiltinFunctionArgument::ByteSizeOf(e)
                        | AstBuiltinFunctionArgument::BitSizeOf(e)
                        | AstBuiltinFunctionArgument::OperandExists(e)
                        | AstBuiltinFunctionArgument::SignedMax(e)
                        | AstBuiltinFunctionArgument::SignedMin(e)
                        | AstBuiltinFunctionArgument::UnsignedMax(e)
                        | AstBuiltinFunctionArgument::UnsignedMin(e)
                        | AstBuiltinFunctionArgument::BitOnes(e)
                        | AstBuiltinFunctionArgument::BitZeros(e) => vec![e],
                        AstBuiltinFunctionArgument::Sized(e1, e2) => vec![e1, e2],
                    };

                    write!(f, "{}(", name)?;
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", arg.to_string_with_config(Some(config)))?;
                    }
                    write!(f, ")")
                }
                AstCall::Unknown(name, args) => {
                    write!(f, "{}(", name)?;
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", arg.to_string_with_config(Some(config)))?;
                    }
                    write!(f, ")")
                }
            },
            AstExpression::Unknown => write!(f, "<UNKNOWN DATA>"),
            AstExpression::Undefined => write!(f, "<UNDEFINED DATA>"),
            AstExpression::Cast(ctype, expression) => write!(
                f,
                "({}){}",
                ctype.to_string_with_config(Some(config)),
                render_prefixed_operand(expression, config)
            ),
            AstExpression::Deref(expression) => {
                write!(f, "*{}", render_prefixed_operand(expression, config))
            }
            AstExpression::AddressOf(expression) => {
                write!(f, "&{}", render_prefixed_operand(expression, config))
            }
            AstExpression::ArrayAccess(expression, expression1) => {
                write!(
                    f,
                    "{}[{}]",
                    render_prefixed_operand(expression, config),
                    expression1.to_string_with_config(Some(config))
                )
            }
            AstExpression::MemberAccess(expression, member) => write!(
                f,
                "{}.{}",
                render_prefixed_operand(expression, config),
                member
            ),
            AstExpression::Ternary(cond, true_expr, false_expr) => {
                write!(
                    f,
                    "{} ? {} : {}",
                    render_prefixed_operand(cond, config),
                    true_expr.to_string_with_config(Some(config)),
                    false_expr.to_string_with_config(Some(config))
                )
            }
            AstExpression::ArchitectureBitSize => write!(f, "ARCH_BIT_SIZE"),
            AstExpression::ArchitectureByteSize => write!(f, "ARCH_BYTE_SIZE"),
        }
    }
}
impl PrintWithConfig for AstLiteral {
    fn to_string_with_config(&self, option: Option<AstPrintConfig>) -> String {
        let mut output = String::new();
        self.print(&mut output, option).unwrap();
        output
    }
    fn print(
        &self,
        f: &mut impl std::fmt::Write,
        config: Option<AstPrintConfig>,
    ) -> std::fmt::Result {
        let _config = config.unwrap_or_default();
        match self {
            AstLiteral::Int(i) => write!(f, "{}", i),
            AstLiteral::UInt(u) => write!(f, "{}", u),
            AstLiteral::Float(fl) => write!(f, "{}", fl),
            AstLiteral::String(s) => write!(f, "\"{}\"", s),
            AstLiteral::Char(c) => write!(f, "'{}'", c),
            AstLiteral::Bool(b) => write!(f, "{}", b),
        }
    }
}
impl PrintWithConfig for AstUnaryOperator {
    fn to_string_with_config(&self, option: Option<AstPrintConfig>) -> String {
        let mut output = String::new();
        self.print(&mut output, option).unwrap();
        output
    }
    fn print(
        &self,
        f: &mut impl std::fmt::Write,
        config: Option<AstPrintConfig>,
    ) -> std::fmt::Result {
        let _config = config.unwrap_or_default();
        match self {
            AstUnaryOperator::Negate => write!(f, "-"),
            AstUnaryOperator::Not => write!(f, "!"),
            AstUnaryOperator::BitNot => write!(f, "~"),
            AstUnaryOperator::PreInc => write!(f, "++"),
            AstUnaryOperator::PreDec => write!(f, "--"),
            AstUnaryOperator::PostInc => write!(f, "++"),
            AstUnaryOperator::PostDec => write!(f, "--"),
            AstUnaryOperator::CastSigned => write!(f, "(signed)"),
            AstUnaryOperator::CastUnsigned => write!(f, "(unsigned)"),
        }
    }
}
impl PrintWithConfig for AstBinaryOperator {
    fn to_string_with_config(&self, option: Option<AstPrintConfig>) -> String {
        let mut output = String::new();
        self.print(&mut output, option).unwrap();
        output
    }
    fn print(
        &self,
        f: &mut impl std::fmt::Write,
        config: Option<AstPrintConfig>,
    ) -> std::fmt::Result {
        let _config = config.unwrap_or_default();
        match self {
            AstBinaryOperator::Add => write!(f, "+"),
            AstBinaryOperator::Sub => write!(f, "-"),
            AstBinaryOperator::Mul => write!(f, "*"),
            AstBinaryOperator::Div => write!(f, "/"),
            AstBinaryOperator::Mod => write!(f, "%"),
            AstBinaryOperator::BitAnd => write!(f, "&"),
            AstBinaryOperator::BitOr => write!(f, "|"),
            AstBinaryOperator::BitXor => write!(f, "^"),
            AstBinaryOperator::LogicAnd => write!(f, "&&"),
            AstBinaryOperator::LogicOr => write!(f, "||"),
            AstBinaryOperator::Equal => write!(f, "=="),
            AstBinaryOperator::NotEqual => write!(f, "!="),
            AstBinaryOperator::Less => write!(f, "<"),
            AstBinaryOperator::LessEqual => write!(f, "<="),
            AstBinaryOperator::Greater => write!(f, ">"),
            AstBinaryOperator::GreaterEqual => write!(f, ">="),
            AstBinaryOperator::LeftShift => write!(f, "<<"),
            AstBinaryOperator::RightShift => write!(f, ">>"),
        }
    }
}
impl PrintWithConfig for AstVariable {
    fn to_string_with_config(&self, option: Option<AstPrintConfig>) -> String {
        let mut output = String::new();
        self.print(&mut output, option).unwrap();
        output
    }
    fn print(
        &self,
        f: &mut impl std::fmt::Write,
        config: Option<AstPrintConfig>,
    ) -> std::fmt::Result {
        let config = config.unwrap_or_default();
        if config.replace_constant
            && let Some(const_value) = &self.const_value
        {
            write!(f, "{}", const_value.to_string_with_config(Some(config)))
        } else {
            write!(f, "{}", self.name())
        }
    }
}
impl PrintWithConfig for WrappedAstStatement {
    fn to_string_with_config(&self, option: Option<AstPrintConfig>) -> String {
        let mut output = String::new();
        self.print(&mut output, option).unwrap();
        output
    }
    fn print(
        &self,
        f: &mut impl std::fmt::Write,
        config: Option<AstPrintConfig>,
    ) -> std::fmt::Result {
        let config = config.unwrap_or_default();
        if let Some(comment) = &self.comment {
            write!(f, "/** {} */", comment)?;
        }
        write!(f, "{}", self.statement.to_string_with_config(Some(config)))
    }
}
impl<T: PrintWithConfig> PrintWithConfig for Wrapped<T> {
    fn to_string_with_config(&self, option: Option<AstPrintConfig>) -> String {
        let mut output = String::new();
        self.print(&mut output, option).unwrap();
        output
    }
    fn print(
        &self,
        f: &mut impl std::fmt::Write,
        config: Option<AstPrintConfig>,
    ) -> std::fmt::Result {
        let config = config.unwrap_or_default();
        match self.comment {
            Some(ref comment) => write!(
                f,
                "{} /* {} */",
                self.item.to_string_with_config(Some(config)),
                comment
            ),
            None => write!(f, "{}", self.item.to_string_with_config(Some(config))),
        }
    }
}

impl PrintWithConfig for AstJumpTarget {
    fn to_string_with_config(&self, option: Option<AstPrintConfig>) -> String {
        let mut output = String::new();
        self.print(&mut output, option).unwrap();
        output
    }
    fn print(
        &self,
        f: &mut impl std::fmt::Write,
        config: Option<AstPrintConfig>,
    ) -> std::fmt::Result {
        let config = config.unwrap_or_default();
        match self {
            AstJumpTarget::Variable {
                scope: _,
                var_map,
                var_id,
            } => {
                let var_map = var_map.read().unwrap();
                let var = var_map.get(var_id).unwrap();
                write!(f, "{}", var.to_string_with_config(Some(config)))
            }
            AstJumpTarget::Function { target } => write!(f, "{}", target.get_default_name()),
            AstJumpTarget::Instruction { target } => {
                write!(f, "ir{}", target.descriptor().ir_index())
            }
            AstJumpTarget::Unknown(name) => write!(f, "{}", name),
        }
    }
}
impl PrintWithConfig for AstValue {
    fn to_string_with_config(&self, option: Option<AstPrintConfig>) -> String {
        let mut output = String::new();
        self.print(&mut output, option).unwrap();
        output
    }
    fn print(
        &self,
        f: &mut impl std::fmt::Write,
        config: Option<AstPrintConfig>,
    ) -> std::fmt::Result {
        let config = config.unwrap_or_default();
        match self {
            AstValue::Void => write!(f, "()"),
            AstValue::Unknown => write!(f, "unknown_v"),
            AstValue::Undefined => write!(f, "undefined"),
            AstValue::Max => write!(f, "max"),
            AstValue::Min => write!(f, "min"),
            AstValue::Num(i) => {
                let i = i.to_u64_digits();
                if i.0 == Sign::Minus {
                    write!(f, "-0x{:X}", i.1.get(0).unwrap_or(&0))
                } else {
                    write!(f, "0x{:X}", i.1.get(0).unwrap_or(&0))
                }
            }
            AstValue::Char(c) => write!(f, "'{}'", c),
            AstValue::Double(d) => write!(f, "{}", d),
            AstValue::Bool(b) => write!(f, "{}", b),
            AstValue::Pointer(p) => write!(f, "*{}", p.to_string_with_config(Some(config))),
            AstValue::Array(arr) => {
                let arr_str: Vec<String> = arr
                    .iter()
                    .map(|v| v.to_string_with_config(Some(config)))
                    .collect();
                write!(f, "[{}]", arr_str.join(", "))
            }
        }
    }
}
