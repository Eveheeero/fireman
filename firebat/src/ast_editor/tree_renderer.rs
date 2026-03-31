use crate::ast_editor::styles::AstStyles;
use eframe::egui::{self, Ui};
use fireball::abstract_syntax_tree::{
    ArcAstVariableMap, Ast, AstBinaryOperator, AstCall, AstExpression, AstFunction, AstLiteral,
    AstNodePath, AstParameter, AstStatement, AstUnaryOperator, AstValueType, AstVariable,
    AstVariableId, ExpressionPathComponent, Wrapped, WrappedAstStatement,
};

pub struct AstTreeRenderer<'a> {
    ast: &'a Ast,
    styles: &'a AstStyles,
    selected_path: Option<&'a AstNodePath>,
    show_ir_comments: bool,
}

impl<'a> AstTreeRenderer<'a> {
    pub fn new(
        ast: &'a Ast,
        styles: &'a AstStyles,
        selected_path: Option<&'a AstNodePath>,
        show_ir_comments: bool,
    ) -> Self {
        Self {
            ast,
            styles,
            selected_path,
            show_ir_comments,
        }
    }

    /// Main entry point - renders entire AST as formatted C code
    pub fn render(&self, ui: &mut Ui) -> Option<AstNodePath> {
        let mut clicked = None;

        // Set width to fill available space
        ui.set_width(ui.available_width());

        // Get functions from AST and sort by address
        let functions = self.ast.functions.read().unwrap();
        let mut func_vec: Vec<_> = functions.iter().collect();
        func_vec.sort_by_key(|(func_id, _)| func_id.address());

        for (idx, (func_id, version_map)) in func_vec.iter().enumerate() {
            let func = version_map.get_last_version();
            let func_path = AstNodePath::function(idx);

            // Show function address before the function
            ui.horizontal(|ui| {
                ui.label(
                    self.styles
                        .comment(&format!("// 0x{:X}", func_id.address())),
                );
            });

            ui.horizontal(|ui| {
                // Render: [return_type] [func_name]([params]) {
                if let Some(path) = self.render_function_header(ui, func, &func_path) {
                    clicked = Some(path);
                }
            });

            // Body with 4-space indentation
            ui.indent("body", |ui| {
                for (stmt_idx, stmt) in func.body.iter().enumerate() {
                    let stmt_path = func_path.child_statement(stmt_idx).unwrap();
                    if let Some(path) = self.render_statement(ui, stmt, &stmt_path, 1) {
                        clicked = Some(path);
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label(self.styles.plain("}"));
            });
            ui.add_space(8.0); // Space between functions
        }

        clicked
    }

    /// Renders function signature: "void func_name(int32_t param1, int32_t param2) {"
    fn render_function_header(
        &self,
        ui: &mut Ui,
        func: &AstFunction,
        path: &AstNodePath,
    ) -> Option<AstNodePath> {
        let mut clicked = None;

        // Return type (clickable)
        let type_text = self
            .styles
            .type_name(&self.type_to_string(&func.return_type));
        let response = ui.add(egui::Label::new(type_text).sense(egui::Sense::click()));
        if response.clicked() {
            clicked = Some(path.clone());
        }

        ui.add_space(4.0);

        // Function name (clickable)
        let name_text = self.styles.function(&func.name());
        let response = ui.add(egui::Label::new(name_text).sense(egui::Sense::click()));
        if response.clicked() && clicked.is_none() {
            clicked = Some(path.clone());
        }

        ui.add_space(4.0);

        // Opening paren
        ui.label(self.styles.plain("("));

        // Parameters (each clickable)
        for (i, param) in func.parameters.iter().enumerate() {
            if i > 0 {
                ui.label(self.styles.plain(","));
                ui.add_space(4.0);
            }

            // Resolve parameter name and type using public methods
            let (name, var_type) = self.resolve_parameter(param, &func.variables);

            // Param type
            let type_text = self.styles.type_name(&var_type);
            ui.add(egui::Label::new(type_text));

            ui.add_space(4.0);

            // Param name (clickable)
            let name_text = self.styles.variable(&name);
            let response = ui.add(egui::Label::new(name_text).sense(egui::Sense::click()));
            if response.clicked() {
                // Create path for parameter
            }
        }

        // Closing paren and opening brace
        ui.label(self.styles.plain(") {"));

        clicked
    }

    /// Renders a statement with proper formatting
    fn render_statement(
        &self,
        ui: &mut Ui,
        stmt: &WrappedAstStatement,
        path: &AstNodePath,
        _depth: usize,
    ) -> Option<AstNodePath> {
        match &stmt.statement {
            AstStatement::Declaration(var, init) => {
                ui.horizontal(|ui| self.render_declaration(ui, var, init.as_ref(), path))
                    .inner
            }
            AstStatement::Assignment(left, right) => {
                ui.horizontal(|ui| self.render_assignment(ui, left, right, path))
                    .inner
            }
            AstStatement::If(cond, then_body, else_body) => {
                self.render_if(ui, cond, then_body, else_body.as_ref(), path, _depth)
            }
            AstStatement::While(cond, body) => self.render_while(ui, cond, body, path, _depth),
            AstStatement::For(init, cond, update, body) => {
                self.render_for(ui, init, cond, update, body, path, _depth)
            }
            AstStatement::Return(expr) => {
                ui.horizontal(|ui| self.render_return(ui, expr.as_ref(), path))
                    .inner
            }
            AstStatement::Call(call) => ui.horizontal(|ui| self.render_call(ui, call, path)).inner,
            AstStatement::Block(stmts) => self.render_block(ui, stmts, path, _depth),
            AstStatement::Comment(text) => {
                ui.horizontal(|ui| {
                    let comment_text = self.styles.comment(&format!("// {}", text));
                    ui.add(egui::Label::new(comment_text));
                    None
                })
                .inner
            }
            AstStatement::Break => {
                ui.horizontal(|ui| {
                    let keyword = self.styles.keyword("break");
                    let response = ui.add(egui::Label::new(keyword).sense(egui::Sense::click()));
                    ui.label(self.styles.plain(";"));
                    if response.clicked() {
                        Some(path.clone())
                    } else {
                        None
                    }
                })
                .inner
            }
            AstStatement::Continue => {
                ui.horizontal(|ui| {
                    let keyword = self.styles.keyword("continue");
                    let response = ui.add(egui::Label::new(keyword).sense(egui::Sense::click()));
                    ui.label(self.styles.plain(";"));
                    if response.clicked() {
                        Some(path.clone())
                    } else {
                        None
                    }
                })
                .inner
            }
            AstStatement::DoWhile(cond, body) => self.render_do_while(ui, cond, body, path, _depth),
            AstStatement::Assembly(asm) => {
                ui.horizontal(|ui| {
                    let asm_text = self.styles.comment(&format!("asm {{ {} }}", asm));
                    ui.add(egui::Label::new(asm_text));
                    None
                })
                .inner
            }
            AstStatement::Ir(ir) => {
                ui.horizontal(|ui| {
                    let ir_text = self.styles.comment(&format!("/* IR: {:?} */", ir));
                    ui.add(egui::Label::new(ir_text));
                    None
                })
                .inner
            }
            _ => {
                ui.horizontal(|ui| {
                    ui.label("[unknown statement]");
                    None
                })
                .inner
            }
        }
    }

    fn render_declaration(
        &self,
        ui: &mut Ui,
        var: &AstVariable,
        init: Option<&Wrapped<AstExpression>>,
        path: &AstNodePath,
    ) -> Option<AstNodePath> {
        let mut clicked = None;

        // Type (clickable)
        let type_text = self.styles.type_name(&self.type_to_string(&var.var_type));
        let response = ui.add(egui::Label::new(type_text).sense(egui::Sense::click()));
        if response.clicked() {
            clicked = Some(
                path.child_expression(ExpressionPathComponent::Body)
                    .unwrap(),
            );
        }

        ui.add_space(4.0);

        // Variable name (clickable)
        let name = var.name();
        let name_text = self.styles.variable(&name);
        let response = ui.add(egui::Label::new(name_text).sense(egui::Sense::click()));
        if response.clicked() && clicked.is_none() {
            clicked = Some(
                path.child_expression(ExpressionPathComponent::Body)
                    .unwrap(),
            );
        }

        // Optional initializer
        if let Some(init_expr) = init {
            ui.add_space(4.0);
            let op_text = self.styles.operator("=");
            let response = ui.add(egui::Label::new(op_text).sense(egui::Sense::click()));
            if response.clicked() {
                clicked = Some(path.clone());
            }
            ui.add_space(4.0);
            let init_path = path
                .child_expression(ExpressionPathComponent::Body)
                .unwrap();
            if let Some(p) = self.render_expression(ui, init_expr, &init_path) {
                clicked = Some(p);
            }
        }

        ui.label(self.styles.plain(";"));

        clicked
    }

    fn render_assignment(
        &self,
        ui: &mut Ui,
        left: &Wrapped<AstExpression>,
        right: &Wrapped<AstExpression>,
        path: &AstNodePath,
    ) -> Option<AstNodePath> {
        let mut clicked = None;

        // Left expression
        let left_path = path
            .child_expression(ExpressionPathComponent::Left)
            .unwrap();
        if let Some(p) = self.render_expression(ui, left, &left_path) {
            clicked = Some(p);
        }

        ui.add_space(4.0);

        // = operator (clickable)
        let op_text = self.styles.operator("=");
        let response = ui.add(egui::Label::new(op_text).sense(egui::Sense::click()));
        if response.clicked() {
            clicked = Some(path.clone());
        }

        ui.add_space(4.0);

        // Right expression
        let right_path = path
            .child_expression(ExpressionPathComponent::Right)
            .unwrap();
        if let Some(p) = self.render_expression(ui, right, &right_path) {
            clicked = Some(p);
        }

        ui.label(self.styles.plain(";"));

        clicked
    }

    fn render_if(
        &self,
        ui: &mut Ui,
        cond: &Wrapped<AstExpression>,
        then_body: &[WrappedAstStatement],
        else_body: Option<&Vec<WrappedAstStatement>>,
        path: &AstNodePath,
        depth: usize,
    ) -> Option<AstNodePath> {
        let mut clicked = None;

        // if keyword (clickable)
        let keyword = self.styles.keyword("if");
        let response = ui.add(egui::Label::new(keyword).sense(egui::Sense::click()));
        if response.clicked() {
            clicked = Some(path.clone());
        }

        ui.add_space(4.0);
        ui.label(self.styles.plain("("));

        // Condition expression
        let cond_path = path
            .child_expression(ExpressionPathComponent::Condition)
            .unwrap();
        if let Some(p) = self.render_expression(ui, cond, &cond_path) {
            clicked = Some(p);
        }

        ui.label(self.styles.plain(")"));
        ui.add_space(4.0);
        ui.label(self.styles.plain("{"));

        // Then body with indentation
        ui.indent("then", |ui| {
            for (idx, stmt) in then_body.iter().enumerate() {
                let stmt_path = path.child_statement(idx).unwrap();
                if let Some(p) = self.render_statement(ui, stmt, &stmt_path, depth + 1) {
                    clicked = Some(p);
                }
            }
        });

        ui.horizontal(|ui| {
            ui.label(self.styles.plain("}"));
        });

        // Else clause
        if let Some(else_stmts) = else_body {
            ui.add_space(4.0);
            let else_keyword = self.styles.keyword("else");
            let response = ui.add(egui::Label::new(else_keyword).sense(egui::Sense::click()));
            if response.clicked() && clicked.is_none() {
                clicked = Some(path.clone());
            }
            ui.add_space(4.0);
            ui.label(self.styles.plain("{"));

            ui.indent("else", |ui| {
                for (idx, stmt) in else_stmts.iter().enumerate() {
                    let stmt_path = path.child_statement(then_body.len() + idx).unwrap();
                    if let Some(p) = self.render_statement(ui, stmt, &stmt_path, depth + 1) {
                        clicked = Some(p);
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label(self.styles.plain("}"));
            });
        }

        clicked
    }

    fn render_while(
        &self,
        ui: &mut Ui,
        cond: &Wrapped<AstExpression>,
        body: &[WrappedAstStatement],
        path: &AstNodePath,
        depth: usize,
    ) -> Option<AstNodePath> {
        let mut clicked = None;

        // while keyword (clickable)
        let keyword = self.styles.keyword("while");
        let response = ui.add(egui::Label::new(keyword).sense(egui::Sense::click()));
        if response.clicked() {
            clicked = Some(path.clone());
        }

        ui.add_space(4.0);
        ui.label(self.styles.plain("("));

        // Condition
        let cond_path = path
            .child_expression(ExpressionPathComponent::Condition)
            .unwrap();
        if let Some(p) = self.render_expression(ui, cond, &cond_path) {
            clicked = Some(p);
        }

        ui.label(self.styles.plain(")"));
        ui.add_space(4.0);
        ui.label(self.styles.plain("{"));

        // Body
        ui.indent("while_body", |ui| {
            for (idx, stmt) in body.iter().enumerate() {
                let stmt_path = path.child_statement(idx).unwrap();
                if let Some(p) = self.render_statement(ui, stmt, &stmt_path, depth + 1) {
                    clicked = Some(p);
                }
            }
        });

        ui.horizontal(|ui| {
            ui.label(self.styles.plain("}"));
        });

        clicked
    }

    fn render_for(
        &self,
        ui: &mut Ui,
        init: &WrappedAstStatement,
        cond: &Wrapped<AstExpression>,
        update: &WrappedAstStatement,
        body: &[WrappedAstStatement],
        path: &AstNodePath,
        depth: usize,
    ) -> Option<AstNodePath> {
        let mut clicked = None;

        // for keyword (clickable)
        let keyword = self.styles.keyword("for");
        let response = ui.add(egui::Label::new(keyword).sense(egui::Sense::click()));
        if response.clicked() {
            clicked = Some(path.clone());
        }

        ui.add_space(4.0);
        ui.label(self.styles.plain("("));

        // Init (inline, no semicolon in for-header)
        let init_path = path.child_statement(0).unwrap();
        self.render_statement_in_header(ui, init, &init_path, &mut clicked);

        ui.label(self.styles.plain(";"));
        ui.add_space(4.0);

        // Condition
        let cond_path = path
            .child_expression(ExpressionPathComponent::Condition)
            .unwrap();
        if let Some(p) = self.render_expression(ui, cond, &cond_path) {
            clicked = Some(p);
        }

        ui.label(self.styles.plain(";"));
        ui.add_space(4.0);

        // Update (inline)
        let update_path = path.child_statement(1).unwrap();
        self.render_statement_in_header(ui, update, &update_path, &mut clicked);

        ui.label(self.styles.plain(")"));
        ui.add_space(4.0);
        ui.label(self.styles.plain("{"));

        // Body
        ui.indent("for_body", |ui| {
            for (idx, stmt) in body.iter().enumerate() {
                let stmt_path = path.child_statement(2 + idx).unwrap();
                if let Some(p) = self.render_statement(ui, stmt, &stmt_path, depth + 1) {
                    clicked = Some(p);
                }
            }
        });

        ui.horizontal(|ui| {
            ui.label(self.styles.plain("}"));
        });

        clicked
    }

    fn render_do_while(
        &self,
        ui: &mut Ui,
        cond: &Wrapped<AstExpression>,
        body: &[WrappedAstStatement],
        path: &AstNodePath,
        depth: usize,
    ) -> Option<AstNodePath> {
        let mut clicked = None;

        // do keyword (clickable)
        let keyword = self.styles.keyword("do");
        let response = ui.add(egui::Label::new(keyword).sense(egui::Sense::click()));
        if response.clicked() {
            clicked = Some(path.clone());
        }

        ui.add_space(4.0);
        ui.label(self.styles.plain("{"));

        // Body
        ui.indent("do_body", |ui| {
            for (idx, stmt) in body.iter().enumerate() {
                let stmt_path = path.child_statement(idx).unwrap();
                if let Some(p) = self.render_statement(ui, stmt, &stmt_path, depth + 1) {
                    clicked = Some(p);
                }
            }
        });

        ui.horizontal(|ui| {
            ui.label(self.styles.plain("}"));
        });

        ui.add_space(4.0);
        let while_kw = self.styles.keyword("while");
        ui.add(egui::Label::new(while_kw));
        ui.add_space(4.0);
        ui.label(self.styles.plain("("));

        // Condition
        let cond_path = path
            .child_expression(ExpressionPathComponent::Condition)
            .unwrap();
        if let Some(p) = self.render_expression(ui, cond, &cond_path) {
            clicked = Some(p);
        }

        ui.label(self.styles.plain(")"));
        ui.label(self.styles.plain(";"));

        clicked
    }

    fn render_return(
        &self,
        ui: &mut Ui,
        expr: Option<&Wrapped<AstExpression>>,
        path: &AstNodePath,
    ) -> Option<AstNodePath> {
        let mut clicked = None;

        // return keyword (clickable)
        let keyword = self.styles.keyword("return");
        let response = ui.add(egui::Label::new(keyword).sense(egui::Sense::click()));
        if response.clicked() {
            clicked = Some(path.clone());
        }

        if let Some(ret_expr) = expr {
            ui.add_space(4.0);
            let expr_path = path
                .child_expression(ExpressionPathComponent::Body)
                .unwrap();
            if let Some(p) = self.render_expression(ui, ret_expr, &expr_path) {
                clicked = Some(p);
            }
        }

        ui.label(self.styles.plain(";"));

        clicked
    }

    fn render_call(&self, ui: &mut Ui, call: &AstCall, path: &AstNodePath) -> Option<AstNodePath> {
        let mut clicked = None;

        // Get function name
        let name = match call {
            AstCall::Variable {
                var_map,
                var_id,
                args: _,
                scope: _,
            } => {
                let vars = var_map.read().unwrap();
                vars.get(var_id)
                    .map(|v| v.name().to_string())
                    .unwrap_or_else(|| var_id.get_default_name())
            }
            AstCall::Function { target, args: _ } => target.get_default_name().to_string(),
            AstCall::Unknown(name, _) => name.clone(),
            AstCall::Builtin(func, _arg) => {
                format!("{:?}", func)
            }
        };

        // Function name (clickable)
        let name_text = self.styles.function(&name);
        let response = ui.add(egui::Label::new(name_text).sense(egui::Sense::click()));
        if response.clicked() {
            clicked = Some(path.clone());
        }

        ui.label(self.styles.plain("("));

        // Arguments
        let args = match call {
            AstCall::Variable { args, .. } => args,
            AstCall::Function { args, .. } => args,
            AstCall::Unknown(_, args) => args,
            AstCall::Builtin(_, _arg) => {
                // Builtins have different arg handling
                ui.label(self.styles.plain("/* builtin args */"));
                ui.label(self.styles.plain(")"));
                ui.label(self.styles.plain(";"));
                return clicked;
            }
        };

        for (idx, arg) in args.iter().enumerate() {
            if idx > 0 {
                ui.label(self.styles.plain(","));
                ui.add_space(4.0);
            }
            let arg_path = path
                .child_expression(ExpressionPathComponent::Argument(idx))
                .unwrap();
            if let Some(p) = self.render_expression(ui, arg, &arg_path) {
                clicked = Some(p);
            }
        }

        ui.label(self.styles.plain(")"));
        ui.label(self.styles.plain(";"));

        clicked
    }

    fn render_block(
        &self,
        ui: &mut Ui,
        stmts: &[WrappedAstStatement],
        path: &AstNodePath,
        depth: usize,
    ) -> Option<AstNodePath> {
        let mut clicked = None;

        ui.horizontal(|ui| {
            ui.label(self.styles.plain("{"));
        });

        ui.indent("block", |ui| {
            for (idx, stmt) in stmts.iter().enumerate() {
                let stmt_path = path.child_statement(idx).unwrap();
                if let Some(p) = self.render_statement(ui, stmt, &stmt_path, depth + 1) {
                    clicked = Some(p);
                }
            }
        });

        ui.horizontal(|ui| {
            ui.label(self.styles.plain("}"));
        });

        clicked
    }

    /// Renders statement inline (for for-loop header)
    fn render_statement_in_header(
        &self,
        ui: &mut Ui,
        stmt: &WrappedAstStatement,
        path: &AstNodePath,
        clicked: &mut Option<AstNodePath>,
    ) {
        match &stmt.statement {
            AstStatement::Declaration(var, init) => {
                // Just type and name, no semicolon
                let type_text = self.styles.type_name(&self.type_to_string(&var.var_type));
                ui.add(egui::Label::new(type_text));
                ui.add_space(4.0);
                let name = var.name();
                let name_text = self.styles.variable(&name);
                let response = ui.add(egui::Label::new(name_text).sense(egui::Sense::click()));
                if response.clicked() {
                    *clicked = Some(path.clone());
                }
                if let Some(init_expr) = init {
                    ui.add_space(4.0);
                    ui.label(self.styles.operator("="));
                    ui.add_space(4.0);
                    let expr_path = path
                        .child_expression(ExpressionPathComponent::Body)
                        .unwrap();
                    if let Some(p) = self.render_expression(ui, init_expr, &expr_path) {
                        *clicked = Some(p);
                    }
                }
            }
            AstStatement::Assignment(left, right) => {
                let left_path = path
                    .child_expression(ExpressionPathComponent::Left)
                    .unwrap();
                if let Some(p) = self.render_expression(ui, left, &left_path) {
                    *clicked = Some(p);
                }
                ui.add_space(4.0);
                ui.label(self.styles.operator("="));
                ui.add_space(4.0);
                let right_path = path
                    .child_expression(ExpressionPathComponent::Right)
                    .unwrap();
                if let Some(p) = self.render_expression(ui, right, &right_path) {
                    *clicked = Some(p);
                }
            }
            _ => {
                ui.label(self.styles.plain("[...]"));
            }
        }
    }

    /// Renders expression inline (horizontal layout)
    fn render_expression(
        &self,
        ui: &mut Ui,
        expr: &Wrapped<AstExpression>,
        path: &AstNodePath,
    ) -> Option<AstNodePath> {
        match expr.as_ref() {
            AstExpression::Literal(lit) => {
                let value = self.literal_to_string(lit);
                let lit_text = self.styles.literal(&value);
                let response = ui.add(egui::Label::new(lit_text).sense(egui::Sense::click()));
                if response.clicked() {
                    Some(path.clone())
                } else {
                    None
                }
            }
            AstExpression::Variable(var_map, var_id) => {
                let name = self.get_variable_name(var_map, var_id);
                let var_text = self.styles.variable(&name);
                let response = ui.add(egui::Label::new(var_text).sense(egui::Sense::click()));
                if response.clicked() {
                    Some(path.clone())
                } else {
                    None
                }
            }
            AstExpression::UnaryOp(op, operand) => {
                let mut clicked = None;
                let op_str = self.unary_op_to_string(op);
                let op_text = self.styles.operator(&op_str);
                let response = ui.add(egui::Label::new(op_text).sense(egui::Sense::click()));
                if response.clicked() {
                    clicked = Some(path.clone());
                }
                let operand_path = path
                    .child_expression(ExpressionPathComponent::Operand(0))
                    .unwrap();
                if let Some(p) = self.render_expression(ui, operand, &operand_path) {
                    clicked = Some(p);
                }
                clicked
            }
            AstExpression::BinaryOp(op, left, right) => {
                let mut clicked = None;

                // Left
                let left_path = path
                    .child_expression(ExpressionPathComponent::Left)
                    .unwrap();
                if let Some(p) = self.render_expression(ui, left, &left_path) {
                    clicked = Some(p);
                }

                ui.add_space(4.0);

                // Operator (clickable)
                let op_str = self.binary_op_to_string(op);
                let op_text = self.styles.operator(&op_str);
                let response = ui.add(egui::Label::new(op_text).sense(egui::Sense::click()));
                if response.clicked() {
                    clicked = Some(path.clone());
                }

                ui.add_space(4.0);

                // Right
                let right_path = path
                    .child_expression(ExpressionPathComponent::Right)
                    .unwrap();
                if let Some(p) = self.render_expression(ui, right, &right_path) {
                    clicked = Some(p);
                }

                clicked
            }
            AstExpression::Cast(ty, val) => {
                let mut clicked = None;
                ui.label(self.styles.plain("("));
                let type_str = self.type_to_string(ty);
                let type_text = self.styles.type_name(&type_str);
                let response = ui.add(egui::Label::new(type_text).sense(egui::Sense::click()));
                if response.clicked() {
                    clicked = Some(path.clone());
                }
                ui.label(self.styles.plain(")"));
                let val_path = path
                    .child_expression(ExpressionPathComponent::CastValue)
                    .unwrap();
                if let Some(p) = self.render_expression(ui, val, &val_path) {
                    clicked = Some(p);
                }
                clicked
            }
            AstExpression::Deref(expr) => {
                let mut clicked = None;
                let op_text = self.styles.operator("*");
                let response = ui.add(egui::Label::new(op_text).sense(egui::Sense::click()));
                if response.clicked() {
                    clicked = Some(path.clone());
                }
                let deref_path = path
                    .child_expression(ExpressionPathComponent::Deref)
                    .unwrap();
                if let Some(p) = self.render_expression(ui, expr, &deref_path) {
                    clicked = Some(p);
                }
                clicked
            }
            AstExpression::AddressOf(expr) => {
                let mut clicked = None;
                let op_text = self.styles.operator("&");
                let response = ui.add(egui::Label::new(op_text).sense(egui::Sense::click()));
                if response.clicked() {
                    clicked = Some(path.clone());
                }
                let addr_path = path
                    .child_expression(ExpressionPathComponent::AddressOf)
                    .unwrap();
                if let Some(p) = self.render_expression(ui, expr, &addr_path) {
                    clicked = Some(p);
                }
                clicked
            }
            AstExpression::Call(call) => self.render_call_expression(ui, call, path),
            AstExpression::ArrayAccess(base, idx) => {
                let mut clicked = None;
                let base_path = path
                    .child_expression(ExpressionPathComponent::Base)
                    .unwrap();
                if let Some(p) = self.render_expression(ui, base, &base_path) {
                    clicked = Some(p);
                }
                ui.label(self.styles.plain("["));
                let idx_path = path
                    .child_expression(ExpressionPathComponent::Index)
                    .unwrap();
                if let Some(p) = self.render_expression(ui, idx, &idx_path) {
                    clicked = Some(p);
                }
                ui.label(self.styles.plain("]"));
                clicked
            }
            AstExpression::MemberAccess(expr, member) => {
                let mut clicked = None;
                let expr_path = path
                    .child_expression(ExpressionPathComponent::Base)
                    .unwrap();
                if let Some(p) = self.render_expression(ui, expr, &expr_path) {
                    clicked = Some(p);
                }
                ui.label(self.styles.plain("."));
                let member_text = self.styles.variable(member);
                let response = ui.add(egui::Label::new(member_text).sense(egui::Sense::click()));
                if response.clicked() {
                    clicked = Some(
                        path.child_expression(ExpressionPathComponent::Member)
                            .unwrap(),
                    );
                }
                clicked
            }
            AstExpression::Ternary(cond, true_expr, false_expr) => {
                let mut clicked = None;
                ui.label(self.styles.plain("("));
                let cond_path = path
                    .child_expression(ExpressionPathComponent::Condition)
                    .unwrap();
                if let Some(p) = self.render_expression(ui, cond, &cond_path) {
                    clicked = Some(p);
                }
                ui.add_space(4.0);
                ui.label(self.styles.operator("?"));
                ui.add_space(4.0);
                let then_path = path
                    .child_expression(ExpressionPathComponent::ThenBranch)
                    .unwrap();
                if let Some(p) = self.render_expression(ui, true_expr, &then_path) {
                    clicked = Some(p);
                }
                ui.add_space(4.0);
                ui.label(self.styles.operator(":"));
                ui.add_space(4.0);
                let else_path = path
                    .child_expression(ExpressionPathComponent::ElseBranch)
                    .unwrap();
                if let Some(p) = self.render_expression(ui, false_expr, &else_path) {
                    clicked = Some(p);
                }
                ui.label(self.styles.plain(")"));
                clicked
            }
            AstExpression::ArchitectureBitSize => {
                let text = self.styles.literal("ARCH_BITS");
                ui.add(egui::Label::new(text));
                None
            }
            AstExpression::ArchitectureByteSize => {
                let text = self.styles.literal("ARCH_BYTES");
                ui.add(egui::Label::new(text));
                None
            }
            _ => {
                ui.label("[expr]");
                None
            }
        }
    }

    fn render_call_expression(
        &self,
        ui: &mut Ui,
        call: &AstCall,
        path: &AstNodePath,
    ) -> Option<AstNodePath> {
        let mut clicked = None;

        let (name, args): (String, &Vec<Wrapped<AstExpression>>) = match call {
            AstCall::Variable {
                var_map,
                var_id,
                args,
                scope: _,
            } => {
                let vars = var_map.read().unwrap();
                let name = vars
                    .get(var_id)
                    .map(|v| v.name().to_string())
                    .unwrap_or_else(|| var_id.get_default_name());
                (name, args)
            }
            AstCall::Function { target, args } => (target.get_default_name().to_string(), args),
            AstCall::Unknown(name, args) => (name.clone(), args),
            AstCall::Builtin(func, _arg) => {
                let name = format!("{:?}", func);
                // Builtins have different arg handling - just show name
                let func_text = self.styles.function(&name);
                ui.add(egui::Label::new(func_text));
                return None;
            }
        };

        // Function name (clickable)
        let name_text = self.styles.function(&name);
        let response = ui.add(egui::Label::new(name_text).sense(egui::Sense::click()));
        if response.clicked() {
            clicked = Some(path.clone());
        }

        ui.label(self.styles.plain("("));

        // Arguments
        for (idx, arg) in args.iter().enumerate() {
            if idx > 0 {
                ui.label(self.styles.plain(","));
                ui.add_space(4.0);
            }
            let arg_path = path
                .child_expression(ExpressionPathComponent::Argument(idx))
                .unwrap();
            if let Some(p) = self.render_expression(ui, arg, &arg_path) {
                clicked = Some(p);
            }
        }

        ui.label(self.styles.plain(")"));

        clicked
    }

    // Helper functions

    fn resolve_parameter(
        &self,
        param: &AstParameter,
        vars: &ArcAstVariableMap,
    ) -> (String, String) {
        // Try to get name and type from the parameter
        let name = param
            .name(vars)
            .unwrap_or_else(|_| format!("param_{:?}", param.location));
        let var_type = param.read_type(vars).unwrap_or(AstValueType::Unknown);
        (name, self.type_to_string(&var_type))
    }

    fn get_variable_name(&self, var_map: &ArcAstVariableMap, var_id: &AstVariableId) -> String {
        let vars = var_map.read().unwrap();
        vars.get(var_id)
            .map(|v| v.name().to_string())
            .unwrap_or_else(|| var_id.get_default_name())
    }

    fn type_to_string(&self, ty: &AstValueType) -> String {
        match ty {
            AstValueType::Void => "void".to_string(),
            AstValueType::Unknown => "unknown_t".to_string(),
            AstValueType::Int => "int".to_string(),
            AstValueType::Int8 => "int8_t".to_string(),
            AstValueType::Int16 => "int16_t".to_string(),
            AstValueType::Int32 => "int32_t".to_string(),
            AstValueType::Int64 => "int64_t".to_string(),
            AstValueType::UInt => "uint".to_string(),
            AstValueType::UInt8 => "uint8_t".to_string(),
            AstValueType::UInt16 => "uint16_t".to_string(),
            AstValueType::UInt32 => "uint32_t".to_string(),
            AstValueType::UInt64 => "uint64_t".to_string(),
            AstValueType::Char => "char".to_string(),
            AstValueType::Float => "float".to_string(),
            AstValueType::Double => "double".to_string(),
            AstValueType::Bool => "bool".to_string(),
            AstValueType::Pointer(t) => format!("{}*", self.type_to_string(t)),
            AstValueType::Array(t, size) => format!("{}[{}]", self.type_to_string(t), size),
            AstValueType::Struct(name, _) => format!("struct {}", name),
            AstValueType::Union(name, _) => format!("union {}", name),
        }
    }

    fn literal_to_string(&self, lit: &AstLiteral) -> String {
        match lit {
            AstLiteral::Int(v) => v.to_string(),
            AstLiteral::UInt(v) => v.to_string(),
            AstLiteral::Char(c) => format!("'{}'", c),
            AstLiteral::String(s) => format!("\"{}\"", s),
            AstLiteral::Bool(b) => b.to_string(),
            AstLiteral::Float(v) => v.to_string(),
        }
    }

    fn unary_op_to_string(&self, op: &AstUnaryOperator) -> String {
        match op {
            AstUnaryOperator::Not => "!".to_string(),
            AstUnaryOperator::Negate => "-".to_string(),
            AstUnaryOperator::BitNot => "~".to_string(),
            AstUnaryOperator::PreInc => "++".to_string(),
            AstUnaryOperator::PreDec => "--".to_string(),
            AstUnaryOperator::PostInc => "++".to_string(),
            AstUnaryOperator::PostDec => "--".to_string(),
            AstUnaryOperator::CastSigned => "(signed)".to_string(),
            AstUnaryOperator::CastUnsigned => "(unsigned)".to_string(),
        }
    }

    fn binary_op_to_string(&self, op: &AstBinaryOperator) -> String {
        match op {
            AstBinaryOperator::Add => "+".to_string(),
            AstBinaryOperator::Sub => "-".to_string(),
            AstBinaryOperator::Mul => "*".to_string(),
            AstBinaryOperator::Div => "/".to_string(),
            AstBinaryOperator::Mod => "%".to_string(),
            AstBinaryOperator::LogicAnd => "&&".to_string(),
            AstBinaryOperator::LogicOr => "||".to_string(),
            AstBinaryOperator::BitAnd => "&".to_string(),
            AstBinaryOperator::BitOr => "|".to_string(),
            AstBinaryOperator::BitXor => "^".to_string(),
            AstBinaryOperator::LeftShift => "<<".to_string(),
            AstBinaryOperator::RightShift => ">>".to_string(),
            AstBinaryOperator::Equal => "==".to_string(),
            AstBinaryOperator::NotEqual => "!=".to_string(),
            AstBinaryOperator::Less => "<".to_string(),
            AstBinaryOperator::LessEqual => "<=".to_string(),
            AstBinaryOperator::Greater => ">".to_string(),
            AstBinaryOperator::GreaterEqual => ">=".to_string(),
        }
    }
}
