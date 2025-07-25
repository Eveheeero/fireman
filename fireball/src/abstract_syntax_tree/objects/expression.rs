use crate::abstract_syntax_tree::objects::*;

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
