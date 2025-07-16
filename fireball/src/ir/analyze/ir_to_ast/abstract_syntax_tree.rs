//! Unusued are for optimization process

mod display;
mod optimize;
mod print;

use crate::{
    ir::{analyze::IrFunction, data::IrData, utils::IrStatementDescriptor},
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
pub struct Ast {
    pub static_variables: ArcAstVariableMap,
    pub functions: ArcAstFunctionMap,
    pub last_variable_id: HashMap<AstFunctionId, u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AstPrintConfig {
    pub print_instruction: bool,
    pub print_ir: bool,
    pub print_empty_statement: bool,
}
impl Default for AstPrintConfig {
    fn default() -> Self {
        Self {
            print_instruction: true,
            print_ir: true,
            print_empty_statement: false,
        }
    }
}
pub trait PrintWithConfig {
    fn to_string_with_config(&self, option: Option<AstPrintConfig>) -> String;
    fn print(
        &self,
        f: &mut impl std::fmt::Write,
        config: Option<AstPrintConfig>,
    ) -> std::fmt::Result;
}

pub type ArcAstFunctionMap = Arc<RwLock<HashMap<AstFunctionId, AstFunction>>>;
pub type ArcAstVariableMap = Arc<RwLock<HashMap<AstVariableId, AstVariable>>>;

#[derive(Debug, Clone)]
pub struct AstDescriptor {
    ir: Arc<IrFunction>,
    descriptor: IrStatementDescriptor,
}
#[derive(Debug, Clone)]
pub struct AstFunction {
    pub name: String,
    pub id: AstFunctionId,
    pub ir: Arc<IrFunction>,
    pub return_type: AstValueType,
    pub parameters: Vec<AstVariable>,
    pub variables: ArcAstVariableMap,
    pub body: Vec<WrappedAstStatement>,
}

#[derive(Debug, Clone)]
pub struct WrappedAstStatement {
    pub statement: AstStatement,
    pub from: Option<AstDescriptor>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wrapped<T> {
    pub item: T,
    pub origin: AstValueOrigin,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstValueOrigin {
    UserInput,
    /// TODO predefined by files. like `func libc::strlen(str: char*) -> usize for ir [o1 = o1 + o2; ...]...` or `for asm [...]...`
    PreDefined,
    Expression(Aos<IrData>),
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstVariable {
    pub name: String,
    pub id: AstVariableId,
    pub var_type: AstValueType,
    pub const_value: Option<Wrapped<AstValue>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct AstVariableId {
    index: u32,
    parent: Option<AstFunctionId>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct AstFunctionId {
    address: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstValueType {
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
    Pointer(Box<AstValueType>),
    Array(Box<AstValueType>, usize),
    Struct(String, Vec<AstVariable>),
    Union(String, Vec<AstVariable>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstValue {
    Void,
    Unknown,
    Undefined,
    Max,
    Min,
    Num(BigInt),
    Char(char),
    Double(f64),
    Bool(bool),
    Pointer(Box<Wrapped<AstValue>>),
    Array(Vec<Wrapped<AstValue>>),
}

#[derive(Debug, Clone)]
pub enum AstLiteral {
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub enum AstStatement {
    Declaration(AstVariable, Option<Wrapped<AstExpression>>),
    Assignment(Wrapped<AstExpression>, Wrapped<AstExpression>),
    If(
        Wrapped<AstExpression>,
        Vec<WrappedAstStatement>,
        Option<Vec<WrappedAstStatement>>,
    ),
    While(Wrapped<AstExpression>, Vec<WrappedAstStatement>),
    For(
        Box<WrappedAstStatement>,
        Wrapped<AstExpression>,
        Box<WrappedAstStatement>,
        Vec<WrappedAstStatement>,
    ),
    Return(Option<Wrapped<AstExpression>>),
    Call(AstJumpTarget, Vec<Wrapped<AstExpression>>),
    Label(String /* TODO need to change */),
    Goto(AstJumpTarget),
    Block(Vec<WrappedAstStatement>),
    Assembly(String),
    Undefined,
    Exception(&'static str),
    Comment(String),
    Empty,
}
#[derive(Debug, Clone)]
pub enum AstJumpTarget {
    Variable {
        scope: AstFunctionId,
        id: AstVariableId,
    },
    Function {
        target: AstFunctionId,
    },
    Instruction {
        target: AstDescriptor,
    },
    Unknown(String),
}

#[derive(Debug, Clone)]
pub enum AstExpression {
    Unknown,
    Undefined,
    ArchitectureBitSize,
    ArchitectureByteSize,
    Literal(AstLiteral),
    Variable(ArcAstVariableMap, AstVariableId),
    UnaryOp(AstUnaryOperator, Box<Wrapped<AstExpression>>),
    BinaryOp(
        AstBinaryOperator,
        Box<Wrapped<AstExpression>>,
        Box<Wrapped<AstExpression>>,
    ),
    Call(String, Vec<Wrapped<AstExpression>>),
    Cast(AstValueType, Box<Wrapped<AstExpression>>),
    Deref(Box<Wrapped<AstExpression>>),
    AddressOf(Box<Wrapped<AstExpression>>),
    ArrayAccess(Box<Wrapped<AstExpression>>, Box<Wrapped<AstExpression>>),
    MemberAccess(Box<Wrapped<AstExpression>>, String),
}

#[derive(Debug, Clone)]
pub enum AstUnaryOperator {
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
pub enum AstBinaryOperator {
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

impl Ast {
    pub fn new() -> Self {
        Self {
            functions: Arc::new(RwLock::new(HashMap::new())),
            static_variables: Arc::new(RwLock::new(HashMap::new())),
            last_variable_id: HashMap::new(),
        }
    }

    pub fn generate_default_function(&mut self, ir: Arc<IrFunction>) -> AstFunctionId {
        let start_address = ir.get_ir().first().map(|x| &x.address).unwrap();
        let id = AstFunctionId {
            address: start_address.get_virtual_address(),
        };
        let name = id.get_default_name();
        let func = AstFunction {
            name,
            id,
            ir,
            return_type: AstValueType::Void,
            parameters: Vec::new(),
            variables: Arc::new(RwLock::new(HashMap::new())),
            body: Vec::new(),
        };
        self.functions.write().unwrap().insert(id, func.clone());
        id
    }
    pub fn new_variable_id(&mut self, current_function: &AstFunctionId) -> AstVariableId {
        let last_index = self.last_variable_id.entry(*current_function).or_insert(0);
        *last_index += 1;
        AstVariableId {
            index: *last_index,
            parent: Some(*current_function),
        }
    }
    pub fn get_variables(
        &self,
        function_id: &AstFunctionId,
    ) -> Result<ArcAstVariableMap, DecompileError> {
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

impl AstValue {
    pub fn num(&self) -> Option<&BigInt> {
        match self {
            AstValue::Num(i) => Some(i),
            _ => None,
        }
    }
    pub fn char(&self) -> Option<&char> {
        match self {
            AstValue::Char(c) => Some(c),
            _ => None,
        }
    }
    pub fn double(&self) -> Option<&f64> {
        match self {
            AstValue::Double(d) => Some(d),
            _ => None,
        }
    }
    pub fn bool(&self) -> Option<&bool> {
        match self {
            AstValue::Bool(b) => Some(b),
            _ => None,
        }
    }
}
impl AsRef<AstStatement> for WrappedAstStatement {
    fn as_ref(&self) -> &AstStatement {
        &self.statement
    }
}
impl Deref for WrappedAstStatement {
    type Target = AstStatement;

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
impl AstVariableId {
    pub fn get_default_name(&self) -> String {
        if self.parent.is_some() {
            format!("v{}", self.index)
        } else {
            format!("g{}", self.index)
        }
    }
}
impl AstFunctionId {
    pub fn get_default_name(&self) -> String {
        format!("f{}", self.address)
    }
}

impl AstDescriptor {
    pub fn new(ir: Arc<IrFunction>, descriptor: IrStatementDescriptor) -> Self {
        Self { ir, descriptor }
    }
    pub fn ir(&self) -> &Arc<IrFunction> {
        &self.ir
    }
    pub fn descriptor(&self) -> &IrStatementDescriptor {
        &self.descriptor
    }
}
