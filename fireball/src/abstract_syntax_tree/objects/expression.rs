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

impl AstExpression {
    pub fn get_related_variables(&self) -> Vec<AstVariableId> {
        match self {
            AstExpression::Variable(_, var_id) => [var_id.clone()].into(),
            AstExpression::UnaryOp(_, arg) => arg.get_related_variables(),
            AstExpression::BinaryOp(_, arg1, arg2) => {
                let mut result = arg1.get_related_variables();
                result.extend(arg2.get_related_variables());
                result
            }
            AstExpression::Call(_, args) => {
                let mut result = Vec::new();
                for arg in args {
                    result.extend(arg.get_related_variables());
                }
                result
            }
            AstExpression::Cast(_, expr) => expr.get_related_variables(),
            AstExpression::Deref(expr) => expr.get_related_variables(),
            AstExpression::AddressOf(expr) => expr.get_related_variables(),
            AstExpression::ArrayAccess(expr, idx) => {
                let mut result = expr.get_related_variables();
                result.extend(idx.get_related_variables());
                result
            }
            AstExpression::MemberAccess(expr, _) => expr.get_related_variables(),

            AstExpression::Literal(_)
            | AstExpression::Unknown
            | AstExpression::Undefined
            | AstExpression::ArchitectureBitSize
            | AstExpression::ArchitectureByteSize => Vec::new(),
        }
    }
}
