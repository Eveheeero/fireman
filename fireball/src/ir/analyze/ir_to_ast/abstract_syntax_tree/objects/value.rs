use crate::ir::analyze::ir_to_ast::abstract_syntax_tree::objects::*;
use num_bigint::BigInt;

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
