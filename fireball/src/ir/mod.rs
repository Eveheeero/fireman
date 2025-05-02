//! IR 분석 관련 모듈

pub mod analyze;
pub mod arm;
pub mod data;
pub mod operator;
mod register;
pub mod statements;
pub mod x86_64;

use crate::{
    core::{Address, Instruction},
    ir::data::{DataAccess, IrData, IrDataContainable as _},
    prelude::BitBox,
    utils::{error::ir_analyze_assertion_error::IrAnalyzeAssertionFailure, Aos},
};
use either::Either;
pub use register::Register;
use statements::IrStatement;
use std::{cell::UnsafeCell, collections::HashSet, sync::LazyLock};

/// 컴퓨터가 동작하는 행동을 재현하기 위한 구조체
///
/// ### Todo
///
/// - 레지스터 데이터 외에도, 메모리 변환, 파일 등을 다뤄야 합니다.
pub struct VirtualMachine {
    /// 0~64비트의 값은 rax, 64~128비트의 값은 rbx 를 가지고 있는 등으로, CPU의 데이터를 가지고 있습니다.
    register: UnsafeCell<BitBox>,
}

/// IR 데이터의 기본적인 행동 인터페이스 (파일 변환 등..., 하지만 구현되지 않았습니다.)
impl VirtualMachine {
    /// 가공되지 앟은 레지스터 데이터를 가져옵니다.
    pub fn get_raw(&self) -> &BitBox {
        unsafe { &*self.register.get() }
    }
    /// 가공되지 앟은 레지스터 데이터를 가져옵니다.
    pub fn get_raw_mut(&mut self) -> &mut BitBox {
        unsafe { &mut *self.register.get() }
    }
}

/// IR statements per block
///
/// 한 블럭 안에서 IR명령이 어떻게 동작하는지를 저장하는 구조체
#[derive(Debug, Clone)]
pub struct IrBlock {
    ir: Box<[Ir]>,
    /// Data Access Per Instruction
    pub data_access_per_ir: Option<Box<[Vec<DataAccess>]>>,
    /// Analyzed Datatypes
    pub known_datatypes: Option<HashSet<analyze::KnownDataType>>,
}

impl IrBlock {
    pub fn new(data: Vec<Ir>) -> Self {
        Self {
            ir: data.into_boxed_slice(),
            data_access_per_ir: None,
            known_datatypes: None,
        }
    }
    pub fn ir(&self) -> &[Ir] {
        &self.ir
    }

    pub fn analyze_data_access(&mut self) {
        let mut data_access_per_ir = Vec::new();
        for ir in self.ir.iter().filter(|ir| ir.statements.is_left()) {
            let mut data_access_per_instruction = analyze::analyze_data_access(ir);
            data_access_per_instruction.shrink_to_fit();
            data_access_per_ir.push(data_access_per_instruction);
        }
        self.data_access_per_ir = Some(data_access_per_ir.into_boxed_slice());
    }

    pub fn analyze_datatypes(&mut self) {
        let mut known_datatypes: HashSet<analyze::KnownDataType> = HashSet::new();
        for ir in self.ir.iter().filter(|ir| ir.statements.is_left()) {
            let analyzed_datatype = analyze::analyze_datatype(ir);
            for datatype in analyzed_datatype {
                known_datatypes.insert(datatype);
            }
        }
        known_datatypes.shrink_to_fit();
        self.known_datatypes = Some(known_datatypes);
    }

    pub fn shrink_to_fit(&mut self) {
        self.data_access_per_ir
            .iter_mut()
            .flat_map(|x| x.iter_mut())
            .for_each(Vec::shrink_to_fit);
        self.known_datatypes
            .iter_mut()
            .for_each(HashSet::shrink_to_fit);
    }

    /// Analyzed Data Must Not Contain Sp Based Data
    pub fn validate(&self) -> Result<(), IrAnalyzeAssertionFailure> {
        self.validate_data_access()?;
        self.validate_datatypes()?;
        Ok(())
    }
    pub fn validate_data_access(&self) -> Result<(), IrAnalyzeAssertionFailure> {
        if self.data_access_per_ir.is_none() {
            return Err(IrAnalyzeAssertionFailure::AnalyzeNotPerformed(
                "Data Access",
            ));
        }
        let data_access_per_ir = self.data_access_per_ir.as_ref().unwrap();

        /* Validate Data Doesn't Contain Sp Based Data */
        for (ir_index, data_access) in data_access_per_ir.iter().enumerate() {
            for (sub_index, item) in data_access.iter().enumerate() {
                let item = item.location();
                let mut related_data = Vec::new();
                item.get_related_ir_data(&mut related_data);
                related_data.push(item.clone());
                if Self::is_sp_related(related_data) {
                    return Err(IrAnalyzeAssertionFailure::SpBasedLocationFound {
                        ir_index: Some(ir_index),
                        sub_index: Some(sub_index),
                    });
                }
            }
        }

        Ok(())
    }
    pub fn validate_datatypes(&self) -> Result<(), IrAnalyzeAssertionFailure> {
        if self.known_datatypes.is_none() {
            return Err(IrAnalyzeAssertionFailure::AnalyzeNotPerformed("Datatype"));
        }
        let known_datatypes = self.known_datatypes.as_ref().unwrap();

        /* Validate Data Doesn't Contain Sp Based Data */
        static VM_INTEL_SP_BIT_START: LazyLock<usize> = LazyLock::new(|| {
            <VirtualMachine as crate::ir::x86_64::X64Range>::sp()
                .bit_range()
                .start
        });
        for datatype in known_datatypes {
            let mut related_data = Vec::new();
            datatype.get_related_ir_data(&mut related_data);

            if Self::is_sp_related(related_data) {
                return Err(IrAnalyzeAssertionFailure::SpBasedLocationFound {
                    ir_index: None,
                    sub_index: None,
                });
            }
        }

        Ok(())
    }

    /// Arg must contain self
    fn is_sp_related(related_data: Vec<Aos<IrData>>) -> bool {
        static VM_INTEL_SP_BIT_START: LazyLock<usize> = LazyLock::new(|| {
            <VirtualMachine as crate::ir::x86_64::X64Range>::sp()
                .bit_range()
                .start
        });

        for item in related_data {
            if let data::IrData::Register(register) = item.as_ref() {
                if register.bit_range().start == *VM_INTEL_SP_BIT_START {
                    return true;
                }
            }
        }
        false
    }
}

/// IR statements per address
///
/// 특정 주소에 대한 IR 명령의 모음
#[derive(Debug, Clone)]
pub struct Ir {
    /// IR 변화가 일어난 주소
    pub address: Address,
    /// 실행된 명령. 파싱 실패시 Instruction
    pub statements: Either<&'static [IrStatement], Instruction>,
}
