use crate::ir::{analyze::ir_to_ast::abstract_syntax_tree::objects::*, statements::IrStatement};

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
    Ir(Box<IrStatement>),
    Empty,
}
