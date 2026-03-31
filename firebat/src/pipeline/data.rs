use fireball::abstract_syntax_tree::Ast;
use std::sync::Arc;

/// Data flowing through the pipeline - now only carries AST
#[derive(Clone, Debug)]
pub enum PipelineData {
    Empty,
    Ast(Arc<Ast>),
}

impl PipelineData {
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Empty => "Empty",
            Self::Ast(_) => "AST",
        }
    }
}

/// Summary information for collapsed node display
#[derive(Clone, Debug, Default)]
pub struct DataSummary {
    pub item_count: usize,
    pub preview_lines: Vec<String>,
    pub search_text: String,
}

impl PipelineData {
    /// Generate a searchable summary for collapsed display
    pub fn summary(&self, max_lines: usize) -> DataSummary {
        match self {
            Self::Empty => DataSummary::default(),

            Self::Ast(ast) => {
                let config = fireball::abstract_syntax_tree::AstPrintConfig::default();
                let code = ast.print(Some(config));
                let funcs = code
                    .lines()
                    .filter(|l| l.contains("void ") || l.contains("int ") || l.contains("func "))
                    .count();
                let first_lines: Vec<String> = code
                    .lines()
                    .take(max_lines)
                    .map(|s| s.to_string())
                    .collect();

                DataSummary {
                    item_count: funcs,
                    preview_lines: first_lines.clone(),
                    search_text: first_lines.join(" "),
                }
            }
        }
    }
}
