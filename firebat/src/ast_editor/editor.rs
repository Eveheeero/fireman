use fireball::abstract_syntax_tree::{AstNodeEdit, AstNodePath, ExpressionPathComponent};

#[derive(Debug, Clone)]
pub struct AstNodeSelection {
    pub path: AstNodePath,
    pub edit_context: AstEditContext,
}

#[derive(Debug, Clone)]
pub enum AstEditContext {
    Variable {
        name: String,
        var_type: String,
    },
    Literal {
        value: String,
        value_type: String,
    },
    UnaryOperator {
        operator: String,
        operand: String,
    },
    BinaryOperator {
        operator: String,
        left: String,
        right: String,
    },
    Statement {
        statement_type: String,
        text: String,
    },
    Function {
        name: String,
        return_type: String,
        parameters: Vec<(String, String)>,
    },
}

#[derive(Debug, Clone)]
pub enum AstNodeEditDraft {
    VariableEdit {
        path: AstNodePath,
        current_name: String,
        current_type: String,
        new_name: String,
        new_type: String,
    },
    LiteralEdit {
        path: AstNodePath,
        current_value: String,
        current_type: String,
        new_value: String,
    },
    UnaryOperatorEdit {
        path: AstNodePath,
        current_op: String,
        new_op: String,
    },
    BinaryOperatorEdit {
        path: AstNodePath,
        current_op: String,
        new_op: String,
    },
    StatementEdit {
        path: AstNodePath,
        statement_type: String,
        replacement: String,
    },
}

impl AstNodeEditDraft {
    pub fn to_edit(&self) -> Option<AstNodeEdit> {
        match self {
            Self::VariableEdit {
                path,
                new_name,
                new_type,
                current_name,
                current_type,
            } => {
                if new_name != current_name {
                    return Some(AstNodeEdit::RenameVariable {
                        path: path.clone(),
                        new_name: new_name.clone(),
                    });
                }
                if new_type != current_type {
                    return Some(AstNodeEdit::ChangeVariableType {
                        path: path.clone(),
                        new_type: new_type.clone(),
                    });
                }
                None
            }
            Self::LiteralEdit {
                path,
                new_value,
                current_value,
                ..
            } => {
                if new_value != current_value {
                    Some(AstNodeEdit::ChangeLiteral {
                        path: path.clone(),
                        new_value: new_value.clone(),
                    })
                } else {
                    None
                }
            }
            Self::UnaryOperatorEdit {
                path,
                new_op,
                current_op,
                ..
            } => {
                if new_op != current_op {
                    Some(AstNodeEdit::ChangeUnaryOperator {
                        path: path.clone(),
                        new_op: new_op.clone(),
                    })
                } else {
                    None
                }
            }
            Self::BinaryOperatorEdit {
                path,
                new_op,
                current_op,
                ..
            } => {
                if new_op != current_op {
                    Some(AstNodeEdit::ChangeBinaryOperator {
                        path: path.clone(),
                        new_op: new_op.clone(),
                    })
                } else {
                    None
                }
            }
            Self::StatementEdit {
                path, replacement, ..
            } => Some(AstNodeEdit::ReplaceStatement {
                path: path.clone(),
                replacement: replacement.clone(),
            }),
        }
    }

    pub fn status_message(&self) -> Option<String> {
        match self {
            Self::VariableEdit {
                new_name, new_type, ..
            } => {
                if new_name.is_empty() {
                    return Some("Variable name cannot be empty".to_string());
                }
                if new_type.is_empty() {
                    return Some("Variable type cannot be empty".to_string());
                }
                None
            }
            Self::LiteralEdit { new_value, .. } => {
                if new_value.is_empty() {
                    return Some("Literal value cannot be empty".to_string());
                }
                None
            }
            Self::StatementEdit { replacement, .. } => {
                if replacement.is_empty() {
                    return Some("Statement cannot be empty".to_string());
                }
                None
            }
            _ => None,
        }
    }
}

pub fn expression_component_for_statement(stmt_type: &str) -> Vec<ExpressionPathComponent> {
    match stmt_type {
        "if" => vec![
            ExpressionPathComponent::Condition,
            ExpressionPathComponent::ThenBranch,
            ExpressionPathComponent::ElseBranch,
        ],
        "while" | "dowhile" => vec![
            ExpressionPathComponent::Condition,
            ExpressionPathComponent::Body,
        ],
        "for" => vec![
            ExpressionPathComponent::Init,
            ExpressionPathComponent::Condition,
            ExpressionPathComponent::Update,
            ExpressionPathComponent::Body,
        ],
        "return" => vec![ExpressionPathComponent::Body],
        "assignment" => vec![
            ExpressionPathComponent::Left,
            ExpressionPathComponent::Right,
        ],
        "declaration" => vec![ExpressionPathComponent::Body],
        _ => vec![],
    }
}
