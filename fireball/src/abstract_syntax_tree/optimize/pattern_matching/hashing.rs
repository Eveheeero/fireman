use crate::{
    abstract_syntax_tree::{
        AstBuiltinFunctionArgument, AstCall, AstExpression, AstJumpTarget, AstLiteral,
        AstStatement, AstValue, AstValueType, AstVariable, Wrapped, WrappedAstStatement,
    },
    ir::statements::{IrStatement, IrStatementSpecial},
};
use blake3::Hasher as Blake3Hasher;
use std::hash::{Hash, Hasher};

pub(in super::super) struct Blake3StdHasher {
    inner: Blake3Hasher,
}

impl Blake3StdHasher {
    pub(in super::super) fn new() -> Self {
        Self {
            inner: Blake3Hasher::new(),
        }
    }

    pub(in super::super) fn finish64(self) -> u64 {
        let digest = self.inner.finalize();
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&digest.as_bytes()[..8]);
        u64::from_le_bytes(bytes)
    }

    pub(in super::super) fn finish_bytes(self) -> [u8; 32] {
        *self.inner.finalize().as_bytes()
    }
}

impl Default for Blake3StdHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for Blake3StdHasher {
    fn finish(&self) -> u64 {
        let digest = self.inner.clone().finalize();
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&digest.as_bytes()[..8]);
        u64::from_le_bytes(bytes)
    }

    fn write(&mut self, bytes: &[u8]) {
        self.inner.update(bytes);
    }
}

pub(in super::super) fn hash_statement_list(
    state: &mut Blake3StdHasher,
    stmts: &[WrappedAstStatement],
) {
    stmts.len().hash(state);
    for stmt in stmts {
        hash_statement(state, &stmt.statement);
    }
}

pub(in super::super) fn structural_statement_hash(stmts: &[WrappedAstStatement]) -> u64 {
    let mut hasher = Blake3StdHasher::new();
    hash_statement_list(&mut hasher, stmts);
    hasher.finish64()
}

fn hash_statement(state: &mut Blake3StdHasher, stmt: &AstStatement) {
    std::mem::discriminant(stmt).hash(state);
    match stmt {
        AstStatement::Declaration(variable, value) => {
            hash_variable(state, variable);
            match value {
                Some(value) => {
                    true.hash(state);
                    hash_wrapped_expression(state, value);
                }
                None => false.hash(state),
            }
        }
        AstStatement::Assignment(from, to) => {
            hash_wrapped_expression(state, from);
            hash_wrapped_expression(state, to);
        }
        AstStatement::If(condition, branch_true, branch_false) => {
            hash_wrapped_expression(state, condition);
            hash_statement_list(state, branch_true);
            match branch_false {
                Some(branch_false) => {
                    true.hash(state);
                    hash_statement_list(state, branch_false);
                }
                None => false.hash(state),
            }
        }
        AstStatement::While(condition, body) => {
            hash_wrapped_expression(state, condition);
            hash_statement_list(state, body);
        }
        AstStatement::For(init, condition, update, body) => {
            hash_statement(state, &init.statement);
            hash_wrapped_expression(state, condition);
            hash_statement(state, &update.statement);
            hash_statement_list(state, body);
        }
        AstStatement::Return(value) => match value {
            Some(value) => {
                true.hash(state);
                hash_wrapped_expression(state, value);
            }
            None => false.hash(state),
        },
        AstStatement::Call(call) => hash_call(state, call),
        AstStatement::Label(label) => label.hash(state),
        AstStatement::Goto(target) => hash_jump_target(state, target),
        AstStatement::Block(body) => hash_statement_list(state, body),
        AstStatement::Assembly(assembly) => assembly.hash(state),
        AstStatement::Exception(message) => message.hash(state),
        AstStatement::Comment(comment) => comment.hash(state),
        AstStatement::Ir(ir) => hash_ir_statement(state, ir.as_ref()),
        AstStatement::Switch(discrim, cases, default) => {
            hash_wrapped_expression(state, discrim);
            cases.len().hash(state);
            for (lit, case_body) in cases {
                hash_literal(state, lit);
                hash_statement_list(state, case_body);
            }
            match default {
                Some(default_body) => {
                    true.hash(state);
                    hash_statement_list(state, default_body);
                }
                None => false.hash(state),
            }
        }
        AstStatement::Undefined | AstStatement::Empty => {}
    }
}

fn hash_wrapped_expression(state: &mut Blake3StdHasher, expr: &Wrapped<AstExpression>) {
    hash_expression(state, expr.as_ref());
}

fn hash_expression(state: &mut Blake3StdHasher, expr: &AstExpression) {
    std::mem::discriminant(expr).hash(state);
    match expr {
        AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize => {}
        AstExpression::Literal(literal) => hash_literal(state, literal),
        AstExpression::Variable(_, variable_id) => variable_id.hash(state),
        AstExpression::UnaryOp(operator, value) => {
            std::mem::discriminant(operator).hash(state);
            hash_wrapped_expression(state, value);
        }
        AstExpression::BinaryOp(operator, left, right) => {
            std::mem::discriminant(operator).hash(state);
            hash_wrapped_expression(state, left);
            hash_wrapped_expression(state, right);
        }
        AstExpression::Call(call) => hash_call(state, call),
        AstExpression::Cast(value_type, value) => {
            hash_value_type(state, value_type);
            hash_wrapped_expression(state, value);
        }
        AstExpression::Deref(value) | AstExpression::AddressOf(value) => {
            hash_wrapped_expression(state, value);
        }
        AstExpression::ArrayAccess(value, index) => {
            hash_wrapped_expression(state, value);
            hash_wrapped_expression(state, index);
        }
        AstExpression::MemberAccess(value, member) => {
            hash_wrapped_expression(state, value);
            member.hash(state);
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            hash_wrapped_expression(state, cond);
            hash_wrapped_expression(state, true_expr);
            hash_wrapped_expression(state, false_expr);
        }
    }
}

fn hash_call(state: &mut Blake3StdHasher, call: &AstCall) {
    std::mem::discriminant(call).hash(state);
    match call {
        AstCall::Variable {
            scope,
            var_id,
            args,
            ..
        } => {
            scope.hash(state);
            var_id.hash(state);
            args.len().hash(state);
            for arg in args {
                hash_wrapped_expression(state, arg);
            }
        }
        AstCall::Function { target, args } => {
            target.hash(state);
            args.len().hash(state);
            for arg in args {
                hash_wrapped_expression(state, arg);
            }
        }
        AstCall::Builtin(function, argument) => {
            std::mem::discriminant(function).hash(state);
            hash_builtin_function_argument(state, argument.as_ref());
        }
        AstCall::Unknown(name, args) => {
            name.hash(state);
            args.len().hash(state);
            for arg in args {
                hash_wrapped_expression(state, arg);
            }
        }
    }
}

fn hash_builtin_function_argument(
    state: &mut Blake3StdHasher,
    argument: &AstBuiltinFunctionArgument,
) {
    std::mem::discriminant(argument).hash(state);
    match argument {
        AstBuiltinFunctionArgument::None => {}
        AstBuiltinFunctionArgument::Print(args) => {
            args.len().hash(state);
            for arg in args {
                hash_wrapped_expression(state, arg);
            }
        }
        AstBuiltinFunctionArgument::ByteSizeOf(expr)
        | AstBuiltinFunctionArgument::BitSizeOf(expr)
        | AstBuiltinFunctionArgument::OperandExists(expr)
        | AstBuiltinFunctionArgument::SignedMax(expr)
        | AstBuiltinFunctionArgument::SignedMin(expr)
        | AstBuiltinFunctionArgument::UnsignedMax(expr)
        | AstBuiltinFunctionArgument::UnsignedMin(expr)
        | AstBuiltinFunctionArgument::BitOnes(expr)
        | AstBuiltinFunctionArgument::BitZeros(expr) => hash_wrapped_expression(state, expr),
        AstBuiltinFunctionArgument::Sized(left, right) => {
            hash_wrapped_expression(state, left);
            hash_wrapped_expression(state, right);
        }
    }
}

fn hash_jump_target(state: &mut Blake3StdHasher, target: &AstJumpTarget) {
    std::mem::discriminant(target).hash(state);
    match target {
        AstJumpTarget::Variable { scope, var_id, .. } => {
            scope.hash(state);
            var_id.hash(state);
        }
        AstJumpTarget::Function { target } => target.hash(state),
        AstJumpTarget::Instruction { target } => target.descriptor().hash(state),
        AstJumpTarget::Unknown(name) => name.hash(state),
    }
}

fn hash_variable(state: &mut Blake3StdHasher, variable: &AstVariable) {
    variable.name.hash(state);
    variable.id.hash(state);
    hash_value_type(state, &variable.var_type);
    match &variable.const_value {
        Some(value) => {
            true.hash(state);
            hash_wrapped_value(state, value);
        }
        None => false.hash(state),
    }
    match &variable.data_access_ir {
        Some(data_access_ir) => {
            true.hash(state);
            let mut descriptors = data_access_ir.keys();
            descriptors.sort_unstable_by_key(|descriptor| descriptor.to_u64());
            descriptors.len().hash(state);
            for descriptor in descriptors {
                descriptor.hash(state);
                if let Some(accesses) = data_access_ir.get(descriptor) {
                    accesses.hash(state);
                }
            }
        }
        None => false.hash(state),
    }
}

fn hash_value_type(state: &mut Blake3StdHasher, value_type: &AstValueType) {
    std::mem::discriminant(value_type).hash(state);
    match value_type {
        AstValueType::Pointer(inner) => hash_value_type(state, inner),
        AstValueType::Array(inner, size) => {
            hash_value_type(state, inner);
            size.hash(state);
        }
        AstValueType::Struct(name, fields) | AstValueType::Union(name, fields) => {
            name.hash(state);
            fields.len().hash(state);
            for field in fields {
                hash_variable(state, field);
            }
        }
        AstValueType::Void
        | AstValueType::Unknown
        | AstValueType::Int
        | AstValueType::Int8
        | AstValueType::Int16
        | AstValueType::Int32
        | AstValueType::Int64
        | AstValueType::UInt
        | AstValueType::UInt8
        | AstValueType::UInt16
        | AstValueType::UInt32
        | AstValueType::UInt64
        | AstValueType::Char
        | AstValueType::Float
        | AstValueType::Double
        | AstValueType::Bool => {}
    }
}

fn hash_wrapped_value(state: &mut Blake3StdHasher, value: &Wrapped<AstValue>) {
    hash_value(state, value.as_ref());
}

fn hash_value(state: &mut Blake3StdHasher, value: &AstValue) {
    std::mem::discriminant(value).hash(state);
    match value {
        AstValue::Num(value) => value.hash(state),
        AstValue::Char(value) => value.hash(state),
        AstValue::Double(value) => value.to_bits().hash(state),
        AstValue::Bool(value) => value.hash(state),
        AstValue::Pointer(value) => hash_wrapped_value(state, value),
        AstValue::Array(values) => {
            values.len().hash(state);
            for value in values {
                hash_wrapped_value(state, value);
            }
        }
        AstValue::Void
        | AstValue::Unknown
        | AstValue::Undefined
        | AstValue::Max
        | AstValue::Min => {}
    }
}

fn hash_literal(state: &mut Blake3StdHasher, literal: &AstLiteral) {
    std::mem::discriminant(literal).hash(state);
    match literal {
        AstLiteral::Int(value) => value.hash(state),
        AstLiteral::UInt(value) => value.hash(state),
        AstLiteral::Float(value) => value.to_bits().hash(state),
        AstLiteral::String(value) => value.hash(state),
        AstLiteral::Char(value) => value.hash(state),
        AstLiteral::Bool(value) => value.hash(state),
    }
}

fn hash_ir_statement(state: &mut Blake3StdHasher, stmt: &IrStatement) {
    std::mem::discriminant(stmt).hash(state);
    match stmt {
        IrStatement::Undefined | IrStatement::Halt => {}
        IrStatement::Exception(message) => message.hash(state),
        IrStatement::Assignment { from, to, size } => {
            from.hash(state);
            to.hash(state);
            size.hash(state);
        }
        IrStatement::Jump { target } | IrStatement::JumpByCall { target } => {
            target.hash(state);
        }
        IrStatement::Condition {
            condition,
            true_branch,
            false_branch,
        } => {
            condition.hash(state);
            true_branch.len().hash(state);
            for stmt in true_branch.iter() {
                hash_ir_statement(state, stmt);
            }
            false_branch.len().hash(state);
            for stmt in false_branch.iter() {
                hash_ir_statement(state, stmt);
            }
        }
        IrStatement::Special(special) => hash_ir_statement_special(state, special),
    }
}

fn hash_ir_statement_special(state: &mut Blake3StdHasher, stmt: &IrStatementSpecial) {
    std::mem::discriminant(stmt).hash(state);
    match stmt {
        IrStatementSpecial::TypeSpecified {
            location,
            size,
            data_type,
        } => {
            location.hash(state);
            size.hash(state);
            data_type.hash(state);
        }
        IrStatementSpecial::CalcFlagsAutomatically {
            operation,
            size,
            flags,
        } => {
            operation.hash(state);
            size.hash(state);
            flags.hash(state);
        }
        IrStatementSpecial::Assertion { condition } => condition.hash(state),
    }
}
