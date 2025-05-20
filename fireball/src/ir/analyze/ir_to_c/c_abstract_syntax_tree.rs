//! Unusued are for optimization process

use crate::{
    core::Address,
    ir::{data::IrData, utils::IrStatementDescriptor},
    prelude::*,
    utils::Aos,
};
use hashbrown::HashMap;
use num_bigint::{BigInt, Sign};
use std::{
    ops::Deref,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub struct CAst {
    pub static_variables: ArcVariableMap,
    pub functions: ArcFunctionMap,
    pub last_variable_id: HashMap<FunctionId, u32>,
}

pub type ArcFunctionMap = Arc<RwLock<HashMap<FunctionId, Function>>>;
pub type ArcVariableMap = Arc<RwLock<HashMap<VariableId, Variable>>>;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub id: FunctionId,
    pub return_type: CType,
    pub parameters: Vec<Variable>,
    pub variables: ArcVariableMap,
    pub body: Vec<WrappedStatement>,
}

#[derive(Debug, Clone)]
pub struct WrappedStatement {
    pub statement: Statement,
    pub from: Option<IrStatementDescriptor>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wrapped<T> {
    pub item: T,
    pub origin_expr: Option<Aos<IrData>>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub id: VariableId,
    pub var_type: CType,
    pub const_value: Option<Wrapped<CValue>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct VariableId {
    index: u32,
    parent: Option<FunctionId>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct FunctionId {
    address: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CType {
    Void,
    Unknown,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Char,
    Float,
    Double,
    Bool,
    Pointer(Box<CType>),
    Array(Box<CType>, usize),
    Struct(String, Vec<Variable>),
    Union(String, Vec<Variable>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CValue {
    Void,
    Unknown,
    Undefined,
    Max,
    Min,
    Num(BigInt),
    Char(char),
    Double(f64),
    Bool(bool),
    Pointer(Box<Wrapped<CValue>>),
    Array(Vec<Wrapped<CValue>>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Declaration(Variable, Option<Wrapped<Expression>>),
    Assignment(Wrapped<Expression>, Wrapped<Expression>),
    If(
        Wrapped<Expression>,
        Vec<WrappedStatement>,
        Option<Vec<WrappedStatement>>,
    ),
    While(Wrapped<Expression>, Vec<WrappedStatement>),
    For(
        Box<WrappedStatement>,
        Wrapped<Expression>,
        Box<WrappedStatement>,
        Vec<WrappedStatement>,
    ),
    Return(Option<Wrapped<Expression>>),
    Call(JumpTarget, Vec<Wrapped<Expression>>),
    Label(String /* TODO need to change */),
    Goto(JumpTarget),
    Block(Vec<WrappedStatement>),
    Assembly(String),
    Undefined,
    Exception(&'static str),
    Comment(String),
    Empty,
}
#[derive(Debug, Clone)]
pub enum JumpTarget {
    Variable { scope: FunctionId, id: VariableId },
    Function { target: FunctionId },
    Instruction { target: IrStatementDescriptor },
    Unknown(String),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Unknown,
    Undefined,
    ArchitectureBitSize,
    ArchitectureByteSize,
    Literal(Literal),
    Variable(ArcVariableMap, VariableId),
    UnaryOp(UnaryOperator, Box<Wrapped<Expression>>),
    BinaryOp(
        BinaryOperator,
        Box<Wrapped<Expression>>,
        Box<Wrapped<Expression>>,
    ),
    Call(String, Vec<Wrapped<Expression>>),
    Cast(CType, Box<Wrapped<Expression>>),
    Deref(Box<Wrapped<Expression>>),
    AddressOf(Box<Wrapped<Expression>>),
    ArrayAccess(Box<Wrapped<Expression>>, Box<Wrapped<Expression>>),
    MemberAccess(Box<Wrapped<Expression>>, String),
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
            functions: Arc::new(RwLock::new(HashMap::new())),
            static_variables: Arc::new(RwLock::new(HashMap::new())),
            last_variable_id: HashMap::new(),
        }
    }

    pub fn generate_default_function(&mut self, start_address: &Address) -> FunctionId {
        let id = FunctionId {
            address: start_address.get_virtual_address(),
        };
        let name = id.get_default_name();
        let func = Function {
            name,
            id,
            return_type: CType::Void,
            parameters: Vec::new(),
            variables: Arc::new(RwLock::new(HashMap::new())),
            body: Vec::new(),
        };
        self.functions.write().unwrap().insert(id, func.clone());
        id
    }
    pub fn new_variable_id(&mut self, current_function: &FunctionId) -> VariableId {
        let last_index = self.last_variable_id.entry(*current_function).or_insert(0);
        *last_index += 1;
        VariableId {
            index: *last_index,
            parent: Some(*current_function),
        }
    }
    pub fn get_variables(
        &self,
        function_id: &FunctionId,
    ) -> Result<ArcVariableMap, DecompileError> {
        if let Some(func) = self.functions.read().unwrap().get(function_id) {
            Ok(func.variables.clone())
        } else {
            error!(
                "Tried to get variables from a non-existing function: {:?}",
                function_id
            );
            Err(DecompileError::UnknownWithMessage(
                "Tried to get variables from a non-existing function".to_string(),
            ))
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
        for var in self.static_variables.read().unwrap().values() {
            if let Some(const_value) = &var.const_value {
                output.push_str(&format!(
                    "const {} {} = {};\n",
                    var.var_type.to_string(),
                    var.name,
                    const_value
                ));
            } else {
                output.push_str(&format!("{} {};\n", var.var_type.to_string(), var.name));
            }
        }

        output.push_str("\n");

        // Functions
        for func in self.functions.read().unwrap().values() {
            output.push_str(&format!("{} {}(", func.return_type.to_string(), func.name));

            // Parameters
            if !func.parameters.is_empty() {
                let params: Vec<String> = func
                    .parameters
                    .iter()
                    .map(|var| {
                        if let Some(const_value) = &var.const_value {
                            format!(
                                "const {} {} = {};\n",
                                var.var_type.to_string(),
                                var.name,
                                const_value
                            )
                        } else {
                            format!("{} {};\n", var.var_type.to_string(), var.name)
                        }
                    })
                    .collect();
                output.push_str(&params.join(", "));
            }

            output.push_str(") {\n");

            // Local variables
            for var in func.variables.read().unwrap().values() {
                if let Some(const_value) = &var.const_value {
                    output.push_str(&format!(
                        "const {} {} = {};\n",
                        var.var_type.to_string(),
                        var.name,
                        const_value
                    ));
                } else {
                    output.push_str(&format!("{} {};\n", var.var_type.to_string(), var.name));
                }
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
            Statement::Declaration(var, None) => write!(f, "{} {};", var.var_type, var.name),
            Statement::Declaration(var, Some(expr)) => {
                write!(f, "{} {} = {};", var.var_type, var.name, expr)
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
                if let Statement::Declaration(var, _) = init.as_ref().as_ref() {
                    write!(f, "{} {};", var.var_type, var.name)?;
                } else {
                    write!(f, "{};", init)?;
                }
                write!(f, " {};", cond)?;
                if let Statement::Assignment(left, right) = update.as_ref().as_ref() {
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
            Statement::Assembly(code) => write!(f, "<ASSEMBLY: {code}>"),
            Statement::Comment(comment) => write!(f, "/* {} */", comment),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(lit) => write!(f, "{}", lit),
            Expression::Variable(var_map, id) => {
                let var_map = var_map.read().unwrap();
                let var = var_map.get(id).unwrap();
                write!(f, "{}", var.name)
            }
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
        write!(f, "{} {}", self.var_type, self.name)
    }
}
impl std::fmt::Display for WrappedStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(comment) = &self.comment {
            write!(f, "/** {} */", comment)?;
        }
        write!(f, "{}", self.statement)
    }
}
impl<T: std::fmt::Display> std::fmt::Display for Wrapped<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.comment {
            Some(ref comment) => write!(f, "{} /* {} */", self.item, comment),
            None => write!(f, "{}", self.item),
        }
    }
}
impl AsRef<Statement> for WrappedStatement {
    fn as_ref(&self) -> &Statement {
        &self.statement
    }
}
impl Deref for WrappedStatement {
    type Target = Statement;

    fn deref(&self) -> &Self::Target {
        &self.statement
    }
}
impl<T> AsRef<T> for Wrapped<T> {
    fn as_ref(&self) -> &T {
        &self.item
    }
}
impl<T> Deref for Wrapped<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}
impl VariableId {
    pub fn get_default_name(&self) -> String {
        if self.parent.is_some() {
            format!("v{}", self.index)
        } else {
            format!("g{}", self.index)
        }
    }
}
impl FunctionId {
    pub fn get_default_name(&self) -> String {
        format!("f{}", self.address)
    }
}
impl std::fmt::Display for JumpTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JumpTarget::Variable { scope: _, id } => write!(f, "var{:?}", id),
            JumpTarget::Function { target } => write!(f, "function{:?}", target),
            JumpTarget::Instruction { target } => write!(f, "ir{}", target.ir_index()),
            JumpTarget::Unknown(name) => write!(f, "{}", name),
        }
    }
}
impl std::fmt::Display for CValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CValue::Void => write!(f, "()"),
            CValue::Unknown => write!(f, "unknown_v"),
            CValue::Undefined => write!(f, "undefined"),
            CValue::Max => write!(f, "max"),
            CValue::Min => write!(f, "min"),
            CValue::Num(i) => {
                let i = i.to_u64_digits();
                if i.0 == Sign::Minus {
                    write!(f, "-0x{:X}", i.1[0])
                } else {
                    write!(f, "0x{:X}", i.1[0])
                }
            }
            CValue::Char(c) => write!(f, "'{}'", c),
            CValue::Double(d) => write!(f, "{}", d),
            CValue::Bool(b) => write!(f, "{}", b),
            CValue::Pointer(p) => write!(f, "*{}", p),
            CValue::Array(arr) => {
                let arr_str: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", arr_str.join(", "))
            }
        }
    }
}
impl CValue {
    pub fn num(&self) -> Option<&BigInt> {
        match self {
            CValue::Num(i) => Some(i),
            _ => None,
        }
    }
    pub fn char(&self) -> Option<&char> {
        match self {
            CValue::Char(c) => Some(c),
            _ => None,
        }
    }
    pub fn double(&self) -> Option<&f64> {
        match self {
            CValue::Double(d) => Some(d),
            _ => None,
        }
    }
    pub fn bool(&self) -> Option<&bool> {
        match self {
            CValue::Bool(b) => Some(b),
            _ => None,
        }
    }
}
