use super::editor::{AstEditContext, AstNodeEditDraft};
use eframe::egui::{self, Color32, Ui};
use fireball::abstract_syntax_tree::AstNodePath;

pub struct AstEditorPopup {
    pub title: String,
    pub path: AstNodePath,
    pub draft: AstNodeEditDraft,
}

pub fn show_variable_editor(
    ui: &mut Ui,
    _title: &str,
    name: &str,
    var_type: &str,
) -> Option<AstNodeEditDraft> {
    let mut edited = false;
    let mut new_name = name.to_string();
    let mut new_type = var_type.to_string();

    ui.label("Name:");
    ui.add_sized(
        [200.0, 24.0],
        egui::TextEdit::singleline(&mut new_name).font(egui::TextStyle::Monospace),
    );
    edited |= new_name != name;

    ui.add_space(8.0);
    ui.label("Type:");
    ui.add_sized(
        [200.0, 24.0],
        egui::TextEdit::singleline(&mut new_type).font(egui::TextStyle::Monospace),
    );
    edited |= new_type != var_type;

    if new_name.is_empty() {
        ui.colored_label(Color32::from_rgb(0xCA, 0x50, 0x10), "Name cannot be empty");
    }

    if edited && !new_name.is_empty() {
        Some(AstNodeEditDraft::VariableEdit {
            path: AstNodePath::function(0), // Will be set by caller
            current_name: name.to_string(),
            current_type: var_type.to_string(),
            new_name,
            new_type,
        })
    } else {
        None
    }
}

pub fn show_literal_editor(ui: &mut Ui, value: &str, value_type: &str) -> Option<AstNodeEditDraft> {
    let mut edited = false;
    let mut new_value = value.to_string();

    ui.label("Value:");
    ui.add_sized(
        [200.0, 24.0],
        egui::TextEdit::singleline(&mut new_value).font(egui::TextStyle::Monospace),
    );
    edited |= new_value != value;

    ui.add_space(4.0);
    ui.label(format!("Type: {}", value_type));

    if new_value.is_empty() {
        ui.colored_label(Color32::from_rgb(0xCA, 0x50, 0x10), "Value cannot be empty");
    }

    if edited && !new_value.is_empty() {
        Some(AstNodeEditDraft::LiteralEdit {
            path: AstNodePath::function(0), // Will be set by caller
            current_value: value.to_string(),
            current_type: value_type.to_string(),
            new_value,
        })
    } else {
        None
    }
}

pub fn show_unary_operator_editor(ui: &mut Ui, current_op: &str) -> Option<AstNodeEditDraft> {
    let operators = vec![
        ("!", "Logical NOT (!)"),
        ("-", "Negate (-)"),
        ("~", "Bitwise NOT (~)"),
        ("++", "Increment (++)"),
        ("--", "Decrement (--)"),
        ("*", "Dereference (*)"),
        ("&", "Address-of (&)"),
    ];

    let mut new_op = current_op.to_string();

    ui.label("Operator:");
    egui::ComboBox::from_id_salt("unary-op")
        .selected_text(new_op.clone())
        .show_ui(ui, |ui| {
            for (op, desc) in operators {
                ui.selectable_value(&mut new_op, op.to_string(), format!("{} - {}", op, desc));
            }
        });

    if new_op != current_op {
        Some(AstNodeEditDraft::UnaryOperatorEdit {
            path: AstNodePath::function(0), // Will be set by caller
            current_op: current_op.to_string(),
            new_op,
        })
    } else {
        None
    }
}

pub fn show_binary_operator_editor(ui: &mut Ui, current_op: &str) -> Option<AstNodeEditDraft> {
    let operators = vec![
        ("+", "Addition"),
        ("-", "Subtraction"),
        ("*", "Multiplication"),
        ("/", "Division"),
        ("%", "Modulo"),
        ("&&", "Logical AND"),
        ("||", "Logical OR"),
        ("&", "Bitwise AND"),
        ("|", "Bitwise OR"),
        ("^", "Bitwise XOR"),
        ("<<", "Left Shift"),
        (">>", "Right Shift"),
        ("==", "Equal"),
        ("!=", "Not Equal"),
        ("<", "Less Than"),
        ("<=", "Less or Equal"),
        (">", "Greater Than"),
        (">=", "Greater or Equal"),
    ];

    let mut new_op = current_op.to_string();

    ui.label("Operator:");
    egui::ComboBox::from_id_salt("binary-op")
        .selected_text(new_op.clone())
        .show_ui(ui, |ui| {
            for (op, desc) in operators {
                ui.selectable_value(&mut new_op, op.to_string(), format!("{} - {}", op, desc));
            }
        });

    if new_op != current_op {
        Some(AstNodeEditDraft::BinaryOperatorEdit {
            path: AstNodePath::function(0), // Will be set by caller
            current_op: current_op.to_string(),
            new_op,
        })
    } else {
        None
    }
}

pub fn show_statement_editor(
    ui: &mut Ui,
    statement_type: &str,
    text: &str,
) -> Option<AstNodeEditDraft> {
    let mut replacement = text.to_string();

    ui.label(format!("Statement Type: {}", statement_type));
    ui.add_space(8.0);
    ui.label("Replacement:");
    ui.add(
        egui::TextEdit::multiline(&mut replacement)
            .desired_rows(5)
            .font(egui::TextStyle::Monospace),
    );

    if replacement.is_empty() {
        ui.colored_label(
            Color32::from_rgb(0xCA, 0x50, 0x10),
            "Statement cannot be empty",
        );
    }

    if replacement != text && !replacement.is_empty() {
        Some(AstNodeEditDraft::StatementEdit {
            path: AstNodePath::function(0), // Will be set by caller
            statement_type: statement_type.to_string(),
            replacement,
        })
    } else {
        None
    }
}

pub fn show_function_editor(
    ui: &mut Ui,
    name: &str,
    return_type: &str,
    _parameters: &[(String, String)],
) -> Option<AstNodeEditDraft> {
    let mut new_name = name.to_string();
    let mut new_return_type = return_type.to_string();

    ui.label("Function Name:");
    ui.add_sized(
        [200.0, 24.0],
        egui::TextEdit::singleline(&mut new_name).font(egui::TextStyle::Monospace),
    );

    ui.add_space(8.0);
    ui.label("Return Type:");
    ui.add_sized(
        [200.0, 24.0],
        egui::TextEdit::singleline(&mut new_return_type).font(egui::TextStyle::Monospace),
    );

    // Note: Function editing is complex - for now, treat as statement edit
    // Full function editor would need parameter management
    None
}

pub fn show_editor_for_context(
    ui: &mut Ui,
    context: &AstEditContext,
    path: &AstNodePath,
) -> Option<AstNodeEditDraft> {
    let mut draft = match context {
        AstEditContext::Variable { name, var_type } => {
            show_variable_editor(ui, "Edit Variable", name, var_type)
        }
        AstEditContext::Literal { value, value_type } => show_literal_editor(ui, value, value_type),
        AstEditContext::UnaryOperator { operator, .. } => show_unary_operator_editor(ui, operator),
        AstEditContext::BinaryOperator { operator, .. } => {
            show_binary_operator_editor(ui, operator)
        }
        AstEditContext::Statement {
            statement_type,
            text,
        } => show_statement_editor(ui, statement_type, text),
        AstEditContext::Function {
            name,
            return_type,
            parameters,
        } => show_function_editor(ui, name, return_type, parameters),
    };

    // Update the path in the draft
    if let Some(ref mut d) = draft {
        match d {
            AstNodeEditDraft::VariableEdit { path: p, .. } => *p = path.clone(),
            AstNodeEditDraft::LiteralEdit { path: p, .. } => *p = path.clone(),
            AstNodeEditDraft::UnaryOperatorEdit { path: p, .. } => *p = path.clone(),
            AstNodeEditDraft::BinaryOperatorEdit { path: p, .. } => *p = path.clone(),
            AstNodeEditDraft::StatementEdit { path: p, .. } => *p = path.clone(),
        }
    }

    draft
}
