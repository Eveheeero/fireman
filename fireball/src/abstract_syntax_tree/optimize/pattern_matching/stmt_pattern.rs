//! Structural AST pattern matching with named captures, predicates, and emit.
//!
//! Parses patterns like `If($cond, [Assignment(Variable($_, $v1), $a)], Some([Assignment(Variable($_, $v2), $b)]))`
//! into a `PatTree`, matches them against `AstStatement` nodes to produce `Captures`,
//! evaluates `where` predicates (e.g. `eq($v1, $v2)`), and constructs replacement
//! statements via `emit`.

use crate::abstract_syntax_tree::{
    ArcAstVariableMap, AstBinaryOperator, AstExpression, AstLiteral, AstStatement,
    AstStatementOrigin, AstUnaryOperator, AstValueOrigin, AstVariable, AstVariableId, Wrapped,
    WrappedAstStatement,
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
    Node { name: String, children: Vec<PatTree> },
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
}

pub type Captures = HashMap<String, Captured>;

// ---------------------------------------------------------------------------
// Where predicates
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum WherePredicate {
    Eq(String, String),
}

pub fn parse_where(input: &str) -> Result<WherePredicate, String> {
    let input = input.trim();
    if let Some(rest) = input.strip_prefix("eq(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let parts: Vec<&str> = rest.splitn(2, ',').collect();
        if parts.len() != 2 {
            return Err(format!("eq() requires exactly 2 arguments: {input}"));
        }
        let a = parts[0].trim();
        let b = parts[1].trim();
        let a = a
            .strip_prefix('$')
            .ok_or_else(|| format!("eq() argument must be a capture ($name): {a}"))?
            .to_string();
        let b = b
            .strip_prefix('$')
            .ok_or_else(|| format!("eq() argument must be a capture ($name): {b}"))?
            .to_string();
        Ok(WherePredicate::Eq(a, b))
    } else {
        Err(format!("unknown where predicate: {input}"))
    }
}

pub fn eval_where(pred: &WherePredicate, caps: &Captures) -> bool {
    match pred {
        WherePredicate::Eq(a, b) => {
            let Some(cap_a) = caps.get(a) else {
                return false;
            };
            let Some(cap_b) = caps.get(b) else {
                return false;
            };
            captured_structurally_equal(cap_a, cap_b)
        }
    }
}

fn captured_structurally_equal(a: &Captured, b: &Captured) -> bool {
    match (a, b) {
        (Captured::VariableId(a), Captured::VariableId(b)) => a == b,
        (Captured::Literal(a), Captured::Literal(b)) => a == b,
        (Captured::UnaryOp(a), Captured::UnaryOp(b)) => {
            std::mem::discriminant(a) == std::mem::discriminant(b)
        }
        (Captured::BinaryOp(a), Captured::BinaryOp(b)) => {
            std::mem::discriminant(a) == std::mem::discriminant(b)
        }
        (Captured::Expression(a), Captured::Expression(b)) => {
            super::super::opt_utils::expr_structurally_equal(&a.item, &b.item)
        }
        (Captured::ExpressionBox(a), Captured::ExpressionBox(b)) => {
            super::super::opt_utils::expr_structurally_equal(&a.item, &b.item)
        }
        (Captured::Expression(a), Captured::ExpressionBox(b)) => {
            super::super::opt_utils::expr_structurally_equal(&a.item, &b.item)
        }
        (Captured::ExpressionBox(a), Captured::Expression(b)) => {
            super::super::opt_utils::expr_structurally_equal(&a.item, &b.item)
        }
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Pattern parser (tokenizer + recursive descent)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Ident(String),
    Capture(String), // $name
    Wildcard,        // _
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' | '\n' | '\r' => i += 1,
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            '[' => {
                tokens.push(Token::LBracket);
                i += 1;
            }
            ']' => {
                tokens.push(Token::RBracket);
                i += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                i += 1;
            }
            '$' => {
                i += 1;
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                if i == start {
                    return Err("expected capture name after '$'".to_string());
                }
                tokens.push(Token::Capture(chars[start..i].iter().collect()));
            }
            c if c.is_alphabetic() || c == '_' => {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                if word == "_" {
                    tokens.push(Token::Wildcard);
                } else {
                    tokens.push(Token::Ident(word));
                }
            }
            other => return Err(format!("unexpected character in pattern: '{other}'")),
        }
    }
    Ok(tokens)
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        match self.next() {
            Some(ref t) if t == expected => Ok(()),
            Some(t) => Err(format!("expected {expected:?}, got {t:?}")),
            None => Err(format!("expected {expected:?}, got end of input")),
        }
    }

    fn parse_tree(&mut self) -> Result<PatTree, String> {
        match self.next() {
            Some(Token::Capture(name)) => Ok(PatTree::Capture(name)),
            Some(Token::Wildcard) => Ok(PatTree::Wildcard),
            Some(Token::LBracket) => {
                let mut items = Vec::new();
                if self.peek() != Some(&Token::RBracket) {
                    items.push(self.parse_tree()?);
                    while self.peek() == Some(&Token::Comma) {
                        self.next();
                        if self.peek() == Some(&Token::RBracket) {
                            break;
                        }
                        items.push(self.parse_tree()?);
                    }
                }
                self.expect(&Token::RBracket)?;
                Ok(PatTree::List(items))
            }
            Some(Token::Ident(name)) => {
                if name == "None" {
                    if self.peek() == Some(&Token::LParen) {
                        // None() — still treat as None
                        self.next();
                        self.expect(&Token::RParen)?;
                    }
                    return Ok(PatTree::OptionNone);
                }
                if name == "Some" {
                    self.expect(&Token::LParen)?;
                    let inner = self.parse_tree()?;
                    self.expect(&Token::RParen)?;
                    return Ok(PatTree::OptionSome(Box::new(inner)));
                }
                if self.peek() == Some(&Token::LParen) {
                    self.next();
                    let mut children = Vec::new();
                    if self.peek() != Some(&Token::RParen) {
                        children.push(self.parse_tree()?);
                        while self.peek() == Some(&Token::Comma) {
                            self.next();
                            if self.peek() == Some(&Token::RParen) {
                                break;
                            }
                            children.push(self.parse_tree()?);
                        }
                    }
                    self.expect(&Token::RParen)?;
                    Ok(PatTree::Node { name, children })
                } else {
                    // Bare identifier — zero-arg constructor
                    Ok(PatTree::Node {
                        name,
                        children: Vec::new(),
                    })
                }
            }
            Some(t) => Err(format!("unexpected token: {t:?}")),
            None => Err("unexpected end of pattern".to_string()),
        }
    }
}

pub fn parse_pattern(input: &str) -> Result<PatTree, String> {
    let tokens = tokenize(input)?;
    if tokens.is_empty() {
        return Err("empty pattern".to_string());
    }
    let mut parser = Parser::new(tokens);
    let tree = parser.parse_tree()?;
    if parser.pos < parser.tokens.len() {
        return Err(format!(
            "trailing tokens after pattern: {:?}",
            &parser.tokens[parser.pos..]
        ));
    }
    Ok(tree)
}

// ---------------------------------------------------------------------------
// Structural matcher
// ---------------------------------------------------------------------------

pub fn match_statement(pat: &PatTree, stmt: &AstStatement) -> Option<Captures> {
    let mut caps = Captures::new();
    if match_stmt_inner(pat, stmt, &mut caps) {
        Some(caps)
    } else {
        None
    }
}

fn match_stmt_inner(pat: &PatTree, stmt: &AstStatement, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::Statement(stmt.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } => match_stmt_node(name, children, stmt, caps),
        _ => false,
    }
}

fn match_stmt_node(
    name: &str,
    children: &[PatTree],
    stmt: &AstStatement,
    caps: &mut Captures,
) -> bool {
    match (name, stmt) {
        ("Assignment", AstStatement::Assignment(lhs, rhs)) if children.len() == 2 => {
            match_wrapped_expr(&children[0], lhs, caps)
                && match_wrapped_expr(&children[1], rhs, caps)
        }
        ("If", AstStatement::If(cond, branch_true, branch_false)) if children.len() == 3 => {
            match_wrapped_expr(&children[0], cond, caps)
                && match_stmt_list(&children[1], branch_true, caps)
                && match_opt_stmt_list(&children[2], branch_false, caps)
        }
        ("While", AstStatement::While(cond, body)) if children.len() == 2 => {
            match_wrapped_expr(&children[0], cond, caps)
                && match_stmt_list(&children[1], body, caps)
        }
        ("DoWhile", AstStatement::DoWhile(cond, body)) if children.len() == 2 => {
            match_wrapped_expr(&children[0], cond, caps)
                && match_stmt_list(&children[1], body, caps)
        }
        ("For", AstStatement::For(init, cond, update, body)) if children.len() == 4 => {
            match_stmt_inner(&children[0], &init.statement, caps)
                && match_wrapped_expr(&children[1], cond, caps)
                && match_stmt_inner(&children[2], &update.statement, caps)
                && match_stmt_list(&children[3], body, caps)
        }
        ("Return", AstStatement::Return(opt_expr)) if children.len() == 1 => {
            match_opt_wrapped_expr(&children[0], opt_expr, caps)
        }
        ("Return", AstStatement::Return(None)) if children.is_empty() => true,
        ("Block", AstStatement::Block(body)) if children.len() == 1 => {
            match_stmt_list(&children[0], body, caps)
        }
        ("Label", AstStatement::Label(s)) if children.len() == 1 => {
            match_string_pat(&children[0], s, caps)
        }
        ("Comment", AstStatement::Comment(s)) if children.len() == 1 => {
            match_string_pat(&children[0], s, caps)
        }
        ("Assembly", AstStatement::Assembly(s)) if children.len() == 1 => {
            match_string_pat(&children[0], s, caps)
        }
        ("Declaration", AstStatement::Declaration(var, opt_init)) if children.len() == 2 => {
            match_variable_pat(&children[0], var, caps)
                && match_opt_wrapped_expr(&children[1], opt_init, caps)
        }
        ("Empty", AstStatement::Empty) if children.is_empty() => true,
        ("Break", AstStatement::Break) if children.is_empty() => true,
        ("Continue", AstStatement::Continue) if children.is_empty() => true,
        ("Undefined", AstStatement::Undefined) if children.is_empty() => true,
        _ => false,
    }
}

fn match_wrapped_expr(pat: &PatTree, expr: &Wrapped<AstExpression>, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::Expression(expr.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } => match_expr_node(name, children, &expr.item, caps),
        _ => false,
    }
}

fn match_boxed_wrapped_expr(
    pat: &PatTree,
    expr: &Box<Wrapped<AstExpression>>,
    caps: &mut Captures,
) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(
                name.clone(),
                Captured::ExpressionBox(expr.clone()),
            );
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } => match_expr_node(name, children, &expr.item, caps),
        _ => false,
    }
}

fn match_expr_node(
    name: &str,
    children: &[PatTree],
    expr: &AstExpression,
    caps: &mut Captures,
) -> bool {
    match (name, expr) {
        ("Variable", AstExpression::Variable(map, var_id)) if children.len() == 2 => {
            match_variable_map_pat(&children[0], map, caps)
                && match_variable_id_pat(&children[1], var_id, caps)
        }
        ("Literal", AstExpression::Literal(lit)) if children.len() == 1 => {
            match_literal_pat(&children[0], lit, caps)
        }
        ("UnaryOp", AstExpression::UnaryOp(op, arg)) if children.len() == 2 => {
            match_unary_op_pat(&children[0], op, caps)
                && match_boxed_wrapped_expr(&children[1], arg, caps)
        }
        ("BinaryOp", AstExpression::BinaryOp(op, lhs, rhs)) if children.len() == 3 => {
            match_binary_op_pat(&children[0], op, caps)
                && match_boxed_wrapped_expr(&children[1], lhs, caps)
                && match_boxed_wrapped_expr(&children[2], rhs, caps)
        }
        ("Ternary", AstExpression::Ternary(cond, t, f)) if children.len() == 3 => {
            match_boxed_wrapped_expr(&children[0], cond, caps)
                && match_boxed_wrapped_expr(&children[1], t, caps)
                && match_boxed_wrapped_expr(&children[2], f, caps)
        }
        ("Cast", AstExpression::Cast(_, arg)) if children.len() == 2 => {
            // children[0] matches the type (wildcard/capture only for now)
            match pat_is_wildcard_or_capture(&children[0], caps) {
                true => match_boxed_wrapped_expr(&children[1], arg, caps),
                false => false,
            }
        }
        ("Deref", AstExpression::Deref(arg)) if children.len() == 1 => {
            match_boxed_wrapped_expr(&children[0], arg, caps)
        }
        ("AddressOf", AstExpression::AddressOf(arg)) if children.len() == 1 => {
            match_boxed_wrapped_expr(&children[0], arg, caps)
        }
        ("ArrayAccess", AstExpression::ArrayAccess(base, idx)) if children.len() == 2 => {
            match_boxed_wrapped_expr(&children[0], base, caps)
                && match_boxed_wrapped_expr(&children[1], idx, caps)
        }
        ("MemberAccess", AstExpression::MemberAccess(base, _field)) if children.len() == 2 => {
            match_boxed_wrapped_expr(&children[0], base, caps)
                && match pat_is_wildcard_or_capture(&children[1], caps) {
                    true => true,
                    false => false,
                }
        }
        ("Unknown", AstExpression::Unknown) if children.is_empty() => true,
        ("Undefined", AstExpression::Undefined) if children.is_empty() => true,
        _ => false,
    }
}

fn pat_is_wildcard_or_capture(pat: &PatTree, _caps: &mut Captures) -> bool {
    matches!(pat, PatTree::Wildcard | PatTree::Capture(_))
}

fn match_variable_map_pat(
    pat: &PatTree,
    map: &ArcAstVariableMap,
    caps: &mut Captures,
) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::VariableMap(map.clone()));
            true
        }
        PatTree::Wildcard => true,
        _ => false,
    }
}

fn match_variable_id_pat(
    pat: &PatTree,
    var_id: &AstVariableId,
    caps: &mut Captures,
) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::VariableId(*var_id));
            true
        }
        PatTree::Wildcard => true,
        _ => false,
    }
}

fn match_literal_pat(pat: &PatTree, lit: &AstLiteral, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::Literal(lit.clone()));
            true
        }
        PatTree::Wildcard => true,
        _ => false,
    }
}

fn match_unary_op_pat(pat: &PatTree, op: &AstUnaryOperator, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::UnaryOp(op.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } if children.is_empty() => {
            match (name.as_str(), op) {
                ("Negate", AstUnaryOperator::Negate) => true,
                ("Not", AstUnaryOperator::Not) => true,
                ("BitNot", AstUnaryOperator::BitNot) => true,
                ("PreInc", AstUnaryOperator::PreInc) => true,
                ("PreDec", AstUnaryOperator::PreDec) => true,
                ("PostInc", AstUnaryOperator::PostInc) => true,
                ("PostDec", AstUnaryOperator::PostDec) => true,
                ("CastSigned", AstUnaryOperator::CastSigned) => true,
                ("CastUnsigned", AstUnaryOperator::CastUnsigned) => true,
                _ => false,
            }
        }
        _ => false,
    }
}

fn match_binary_op_pat(pat: &PatTree, op: &AstBinaryOperator, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::BinaryOp(op.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } if children.is_empty() => {
            match (name.as_str(), op) {
                ("Add", AstBinaryOperator::Add) => true,
                ("Sub", AstBinaryOperator::Sub) => true,
                ("Mul", AstBinaryOperator::Mul) => true,
                ("Div", AstBinaryOperator::Div) => true,
                ("Mod", AstBinaryOperator::Mod) => true,
                ("BitAnd", AstBinaryOperator::BitAnd) => true,
                ("BitOr", AstBinaryOperator::BitOr) => true,
                ("BitXor", AstBinaryOperator::BitXor) => true,
                ("LogicAnd", AstBinaryOperator::LogicAnd) => true,
                ("LogicOr", AstBinaryOperator::LogicOr) => true,
                ("Equal", AstBinaryOperator::Equal) => true,
                ("NotEqual", AstBinaryOperator::NotEqual) => true,
                ("Less", AstBinaryOperator::Less) => true,
                ("LessEqual", AstBinaryOperator::LessEqual) => true,
                ("Greater", AstBinaryOperator::Greater) => true,
                ("GreaterEqual", AstBinaryOperator::GreaterEqual) => true,
                ("LeftShift", AstBinaryOperator::LeftShift) => true,
                ("RightShift", AstBinaryOperator::RightShift) => true,
                _ => false,
            }
        }
        _ => false,
    }
}

fn match_variable_pat(pat: &PatTree, var: &AstVariable, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::Variable(var.clone()));
            true
        }
        PatTree::Wildcard => true,
        _ => false,
    }
}

fn match_string_pat(_pat: &PatTree, _s: &str, _caps: &mut Captures) -> bool {
    // For now only wildcard/capture
    matches!(_pat, PatTree::Wildcard | PatTree::Capture(_))
}

fn match_stmt_list(
    pat: &PatTree,
    stmts: &Vec<WrappedAstStatement>,
    caps: &mut Captures,
) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::StmtList(stmts.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::List(pats) => {
            if pats.len() != stmts.len() {
                return false;
            }
            for (p, s) in pats.iter().zip(stmts.iter()) {
                if !match_stmt_inner(p, &s.statement, caps) {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

fn match_opt_stmt_list(
    pat: &PatTree,
    opt: &Option<Vec<WrappedAstStatement>>,
    caps: &mut Captures,
) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::OptStmtList(opt.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::OptionNone => opt.is_none(),
        PatTree::OptionSome(inner) => match opt {
            Some(stmts) => match_stmt_list(inner, stmts, caps),
            None => false,
        },
        _ => false,
    }
}

fn match_opt_wrapped_expr(
    pat: &PatTree,
    opt: &Option<Wrapped<AstExpression>>,
    caps: &mut Captures,
) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::OptExpression(opt.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::OptionNone => opt.is_none(),
        PatTree::OptionSome(inner) => match opt {
            Some(expr) => match_wrapped_expr(inner, expr, caps),
            None => false,
        },
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Emit (construct replacement)
// ---------------------------------------------------------------------------

pub fn construct_statement(pat: &PatTree, caps: &Captures) -> Option<AstStatement> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::Statement(s) => Some(s.clone()),
            _ => None,
        },
        PatTree::Node { name, children } => construct_stmt_node(name, children, caps),
        _ => None,
    }
}

fn construct_stmt_node(
    name: &str,
    children: &[PatTree],
    caps: &Captures,
) -> Option<AstStatement> {
    match name {
        "Assignment" if children.len() == 2 => {
            let lhs = construct_wrapped_expr(&children[0], caps)?;
            let rhs = construct_wrapped_expr(&children[1], caps)?;
            Some(AstStatement::Assignment(lhs, rhs))
        }
        "If" if children.len() == 3 => {
            let cond = construct_wrapped_expr(&children[0], caps)?;
            let branch_true = construct_stmt_list(&children[1], caps)?;
            let branch_false = construct_opt_stmt_list(&children[2], caps)?;
            Some(AstStatement::If(cond, branch_true, branch_false))
        }
        "While" if children.len() == 2 => {
            let cond = construct_wrapped_expr(&children[0], caps)?;
            let body = construct_stmt_list(&children[1], caps)?;
            Some(AstStatement::While(cond, body))
        }
        "DoWhile" if children.len() == 2 => {
            let cond = construct_wrapped_expr(&children[0], caps)?;
            let body = construct_stmt_list(&children[1], caps)?;
            Some(AstStatement::DoWhile(cond, body))
        }
        "Return" if children.len() == 1 => {
            let opt = construct_opt_wrapped_expr(&children[0], caps)?;
            Some(AstStatement::Return(opt))
        }
        "Return" if children.is_empty() => Some(AstStatement::Return(None)),
        "Block" if children.len() == 1 => {
            let body = construct_stmt_list(&children[0], caps)?;
            Some(AstStatement::Block(body))
        }
        "Empty" if children.is_empty() => Some(AstStatement::Empty),
        "Break" if children.is_empty() => Some(AstStatement::Break),
        "Continue" if children.is_empty() => Some(AstStatement::Continue),
        _ => None,
    }
}

fn construct_wrapped_expr(pat: &PatTree, caps: &Captures) -> Option<Wrapped<AstExpression>> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::Expression(e) => Some(e.clone()),
            Captured::ExpressionBox(e) => Some((**e).clone()),
            _ => None,
        },
        PatTree::Node { name, children } => {
            let expr = construct_expr_node(name, children, caps)?;
            // Find an origin from any captured expression to use
            let origin = find_any_origin(caps);
            Some(Wrapped {
                item: expr,
                origin,
                comment: None,
            })
        }
        _ => None,
    }
}

fn construct_boxed_wrapped_expr(
    pat: &PatTree,
    caps: &Captures,
) -> Option<Box<Wrapped<AstExpression>>> {
    construct_wrapped_expr(pat, caps).map(Box::new)
}

fn construct_expr_node(
    name: &str,
    children: &[PatTree],
    caps: &Captures,
) -> Option<AstExpression> {
    match name {
        "Variable" if children.len() == 2 => {
            let map = construct_variable_map(&children[0], caps)?;
            let var_id = construct_variable_id(&children[1], caps)?;
            Some(AstExpression::Variable(map, var_id))
        }
        "Literal" if children.len() == 1 => {
            let lit = construct_literal(&children[0], caps)?;
            Some(AstExpression::Literal(lit))
        }
        "UnaryOp" if children.len() == 2 => {
            let op = construct_unary_op(&children[0], caps)?;
            let arg = construct_boxed_wrapped_expr(&children[1], caps)?;
            Some(AstExpression::UnaryOp(op, arg))
        }
        "BinaryOp" if children.len() == 3 => {
            let op = construct_binary_op(&children[0], caps)?;
            let lhs = construct_boxed_wrapped_expr(&children[1], caps)?;
            let rhs = construct_boxed_wrapped_expr(&children[2], caps)?;
            Some(AstExpression::BinaryOp(op, lhs, rhs))
        }
        "Ternary" if children.len() == 3 => {
            let cond = construct_boxed_wrapped_expr(&children[0], caps)?;
            let t = construct_boxed_wrapped_expr(&children[1], caps)?;
            let f = construct_boxed_wrapped_expr(&children[2], caps)?;
            Some(AstExpression::Ternary(cond, t, f))
        }
        "Deref" if children.len() == 1 => {
            let arg = construct_boxed_wrapped_expr(&children[0], caps)?;
            Some(AstExpression::Deref(arg))
        }
        "AddressOf" if children.len() == 1 => {
            let arg = construct_boxed_wrapped_expr(&children[0], caps)?;
            Some(AstExpression::AddressOf(arg))
        }
        "ArrayAccess" if children.len() == 2 => {
            let base = construct_boxed_wrapped_expr(&children[0], caps)?;
            let idx = construct_boxed_wrapped_expr(&children[1], caps)?;
            Some(AstExpression::ArrayAccess(base, idx))
        }
        "Unknown" if children.is_empty() => Some(AstExpression::Unknown),
        "Undefined" if children.is_empty() => Some(AstExpression::Undefined),
        _ => None,
    }
}

fn construct_variable_map(pat: &PatTree, caps: &Captures) -> Option<ArcAstVariableMap> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::VariableMap(m) => Some(m.clone()),
            _ => None,
        },
        PatTree::Wildcard => {
            // Fallback: find any VariableMap in captures
            for v in caps.values() {
                if let Captured::VariableMap(m) = v {
                    return Some(m.clone());
                }
            }
            None
        }
        _ => None,
    }
}

fn construct_variable_id(pat: &PatTree, caps: &Captures) -> Option<AstVariableId> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::VariableId(id) => Some(*id),
            _ => None,
        },
        _ => None,
    }
}

fn construct_literal(pat: &PatTree, caps: &Captures) -> Option<AstLiteral> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::Literal(l) => Some(l.clone()),
            _ => None,
        },
        _ => None,
    }
}

fn construct_unary_op(pat: &PatTree, caps: &Captures) -> Option<AstUnaryOperator> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::UnaryOp(op) => Some(op.clone()),
            _ => None,
        },
        PatTree::Node { name, children } if children.is_empty() => match name.as_str() {
            "Negate" => Some(AstUnaryOperator::Negate),
            "Not" => Some(AstUnaryOperator::Not),
            "BitNot" => Some(AstUnaryOperator::BitNot),
            "PreInc" => Some(AstUnaryOperator::PreInc),
            "PreDec" => Some(AstUnaryOperator::PreDec),
            "PostInc" => Some(AstUnaryOperator::PostInc),
            "PostDec" => Some(AstUnaryOperator::PostDec),
            "CastSigned" => Some(AstUnaryOperator::CastSigned),
            "CastUnsigned" => Some(AstUnaryOperator::CastUnsigned),
            _ => None,
        },
        _ => None,
    }
}

fn construct_binary_op(pat: &PatTree, caps: &Captures) -> Option<AstBinaryOperator> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::BinaryOp(op) => Some(op.clone()),
            _ => None,
        },
        PatTree::Node { name, children } if children.is_empty() => match name.as_str() {
            "Add" => Some(AstBinaryOperator::Add),
            "Sub" => Some(AstBinaryOperator::Sub),
            "Mul" => Some(AstBinaryOperator::Mul),
            "Div" => Some(AstBinaryOperator::Div),
            "Mod" => Some(AstBinaryOperator::Mod),
            "BitAnd" => Some(AstBinaryOperator::BitAnd),
            "BitOr" => Some(AstBinaryOperator::BitOr),
            "BitXor" => Some(AstBinaryOperator::BitXor),
            "LogicAnd" => Some(AstBinaryOperator::LogicAnd),
            "LogicOr" => Some(AstBinaryOperator::LogicOr),
            "Equal" => Some(AstBinaryOperator::Equal),
            "NotEqual" => Some(AstBinaryOperator::NotEqual),
            "Less" => Some(AstBinaryOperator::Less),
            "LessEqual" => Some(AstBinaryOperator::LessEqual),
            "Greater" => Some(AstBinaryOperator::Greater),
            "GreaterEqual" => Some(AstBinaryOperator::GreaterEqual),
            "LeftShift" => Some(AstBinaryOperator::LeftShift),
            "RightShift" => Some(AstBinaryOperator::RightShift),
            _ => None,
        },
        _ => None,
    }
}

fn construct_stmt_list(pat: &PatTree, caps: &Captures) -> Option<Vec<WrappedAstStatement>> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::StmtList(l) => Some(l.clone()),
            _ => None,
        },
        PatTree::List(pats) => {
            let mut result = Vec::new();
            for p in pats {
                let stmt = construct_statement(p, caps)?;
                result.push(WrappedAstStatement {
                    statement: stmt,
                    origin: AstStatementOrigin::Unknown,
                    comment: None,
                });
            }
            Some(result)
        }
        _ => None,
    }
}

fn construct_opt_stmt_list(
    pat: &PatTree,
    caps: &Captures,
) -> Option<Option<Vec<WrappedAstStatement>>> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::OptStmtList(l) => Some(l.clone()),
            _ => None,
        },
        PatTree::OptionNone => Some(None),
        PatTree::OptionSome(inner) => {
            let list = construct_stmt_list(inner, caps)?;
            Some(Some(list))
        }
        _ => None,
    }
}

fn construct_opt_wrapped_expr(
    pat: &PatTree,
    caps: &Captures,
) -> Option<Option<Wrapped<AstExpression>>> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::OptExpression(e) => Some(e.clone()),
            _ => None,
        },
        PatTree::OptionNone => Some(None),
        PatTree::OptionSome(inner) => {
            let expr = construct_wrapped_expr(inner, caps)?;
            Some(Some(expr))
        }
        _ => None,
    }
}

/// Inject captured values into a rhai `Scope` as debug-string variables.
/// Each capture `$name` becomes a rhai variable `name` with its `Debug` representation.
pub fn inject_captures_into_rhai_scope(caps: &Captures, scope: &mut rhai::Scope<'static>) {
    for (name, captured) in caps {
        let value = match captured {
            Captured::Statement(s) => format!("{s:?}"),
            Captured::Expression(e) => format!("{:?}", e.item),
            Captured::ExpressionBox(e) => format!("{:?}", e.item),
            Captured::VariableId(id) => format!("{id:?}"),
            Captured::VariableMap(_) => "VariableMap(...)".to_string(),
            Captured::Literal(l) => format!("{l:?}"),
            Captured::StmtList(l) => format!("{l:?}"),
            Captured::OptStmtList(l) => format!("{l:?}"),
            Captured::OptExpression(e) => format!("{e:?}"),
            Captured::UnaryOp(op) => format!("{op:?}"),
            Captured::BinaryOp(op) => format!("{op:?}"),
            Captured::Variable(v) => format!("{v:?}"),
        };
        scope.push(name.clone(), value);
    }
}

fn find_any_origin(caps: &Captures) -> AstValueOrigin {
    for v in caps.values() {
        match v {
            Captured::Expression(e) => return e.origin.clone(),
            Captured::ExpressionBox(e) => return e.origin.clone(),
            _ => {}
        }
    }
    AstValueOrigin::Unknown
}
