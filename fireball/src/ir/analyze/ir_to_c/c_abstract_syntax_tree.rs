//! Unusued are for optimization process

mod display;
mod optimize;
mod to_c_code;

use crate::{
    core::Address,
    ir::{analyze::MergedIr, data::IrData, utils::IrStatementDescriptor},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CAstPrintConfig {
    pub print_instruction: bool,
    pub print_ir: bool,
    pub print_empty_statement: bool,
}
impl Default for CAstPrintConfig {
    fn default() -> Self {
        Self {
            print_instruction: true,
            print_ir: true,
            print_empty_statement: false,
        }
    }
}
pub trait PrintWithConfig {
    fn to_string_with_config(&self, option: Option<CAstPrintConfig>) -> String;
    fn print(
        &self,
        f: &mut impl std::fmt::Write,
        config: Option<CAstPrintConfig>,
    ) -> std::fmt::Result;
}

pub type ArcFunctionMap = Arc<RwLock<HashMap<FunctionId, Function>>>;
pub type ArcVariableMap = Arc<RwLock<HashMap<VariableId, Variable>>>;

#[derive(Debug, Clone)]
pub struct AstDescriptor {
    ir: Arc<MergedIr>,
    descriptor: IrStatementDescriptor,
}
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
    pub from: Option<AstDescriptor>,
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
pub enum Literal {
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
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
    Instruction { target: AstDescriptor },
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
    /// A < B
    Less,
    /// A <= B
    LessEqual,
    /// A > B
    Greater,
    /// A >= B
    GreaterEqual,
    LeftShift,
    RightShift,
}

impl Default for CAst {
    fn default() -> Self {
        Self::new()
    }
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
            Err(DecompileError::Unknown(Some(
                "Tried to get variables from a non-existing function".to_string(),
            )))
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

impl AstDescriptor {
    pub fn new(ir: Arc<MergedIr>, descriptor: IrStatementDescriptor) -> Self {
        Self { ir, descriptor }
    }
    pub fn ir(&self) -> &Arc<MergedIr> {
        &self.ir
    }
    pub fn descriptor(&self) -> &IrStatementDescriptor {
        &self.descriptor
    }
}
