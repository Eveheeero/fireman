//! Module for IR analysis

pub mod analyze;
pub mod arm;
pub mod data;
pub mod high_ir;
pub mod low_ir;
pub mod medium_ir;
pub mod operator;
mod register;
pub mod statements;
pub mod utils;
pub mod x86_64;

use crate::{
    core::{Address, Instruction},
    ir::{
        analyze::{IrVariable, KnownDataType},
        data::{DataAccess, IrData},
    },
    prelude::BitBox,
    prelude::*,
    utils::{Aos, error::ir_analyze_assertion_error::IrAnalyzeAssertionFailure},
};
pub use register::Register;
use statements::IrStatement;
use std::{
    cell::UnsafeCell,
    sync::{Arc, LazyLock},
};
use utils::IrStatementDescriptorMap;

/// A structure to simulate the computer's behavior
///
/// ### Todo
/// - Handle memory transformations, file I/O, etc., in addition to register data.
pub struct VirtualMachine {
    /// CPU registers storage (e.g., bits 0-64 for rax, 64-128 for rbx)
    register: UnsafeCell<BitBox>,
}

/// Basic interface for IR data behavior (e.g., file transformations); not yet implemented
impl VirtualMachine {
    /// Returns the raw register data
    pub fn get_raw(&self) -> &BitBox {
        unsafe { &*self.register.get() }
    }
    /// Returns the raw register data (mutable)
    pub fn get_raw_mut(&mut self) -> &mut BitBox {
        unsafe { &mut *self.register.get() }
    }
}

/// Structure that stores how IR instructions operate within a block
#[derive(Debug, Clone)]
pub struct IrBlock {
    ir: Box<[Ir]>,
    instructions: Arc<[Instruction]>,
    pub data_access: Option<IrStatementDescriptorMap<Vec<DataAccess>>>,
    /// Analyzed Datatypes.
    pub known_datatypes: Option<IrStatementDescriptorMap<Vec<KnownDataType>>>,
    /// Analyzed Variables
    pub variables: Option<Vec<IrVariable>>,
}

impl IrBlock {
    pub fn new(data: Vec<Ir>, instructions: Arc<[Instruction]>) -> Self {
        Self {
            ir: data.into_boxed_slice(),
            instructions,
            data_access: None,
            known_datatypes: None,
            variables: None,
        }
    }
    pub fn ir(&self) -> &[Ir] {
        &self.ir
    }
    pub fn instructions(&self) -> &Arc<[Instruction]> {
        &self.instructions
    }

    pub fn analyze_data_access(&mut self) {
        debug!("Analyzing data access");
        let mut result = IrStatementDescriptorMap::new();
        for (ir_index, ir) in self.ir.iter().enumerate() {
            analyze::analyze_data_access(&mut result, ir_index as u32, ir);
        }
        self.data_access = Some(result);
    }

    pub fn analyze_datatypes(&mut self) {
        debug!("Analyzing datatypes");
        let mut result = IrStatementDescriptorMap::new();
        for (ir_index, ir) in self.ir.iter().enumerate() {
            analyze::analyze_datatype(&mut result, ir_index as u32, ir);
        }
        self.known_datatypes = Some(result);
    }

    pub fn analyze_variables(&mut self) -> Result<(), &'static str> {
        debug!("Analyzing variables");
        let mut variables = analyze::analyze_variables(self)?;
        variables.shrink_to_fit();
        self.variables = Some(variables);
        Ok(())
    }

    pub fn validate(&self) -> Result<(), IrAnalyzeAssertionFailure> {
        self.validate_data_access()?;
        self.validate_datatypes()?;
        self.validate_variables()?;
        Ok(())
    }
    pub fn validate_data_access(&self) -> Result<(), IrAnalyzeAssertionFailure> {
        if self.data_access.is_none() {
            return Err(IrAnalyzeAssertionFailure::AnalyzeNotPerformed(
                "Data Access",
            ));
        }
        let _data_access_per_ir = self.data_access.as_ref().unwrap();

        Ok(())
    }
    pub fn validate_datatypes(&self) -> Result<(), IrAnalyzeAssertionFailure> {
        if self.known_datatypes.is_none() {
            return Err(IrAnalyzeAssertionFailure::AnalyzeNotPerformed("Datatype"));
        }
        let _known_datatypes_per_ir = self.known_datatypes.as_ref().unwrap();

        Ok(())
    }
    pub fn validate_variables(&self) -> Result<(), IrAnalyzeAssertionFailure> {
        if self.variables.is_none() {
            return Err(IrAnalyzeAssertionFailure::AnalyzeNotPerformed("Variables"));
        }
        let _variables = self.variables.as_ref().unwrap();

        Ok(())
    }

    /// Arg must contain self
    #[deprecated]
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
#[derive(Debug, Clone)]
pub struct Ir {
    /// Address of the instruction
    pub address: Address,
    /// Executed statements
    pub statements: Option<&'static [IrStatement]>,
}
