use crate::{
    abstract_syntax_tree::objects::*,
    ir::{analyze::IrFunction, data::IrData, utils::IrStatementDescriptor},
    utils::Aos,
};
use std::{ops::Deref, sync::Arc};

#[derive(Debug, Clone)]
pub struct WrappedAstStatement {
    pub statement: AstStatement,
    pub origin: AstStatementOrigin,
    pub comment: Option<String>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Wrapped<T> {
    pub item: T,
    pub origin: AstValueOrigin,
    pub comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AstDescriptor {
    ir: Arc<IrFunction>,
    descriptor: IrStatementDescriptor,
}
#[derive(Debug, Clone)]
pub enum AstStatementOrigin {
    UserInput,
    PreDefined,
    Ir(AstDescriptor),
    Combination(Vec<AstStatementOrigin>),
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstValueOrigin {
    UserInput,
    /// TODO predefined by files. like `func libc::strlen(str: char*) -> usize for ir [o1 = o1 + o2; ...]...` or `for asm [...]...`
    PreDefined,
    Expression(Aos<IrData>),
    Combination(Vec<AstValueOrigin>),
    Unknown,
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
