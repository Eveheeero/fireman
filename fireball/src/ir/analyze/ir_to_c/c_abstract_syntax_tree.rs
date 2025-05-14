//! Unusued are for optimization process

use crate::{
    ir::{data::IrData, utils::IrStatementDescriptor},
    utils::Aos,
};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct CAst {
    pub static_variables: HashMap<VariableId, Variable>,
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Option<String>,
    pub id: FunctionId,
    pub return_type: CType,
    pub parameters: Vec<Variable>,
    pub variables: HashMap<VariableId, Variable>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum WrappedItem<T> {
    FromIrData {
        item: T,
        from: Aos<IrData>,
        comment: String,
    },
    FromIrStatement {
        item: T,
        from: IrStatementDescriptor,
        comment: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: Option<String>,
    pub id: VariableId,
    pub var_type: CType,
    pub is_const: bool,
}

pub type VariableId = u32;
pub type FunctionId = u64;

#[derive(Debug, Clone, PartialEq)]
pub enum CType {
    Void,
    Unknown,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Char,
    Float,
    Double,
    Pointer(Box<CType>),
    Array(Box<CType>, usize),
    Struct(String, Vec<Variable>),
    Union(String, Vec<Variable>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Declaration(Variable, Option<Expression>),
    Assignment(Expression, Expression),
    If(Expression, Vec<Statement>, Option<Vec<Statement>>),
    While(Expression, Vec<Statement>),
    For(Box<Statement>, Expression, Box<Statement>, Vec<Statement>),
    Return(Option<Expression>),
    Call(String, Vec<Expression>),
    Label(String),
    Goto(String),
    Block(Vec<Statement>),
    Undefined,
    Exception(&'static str),
    Empty,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Unknown,
    Undefined,
    ArchitectureBitSize,
    ArchitectureByteSize,
    Comment(String),
    Literal(Literal),
    Variable(VariableId),
    UnaryOp(UnaryOperator, Box<Expression>),
    BinaryOp(BinaryOperator, Box<Expression>, Box<Expression>),
    Call(String, Vec<Expression>),
    Cast(CType, Box<Expression>),
    Deref(Box<Expression>),
    AddressOf(Box<Expression>),
    ArrayAccess(Box<Expression>, Box<Expression>),
    MemberAccess(Box<Expression>, String),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,  // -
    Not,     // !
    BitNot,  // ~
    PreInc,  // ++x
    PreDec,  // --x
    PostInc, // x++
    PostDec, // x--
    CastSigned,
    CastUnsigned,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    LogicAnd,
    LogicOr,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    LeftShift,
    RightShift,
}

impl CAst {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            static_variables: HashMap::new(),
        }
    }

    pub fn optimize(&mut self) {
        // TODO: Implement optimization passes:
        // 1. Dead code elimination
        // 2. Constant folding
        // 3. Common subexpression elimination
        // 4. Loop optimization
        // 5. Function inlining
    }

    pub fn to_c_code(&self) -> String {
        let mut output = String::new();

        // Global variables
        for (&id, var) in self.static_variables.iter() {
            output.push_str(&format!(
                "{}{} g{};\n",
                if var.is_const { "const " } else { "" },
                var.var_type.to_string(),
                id
            ));
        }

        output.push_str("\n");

        // Functions
        for func in &self.functions {
            output.push_str(&format!("{} f{}(", func.return_type.to_string(), func.id));

            // Parameters
            if !func.parameters.is_empty() {
                let params: Vec<String> = func
                    .parameters
                    .iter()
                    .map(|p| {
                        format!(
                            "{}{} v{}",
                            if p.is_const { "const " } else { "" },
                            p.var_type.to_string(),
                            p.id
                        )
                    })
                    .collect();
                output.push_str(&params.join(", "));
            }

            output.push_str(") {\n");

            // Local variables
            for (&id, var) in func.variables.iter() {
                output.push_str(&format!(
                    "{}{} v{};\n",
                    if var.is_const { "const " } else { "" },
                    var.var_type.to_string(),
                    id
                ));
            }
            output.push_str("\n");

            // Function body
            for stmt in &func.body {
                output.push_str(&format!("    {}\n", stmt.to_string()));
            }

            output.push_str("}\n\n");
        }

        output
    }
}

// Implement Display traits for pretty printing
impl std::fmt::Display for CType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CType::Void => write!(f, "void"),
            CType::Unknown => write!(f, "unknown_t"),
            CType::Int8 => write!(f, "int8_t"),
            CType::Int16 => write!(f, "int16_t"),
            CType::Int32 => write!(f, "int32_t"),
            CType::Int64 => write!(f, "int64_t"),
            CType::UInt8 => write!(f, "uint8_t"),
            CType::UInt16 => write!(f, "uint16_t"),
            CType::UInt32 => write!(f, "uint32_t"),
            CType::UInt64 => write!(f, "uint64_t"),
            CType::Char => write!(f, "char"),
            CType::Float => write!(f, "float"),
            CType::Double => write!(f, "double"),
            CType::Pointer(t) => write!(f, "{}*", t),
            CType::Array(t, size) => write!(f, "{}[{}]", t, size),
            CType::Struct(name, _) => write!(f, "struct {}", name),
            CType::Union(name, _) => write!(f, "union {}", name),
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Declaration(var, None) => write!(f, "{} v{};", var.var_type, var.id),
            Statement::Declaration(var, Some(expr)) => {
                write!(f, "{} v{} = {};", var.var_type, var.id, expr)
            }
            Statement::Assignment(left, right) => write!(f, "{} = {};", left, right),
            Statement::If(cond, then_body, else_body) => {
                write!(f, "if ({}) {{ ", cond)?;
                for stmt in then_body {
                    write!(f, "{} ", stmt)?;
                }
                if let Some(else_stmts) = else_body {
                    write!(f, "}} else {{ ")?;
                    for stmt in else_stmts {
                        write!(f, "{} ", stmt)?;
                    }
                }
                write!(f, "}}")
            }
            Statement::While(cond, body) => {
                write!(f, "while ({}) {{ ", cond)?;
                for stmt in body {
                    write!(f, "{} ", stmt)?;
                }
                write!(f, "}}")
            }
            Statement::For(init, cond, update, body) => {
                write!(f, "for (")?;
                if let Statement::Declaration(var, _) = &**init {
                    write!(f, "{} v{};", var.var_type, var.id)?;
                } else {
                    write!(f, "{};", init)?;
                }
                write!(f, " {};", cond)?;
                if let Statement::Assignment(left, right) = &**update {
                    write!(f, "{} = {};", left, right)?;
                } else {
                    write!(f, "{};", update)?;
                }
                write!(f, ") {{ ")?;
                for stmt in body {
                    write!(f, "{} ", stmt)?;
                }
                write!(f, "}}")
            }
            Statement::Return(expr) => {
                if let Some(expr) = expr {
                    write!(f, "return {};", expr)
                } else {
                    write!(f, "return;")
                }
            }
            Statement::Call(name, args) => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ");")
            }
            Statement::Label(name) => write!(f, "{}:", name),
            Statement::Goto(name) => write!(f, "goto {}; ", name),
            Statement::Block(stmts) => {
                write!(f, "{{ ")?;
                for stmt in stmts {
                    write!(f, "{} ", stmt)?;
                }
                write!(f, "}}")
            }
            Statement::Empty => write!(f, ";"),
            Statement::Undefined => write!(f, "<UNDEFINED BEHAVIOR>"),
            Statement::Exception(e) => write!(f, "<EXCEPTION: {e}>"),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(lit) => write!(f, "{}", lit),
            Expression::Variable(id) => write!(f, "v{}", id),
            Expression::UnaryOp(op, expr) => write!(f, "{}{}", op, expr),
            Expression::BinaryOp(op, left, right) => write!(f, "({} {} {})", left, op, right),
            Expression::Call(name, args) => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expression::Unknown => write!(f, "<UNKNOWN DATA>"),
            Expression::Undefined => write!(f, "<UNDEFINED DATA>"),
            Expression::Cast(ctype, expression) => write!(f, "({}){}", ctype, expression),
            Expression::Deref(expression) => write!(f, "*{}", expression),
            Expression::AddressOf(expression) => write!(f, "&{}", expression),
            Expression::ArrayAccess(expression, expression1) => {
                write!(f, "{}[{}]", expression, expression1)
            }
            Expression::MemberAccess(expression, member) => write!(f, "{}.{}", expression, member),
            Expression::ArchitectureBitSize => write!(f, "ARCH_BIT_SIZE"),
            Expression::ArchitectureByteSize => write!(f, "ARCH_BYTE_SIZE"),
            Expression::Comment(comment) => write!(f, "// {}", comment),
        }
    }
}
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} v{}", self.var_type, self.id)
    }
}
