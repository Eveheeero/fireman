use crate::model::{AssemblyEditorDraft, AstNodeEditorDraft, EditRequest, IrEditorDraft};
use eframe::egui;

pub enum EditorAction {
    Apply(EditRequest),
    Reset,
    Export,
    Close,
}

pub enum EditorWindowContent {
    Idle,
    Assembly {
        row: usize,
        draft: AssemblyEditorDraft,
    },
    Ir {
        row: usize,
        draft: IrEditorDraft,
    },
    AstNode {
        path: fireball::abstract_syntax_tree::AstNodePath,
        draft: AstNodeEditorDraft,
    },
}

pub struct FloatingEditorWindow {
    pub id: egui::Id,
    pub title: String,
    pub open: bool,
    pub position: Option<egui::Pos2>,
    pub size: egui::Vec2,
    pub content: EditorWindowContent,
}

impl FloatingEditorWindow {
    pub fn new(title: &str) -> Self {
        Self {
            id: egui::Id::new(title),
            title: title.to_string(),
            open: false, // Closed by default on startup
            position: None,
            size: egui::vec2(380.0, 280.0),
            content: EditorWindowContent::Idle,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, controls_enabled: bool) -> Option<EditorAction> {
        let mut action = None;
        let mut window_open = self.open;

        let window_response = egui::Window::new(&self.title)
            .id(self.id)
            .open(&mut window_open)
            .default_size(self.size)
            .collapsible(true)
            .resizable(true)
            .show(ctx, |ui| match &mut self.content {
                EditorWindowContent::Idle => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(40.0);
                        ui.label("Click an element to edit");
                    });
                }
                EditorWindowContent::Assembly { row, draft } => {
                    if let Some(act) =
                        Self::render_assembly_editor(ui, *row, draft, controls_enabled)
                    {
                        action = Some(act);
                    }
                }
                EditorWindowContent::Ir { row, draft } => {
                    if let Some(act) = Self::render_ir_editor(ui, *row, draft, controls_enabled) {
                        action = Some(act);
                    }
                }
                EditorWindowContent::AstNode { path, draft } => {
                    if let Some(act) = Self::render_ast_editor(ui, path, draft, controls_enabled) {
                        action = Some(act);
                    }
                }
            });

        if let Some(response) = window_response {
            if response.response.clicked() || response.response.dragged() {
                self.position = Some(response.response.rect.min);
            }
        }

        if !window_open && self.open {
            action = Some(EditorAction::Close);
        }
        self.open = window_open;

        action
    }

    fn render_assembly_editor(
        ui: &mut egui::Ui,
        row: usize,
        draft: &mut AssemblyEditorDraft,
        controls_enabled: bool,
    ) -> Option<EditorAction> {
        let mut action = None;

        ui.label(egui::RichText::new(format!("Assembly Editor - Row {}", row)).strong());
        ui.add_space(8.0);

        // Check if keystone feature is enabled
        #[cfg(not(feature = "keystone"))]
        {
            ui.label("Assembly editing is not available.");
            ui.label("Build with 'keystone' feature to enable assembly patching.");
            ui.add_space(8.0);
            ui.label("Original:");
            ui.add(
                egui::TextEdit::multiline(&mut draft.raw_text)
                    .desired_rows(2)
                    .font(egui::TextStyle::Monospace)
                    .interactive(false),
            );
            return action;
        }

        #[cfg(feature = "keystone")]
        {
            // Show the original line with address (read-only)
            ui.label("Original:");
            ui.add(
                egui::TextEdit::multiline(&mut draft.raw_text)
                    .desired_rows(2)
                    .font(egui::TextStyle::Monospace)
                    .interactive(false),
            );

            ui.add_space(8.0);
            ui.label("Edit instruction (address cannot be changed):");

            // Extract address for display
            let address = extract_assembly_address(&draft.raw_text);
            ui.horizontal(|ui| {
                ui.label("Address:");
                ui.add(
                    egui::TextEdit::singleline(&mut address.clone())
                        .font(egui::TextStyle::Monospace)
                        .interactive(false),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Mnemonic:");
                ui.add_sized(
                    [100.0, 24.0],
                    egui::TextEdit::singleline(&mut draft.mnemonic)
                        .font(egui::TextStyle::Monospace),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Operands:");
                ui.add(
                    egui::TextEdit::singleline(&mut draft.operands)
                        .font(egui::TextStyle::Monospace),
                );
            });

            if let Some(msg) = &draft.status_message {
                ui.add_space(6.0);
                ui.colored_label(egui::Color32::from_rgb(0xCA, 0x50, 0x10), msg);
            }

            ui.add_space(12.0);
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(controls_enabled, egui::Button::new("Apply"))
                    .clicked()
                {
                    // Compose instruction from mnemonic and operands (without address)
                    let instruction = compose_head_tail(&draft.mnemonic, &draft.operands);
                    action = Some(EditorAction::Apply(EditRequest {
                        layer: crate::model::EditorLayer::Assembly,
                        row,
                        position: crate::model::EditPosition::Replace,
                        text: instruction,
                    }));
                }
                if ui.button("Reset").clicked() {
                    action = Some(EditorAction::Reset);
                }
                if ui
                    .add_enabled(controls_enabled, egui::Button::new("Export"))
                    .clicked()
                {
                    action = Some(EditorAction::Export);
                }
            });
        }

        action
    }

    fn render_ir_editor(
        ui: &mut egui::Ui,
        row: usize,
        draft: &mut IrEditorDraft,
        controls_enabled: bool,
    ) -> Option<EditorAction> {
        let mut action = None;

        ui.label(egui::RichText::new(format!("IR Editor - Row {}", row)).strong());
        ui.add_space(8.0);

        ui.label("IR Statement:");
        ui.add(
            egui::TextEdit::multiline(&mut draft.raw_text)
                .desired_rows(3)
                .font(egui::TextStyle::Monospace),
        );

        ui.add_space(6.0);
        egui::ComboBox::from_id_salt(format!("ir-pos-{}", row))
            .selected_text(draft.position.label())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut draft.position,
                    crate::model::EditPosition::Replace,
                    "Replace",
                );
                ui.selectable_value(
                    &mut draft.position,
                    crate::model::EditPosition::Before,
                    "Insert Before",
                );
                ui.selectable_value(
                    &mut draft.position,
                    crate::model::EditPosition::After,
                    "Insert After",
                );
            });

        if let Some(msg) = &draft.status_message {
            ui.add_space(6.0);
            ui.colored_label(egui::Color32::from_rgb(0xCA, 0x50, 0x10), msg);
        }

        ui.add_space(12.0);
        ui.horizontal(|ui| {
            if ui
                .add_enabled(controls_enabled, egui::Button::new("Apply"))
                .clicked()
            {
                // Validate the IR statement before applying
                let trimmed = draft.raw_text.trim();
                if trimmed.is_empty() {
                    draft.status_message = Some("IR statement cannot be empty".to_string());
                } else {
                    action = Some(EditorAction::Apply(EditRequest {
                        layer: crate::model::EditorLayer::Ir,
                        row,
                        position: draft.position,
                        text: trimmed.to_string(),
                    }));
                }
            }
            if ui.button("Reset").clicked() {
                action = Some(EditorAction::Reset);
            }
            if ui
                .add_enabled(controls_enabled, egui::Button::new("Export"))
                .clicked()
            {
                action = Some(EditorAction::Export);
            }
        });

        action
    }

    fn render_ast_editor(
        ui: &mut egui::Ui,
        path: &fireball::abstract_syntax_tree::AstNodePath,
        draft: &mut AstNodeEditorDraft,
        controls_enabled: bool,
    ) -> Option<EditorAction> {
        let mut action = None;

        ui.label(egui::RichText::new("AST Editor").strong());
        ui.add_space(4.0);
        ui.label(format!("Path: {:?}", path));
        ui.add_space(8.0);

        use crate::model::AstNodeDraftData;
        match &mut draft.draft_data {
            AstNodeDraftData::Variable {
                new_name,
                new_type,
                current_name,
                current_type,
            } => {
                ui.label("Variable:");
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.add_sized(
                        [150.0, 24.0],
                        egui::TextEdit::singleline(new_name).font(egui::TextStyle::Monospace),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Type:");
                    ui.add_sized(
                        [150.0, 24.0],
                        egui::TextEdit::singleline(new_type).font(egui::TextStyle::Monospace),
                    );
                });
                ui.small(format!("Original: {} {}", current_type, current_name));
            }
            AstNodeDraftData::Literal {
                new_value,
                current_value,
                current_type,
            } => {
                ui.label("Literal:");
                ui.horizontal(|ui| {
                    ui.label("Value:");
                    ui.add_sized(
                        [150.0, 24.0],
                        egui::TextEdit::singleline(new_value).font(egui::TextStyle::Monospace),
                    );
                });
                ui.small(format!("Type: {}", current_type));
                ui.small(format!("Original: {}", current_value));
            }
            AstNodeDraftData::UnaryOperator {
                new_op,
                current_op,
                operand,
            } => {
                ui.label(format!("Unary Operator: {}({})", current_op, operand));
                ui.horizontal(|ui| {
                    ui.label("Operator:");
                    let ops = vec!["!", "-", "~", "++", "--", "*", "&"];
                    egui::ComboBox::from_id_salt("unary-op")
                        .selected_text(new_op.clone())
                        .show_ui(ui, |ui| {
                            for op in ops {
                                ui.selectable_value(new_op, op.to_string(), op);
                            }
                        });
                });
            }
            AstNodeDraftData::BinaryOperator {
                new_op,
                current_op,
                left,
                right,
            } => {
                ui.label(format!("Binary: {} {} {}", left, current_op, right));
                ui.horizontal(|ui| {
                    ui.label("Operator:");
                    let ops = vec![
                        "+", "-", "*", "/", "%", "&&", "||", "&", "|", "^", "<<", ">>", "==", "!=",
                        "<", "<=", ">", ">=",
                    ];
                    egui::ComboBox::from_id_salt("binary-op")
                        .selected_text(new_op.clone())
                        .show_ui(ui, |ui| {
                            for op in ops {
                                ui.selectable_value(new_op, op.to_string(), op);
                            }
                        });
                });
            }
            AstNodeDraftData::Statement {
                statement_type,
                replacement,
            } => {
                ui.label(format!("Statement ({}):", statement_type));
                ui.add(
                    egui::TextEdit::multiline(replacement)
                        .desired_rows(4)
                        .font(egui::TextStyle::Monospace),
                );
            }
            AstNodeDraftData::Function {
                new_name,
                new_return_type,
                current_name,
                current_return_type,
            } => {
                ui.label("Function:");
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.add_sized(
                        [150.0, 24.0],
                        egui::TextEdit::singleline(new_name).font(egui::TextStyle::Monospace),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Return Type:");
                    ui.add_sized(
                        [150.0, 24.0],
                        egui::TextEdit::singleline(new_return_type)
                            .font(egui::TextStyle::Monospace),
                    );
                });
                ui.small(format!(
                    "Original: {} {}",
                    current_return_type, current_name
                ));
            }
        }

        if let Some(msg) = &draft.status_message {
            ui.add_space(6.0);
            ui.colored_label(egui::Color32::from_rgb(0xCA, 0x50, 0x10), msg);
        }

        ui.add_space(12.0);
        ui.horizontal(|ui| {
            if ui
                .add_enabled(controls_enabled, egui::Button::new("Apply"))
                .clicked()
            {
                // Convert to text edit for now
                let text = match &draft.draft_data {
                    AstNodeDraftData::Statement { replacement, .. } => replacement.clone(),
                    AstNodeDraftData::Variable { new_name, .. } => {
                        format!("// rename to {}", new_name)
                    }
                    AstNodeDraftData::Literal { new_value, .. } => new_value.clone(),
                    AstNodeDraftData::UnaryOperator {
                        new_op, operand, ..
                    } => format!("{}({})", new_op, operand),
                    AstNodeDraftData::BinaryOperator {
                        new_op,
                        left,
                        right,
                        ..
                    } => format!("{} {} {}", left, new_op, right),
                    AstNodeDraftData::Function { new_name, .. } => {
                        format!("// rename to {}", new_name)
                    }
                };
                action = Some(EditorAction::Apply(EditRequest {
                    layer: crate::model::EditorLayer::Ast,
                    row: 0, // TODO: map path to row
                    position: crate::model::EditPosition::Replace,
                    text,
                }));
            }
            if ui.button("Reset").clicked() {
                action = Some(EditorAction::Reset);
            }
            if ui
                .add_enabled(controls_enabled, egui::Button::new("Export"))
                .clicked()
            {
                action = Some(EditorAction::Export);
            }
        });

        action
    }

    pub fn set_assembly(&mut self, row: usize, draft: AssemblyEditorDraft) {
        self.content = EditorWindowContent::Assembly { row, draft };
        self.open = true;
        self.title = format!("Assembly Editor - Row {}", row);
    }

    pub fn set_ir(&mut self, row: usize, draft: IrEditorDraft) {
        self.content = EditorWindowContent::Ir { row, draft };
        self.open = true;
        self.title = format!("IR Editor - Row {}", row);
    }

    pub fn set_ast_node(
        &mut self,
        path: fireball::abstract_syntax_tree::AstNodePath,
        draft: AstNodeEditorDraft,
    ) {
        self.title = format!("AST Editor - {:?}", path);
        self.content = EditorWindowContent::AstNode { path, draft };
        self.open = true;
    }

    pub fn set_idle(&mut self, title: &str) {
        self.title = title.to_string();
        self.content = EditorWindowContent::Idle;
    }
}

// Helper function to strip assembly address prefix
fn strip_assembly_address(text: &str) -> &str {
    let trimmed = text.trim();
    let Some((head, tail)) = trimmed.split_once(char::is_whitespace) else {
        return trimmed;
    };
    if head.starts_with("0x")
        && head.len() > 2
        && head[2..].chars().all(|ch| ch.is_ascii_hexdigit())
    {
        tail.trim()
    } else {
        trimmed
    }
}

// Helper function to extract assembly address
fn extract_assembly_address(text: &str) -> String {
    let trimmed = text.trim();
    let Some((head, _tail)) = trimmed.split_once(char::is_whitespace) else {
        return String::new();
    };
    if head.starts_with("0x")
        && head.len() > 2
        && head[2..].chars().all(|ch| ch.is_ascii_hexdigit())
    {
        head.to_string()
    } else {
        String::new()
    }
}

// Helper function to compose instruction from head and tail
fn compose_head_tail(head: &str, tail: &str) -> String {
    let head = head.trim();
    let tail = tail.trim();
    if tail.is_empty() {
        head.to_string()
    } else {
        format!("{} {}", head, tail)
    }
}
