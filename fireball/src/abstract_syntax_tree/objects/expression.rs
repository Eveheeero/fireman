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
    Call(AstCall),
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
            AstExpression::Call(call) => match call {
                AstCall::Variable { var_id, args, .. } => {
                    let mut result = vec![var_id.clone()];
                    for arg in args.iter() {
                        result.extend(arg.get_related_variables());
                    }
                    result
                }
                AstCall::Builtin(_, arg) => {
                    let mut result = Vec::new();
                    match arg.as_ref() {
                        AstBuiltinFunctionArgument::None => {}
                        AstBuiltinFunctionArgument::Print(args) => {
                            for expr in args.iter() {
                                result.extend(expr.get_related_variables());
                            }
                        }
                        AstBuiltinFunctionArgument::ByteSizeOf(expr)
                        | AstBuiltinFunctionArgument::BitSizeOf(expr)
                        | AstBuiltinFunctionArgument::OperandExists(expr)
                        | AstBuiltinFunctionArgument::SignedMax(expr)
                        | AstBuiltinFunctionArgument::SignedMin(expr)
                        | AstBuiltinFunctionArgument::UnsignedMax(expr)
                        | AstBuiltinFunctionArgument::UnsignedMin(expr)
                        | AstBuiltinFunctionArgument::BitOnes(expr)
                        | AstBuiltinFunctionArgument::BitZeros(expr) => {
                            result.extend(expr.get_related_variables());
                        }
                        AstBuiltinFunctionArgument::Sized(expr1, expr2) => {
                            result.extend(expr1.get_related_variables());
                            result.extend(expr2.get_related_variables());
                        }
                    }
                    result
                }
                AstCall::Unknown(_, args) => {
                    let mut result = Vec::new();
                    for arg in args.iter() {
                        result.extend(arg.get_related_variables());
                    }
                    result
                }
                AstCall::Function { args, .. } => {
                    let mut result = Vec::new();
                    for arg in args.iter() {
                        result.extend(arg.get_related_variables());
                    }
                    result
                }
            },
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
