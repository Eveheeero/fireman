use crate::binary::pe::Pe;
use crate::{
    core::{Address, Block, BlockRelationInformation, DestinationType, RelationType},
    prelude::*,
};
use std::sync::Arc;

impl Pe {
    /// Generate and return a block from the given address.
    ///
    /// ### Arguments
    /// - `address: &Address` - The address from which to generate the block.
    ///
    /// ### Returns
    /// - `Arc<Block>` - The block generated from the address.
    pub(crate) fn generate_block_from_address(&self, address: &Address) -> Arc<Block> {
        if let Some(block) = self.blocks.get_by_start_address(address) {
            return block;
        }
        debug!("Block generation started {}", address);
        let mut instructions = Vec::new();
        let mut address = address.clone();
        let start_address = address.clone();
        let mut last_instruction_address = None;
        let mut block_size = None;
        // loop until we find a jump or call instruction
        loop {
            let inst = self.parse_assem_count(&address, 1);
            if inst.is_err() || inst.as_ref().unwrap().is_empty() {
                warn!(
                    "Instruction parsing failed: {:#x}",
                    address.get_virtual_address()
                );
                break;
            }
            let mut inst = inst.unwrap();
            debug_assert_eq!(inst.len(), 1);
            let inst = inst.pop().unwrap();
            instructions.push(inst);
            let inst = &instructions.last().unwrap().inner;
            if let Err(e) = inst.statement {
                error!(
                    "Instruction converting failed: {:#x} {:?}",
                    address.get_virtual_address(),
                    e
                );
                break;
            }
            if inst.is_jcc() || inst.is_jmp() || inst.is_call() || inst.is_ret() {
                last_instruction_address = Some(address);
                break;
            }
            address += inst.bytes.as_ref().unwrap().len() as u64;
        }

        /* Find connected blocks */
        let mut connected_to = Vec::new();
        // if the last instruction is not set, there is no connected block
        if let Some(last_instruction_address) = &last_instruction_address {
            let inst = &self.parse_assem_count(last_instruction_address, 1).unwrap()[0].inner;
            block_size = Some(
                last_instruction_address - &start_address
                    + inst.bytes.as_ref().unwrap().len() as u64,
            );
            if inst.is_jcc() || inst.is_call() {
                // false branch or halt
                let relation_type = if inst.is_jcc() {
                    RelationType::Continued
                } else {
                    RelationType::Halt
                };
                connected_to.push(BlockRelationInformation {
                    destination: Some(
                        last_instruction_address + inst.bytes.as_ref().unwrap().len() as u64,
                    ),
                    destination_type: DestinationType::Static,
                    relation_type,
                });
            }
            // address that the last instruction points to
            connected_to
                .push(self.get_connected_address_and_relation_type(last_instruction_address, inst));
        }

        debug!(
            ?connected_to,
            "Block generation done for size {:?}", block_size
        );
        self.blocks.generate_block(
            start_address,
            block_size,
            &connected_to,
            None,
            instructions.into(),
        )
    }

    /// Returns the target address and relation type from the final instruction.
    fn get_connected_address_and_relation_type(
        &self,
        ip: &Address,
        inst: &iceball::Instruction,
    ) -> BlockRelationInformation {
        let relation_type = match () {
            _ if inst.is_ret() => RelationType::Return,
            _ if inst.is_jcc() => RelationType::Jcc,
            _ if inst.is_jmp() => RelationType::Jump,
            _ if inst.is_call() => RelationType::Call,
            _ => unreachable!("{:?}", inst),
        };
        if inst.arguments.len() != 1 {
            return BlockRelationInformation {
                destination: None,
                destination_type: DestinationType::Dynamic,
                relation_type,
            };
        }
        let arg = &inst.arguments[0];
        match arg {
            // only rip is predictable target but we can't get it
            iceball::Argument::Register(_) => BlockRelationInformation {
                destination: None,
                destination_type: DestinationType::Dynamic,
                relation_type,
            },
            iceball::Argument::Memory(mem) => {
                // Check if this is absolute addressing (no base register)
                if mem.base.is_none() && mem.index.is_none() {
                    BlockRelationInformation {
                        destination: Some(Address::from_virtual_address(
                            &self.sections,
                            mem.displacement as u64,
                        )),
                        destination_type: DestinationType::Static,
                        relation_type,
                    }
                } else {
                    // Check if this is RIP-relative addressing
                    let is_rip_relative = match &mem.base {
                        Some(iceball::Register::X64(iceball::X64Register::Eip))
                        | Some(iceball::Register::X64(iceball::X64Register::Rip)) => true,
                        _ => false,
                    } && mem.index.is_none();

                    if is_rip_relative {
                        // Calculate absolute address for RIP-relative operand
                        let absolute_addr = ip.get_virtual_address() as i64 + mem.displacement;
                        BlockRelationInformation {
                            destination: Some(Address::from_virtual_address(
                                &self.sections,
                                absolute_addr as u64,
                            )),
                            destination_type: DestinationType::Static,
                            relation_type,
                        }
                    } else {
                        BlockRelationInformation {
                            destination: None,
                            destination_type: DestinationType::Dynamic,
                            relation_type,
                        }
                    }
                }
            }
            iceball::Argument::Constant(arg) => BlockRelationInformation {
                destination: Some(Address::from_virtual_address(&self.sections, *arg)),
                destination_type: DestinationType::Static,
                relation_type,
            },
        }
    }
}
