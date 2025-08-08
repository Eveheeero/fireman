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
