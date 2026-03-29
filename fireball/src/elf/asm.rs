//! Assembly parsing module for ELF binaries

use super::Elf;
use crate::{
    core::{Address, Instruction},
    prelude::*,
};

impl Elf {
    /// Parses assembly code within the specified range.
    pub(crate) fn parse_assem_range(
        &self,
        offset: &Address,
        size: u64,
    ) -> Result<Vec<Instruction>, DisassembleError> {
        let parser_architecture = self.architecture();
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
        // Check bounds to prevent slice overflow
        let end = file_offset
            .checked_add(size)
            .and_then(|e| e.try_into().ok())
            .filter(|&e: &u64| e <= self.binary.len() as u64);
        let Some(end) = end else {
            error!(
                file_offset,
                size,
                binary_len = self.binary.len(),
                "Slice bounds check failed: file_offset + size exceeds binary length"
            );
            return Err(DisassembleError::TriedToParseOutsideOfSection);
        };
        let insns = match self.capstone.disasm_all(
            &self.binary[file_offset as usize..end as usize],
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
        Ok(self.transform_instructions(parser_architecture, insns))
    }

    /// Parses the specified number of assembly instructions.
    pub(crate) fn parse_assem_count(
        &self,
        offset: &Address,
        count: usize,
    ) -> Result<Vec<Instruction>, DisassembleError> {
        let parser_architecture = self.architecture();
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
        Ok(self.transform_instructions(parser_architecture, insns))
    }

    fn transform_instructions(
        &self,
        parser_architecture: iceball::MachineArchitecture,
        input: capstone::Instructions,
    ) -> Vec<Instruction> {
        let mut result = Vec::new();
        for item in input.iter() {
            let mnemonic = item.mnemonic().unwrap();
            let op = item.op_str();
            let statement = iceball::parse_statement(parser_architecture, mnemonic);
            let mut arguments = Vec::new();
            if op.is_some() {
                for op in split_operands(op.unwrap()) {
                    if op.is_empty() {
                        continue;
                    }
                    if let Some(argument) = Self::parse_argument_lossy(parser_architecture, &op) {
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

    fn parse_argument_lossy(
        parser_architecture: iceball::MachineArchitecture,
        op: &str,
    ) -> Option<iceball::Argument> {
        if let Some(arg) = Self::parse_argument_safe(parser_architecture, op) {
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
            && let Some(arg) = Self::parse_argument_safe(parser_architecture, candidate)
        {
            return Some(arg);
        }

        None
    }

    fn parse_argument_safe(
        parser_architecture: iceball::MachineArchitecture,
        op: &str,
    ) -> Option<iceball::Argument> {
        std::panic::catch_unwind(|| iceball::parse_argument(parser_architecture, op))
            .ok()
            .and_then(Result::ok)
    }
}

fn split_operands(op: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut bracket_depth = 0usize;
    let mut brace_depth = 0usize;
    let mut paren_depth = 0usize;

    for ch in op.chars() {
        match ch {
            '[' => {
                bracket_depth += 1;
                current.push(ch);
            }
            ']' => {
                bracket_depth = bracket_depth.saturating_sub(1);
                current.push(ch);
            }
            '{' => {
                brace_depth += 1;
                current.push(ch);
            }
            '}' => {
                brace_depth = brace_depth.saturating_sub(1);
                current.push(ch);
            }
            '(' => {
                paren_depth += 1;
                current.push(ch);
            }
            ')' => {
                paren_depth = paren_depth.saturating_sub(1);
                current.push(ch);
            }
            ',' if bracket_depth == 0 && brace_depth == 0 && paren_depth == 0 => {
                let operand = current.trim();
                if !operand.is_empty() {
                    result.push(operand.to_string());
                }
                current.clear();
            }
            _ => current.push(ch),
        }
    }

    let operand = current.trim();
    if !operand.is_empty() {
        result.push(operand.to_string());
    }

    result
}
