use super::node_name::{CaptureRef, FitsTarget, NodeName};
use crate::abstract_syntax_tree::{
    ArcAstVariableMap, AstBinaryOperator, AstCall, AstExpression, AstLiteral, AstStatement,
    AstUnaryOperator, AstValueType, AstVariable, AstVariableId, Wrapped, WrappedAstStatement,
};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Pattern tree
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum PatTree {
    /// `$name` — captures a subtree
    Capture(String),
    /// `_` — matches anything, discards
    Wildcard,
    /// `Name(child1, child2, ...)` — matches a constructor
    Node {
        name: NodeName,
        children: Vec<PatTree>,
    },
    /// Integer literal in pattern (e.g. `Int(42)`, `Int(-1)`)
    NumberLiteral(i64),
    /// Unsigned integer literal for values that overflow i64
    UIntLiteral(u64),
    /// Bare string identifier that is not a known NodeName (e.g. "cleanup" in `Label(cleanup)`)
    StringLiteral(String),
    /// `[a, b, c]` — matches a Vec
    List(Vec<PatTree>),
    /// `Some(inner)` — matches Option::Some
    OptionSome(Box<PatTree>),
    /// `None` — matches Option::None
    OptionNone,
}

// ---------------------------------------------------------------------------
// Captured values
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Captured {
    Statement(AstStatement),
    Expression(Wrapped<AstExpression>),
    ExpressionBox(Box<Wrapped<AstExpression>>),
    VariableId(AstVariableId),
    VariableMap(ArcAstVariableMap),
    Literal(AstLiteral),
    StmtList(Vec<WrappedAstStatement>),
    OptStmtList(Option<Vec<WrappedAstStatement>>),
    OptExpression(Option<Wrapped<AstExpression>>),
    UnaryOp(AstUnaryOperator),
    BinaryOp(AstBinaryOperator),
    Variable(AstVariable),
    ValueType(AstValueType),
    Call(AstCall),
}

pub type Captures = HashMap<String, Captured>;

// ---------------------------------------------------------------------------
// Where predicates
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct UnaryCapturePredicate {
    pub capture: CaptureRef,
}

#[derive(Debug, Clone)]
pub struct BinaryCapturePredicate {
    pub left: CaptureRef,
    pub right: CaptureRef,
}

#[derive(Debug, Clone)]
pub struct FitsPredicate {
    pub capture: CaptureRef,
    pub target: FitsTarget,
}

#[derive(Debug, Clone)]
pub struct SumEqualsPredicate {
    pub left: CaptureRef,
    pub right: CaptureRef,
    pub target: SumEqualsTarget,
}

#[derive(Debug, Clone)]
pub struct CallNameMatchesPredicate {
    pub capture: CaptureRef,
    pub pattern: CaseInsensitivePattern,
}

#[derive(Debug, Clone)]
pub struct StmtListPredicate {
    pub capture: CaptureRef,
    pub kind: StmtListPredicateKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StmtListPredicateKind {
    NoUnsafeStmts,
    EndsWithContinue,
    EndsWithIfNotCondBreak,
    EndsWithIfCondElseBreak,
    IsEmpty,
    IsNonEmpty,
    EndsWithBreak,
    EndsWithReturn,
}

#[derive(Debug, Clone)]
pub enum WherePredicate {
    Eq(BinaryCapturePredicate),
    Ne(BinaryCapturePredicate),
    /// Check that a captured StmtList satisfies a statement-list predicate.
    StmtList(StmtListPredicate),
    /// Check that a captured literal fits in the named type's range.
    /// `fits($lit_capture, TypeName)` where TypeName is e.g. Int8, Int16, Int32, Int64, Int.
    Fits(FitsPredicate),
    /// Check that a captured expression is a literal value.
    IsLiteral(UnaryCapturePredicate),
    /// Check that a captured expression is NOT a literal value.
    NotLiteral(UnaryCapturePredicate),
    /// Check that a captured expression is pure (no side effects: no calls, deref, etc.).
    IsPure(UnaryCapturePredicate),
    /// Check that two captured expressions are structurally equal.
    StructurallyEqual(BinaryCapturePredicate),
    /// Check that two captured values have the same discriminant (e.g. same operator variant).
    SameDiscriminant(BinaryCapturePredicate),
    /// Check that the first captured StmtList has more statements than the second.
    GreaterCount(BinaryCapturePredicate),
    /// Check that a captured literal is zero (Int(0), UInt(0), Bool(false)).
    IsZero(UnaryCapturePredicate),
    /// Check that a captured literal is non-zero (Int(n) where n!=0, UInt(n) where n!=0, Bool(true)).
    IsNonZero(UnaryCapturePredicate),
    /// Check that a captured expression is a Variable (not a complex expression).
    IsVariable(UnaryCapturePredicate),
    /// Check that two captured Int literals sum to a specific value (capture or literal).
    SumEquals(SumEqualsPredicate),
    /// Check that a captured Call's name contains a given substring (case-insensitive).
    /// `call_name_matches($f, "pattern")`
    CallNameMatches(CallNameMatchesPredicate),
}

/// The third operand of `sum_equals()`: either a capture reference or an inline integer literal.
#[derive(Debug, Clone)]
pub enum SumEqualsTarget {
    Capture(CaptureRef),
    Literal(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseInsensitivePattern(String);

impl CaseInsensitivePattern {
    pub fn new(pattern: impl Into<String>) -> Self {
        Self(pattern.into().to_ascii_lowercase())
    }

    pub fn matches(&self, value: &str) -> bool {
        value.to_ascii_lowercase().contains(&self.0)
    }
}
