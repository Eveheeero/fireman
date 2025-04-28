//! IR 분석 관련 모듈

pub mod analyze;
pub mod arm;
pub mod data;
pub mod operator;
mod register;
pub mod statements;
pub mod x86_64;

use crate::{core::Address, ir::data::DataAccess, prelude::BitBox};
pub use register::Register;
use statements::IrStatement;
use std::{cell::UnsafeCell, collections::HashSet};

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
    known_datatypes: Option<HashSet<analyze::KnownDataType>>,
}

impl IrBlock {
    pub fn new(data: Vec<Ir>) -> Self {
        Self {
            ir: data.into_boxed_slice(),
            known_datatypes: None,
        }
    }
    pub fn ir(&self) -> &[Ir] {
        &self.ir
    }
    pub fn analyze_datatypes(&mut self) {
        let mut known_datatypes: HashSet<analyze::KnownDataType> = HashSet::new();
        for ir in self.ir.iter() {
            let analyzed_datatype = analyze::analyze_datatype(ir);
            for datatype in analyzed_datatype {
                known_datatypes.insert(datatype);
            }
        }
        self.set_datatypes(known_datatypes);
    }
    pub fn get_datatypes(&self) -> Option<&HashSet<analyze::KnownDataType>> {
        self.known_datatypes.as_ref()
    }
    pub fn set_datatypes(&mut self, mut data: HashSet<analyze::KnownDataType>) {
        data.shrink_to_fit();
        self.known_datatypes = Some(data);
    }
}

/// IR statements per address
///
/// 특정 주소에 대한 IR 명령의 모음
#[derive(Debug, Clone)]
pub struct Ir {
    /// IR 변화가 일어난 주소
    pub address: Address,
    /// 실행된 명령
    pub statements: Box<[IrStatement]>,
    /// 해당 IR이 어떤 영역에 영향을 받는지
    pub affected: Vec<DataAccess>,
}
