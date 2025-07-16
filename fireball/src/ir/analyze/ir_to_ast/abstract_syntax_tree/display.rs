use super::*;

// to_string_with_config(Some(config))

impl PrintWithConfig for CType {
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
            CType::Void => write!(f, "void"),
            CType::Unknown => write!(f, "unknown_t"),
            CType::Int => write!(f, "int"),
            CType::Int8 => write!(f, "int8_t"),
            CType::Int16 => write!(f, "int16_t"),
            CType::Int32 => write!(f, "int32_t"),
            CType::Int64 => write!(f, "int64_t"),
            CType::UInt => write!(f, "uint"),
            CType::UInt8 => write!(f, "uint8_t"),
            CType::UInt16 => write!(f, "uint16_t"),
            CType::UInt32 => write!(f, "uint32_t"),
            CType::UInt64 => write!(f, "uint64_t"),
            CType::Char => write!(f, "char"),
            CType::Float => write!(f, "float"),
            CType::Double => write!(f, "double"),
            CType::Bool => write!(f, "bool"),
            CType::Pointer(t) => write!(f, "{}*", t.to_string_with_config(Some(config))),
            CType::Array(t, size) => {
                write!(f, "{}[{}]", t.to_string_with_config(Some(config)), size)
            }
            CType::Struct(name, _) => write!(f, "struct {}", name),
            CType::Union(name, _) => write!(f, "union {}", name),
        }
    }
}

impl PrintWithConfig for Statement {
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
            Statement::Declaration(var, None) => write!(
                f,
                "{} {};",
                var.var_type.to_string_with_config(Some(config)),
                var.name
            ),
            Statement::Declaration(var, Some(expr)) => {
                write!(
                    f,
                    "{} {} = {};",
                    var.var_type.to_string_with_config(Some(config)),
                    var.name,
                    expr.to_string_with_config(Some(config))
                )
            }
            Statement::Assignment(left, right) => write!(
                f,
                "{} = {};",
                left.to_string_with_config(Some(config)),
                right.to_string_with_config(Some(config))
            ),
            Statement::If(cond, then_body, else_body) => {
                write!(f, "if ({}) {{ ", cond.to_string_with_config(Some(config)))?;
                for stmt in then_body {
                    write!(f, "{} ", stmt.to_string_with_config(Some(config)))?;
                }
                if let Some(else_stmts) = else_body {
                    write!(f, "}} else {{ ")?;
                    for stmt in else_stmts {
                        write!(f, "{} ", stmt.to_string_with_config(Some(config)))?;
                    }
                }
                write!(f, "}}")
            }
            Statement::While(cond, body) => {
                write!(
                    f,
                    "while ({}) {{ ",
                    cond.to_string_with_config(Some(config))
                )?;
                for stmt in body {
                    write!(f, "{} ", stmt.to_string_with_config(Some(config)))?;
                }
                write!(f, "}}")
            }
            Statement::For(init, cond, update, body) => {
                write!(f, "for (")?;
                if let Statement::Declaration(var, _) = init.as_ref().as_ref() {
                    write!(
                        f,
                        "{} {};",
                        var.var_type.to_string_with_config(Some(config)),
                        var.name
                    )?;
                } else {
                    write!(f, "{};", init.to_string_with_config(Some(config)))?;
                }
                write!(f, " {};", cond.to_string_with_config(Some(config)))?;
                if let Statement::Assignment(left, right) = update.as_ref().as_ref() {
                    write!(
                        f,
                        "{} = {};",
                        left.to_string_with_config(Some(config)),
                        right.to_string_with_config(Some(config))
                    )?;
                } else {
                    write!(f, "{};", update.to_string_with_config(Some(config)))?;
                }
                write!(f, ") {{ ")?;
                for stmt in body {
                    write!(f, "{} ", stmt.to_string_with_config(Some(config)))?;
                }
                write!(f, "}}")
            }
            Statement::Return(expr) => {
                if let Some(expr) = expr {
                    write!(f, "return {};", expr.to_string_with_config(Some(config)))
                } else {
                    write!(f, "return;")
                }
            }
            Statement::Call(name, args) => {
                write!(f, "{}(", name.to_string_with_config(Some(config)))?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg.to_string_with_config(Some(config)))?;
                }
                write!(f, ");")
            }
            Statement::Label(name) => write!(f, "{}:", name),
            Statement::Goto(name) => {
                write!(f, "goto {}; ", name.to_string_with_config(Some(config)))
            }
            Statement::Block(stmts) => {
                write!(f, "{{ ")?;
                for stmt in stmts {
                    write!(f, "{} ", stmt.to_string_with_config(Some(config)))?;
                }
                write!(f, "}}")
            }
            Statement::Empty => {
                if config.print_empty_statement {
                    write!(f, ";")
                } else {
                    Ok(())
                }
            }
            Statement::Undefined => write!(f, "<UNDEFINED BEHAVIOR>"),
            Statement::Exception(e) => write!(f, "<EXCEPTION: {e}>"),
            Statement::Assembly(code) => write!(f, "<ASSEMBLY: {code}>"),
            Statement::Comment(comment) => write!(f, "/* {} */", comment),
        }
    }
}

impl PrintWithConfig for Expression {
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
            Expression::Literal(lit) => write!(f, "{}", lit.to_string_with_config(Some(config))),
            Expression::Variable(var_map, id) => {
                let var_map = var_map.read().unwrap();
                let var = var_map.get(id).unwrap();
                write!(f, "{}", var.name)
            }
            Expression::UnaryOp(op, expr) => write!(
                f,
                "{}{}",
                op.to_string_with_config(Some(config)),
                expr.to_string_with_config(Some(config))
            ),
            Expression::BinaryOp(op, left, right) => write!(
                f,
                "({} {} {})",
                left.to_string_with_config(Some(config)),
                op.to_string_with_config(Some(config)),
                right.to_string_with_config(Some(config))
            ),
            Expression::Call(name, args) => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?
                    }
                    write!(f, "{}", arg.to_string_with_config(Some(config)))?;
                }
                write!(f, ")")
            }
            Expression::Unknown => write!(f, "<UNKNOWN DATA>"),
            Expression::Undefined => write!(f, "<UNDEFINED DATA>"),
            Expression::Cast(ctype, expression) => write!(
                f,
                "({}){}",
                ctype.to_string_with_config(Some(config)),
                expression.to_string_with_config(Some(config))
            ),
            Expression::Deref(expression) => {
                write!(f, "*{}", expression.to_string_with_config(Some(config)))
            }
            Expression::AddressOf(expression) => {
                write!(f, "&{}", expression.to_string_with_config(Some(config)))
            }
            Expression::ArrayAccess(expression, expression1) => {
                write!(
                    f,
                    "{}[{}]",
                    expression.to_string_with_config(Some(config)),
                    expression1.to_string_with_config(Some(config))
                )
            }
            Expression::MemberAccess(expression, member) => write!(
                f,
                "{}.{}",
                expression.to_string_with_config(Some(config)),
                member
            ),
            Expression::ArchitectureBitSize => write!(f, "ARCH_BIT_SIZE"),
            Expression::ArchitectureByteSize => write!(f, "ARCH_BYTE_SIZE"),
        }
    }
}
impl PrintWithConfig for Literal {
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
            Literal::Int(i) => write!(f, "{}", i),
            Literal::UInt(u) => write!(f, "{}", u),
            Literal::Float(fl) => write!(f, "{}", fl),
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Char(c) => write!(f, "'{}'", c),
            Literal::Bool(b) => write!(f, "{}", b),
        }
    }
}
impl PrintWithConfig for UnaryOperator {
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
            UnaryOperator::Negate => write!(f, "-"),
            UnaryOperator::Not => write!(f, "!"),
            UnaryOperator::BitNot => write!(f, "~"),
            UnaryOperator::PreInc => write!(f, "++"),
            UnaryOperator::PreDec => write!(f, "--"),
            UnaryOperator::PostInc => write!(f, "++"),
            UnaryOperator::PostDec => write!(f, "--"),
            UnaryOperator::CastSigned => write!(f, "(signed)"),
            UnaryOperator::CastUnsigned => write!(f, "(unsigned)"),
        }
    }
}
impl PrintWithConfig for BinaryOperator {
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
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Div => write!(f, "/"),
            BinaryOperator::Mod => write!(f, "%"),
            BinaryOperator::BitAnd => write!(f, "&"),
            BinaryOperator::BitOr => write!(f, "|"),
            BinaryOperator::BitXor => write!(f, "^"),
            BinaryOperator::LogicAnd => write!(f, "&&"),
            BinaryOperator::LogicOr => write!(f, "||"),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::Less => write!(f, "<"),
            BinaryOperator::LessEqual => write!(f, "<="),
            BinaryOperator::Greater => write!(f, ">"),
            BinaryOperator::GreaterEqual => write!(f, ">="),
            BinaryOperator::LeftShift => write!(f, "<<"),
            BinaryOperator::RightShift => write!(f, ">>"),
        }
    }
}
impl PrintWithConfig for Variable {
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
        write!(
            f,
            "{} {}",
            self.var_type.to_string_with_config(Some(config)),
            self.name
        )
    }
}
impl PrintWithConfig for WrappedStatement {
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

impl PrintWithConfig for JumpTarget {
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
            JumpTarget::Variable { scope: _, id } => write!(f, "var{:?}", id),
            JumpTarget::Function { target } => write!(f, "function{:?}", target),
            JumpTarget::Instruction { target } => {
                write!(f, "ir{}", target.descriptor().ir_index())
            }
            JumpTarget::Unknown(name) => write!(f, "{}", name),
        }
    }
}
impl PrintWithConfig for CValue {
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
            CValue::Void => write!(f, "()"),
            CValue::Unknown => write!(f, "unknown_v"),
            CValue::Undefined => write!(f, "undefined"),
            CValue::Max => write!(f, "max"),
            CValue::Min => write!(f, "min"),
            CValue::Num(i) => {
                let i = i.to_u64_digits();
                if i.0 == Sign::Minus {
                    write!(f, "-0x{:X}", i.1.get(0).unwrap_or(&0))
                } else {
                    write!(f, "0x{:X}", i.1.get(0).unwrap_or(&0))
                }
            }
            CValue::Char(c) => write!(f, "'{}'", c),
            CValue::Double(d) => write!(f, "{}", d),
            CValue::Bool(b) => write!(f, "{}", b),
            CValue::Pointer(p) => write!(f, "*{}", p.to_string_with_config(Some(config))),
            CValue::Array(arr) => {
                let arr_str: Vec<String> = arr
                    .iter()
                    .map(|v| v.to_string_with_config(Some(config)))
                    .collect();
                write!(f, "[{}]", arr_str.join(", "))
            }
        }
    }
}
