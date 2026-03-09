use super::{node_name::NodeName, types::PatTree};

// ---------------------------------------------------------------------------
// Pattern parser (tokenizer + recursive descent)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Ident(String),
    Capture(String), // $name
    Wildcard,        // _
    Number(i64),     // integer literal (positive or negative)
    UIntNumber(u64), // unsigned integer literal (overflow from i64)
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
            '-' if i + 1 < chars.len() && chars[i + 1].is_ascii_digit() => {
                i += 1; // skip '-'
                let start = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let num_str: String = chars[start..i].iter().collect();
                let n: i64 = num_str
                    .parse()
                    .map_err(|e| format!("invalid number literal -{num_str}: {e}"))?;
                tokens.push(Token::Number(-n));
            }
            c if c.is_ascii_digit() => {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let num_str: String = chars[start..i].iter().collect();
                if let Ok(n) = num_str.parse::<i64>() {
                    tokens.push(Token::Number(n));
                } else if let Ok(n) = num_str.parse::<u64>() {
                    tokens.push(Token::UIntNumber(n));
                } else {
                    return Err(format!("number literal too large: {num_str}"));
                }
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
            Some(Token::Number(n)) => Ok(PatTree::NumberLiteral(n)),
            Some(Token::UIntNumber(n)) => Ok(PatTree::UIntLiteral(n)),
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
            Some(Token::Ident(word)) => {
                if word == "None" {
                    if self.peek() == Some(&Token::LParen) {
                        // None() — still treat as None
                        self.next();
                        self.expect(&Token::RParen)?;
                    }
                    return Ok(PatTree::OptionNone);
                }
                if word == "Some" {
                    self.expect(&Token::LParen)?;
                    let inner = self.parse_tree()?;
                    self.expect(&Token::RParen)?;
                    return Ok(PatTree::OptionSome(Box::new(inner)));
                }
                let Some(name) = NodeName::from_ident(&word) else {
                    // Unknown identifier — treat as a bare string literal
                    // (valid as child of Label, Comment, Assembly, Goto)
                    return Ok(PatTree::StringLiteral(word));
                };
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
