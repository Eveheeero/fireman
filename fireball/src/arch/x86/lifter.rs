//! x86 (32-bit) IR lifter - converts x86 instructions to Low IR

use crate::core::{Address, Block as IrBlock};
use crate::ir::low_ir::{
    BasicBlock, BlockId, Endianness, Function, FunctionId, Module, TargetInfo, Terminator, Type,
};
use std::collections::BTreeMap;

/// x86 (32-bit) specific IR lifter
pub struct X86Lifter {
    /// Current module being built
    module: Module,
}

impl X86Lifter {
    /// Create a new x86 lifter
    pub fn new() -> Self {
        Self {
            module: Module {
                target: TargetInfo {
                    arch: "x86".to_string(),
                    bits: 32,
                    endian: Endianness::Little,
                },
                functions: BTreeMap::new(),
                globals: BTreeMap::new(),
                externals: BTreeMap::new(),
            },
        }
    }

    /// Convert IrBlock to Low IR Module
    pub fn lift_block(
        &mut self,
        ir_block: &IrBlock,
        start_addr: &Address,
    ) -> Result<Module, String> {
        // Create function ID from start address
        let func_id = FunctionId(start_addr.get_virtual_address());

        // Create a single function for the block
        let mut function = Function {
            id: func_id.clone(),
            signature: Type::Function {
                ret: Box::new(Type::Void),
                params: vec![],
                varargs: false,
            },
            entry: BlockId(start_addr.get_virtual_address()),
            blocks: BTreeMap::new(),
            locals: BTreeMap::new(),
        };

        let mut instructions = Vec::new();

        // Convert each IR statement to Low IR instruction
        if let Some(ir_block_data) = ir_block.get_ir().as_ref() {
            for ir in ir_block_data.ir() {
                // TODO: Implement x86 IR conversion
                // Many conversions can be shared with x86_64
                // but with 32-bit register names and sizes
            }
        }

        // Create basic block
        let bb = BasicBlock {
            id: BlockId(start_addr.get_virtual_address()),
            phis: vec![],
            instructions,
            terminator: Terminator::Return(None),
        };

        function.blocks.insert(bb.id.clone(), bb);
        self.module.functions.insert(func_id, function);

        Ok(self.module.clone())
    }
}
