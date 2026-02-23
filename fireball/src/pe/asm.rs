//! Assembly parsing module

use super::Pe;
use crate::{
    core::{Address, Instruction},
    prelude::*,
};

impl Pe {
    /// Parses assembly code within the specified range.
    pub(crate) fn parse_assem_range(
        &self,
        offset: &Address,
        size: u64,
    ) -> Result<Vec<Instruction>, DisassembleError> {
        let file_offset = if let Some(file_offset) = offset.get_file_offset() {
            file_offset
        } else {
            warn!(
                "Could not determine file offset: virtual address {:#x}",
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
                error!(
                    ?e,
                    "Assembly parsing failed: virtual address {:#x}, file offset {:#x}",
                    virtual_offset,
                    file_offset
                );
                return Err(DisassembleError::CapstoneFailed(e.to_string()));
            }
        };
        Ok(self.transform_instructions(insns))
    }

    /// Parses the specified number of assembly instructions.
    pub(crate) fn parse_assem_count(
        &self,
        offset: &Address,
        count: usize,
    ) -> Result<Vec<Instruction>, DisassembleError> {
        let file_offset = if let Some(file_offset) = offset.get_file_offset() {
            file_offset
        } else {
            warn!(
                "Could not determine file offset: virtual address {:#x}",
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
                error!(
                    ?e,
                    "Assembly parsing failed: virtual address {:#x}, file offset {:#x}",
                    virtual_offset,
                    file_offset
                );
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
            let statement = iceball::parse_statement(iceball::Architecture::X64, mnemonic);
            let mut arguments = Vec::new();
            if op.is_some() {
                for op in op.unwrap().split(", ") {
                    if op.is_empty() {
                        continue;
                    }
                    if let Some(argument) = Self::parse_argument_lossy(op) {
                        arguments.push(argument);
                    } else {
                        warn!(
                            "Failed to parse argument `{}` at {:#x}; dropping operand",
                            op,
                            item.address()
                        );
                    }
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

    fn parse_argument_lossy(op: &str) -> Option<iceball::Argument> {
        if let Some(arg) = Self::parse_argument_safe(op) {
            return Some(arg);
        }

        let lowered = op.to_ascii_lowercase();
        let stripped = [
            "byte ptr ",
            "word ptr ",
            "dword ptr ",
            "qword ptr ",
            "xmmword ptr ",
            "ymmword ptr ",
            "zmmword ptr ",
            "ptr ",
        ]
        .iter()
        .find_map(|prefix| lowered.strip_prefix(prefix).map(str::trim));

        if let Some(candidate) = stripped
            && let Some(arg) = Self::parse_argument_safe(candidate)
        {
            return Some(arg);
        }

        None
    }

    fn parse_argument_safe(op: &str) -> Option<iceball::Argument> {
        std::panic::catch_unwind(|| iceball::parse_argument(iceball::Architecture::X64, op))
            .ok()
            .and_then(Result::ok)
    }
}
