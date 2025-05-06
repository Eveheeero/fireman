use crate::{
    core::Address,
    ir::{
        analyze::DataType,
        data::{AccessSize, DataAccess, IrData, IrDataContainable},
        statements::IrStatement,
        Ir, IrBlock,
    },
    utils::Aos,
};
pub use private::IrVariable;
use std::collections::HashSet;

mod private {
    use super::*;
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct IrVariable {
        /// Index of Ir in IrBlock
        pub live_in: usize,
        /// Index of Ir in IrBlock
        pub shown_in: Vec<usize>,
        /// Index of Ir in IrBlock
        pub live_out: Option<usize>,
        /// Index of Ir in IrBlock
        accesses: Vec<Option<Vec<DataAccess>>>,
        pub data_type: DataType,
    }
    impl IrVariable {
        #[inline]
        pub fn new(live_in_ir_index: usize, data_type: DataType) -> Self {
            Self {
                live_in: live_in_ir_index,
                shown_in: [live_in_ir_index].into(),
                live_out: None,
                accesses: Vec::new(),
                data_type,
            }
        }
        #[inline]
        pub fn get_data_accesses(&self, ir_index: usize) -> &[DataAccess] {
            self.accesses
                .get(ir_index)
                .unwrap_or(&None)
                .as_ref()
                .map(Vec::as_slice)
                .unwrap_or(&[])
        }
        #[inline]
        pub fn add_data_access(&mut self, ir_index: usize, access: DataAccess) {
            if self.accesses.len() <= ir_index {
                self.accesses.resize_with(ir_index + 1, || None);
            }
            if self.accesses[ir_index].is_none() {
                self.accesses[ir_index] = Some(Vec::new());
            }
            self.accesses[ir_index].as_mut().unwrap().push(access);
        }
    }
}

pub fn analyze_variables(ir_block: &IrBlock) -> Result<HashSet<IrVariable>, &'static str> {
    let mut result = HashSet::new();
    let mut living_variables = Vec::new();
    let irs = ir_block.ir.as_ref();
    let known_datatypes_per_ir = ir_block
        .known_datatypes_per_ir
        .as_ref()
        .ok_or_else(|| "Datatypes Not Analyzed")?;
    let data_access_per_ir = ir_block
        .data_access_per_ir
        .as_ref()
        .ok_or_else(|| "Data Access Not Analyzed")?;

    for (ir_index, ir) in irs.iter().enumerate() {
        if ir.statements.is_right() {
            continue;
        }
        /* 기본 변수 설정 */
        let statements = ir.statements.as_ref().unwrap_left();
        let known_datatypes = known_datatypes_per_ir.get(ir_index).unwrap();
        let data_access = data_access_per_ir.get(ir_index).unwrap();
        let related_data_per_statement: Vec<Vec<&Aos<IrData>>> = statements
            .iter()
            .map(|statement| {
                let mut related_data = Vec::new();
                statement.get_related_ir_data(&mut related_data);
                related_data
            })
            .collect();

        for (statement_index, (statement, related_data)) in statements
            .iter()
            .zip(related_data_per_statement.iter())
            .enumerate()
        {}
    }

    // 분석이 끝날때까지 살아있는 변수 추가
    result.extend(living_variables);
    Ok(result)
}
