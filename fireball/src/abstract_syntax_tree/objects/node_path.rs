use crate::abstract_syntax_tree::objects::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstNodePath {
    Function {
        index: usize,
    },
    Statement {
        function_index: usize,
        statement_path: Vec<usize>,
    },
    Expression {
        function_index: usize,
        statement_path: Vec<usize>,
        expression_path: Vec<ExpressionPathComponent>,
    },
    Variable {
        function_index: usize,
        var_id: AstVariableId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionPathComponent {
    Left,
    Right,
    Condition,
    ThenBranch,
    ElseBranch,
    Operand(usize),
    Body,
    Init,
    Update,
    Target,
    Argument(usize),
    Base,
    Index,
    Member,
    CastType,
    CastValue,
    Deref,
    AddressOf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstNodeType {
    Function,
    Statement(AstStatementType),
    Expression(AstExpressionType),
    Variable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstStatementType {
    Declaration,
    Assignment,
    If,
    While,
    For,
    Return,
    Call,
    Label,
    Goto,
    Block,
    Assembly,
    Undefined,
    Exception,
    Comment,
    Ir,
    Empty,
    Switch,
    Break,
    Continue,
    DoWhile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstExpressionType {
    Unknown,
    Undefined,
    ArchitectureBitSize,
    ArchitectureByteSize,
    Literal,
    Variable,
    UnaryOp,
    BinaryOp,
    Call,
    Cast,
    Deref,
    AddressOf,
    ArrayAccess,
    MemberAccess,
    Ternary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstNodeEdit {
    RenameVariable {
        path: AstNodePath,
        new_name: String,
    },
    ChangeVariableType {
        path: AstNodePath,
        new_type: String,
    },
    ChangeLiteral {
        path: AstNodePath,
        new_value: String,
    },
    ChangeUnaryOperator {
        path: AstNodePath,
        new_op: String,
    },
    ChangeBinaryOperator {
        path: AstNodePath,
        new_op: String,
    },
    ReplaceStatement {
        path: AstNodePath,
        replacement: String,
    },
}

impl AstNodePath {
    pub fn function(index: usize) -> Self {
        Self::Function { index }
    }

    pub fn statement(function_index: usize, statement_path: Vec<usize>) -> Self {
        Self::Statement {
            function_index,
            statement_path,
        }
    }

    pub fn expression(
        function_index: usize,
        statement_path: Vec<usize>,
        expression_path: Vec<ExpressionPathComponent>,
    ) -> Self {
        Self::Expression {
            function_index,
            statement_path,
            expression_path,
        }
    }

    pub fn variable(function_index: usize, var_id: AstVariableId) -> Self {
        Self::Variable {
            function_index,
            var_id,
        }
    }

    pub fn parent(&self) -> Option<Self> {
        match self {
            Self::Function { .. } => None,
            Self::Statement {
                function_index,
                statement_path,
            } => {
                if statement_path.is_empty() {
                    Some(Self::Function {
                        index: *function_index,
                    })
                } else {
                    let mut parent_path = statement_path.clone();
                    parent_path.pop();
                    Some(Self::Statement {
                        function_index: *function_index,
                        statement_path: parent_path,
                    })
                }
            }
            Self::Expression {
                function_index,
                statement_path,
                expression_path,
            } => {
                if expression_path.is_empty() {
                    Some(Self::Statement {
                        function_index: *function_index,
                        statement_path: statement_path.clone(),
                    })
                } else {
                    let mut parent_path = expression_path.clone();
                    parent_path.pop();
                    Some(Self::Expression {
                        function_index: *function_index,
                        statement_path: statement_path.clone(),
                        expression_path: parent_path,
                    })
                }
            }
            Self::Variable { function_index, .. } => Some(Self::Function {
                index: *function_index,
            }),
        }
    }

    pub fn child_statement(&self, index: usize) -> Option<Self> {
        match self {
            Self::Function {
                index: function_index,
            } => Some(Self::Statement {
                function_index: *function_index,
                statement_path: vec![index],
            }),
            Self::Statement {
                function_index,
                statement_path,
            } => {
                let mut new_path = statement_path.clone();
                new_path.push(index);
                Some(Self::Statement {
                    function_index: *function_index,
                    statement_path: new_path,
                })
            }
            _ => None,
        }
    }

    pub fn child_expression(&self, component: ExpressionPathComponent) -> Option<Self> {
        match self {
            Self::Statement {
                function_index,
                statement_path,
            } => Some(Self::Expression {
                function_index: *function_index,
                statement_path: statement_path.clone(),
                expression_path: vec![component],
            }),
            Self::Expression {
                function_index,
                statement_path,
                expression_path,
            } => {
                let mut new_path = expression_path.clone();
                new_path.push(component);
                Some(Self::Expression {
                    function_index: *function_index,
                    statement_path: statement_path.clone(),
                    expression_path: new_path,
                })
            }
            _ => None,
        }
    }

    pub fn node_type(&self) -> AstNodeType {
        match self {
            Self::Function { .. } => AstNodeType::Function,
            Self::Statement { .. } => AstNodeType::Statement(AstStatementType::Undefined),
            Self::Expression { .. } => AstNodeType::Expression(AstExpressionType::Unknown),
            Self::Variable { .. } => AstNodeType::Variable,
        }
    }
}

impl ExpressionPathComponent {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Condition => "condition",
            Self::ThenBranch => "then",
            Self::ElseBranch => "else",
            Self::Operand(idx) => match idx {
                0 => "operand1",
                1 => "operand2",
                _ => "operand",
            },
            Self::Body => "body",
            Self::Init => "init",
            Self::Update => "update",
            Self::Target => "target",
            Self::Argument(idx) => match idx {
                0 => "arg1",
                1 => "arg2",
                _ => "arg",
            },
            Self::Base => "base",
            Self::Index => "index",
            Self::Member => "member",
            Self::CastType => "cast_type",
            Self::CastValue => "cast_value",
            Self::Deref => "deref",
            Self::AddressOf => "addr_of",
        }
    }
}
