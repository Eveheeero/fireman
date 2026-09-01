use crate::abstract_syntax_tree::objects::*;
use hashbrown::HashMap;
use std::sync::{Arc, RwLock};

pub type AstFunctionMap = HashMap<AstFunctionId, AstFunction>;
pub type ArcAstVariableMap = Arc<RwLock<HashMap<AstVariableId, AstVariable>>>;

#[derive(Debug, Clone)]
pub enum AstJumpTarget {
    Variable {
        scope: AstFunctionId,
        var_map: ArcAstVariableMap,
        var_id: AstVariableId,
    },
    Function {
        target: AstFunctionId,
    },
    Unknown(String),
}

#[derive(Debug, Clone)]
pub enum AstCall {
    Variable {
        scope: AstFunctionId,
        var_map: ArcAstVariableMap,
        var_id: AstVariableId,
        args: Vec<Wrapped<AstExpression>>,
    },
    Function {
        target: AstFunctionId,
        args: Vec<Wrapped<AstExpression>>,
    },
    Builtin(AstBuiltinFunction, Box<AstBuiltinFunctionArgument>),
    Unknown(String, Vec<Wrapped<AstExpression>>),
}

#[derive(Debug, Clone)]
pub enum AstBuiltinFunctionArgument {
    None,

    Print(Vec<Wrapped<AstExpression>>),
    ByteSizeOf(Wrapped<AstExpression>),
    BitSizeOf(Wrapped<AstExpression>),
    Sized(Wrapped<AstExpression>, Wrapped<AstExpression>),
    OperandExists(Wrapped<AstExpression>),

    SignedMax(Wrapped<AstExpression>),
    SignedMin(Wrapped<AstExpression>),
    UnsignedMax(Wrapped<AstExpression>),
    UnsignedMin(Wrapped<AstExpression>),

    BitOnes(Wrapped<AstExpression>),
    BitZeros(Wrapped<AstExpression>),
}

#[derive(Debug, Clone)]
pub enum AstBuiltinFunction {
    ArchBitPerByte,
    InstructionByteSize,

    Print,
    ByteSizeOf,
    BitSizeOf,
    Sized,
    OperandExists,

    SignedMax,
    SignedMin,
    UnsignedMax,
    UnsignedMin,

    BitOnes,
    BitZeros,
}
