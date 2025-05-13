pub mod c_abstract_syntax_tree;

use crate::{
    ir::{
        analyze::{
            ir_to_c::c_abstract_syntax_tree::{
                CAst, CType, Expression, Function, Literal, Statement, Variable,
            },
            DataType, MergedIr,
        },
        data::{IrData, IrDataOperation},
        operator::{BinaryOperator as IrBinaryOp, UnaryOperator as IrUnaryOp},
        statements::IrStatement,
    },
    utils::Aos,
};
use hashbrown::HashMap;

pub fn generate_c(data: &MergedIr) -> CAst {
    let mut ast = CAst::new();

    let mut locals = HashMap::new();
    for (i, var) in data.variables.iter().enumerate() {
        let c_type = match var.data_type {
            DataType::Unknown => CType::Int32,
            DataType::Int => CType::Int32,
            DataType::Float => CType::Double,
            DataType::StringPointer => CType::Pointer(Box::new(CType::Char)),
            DataType::Char => CType::Char,
            DataType::Address => CType::Pointer(Box::new(CType::Void)),
        };
        locals.insert(
            i as u32,
            Variable {
                name: None,
                id: i as u32,
                var_type: c_type,
                is_const: false,
            },
        );
    }
    let mut func = Function {
        name: None,
        id: data
            .ir
            .first()
            .map(|ir| ir.address.get_virtual_address())
            .unwrap_or(0),
        return_type: CType::Void,
        parameters: Vec::new(),
        variables: locals,
        body: Vec::new(),
    };

    for ir in &data.ir {
        if let Some(stmts) = ir.statements {
            for stmt in stmts.iter() {
                func.body.push(convert_stmt(stmt));
            }
        }
    }
    ast.functions.push(func);

    ast
}
fn convert_expr(data: &Aos<IrData>) -> Expression {
    match data.as_ref() {
        IrData::Constant(c) => Expression::Literal(Literal::Int(*c as i64)),
        IrData::Register(_) => todo!(),
        IrData::Dereference(inner) => Expression::Deref(Box::new(convert_expr(inner))),
        IrData::Operation(op) => match op {
            IrDataOperation::Unary { operator, arg } => {
                Expression::UnaryOp(to_c_unary_operator(*operator), Box::new(convert_expr(arg)))
            }
            IrDataOperation::Binary {
                operator,
                arg1,
                arg2,
            } => Expression::BinaryOp(
                to_c_binary_operator(operator),
                Box::new(convert_expr(arg1)),
                Box::new(convert_expr(arg2)),
            ),
        },
        IrData::Intrinsic(ir_intrinsic) => todo!(),
        IrData::Operand(non_zero) => todo!(),
    }
}

fn convert_stmt(stmt: &IrStatement) -> Statement {
    match stmt {
        IrStatement::Assignment { from, to, .. } => {
            let from_expr = convert_expr(from);
            let to_expr = convert_expr(to);
            Statement::Assignment(to_expr, from_expr)
        }
        IrStatement::JumpByCall { target } => Statement::Call(target.to_string(), Vec::new()),
        IrStatement::Jump { target } => Statement::Goto(target.to_string()),
        IrStatement::Condition {
            condition,
            true_branch,
            false_branch,
        } => {
            let cond = convert_expr(condition);
            let then_stmts = true_branch.iter().map(convert_stmt).collect();
            let else_stmts = false_branch.iter().map(convert_stmt).collect();
            Statement::If(cond, then_stmts, Some(else_stmts))
        }
        IrStatement::Undefined => todo!(),
        IrStatement::Exception(_) => todo!(),
        IrStatement::Halt => Statement::Return(None),
        IrStatement::Special(_) => todo!(),
    }
}
fn to_c_unary_operator(op: IrUnaryOp) -> c_abstract_syntax_tree::UnaryOperator {
    match op {
        IrUnaryOp::Not => c_abstract_syntax_tree::UnaryOperator::Not,
        IrUnaryOp::Negation => c_abstract_syntax_tree::UnaryOperator::Negate,
        IrUnaryOp::SignExtend => todo!(),
        IrUnaryOp::ZeroExtend => todo!(),
    }
}

fn to_c_binary_operator(op: &IrBinaryOp) -> c_abstract_syntax_tree::BinaryOperator {
    match op {
        IrBinaryOp::Add => c_abstract_syntax_tree::BinaryOperator::Add,
        IrBinaryOp::Sub => c_abstract_syntax_tree::BinaryOperator::Sub,
        IrBinaryOp::Mul => c_abstract_syntax_tree::BinaryOperator::Mul,
        IrBinaryOp::SignedDiv | IrBinaryOp::UnsignedDiv => {
            c_abstract_syntax_tree::BinaryOperator::Div
        }
        IrBinaryOp::SignedRem | IrBinaryOp::UnsignedRem => {
            c_abstract_syntax_tree::BinaryOperator::Mod
        }
        IrBinaryOp::And => c_abstract_syntax_tree::BinaryOperator::BitAnd,
        IrBinaryOp::Or => c_abstract_syntax_tree::BinaryOperator::BitOr,
        IrBinaryOp::Xor => c_abstract_syntax_tree::BinaryOperator::BitXor,
        IrBinaryOp::Shl => c_abstract_syntax_tree::BinaryOperator::LeftShift,
        IrBinaryOp::Shr | IrBinaryOp::Sar => c_abstract_syntax_tree::BinaryOperator::RightShift,
        IrBinaryOp::Equal(_) => c_abstract_syntax_tree::BinaryOperator::Equal,
        IrBinaryOp::SignedLess(_) | IrBinaryOp::UnsignedLess(_) => {
            c_abstract_syntax_tree::BinaryOperator::Less
        }
        IrBinaryOp::SignedLessOrEqual(_) | IrBinaryOp::UnsignedLessOrEqual(_) => {
            c_abstract_syntax_tree::BinaryOperator::LessEqual
        }
    }
}
