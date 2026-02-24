use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstPattern {
    pub name: String,
    pub origin: AstPatternOrigin,
    pub arg: AstPatternArgType,
    pub pattern: String,
}
impl AstPattern {
    pub const ALL: Vec<Self> = vec![];
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstPatternOrigin {
    PreDefined,
    UserInput,
    File,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstPatternArgType {
    WithAssembly,
    WithIr,
    WithAst,
    WithOptimizedAst,
}

pub(super) fn apply_patterns(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
    patterns: &[AstPattern],
) -> Result<(), DecompileError> {
    let mut body;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        body = std::mem::take(&mut function.body);
    }

    apply_patterns_in_statements(&mut body, patterns);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::PatternMatching);
    }

    Ok(())
}

fn apply_patterns_in_statements(stmts: &mut Vec<WrappedAstStatement>, patterns: &[AstPattern]) {
    for stmt in stmts.iter_mut() {
        apply_patterns_in_statement(stmt, patterns);
    }

    if pattern_enabled(patterns, "remove-empty-statements") {
        stmts.retain(|stmt| !matches!(&stmt.statement, AstStatement::Empty));
    }
}

fn apply_patterns_in_statement(stmt: &mut WrappedAstStatement, patterns: &[AstPattern]) {
    match &mut stmt.statement {
        AstStatement::If(_, branch_true, branch_false) => {
            apply_patterns_in_statements(branch_true, patterns);
            if let Some(branch_false) = branch_false {
                apply_patterns_in_statements(branch_false, patterns);
            }
            if pattern_enabled(patterns, "prune-empty-else") {
                let remove_else =
                    matches!(branch_false.as_ref(), Some(branch) if branch.is_empty());
                if remove_else {
                    *branch_false = None;
                }
            }
        }
        AstStatement::While(_, body) => {
            apply_patterns_in_statements(body, patterns);
        }
        AstStatement::For(init, _, update, body) => {
            apply_patterns_in_statement(init, patterns);
            apply_patterns_in_statement(update, patterns);
            apply_patterns_in_statements(body, patterns);
        }
        AstStatement::Block(body) => {
            apply_patterns_in_statements(body, patterns);
            if pattern_enabled(patterns, "collapse-empty-blocks") && body.is_empty() {
                stmt.statement = AstStatement::Empty;
            }
        }
        AstStatement::Declaration(_, _)
        | AstStatement::Assignment(_, _)
        | AstStatement::Return(_)
        | AstStatement::Call(_)
        | AstStatement::Label(_)
        | AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_)
        | AstStatement::Comment(_)
        | AstStatement::Ir(_)
        | AstStatement::Empty => {}
    }
}

fn pattern_enabled(patterns: &[AstPattern], expected: &str) -> bool {
    if patterns.is_empty() {
        return true;
    }
    patterns.iter().any(|pattern| pattern.name == expected)
}
