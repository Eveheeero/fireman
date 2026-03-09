use crate::{
    abstract_syntax_tree::{
        AstCall, AstExpression, AstStatement, AstStatementOrigin, WrappedAstStatement,
    },
    ir::{
        data::{IrData, IrDataOperation, IrIntrinsic},
        statements::IrStatement,
    },
};
use rhai::{Array, Dynamic, Engine};

// ── AST Statement wrapper ──

#[derive(Debug, Clone)]
pub(super) struct RhaiAstStmt {
    pub(super) kind: &'static str,
    wrapped: WrappedAstStatement,
}

impl RhaiAstStmt {
    pub fn from_wrapped(wrapped: &WrappedAstStatement) -> Self {
        let kind = match &wrapped.statement {
            AstStatement::Declaration(_, _) => "declaration",
            AstStatement::Assignment(_, _) => "assignment",
            AstStatement::If(_, _, _) => "if",
            AstStatement::While(_, _) => "while",
            AstStatement::For(_, _, _, _) => "for",
            AstStatement::Return(_) => "return",
            AstStatement::Call(_) => "call",
            AstStatement::Label(_) => "label",
            AstStatement::Goto(_) => "goto",
            AstStatement::Block(_) => "block",
            AstStatement::Assembly(_) => "assembly",
            AstStatement::Undefined => "undefined",
            AstStatement::Exception(_) => "exception",
            AstStatement::Comment(_) => "comment",
            AstStatement::Ir(_) => "ir",
            AstStatement::Empty => "empty",
            AstStatement::Switch(_, _, _) => "switch",
            AstStatement::Break => "break",
            AstStatement::Continue => "continue",
            AstStatement::DoWhile(_, _) => "dowhile",
        };
        Self {
            kind,
            wrapped: wrapped.clone(),
        }
    }

    #[inline]
    fn stmt(&self) -> &AstStatement {
        &self.wrapped.statement
    }

    fn kind(&mut self) -> String {
        self.kind.to_string()
    }

    fn to_str(&mut self) -> String {
        format!("{:?}", self.wrapped.statement)
    }

    fn origin(&mut self) -> String {
        match &self.wrapped.origin {
            AstStatementOrigin::UserInput => "user_input".to_string(),
            AstStatementOrigin::PreDefined => "predefined".to_string(),
            AstStatementOrigin::Ir(desc) => format!("ir({desc:?})"),
            AstStatementOrigin::Combination(_) => "combination".to_string(),
            AstStatementOrigin::Unknown => "unknown".to_string(),
        }
    }

    fn wrapper_comment(&mut self) -> String {
        self.wrapped.comment.clone().unwrap_or_default()
    }

    fn is_call(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::Call(_))
    }

    fn is_if(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::If(_, _, _))
    }

    fn is_return(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::Return(_))
    }

    fn is_comment(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::Comment(_))
    }

    fn is_assignment(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::Assignment(_, _))
    }

    fn is_declaration(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::Declaration(_, _))
    }

    fn is_while(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::While(_, _))
    }

    fn is_for(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::For(_, _, _, _))
    }

    fn is_goto(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::Goto(_))
    }

    fn is_label(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::Label(_))
    }

    fn is_block(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::Block(_))
    }

    fn is_switch(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::Switch(_, _, _))
    }

    fn is_break(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::Break)
    }

    fn is_continue(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::Continue)
    }

    fn is_dowhile(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::DoWhile(_, _))
    }

    fn is_empty(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::Empty)
    }

    fn is_ir(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::Ir(_))
    }

    fn is_assembly(&mut self) -> bool {
        matches!(self.wrapped.statement, AstStatement::Assembly(_))
    }

    fn call_name(&mut self) -> String {
        match &self.wrapped.statement {
            AstStatement::Call(call) => match call {
                AstCall::Unknown(name, _) => name.clone(),
                AstCall::Variable { var_id, .. } => format!("{var_id:?}"),
                AstCall::Function { target, .. } => format!("{target:?}"),
                AstCall::Builtin(func, _) => format!("{func:?}"),
            },
            _ => String::new(),
        }
    }

    fn comment_text(&mut self) -> String {
        match &self.wrapped.statement {
            AstStatement::Comment(text) => text.clone(),
            _ => String::new(),
        }
    }

    fn label_text(&mut self) -> String {
        match &self.wrapped.statement {
            AstStatement::Label(text) => text.clone(),
            _ => String::new(),
        }
    }

    fn has_else(&mut self) -> bool {
        matches!(&self.wrapped.statement, AstStatement::If(_, _, Some(_)))
    }

    fn body_count(&mut self) -> i64 {
        match &self.wrapped.statement {
            AstStatement::If(_, body, _) => body.len() as i64,
            AstStatement::While(_, body) => body.len() as i64,
            AstStatement::For(_, _, _, body) => body.len() as i64,
            AstStatement::Block(body) => body.len() as i64,
            AstStatement::DoWhile(_, body) => body.len() as i64,
            _ => 0,
        }
    }

    fn else_count(&mut self) -> i64 {
        match &self.wrapped.statement {
            AstStatement::If(_, _, Some(else_body)) => else_body.len() as i64,
            _ => 0,
        }
    }

    fn contains_call_to(&mut self, name: &str) -> bool {
        let needle = name.to_lowercase();
        stmt_contains_call(&self.wrapped.statement, &needle)
    }

    // ── Comment modulation ──

    fn set_comment(&mut self, text: String) {
        if matches!(self.wrapped.statement, AstStatement::Comment(_)) {
            self.wrapped.statement = AstStatement::Comment(text);
        }
    }

    fn prepend_comment(&mut self, prefix: String) {
        if let AstStatement::Comment(ref mut text) = self.wrapped.statement {
            *text = format!("{prefix}{text}");
        }
    }

    fn append_comment(&mut self, suffix: String) {
        if let AstStatement::Comment(ref mut text) = self.wrapped.statement {
            text.push_str(&suffix);
        }
    }

    fn replace_in_comment(&mut self, old: &str, new: &str) {
        if let AstStatement::Comment(ref mut text) = self.wrapped.statement {
            *text = text.replace(old, new);
        }
    }

    fn to_comment(&mut self) -> RhaiAstStmt {
        let text = match &self.wrapped.statement {
            AstStatement::Comment(t) => t.clone(),
            other => format!("{other:?}"),
        };
        let wrapped = WrappedAstStatement {
            statement: AstStatement::Comment(text),
            origin: self.wrapped.origin.clone(),
            comment: self.wrapped.comment.clone(),
        };
        RhaiAstStmt::from_wrapped(&wrapped)
    }

    fn has_operator(&mut self, op_name: &str) -> bool {
        stmt_has_operator(&self.wrapped.statement, op_name)
    }

    // ── Deep getters for statement fields ──

    fn condition(&mut self) -> String {
        match &self.wrapped.statement {
            AstStatement::If(cond, _, _)
            | AstStatement::While(cond, _)
            | AstStatement::DoWhile(cond, _) => format!("{:?}", cond.item),
            _ => String::new(),
        }
    }

    fn body(&mut self) -> Array {
        let stmts: &[WrappedAstStatement] = match &self.wrapped.statement {
            AstStatement::If(_, body, _)
            | AstStatement::While(_, body)
            | AstStatement::DoWhile(_, body) => body,
            AstStatement::Block(body) => body,
            AstStatement::For(_, _, _, body) => body,
            _ => return Array::new(),
        };
        stmts
            .iter()
            .map(|s| Dynamic::from(RhaiAstStmt::from_wrapped(s)))
            .collect()
    }

    fn else_body(&mut self) -> Array {
        match &self.wrapped.statement {
            AstStatement::If(_, _, Some(else_stmts)) => else_stmts
                .iter()
                .map(|s| Dynamic::from(RhaiAstStmt::from_wrapped(s)))
                .collect(),
            _ => Array::new(),
        }
    }

    fn lhs(&mut self) -> String {
        match &self.wrapped.statement {
            AstStatement::Assignment(lhs, _) => format!("{:?}", lhs.item),
            AstStatement::Declaration(var, _) => format!("{var:?}"),
            _ => String::new(),
        }
    }

    fn rhs(&mut self) -> String {
        match &self.wrapped.statement {
            AstStatement::Assignment(_, rhs) => format!("{:?}", rhs.item),
            AstStatement::Declaration(_, Some(init)) => format!("{:?}", init.item),
            _ => String::new(),
        }
    }

    fn return_expr(&mut self) -> String {
        match &self.wrapped.statement {
            AstStatement::Return(Some(expr)) => format!("{:?}", expr.item),
            _ => String::new(),
        }
    }

    fn ir_stmt(&mut self) -> Dynamic {
        match &self.wrapped.statement {
            AstStatement::Ir(ir) => Dynamic::from(RhaiIrStmt::from_statement(ir)),
            _ => Dynamic::UNIT,
        }
    }

    fn assembly_text(&mut self) -> String {
        match &self.wrapped.statement {
            AstStatement::Assembly(text) => text.clone(),
            _ => String::new(),
        }
    }

    fn goto_target(&mut self) -> String {
        match &self.wrapped.statement {
            AstStatement::Goto(target) => format!("{target:?}"),
            _ => String::new(),
        }
    }

    fn exception_text(&mut self) -> String {
        match &self.wrapped.statement {
            AstStatement::Exception(text) => text.to_string(),
            _ => String::new(),
        }
    }
}

// ── IR Statement wrapper ──

#[derive(Debug, Clone)]
pub(super) struct RhaiIrStmt {
    pub(super) kind: &'static str,
    inner: IrStatement,
}

impl RhaiIrStmt {
    pub fn from_statement(stmt: &IrStatement) -> Self {
        let kind = match stmt {
            IrStatement::Undefined => "undefined",
            IrStatement::Exception(_) => "exception",
            IrStatement::Assignment { .. } => "assignment",
            IrStatement::Jump { .. } => "jump",
            IrStatement::JumpByCall { .. } => "call",
            IrStatement::Halt => "halt",
            IrStatement::Condition { .. } => "condition",
            IrStatement::Special(_) => "special",
        };
        Self {
            kind,
            inner: stmt.clone(),
        }
    }

    fn kind(&mut self) -> String {
        self.kind.to_string()
    }

    fn to_str(&mut self) -> String {
        format!("{:?}", self.inner)
    }

    fn is_assignment(&mut self) -> bool {
        matches!(self.inner, IrStatement::Assignment { .. })
    }

    fn is_jump(&mut self) -> bool {
        matches!(self.inner, IrStatement::Jump { .. })
    }

    fn is_call(&mut self) -> bool {
        matches!(self.inner, IrStatement::JumpByCall { .. })
    }

    fn is_condition(&mut self) -> bool {
        matches!(self.inner, IrStatement::Condition { .. })
    }

    fn is_halt(&mut self) -> bool {
        matches!(self.inner, IrStatement::Halt)
    }

    fn is_special(&mut self) -> bool {
        matches!(self.inner, IrStatement::Special(_))
    }

    // ── Deep getters for IR statement fields ──

    fn from_data(&mut self) -> Dynamic {
        match &self.inner {
            IrStatement::Assignment { from, .. } => {
                Dynamic::from(RhaiIrData::from_ir_data(from.as_ref()))
            }
            _ => Dynamic::UNIT,
        }
    }

    fn to_data(&mut self) -> Dynamic {
        match &self.inner {
            IrStatement::Assignment { to, .. } => {
                Dynamic::from(RhaiIrData::from_ir_data(to.as_ref()))
            }
            _ => Dynamic::UNIT,
        }
    }

    fn size_str(&mut self) -> String {
        match &self.inner {
            IrStatement::Assignment { size, .. } => format!("{size:?}"),
            _ => String::new(),
        }
    }

    fn target(&mut self) -> Dynamic {
        match &self.inner {
            IrStatement::Jump { target } | IrStatement::JumpByCall { target } => {
                Dynamic::from(RhaiIrData::from_ir_data(target.as_ref()))
            }
            _ => Dynamic::UNIT,
        }
    }

    fn condition_data(&mut self) -> Dynamic {
        match &self.inner {
            IrStatement::Condition { condition, .. } => {
                Dynamic::from(RhaiIrData::from_ir_data(condition.as_ref()))
            }
            _ => Dynamic::UNIT,
        }
    }

    fn true_branch(&mut self) -> Array {
        match &self.inner {
            IrStatement::Condition { true_branch, .. } => true_branch
                .iter()
                .map(|s| Dynamic::from(RhaiIrStmt::from_statement(s)))
                .collect(),
            _ => Array::new(),
        }
    }

    fn false_branch(&mut self) -> Array {
        match &self.inner {
            IrStatement::Condition { false_branch, .. } => false_branch
                .iter()
                .map(|s| Dynamic::from(RhaiIrStmt::from_statement(s)))
                .collect(),
            _ => Array::new(),
        }
    }

    fn exception_text(&mut self) -> String {
        match &self.inner {
            IrStatement::Exception(text) => text.to_string(),
            _ => String::new(),
        }
    }
}

// ── IR Data wrapper ──

#[derive(Debug, Clone)]
pub(super) struct RhaiIrData {
    pub(super) kind: &'static str,
    inner: IrData,
}

impl RhaiIrData {
    pub fn from_ir_data(data: &IrData) -> Self {
        let kind = match data {
            IrData::Constant(_) => "constant",
            IrData::Intrinsic(_) => "intrinsic",
            IrData::Register(_) => "register",
            IrData::Dereference(_) => "dereference",
            IrData::Operation(_) => "operation",
            IrData::Operand(_) => "operand",
        };
        Self {
            kind,
            inner: data.clone(),
        }
    }

    fn kind(&mut self) -> String {
        self.kind.to_string()
    }

    fn to_str(&mut self) -> String {
        format!("{:?}", self.inner)
    }

    fn is_constant(&mut self) -> bool {
        matches!(self.inner, IrData::Constant(_))
    }

    fn is_intrinsic(&mut self) -> bool {
        matches!(self.inner, IrData::Intrinsic(_))
    }

    fn is_register(&mut self) -> bool {
        matches!(self.inner, IrData::Register(_))
    }

    fn is_dereference(&mut self) -> bool {
        matches!(self.inner, IrData::Dereference(_))
    }

    fn is_operation(&mut self) -> bool {
        matches!(self.inner, IrData::Operation(_))
    }

    fn is_operand(&mut self) -> bool {
        matches!(self.inner, IrData::Operand(_))
    }

    fn value(&mut self) -> i64 {
        match &self.inner {
            IrData::Constant(v) => *v as i64,
            IrData::Operand(n) => n.get() as i64,
            _ => -1,
        }
    }

    fn register_name(&mut self) -> String {
        match &self.inner {
            IrData::Register(reg) => format!("{reg:?}"),
            _ => String::new(),
        }
    }

    fn intrinsic_name(&mut self) -> String {
        match &self.inner {
            IrData::Intrinsic(intr) => match intr {
                IrIntrinsic::Unknown => "unknown".to_string(),
                IrIntrinsic::Undefined => "undefined".to_string(),
                IrIntrinsic::ArchitectureByteSize => "arch_byte_size".to_string(),
                IrIntrinsic::ArchitectureBitSize => "arch_bit_size".to_string(),
                IrIntrinsic::ArchitectureBitPerByte => "arch_bit_per_byte".to_string(),
                IrIntrinsic::InstructionByteSize => "instruction_byte_size".to_string(),
                other => format!("{other:?}"),
            },
            _ => String::new(),
        }
    }

    fn inner(&mut self) -> Dynamic {
        match &self.inner {
            IrData::Dereference(inner) => {
                Dynamic::from(RhaiIrData::from_ir_data(inner.as_ref()))
            }
            _ => Dynamic::UNIT,
        }
    }

    fn operator(&mut self) -> String {
        match &self.inner {
            IrData::Operation(IrDataOperation::Unary { operator, .. }) => format!("{operator:?}"),
            IrData::Operation(IrDataOperation::Binary { operator, .. }) => format!("{operator:?}"),
            _ => String::new(),
        }
    }

    fn is_unary(&mut self) -> bool {
        matches!(
            self.inner,
            IrData::Operation(IrDataOperation::Unary { .. })
        )
    }

    fn is_binary(&mut self) -> bool {
        matches!(
            self.inner,
            IrData::Operation(IrDataOperation::Binary { .. })
        )
    }

    fn arg(&mut self) -> Dynamic {
        match &self.inner {
            IrData::Operation(IrDataOperation::Unary { arg, .. }) => {
                Dynamic::from(RhaiIrData::from_ir_data(arg.as_ref()))
            }
            _ => Dynamic::UNIT,
        }
    }

    fn arg1(&mut self) -> Dynamic {
        match &self.inner {
            IrData::Operation(IrDataOperation::Binary { arg1, .. }) => {
                Dynamic::from(RhaiIrData::from_ir_data(arg1.as_ref()))
            }
            _ => Dynamic::UNIT,
        }
    }

    fn arg2(&mut self) -> Dynamic {
        match &self.inner {
            IrData::Operation(IrDataOperation::Binary { arg2, .. }) => {
                Dynamic::from(RhaiIrData::from_ir_data(arg2.as_ref()))
            }
            _ => Dynamic::UNIT,
        }
    }
}

// ── ASM Line wrapper ──

#[derive(Debug, Clone)]
pub(super) struct RhaiAsmLine {
    pub(super) index: i64,
    pub(super) line: String,
    pub(super) mnemonic: String,
    pub(super) operands: String,
}

impl RhaiAsmLine {
    pub fn from_normalized(index: usize, line: &str) -> Self {
        let trimmed = line.trim();
        let (mnemonic, operands) = match trimmed.find(|c: char| c.is_whitespace()) {
            Some(pos) => (trimmed[..pos].to_string(), trimmed[pos..].trim().to_string()),
            None => (trimmed.to_string(), String::new()),
        };
        Self {
            index: index as i64,
            line: trimmed.to_string(),
            mnemonic,
            operands,
        }
    }

    fn index(&mut self) -> i64 {
        self.index
    }

    fn line(&mut self) -> String {
        self.line.clone()
    }

    fn mnemonic(&mut self) -> String {
        self.mnemonic.clone()
    }

    fn operands(&mut self) -> String {
        self.operands.clone()
    }

    fn to_str(&mut self) -> String {
        self.line.clone()
    }

    fn contains(&mut self, needle: &str) -> bool {
        let needle_lower = needle.to_lowercase();
        memchr::memmem::find(self.line.to_lowercase().as_bytes(), needle_lower.as_bytes()).is_some()
    }

    fn is_call(&mut self) -> bool {
        self.mnemonic == "call"
    }

    fn is_jump(&mut self) -> bool {
        self.mnemonic.starts_with('j')
    }

    fn is_ret(&mut self) -> bool {
        self.mnemonic == "ret" || self.mnemonic == "retn"
    }

    fn is_nop(&mut self) -> bool {
        self.mnemonic == "nop"
    }

    fn is_push(&mut self) -> bool {
        self.mnemonic == "push"
    }

    fn is_pop(&mut self) -> bool {
        self.mnemonic == "pop"
    }

    fn is_mov(&mut self) -> bool {
        self.mnemonic == "mov" || self.mnemonic == "movzx" || self.mnemonic == "movsx"
    }
}

// ── Utility functions ──

fn stmt_contains_call(stmt: &AstStatement, needle: &str) -> bool {
    match stmt {
        AstStatement::Call(call) => call_matches(call, needle),
        AstStatement::If(_, body, else_body) => {
            body.iter().any(|s| stmt_contains_call(&s.statement, needle))
                || else_body
                    .as_ref()
                    .is_some_and(|e| e.iter().any(|s| stmt_contains_call(&s.statement, needle)))
        }
        AstStatement::While(_, body) | AstStatement::Block(body) | AstStatement::DoWhile(_, body) => {
            body.iter().any(|s| stmt_contains_call(&s.statement, needle))
        }
        AstStatement::Assignment(_, rhs) => expr_contains_call(&rhs.item, needle),
        AstStatement::Return(Some(expr)) => expr_contains_call(&expr.item, needle),
        _ => false,
    }
}

fn expr_contains_call(expr: &AstExpression, needle: &str) -> bool {
    match expr {
        AstExpression::Call(call) => call_matches(call, needle),
        AstExpression::UnaryOp(_, inner) => expr_contains_call(&inner.item, needle),
        AstExpression::BinaryOp(_, lhs, rhs) => {
            expr_contains_call(&lhs.item, needle) || expr_contains_call(&rhs.item, needle)
        }
        AstExpression::Ternary(c, t, f) => {
            expr_contains_call(&c.item, needle)
                || expr_contains_call(&t.item, needle)
                || expr_contains_call(&f.item, needle)
        }
        AstExpression::Cast(_, inner) | AstExpression::Deref(inner) | AstExpression::AddressOf(inner) => {
            expr_contains_call(&inner.item, needle)
        }
        AstExpression::ArrayAccess(base, idx) => {
            expr_contains_call(&base.item, needle) || expr_contains_call(&idx.item, needle)
        }
        AstExpression::MemberAccess(base, _) => expr_contains_call(&base.item, needle),
        _ => false,
    }
}

fn call_matches(call: &AstCall, needle: &str) -> bool {
    match call {
        AstCall::Unknown(name, _) => name.to_lowercase().contains(needle),
        AstCall::Variable { var_id, .. } => format!("{var_id:?}").to_lowercase().contains(needle),
        AstCall::Function { target, .. } => format!("{target:?}").to_lowercase().contains(needle),
        AstCall::Builtin(func, _) => format!("{func:?}").to_lowercase().contains(needle),
    }
}

fn stmt_has_operator(stmt: &AstStatement, op_name: &str) -> bool {
    match stmt {
        AstStatement::Assignment(lhs, rhs) => {
            expr_has_operator(&lhs.item, op_name) || expr_has_operator(&rhs.item, op_name)
        }
        AstStatement::If(cond, _, _) | AstStatement::While(cond, _) | AstStatement::DoWhile(cond, _) => {
            expr_has_operator(&cond.item, op_name)
        }
        AstStatement::Return(Some(expr)) => expr_has_operator(&expr.item, op_name),
        _ => false,
    }
}

fn expr_has_operator(expr: &AstExpression, op_name: &str) -> bool {
    let op_lower = op_name.to_lowercase();
    match expr {
        AstExpression::UnaryOp(op, inner) => {
            format!("{op:?}").to_lowercase() == op_lower || expr_has_operator(&inner.item, op_name)
        }
        AstExpression::BinaryOp(op, lhs, rhs) => {
            format!("{op:?}").to_lowercase() == op_lower
                || expr_has_operator(&lhs.item, op_name)
                || expr_has_operator(&rhs.item, op_name)
        }
        AstExpression::Ternary(c, t, f) => {
            expr_has_operator(&c.item, op_name)
                || expr_has_operator(&t.item, op_name)
                || expr_has_operator(&f.item, op_name)
        }
        AstExpression::Cast(_, inner) | AstExpression::Deref(inner) | AstExpression::AddressOf(inner) => {
            expr_has_operator(&inner.item, op_name)
        }
        _ => false,
    }
}

// ── Rhai global analysis functions ──

fn rhai_count_calls(stmts: Array) -> i64 {
    stmts
        .iter()
        .filter(|d| {
            Dynamic::clone(d)
                .try_cast::<RhaiAstStmt>()
                .is_some_and(|s| matches!(s.wrapped.statement, AstStatement::Call(_)))
        })
        .count() as i64
}

fn rhai_count_assignments(stmts: Array) -> i64 {
    stmts
        .iter()
        .filter(|d| {
            Dynamic::clone(d)
                .try_cast::<RhaiAstStmt>()
                .is_some_and(|s| matches!(s.wrapped.statement, AstStatement::Assignment(_, _)))
        })
        .count() as i64
}

fn rhai_count_comments(stmts: Array) -> i64 {
    stmts
        .iter()
        .filter(|d| {
            Dynamic::clone(d)
                .try_cast::<RhaiAstStmt>()
                .is_some_and(|s| matches!(s.wrapped.statement, AstStatement::Comment(_)))
        })
        .count() as i64
}

fn rhai_find_calls_to(stmts: Array, name: String) -> Array {
    let needle = name.to_lowercase();
    stmts
        .into_iter()
        .filter(|d| {
            Dynamic::clone(d)
                .try_cast::<RhaiAstStmt>()
                .is_some_and(|s| stmt_contains_call(&s.wrapped.statement, &needle))
        })
        .collect()
}

fn rhai_is_arithmetic_op(op: &str) -> bool {
    matches!(
        op.to_lowercase().as_str(),
        "add" | "sub" | "mul" | "div" | "mod" | "negate"
    )
}

fn rhai_is_comparison_op(op: &str) -> bool {
    matches!(
        op.to_lowercase().as_str(),
        "equal" | "notequal" | "less" | "lessequal" | "greater" | "greaterequal"
    )
}

fn rhai_is_bitwise_op(op: &str) -> bool {
    matches!(
        op.to_lowercase().as_str(),
        "bitand" | "bitor" | "bitxor" | "bitnot" | "leftshift" | "rightshift"
    )
}

fn rhai_is_logical_op(op: &str) -> bool {
    matches!(
        op.to_lowercase().as_str(),
        "logicand" | "logicor" | "not"
    )
}

// ── Engine registration ──

pub(super) fn register_analysis_types(engine: &mut Engine) {
    // AstStmt
    engine
        .register_type_with_name::<RhaiAstStmt>("AstStmt")
        .register_fn("kind", RhaiAstStmt::kind)
        .register_fn("to_string", RhaiAstStmt::to_str)
        .register_fn("is_call", RhaiAstStmt::is_call)
        .register_fn("is_if", RhaiAstStmt::is_if)
        .register_fn("is_return", RhaiAstStmt::is_return)
        .register_fn("is_comment", RhaiAstStmt::is_comment)
        .register_fn("is_assignment", RhaiAstStmt::is_assignment)
        .register_fn("is_declaration", RhaiAstStmt::is_declaration)
        .register_fn("is_while", RhaiAstStmt::is_while)
        .register_fn("is_for", RhaiAstStmt::is_for)
        .register_fn("is_goto", RhaiAstStmt::is_goto)
        .register_fn("is_label", RhaiAstStmt::is_label)
        .register_fn("is_block", RhaiAstStmt::is_block)
        .register_fn("is_switch", RhaiAstStmt::is_switch)
        .register_fn("is_break", RhaiAstStmt::is_break)
        .register_fn("is_continue", RhaiAstStmt::is_continue)
        .register_fn("is_dowhile", RhaiAstStmt::is_dowhile)
        .register_fn("is_empty", RhaiAstStmt::is_empty)
        .register_fn("is_ir", RhaiAstStmt::is_ir)
        .register_fn("is_assembly", RhaiAstStmt::is_assembly)
        .register_fn("call_name", RhaiAstStmt::call_name)
        .register_fn("comment_text", RhaiAstStmt::comment_text)
        .register_fn("label_text", RhaiAstStmt::label_text)
        .register_fn("has_else", RhaiAstStmt::has_else)
        .register_fn("body_count", RhaiAstStmt::body_count)
        .register_fn("else_count", RhaiAstStmt::else_count)
        .register_fn("contains_call_to", RhaiAstStmt::contains_call_to)
        .register_fn("has_operator", RhaiAstStmt::has_operator)
        .register_fn("set_comment", RhaiAstStmt::set_comment)
        .register_fn("prepend_comment", RhaiAstStmt::prepend_comment)
        .register_fn("append_comment", RhaiAstStmt::append_comment)
        .register_fn("replace_in_comment", RhaiAstStmt::replace_in_comment)
        .register_fn("to_comment", RhaiAstStmt::to_comment)
        .register_fn("condition", RhaiAstStmt::condition)
        .register_fn("body", RhaiAstStmt::body)
        .register_fn("else_body", RhaiAstStmt::else_body)
        .register_fn("lhs", RhaiAstStmt::lhs)
        .register_fn("rhs", RhaiAstStmt::rhs)
        .register_fn("return_expr", RhaiAstStmt::return_expr)
        .register_fn("ir_stmt", RhaiAstStmt::ir_stmt)
        .register_fn("assembly_text", RhaiAstStmt::assembly_text)
        .register_fn("goto_target", RhaiAstStmt::goto_target)
        .register_fn("exception_text", RhaiAstStmt::exception_text);

    // IrStmt
    engine
        .register_type_with_name::<RhaiIrStmt>("IrStmt")
        .register_fn("kind", RhaiIrStmt::kind)
        .register_fn("to_string", RhaiIrStmt::to_str)
        .register_fn("is_assignment", RhaiIrStmt::is_assignment)
        .register_fn("is_jump", RhaiIrStmt::is_jump)
        .register_fn("is_call", RhaiIrStmt::is_call)
        .register_fn("is_condition", RhaiIrStmt::is_condition)
        .register_fn("is_halt", RhaiIrStmt::is_halt)
        .register_fn("is_special", RhaiIrStmt::is_special)
        .register_fn("from_data", RhaiIrStmt::from_data)
        .register_fn("to_data", RhaiIrStmt::to_data)
        .register_fn("size_str", RhaiIrStmt::size_str)
        .register_fn("target", RhaiIrStmt::target)
        .register_fn("condition_data", RhaiIrStmt::condition_data)
        .register_fn("true_branch", RhaiIrStmt::true_branch)
        .register_fn("false_branch", RhaiIrStmt::false_branch)
        .register_fn("exception_text", RhaiIrStmt::exception_text);

    // AsmLine
    engine
        .register_type_with_name::<RhaiAsmLine>("AsmLine")
        .register_fn("index", RhaiAsmLine::index)
        .register_fn("line", RhaiAsmLine::line)
        .register_fn("mnemonic", RhaiAsmLine::mnemonic)
        .register_fn("operands", RhaiAsmLine::operands)
        .register_fn("to_string", RhaiAsmLine::to_str)
        .register_fn("contains", RhaiAsmLine::contains)
        .register_fn("is_call", RhaiAsmLine::is_call)
        .register_fn("is_jump", RhaiAsmLine::is_jump)
        .register_fn("is_ret", RhaiAsmLine::is_ret)
        .register_fn("is_nop", RhaiAsmLine::is_nop)
        .register_fn("is_push", RhaiAsmLine::is_push)
        .register_fn("is_pop", RhaiAsmLine::is_pop)
        .register_fn("is_mov", RhaiAsmLine::is_mov);

    // IrData
    engine
        .register_type_with_name::<RhaiIrData>("IrData")
        .register_fn("kind", RhaiIrData::kind)
        .register_fn("to_string", RhaiIrData::to_str)
        .register_fn("is_constant", RhaiIrData::is_constant)
        .register_fn("is_intrinsic", RhaiIrData::is_intrinsic)
        .register_fn("is_register", RhaiIrData::is_register)
        .register_fn("is_dereference", RhaiIrData::is_dereference)
        .register_fn("is_operation", RhaiIrData::is_operation)
        .register_fn("is_operand", RhaiIrData::is_operand)
        .register_fn("value", RhaiIrData::value)
        .register_fn("register_name", RhaiIrData::register_name)
        .register_fn("intrinsic_name", RhaiIrData::intrinsic_name)
        .register_fn("inner", RhaiIrData::inner)
        .register_fn("operator", RhaiIrData::operator)
        .register_fn("is_unary", RhaiIrData::is_unary)
        .register_fn("is_binary", RhaiIrData::is_binary)
        .register_fn("arg", RhaiIrData::arg)
        .register_fn("arg1", RhaiIrData::arg1)
        .register_fn("arg2", RhaiIrData::arg2);

    // Global analysis functions
    engine.register_fn("count_calls", rhai_count_calls);
    engine.register_fn("count_assignments", rhai_count_assignments);
    engine.register_fn("count_comments", rhai_count_comments);
    engine.register_fn("find_calls_to", rhai_find_calls_to);
    engine.register_fn("is_arithmetic_op", rhai_is_arithmetic_op);
    engine.register_fn("is_comparison_op", rhai_is_comparison_op);
    engine.register_fn("is_bitwise_op", rhai_is_bitwise_op);
    engine.register_fn("is_logical_op", rhai_is_logical_op);
}
