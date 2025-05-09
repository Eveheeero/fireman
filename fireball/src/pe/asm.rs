//! 어셈블리 파싱 모듈

use super::Pe;
use crate::{
    core::{Address, Instruction},
    prelude::*,
};

impl Pe {
    /// 범위만큼의 어셈블리 코드를 파싱한다.
    pub(crate) fn parse_assem_range(
        &self,
        offset: &Address,
        size: u64,
    ) -> Result<Vec<Instruction>, DisassembleError> {
        let file_offset = if let Some(file_offset) = offset.get_file_offset() {
            file_offset
        } else {
            trace!(
                "파일 오프셋을 찾을 수 없음 : 가상주소 {:#x}",
                offset.get_virtual_address()
            );
            return Err(DisassembleError::TriedToParseOutsideOfSection);
        };
        let virtual_offset = offset.get_virtual_address();
        let insns = match self.capstone.disasm_all(
            &self.binary[file_offset as usize..(file_offset + size) as usize],
            virtual_offset as u64,
        ) {
            Ok(insts) => insts,
            Err(e) => {
                trace!(
                    "어셈블리 코드 파싱 실패 : 가상주소 {:#x}, 파일주소 {:#x}",
                    virtual_offset,
                    file_offset
                );
                error!(?e);
                return Err(DisassembleError::CapstoneFailed(e.to_string()));
            }
        };
        Ok(self.transform_instructions(insns))
    }

    /// 어셈블리 코드를 N개 파싱한다.
    pub(crate) fn parse_assem_count(
        &self,
        offset: &Address,
        count: usize,
    ) -> Result<Vec<Instruction>, DisassembleError> {
        let file_offset = if let Some(file_offset) = offset.get_file_offset() {
            file_offset
        } else {
            trace!(
                "파일 오프셋을 찾을 수 없음 : 가상주소 {:#x}",
                offset.get_virtual_address()
            );
            return Err(DisassembleError::TriedToParseOutsideOfSection);
        };
        let virtual_offset = offset.get_virtual_address();
        let insns = match self.capstone.disasm_count(
            &self.binary[file_offset as usize..],
            virtual_offset as u64,
            count,
        ) {
            Ok(insts) => insts,
            Err(e) => {
                trace!(
                    "어셈블리 코드 파싱 실패 : 가상주소 {:#x}, 파일주소 {:#x}",
                    virtual_offset,
                    file_offset
                );
                error!(?e);
                return Err(DisassembleError::CapstoneFailed(e.to_string()));
            }
        };
        Ok(self.transform_instructions(insns))
    }

    fn transform_instructions(&self, input: capstone::Instructions) -> Vec<Instruction> {
        let mut result = Vec::new();
        for item in input.iter() {
            let mnemonic = item.mnemonic().unwrap();
            let op = item.op_str();
            trace!("{} {:?} 인스트럭션 파싱", mnemonic, op);
            let statement = iceball::parse_statement(iceball::Architecture::X64, mnemonic);
            let mut arguments = Vec::new();
            if op.is_some() {
                for op in op.unwrap().split(", ") {
                    if op.is_empty() {
                        continue;
                    }
                    let argument = iceball::parse_argument(iceball::Architecture::X64, op)
                        .unwrap_or_else(|_| panic!("{} 파싱 실패", op));
                    arguments.push(argument);
                }
            }
            let bytes = Some(item.bytes().into());
            let data = Instruction {
                address: item.address(),
                inner: iceball::Instruction {
                    statement,
                    arguments: arguments.into_boxed_slice(),
                    bytes,
                },
            };
            result.push(data);
        }
        result
    }
}
