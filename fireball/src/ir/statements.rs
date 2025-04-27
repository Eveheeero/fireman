//! IR의 각 명령이 담겨져 있는 모듈

use crate::core::Instruction;
use crate::ir::data::{AccessType, IrData};
use std::num::NonZeroU16;

/// IR의 각 명령에 대한 Enum
///
/// ### Note
/// snowman's expressions.h, StatementBase based classes, or snowman's ir::statement.h classes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrStatement {
    /// 해석할 수 없는 명령, 인라인 어셈블리로 처리됩니다.
    Unknown(IrStatementUnknown),
    /// 변수 할당
    Assignment {
        from: IrData,
        to: IrData,
        size: Option<NonZeroU16>,
    },
    /// 명령 라인 변경
    Jump(IrStatementJump),
    /// 함수 호출
    Call {
        target: IrData,
    },
    /// 함수 호출 후 반환
    Halt,
    /// 값 접근
    Touch {
        data: IrData,
        access_type: AccessType,
        size: Option<NonZeroU16>,
    },
    /// 조건문
    Condition {
        condition: IrData,
        true_branch: Box<[IrStatement]>,
        false_branch: Box<[IrStatement]>,
    },
    Special(IrStatementSpecial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrStatementUnknown {
    Instruction(Instruction),
    Bytecode(Box<[u8]>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrStatementJump {
    Conditional { ok: IrData, fail: IrData },
    Unconditional(IrData),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrStatementSpecial {
    TypeSpecified {
        location: IrData,
        size: Option<NonZeroU16>,
        data_type: crate::ir::analyze::DataType,
    },
    ArchitectureByteSizeCondition {
        condition: NumCondition,
        true_branch: Box<[IrStatement]>,
        false_branch: Box<[IrStatement]>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum NumCondition {
    Higher(u16),
    HigherOrEqual(u16),
    Lower(u16),
    LowerOrEqual(u16),
    Equal(u16),
    NotEqual(u16),
    Between(u16, u16),

    NotBetween(u16, u16),
}
