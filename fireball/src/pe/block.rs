use crate::{
    core::{Address, Block, BlockRelationInformation, DestinationType, RelationType},
    pe::Pe,
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
            iceball::Argument::Memory(iceball::Memory::AbsoluteAddressing(offset)) => {
                BlockRelationInformation {
                    destination: Some(Address::from_virtual_address(&self.sections, *offset)),
                    destination_type: DestinationType::Static,
                    relation_type,
                }
            }
            iceball::Argument::Memory(iceball::Memory::RelativeAddressing(args)) => {
                if args
                    .iter()
                    .filter(|x| matches!(x, iceball::RelativeAddressingArgument::Register(_)))
                    .all(|x| {
                        matches!(
                            x,
                            iceball::RelativeAddressingArgument::Register(iceball::Register::X64(
                                iceball::X64Register::Eip,
                            )) | iceball::RelativeAddressingArgument::Register(
                                iceball::Register::X64(iceball::X64Register::Rip,)
                            )
                        )
                    })
                {
                    BlockRelationInformation {
                        destination: Some(self.calc_relative_address_with_ip(ip, args)),
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
            iceball::Argument::Constant(arg) => BlockRelationInformation {
                destination: Some(Address::from_virtual_address(&self.sections, *arg)),
                destination_type: DestinationType::Static,
                relation_type,
            },
        }
    }

    /// Calculates the absolute address for a RIP/EIP-relative operand.
    fn calc_relative_address_with_ip(
        &self,
        ip: &Address,
        args: &[iceball::RelativeAddressingArgument],
    ) -> Address {
        let extract_constant = |arg: &iceball::RelativeAddressingArgument| match arg {
            iceball::RelativeAddressingArgument::Constant(x) => *x,
            _ => unreachable!("{:?}", arg),
        };
        let mut args: Vec<_> = args.into();

        // turn ip to constant
        for i in &mut args {
            match i {
                iceball::RelativeAddressingArgument::Register(iceball::Register::X64(
                    iceball::X64Register::Eip,
                ))
                | iceball::RelativeAddressingArgument::Register(iceball::Register::X64(
                    iceball::X64Register::Rip,
                )) => {
                    *i = iceball::RelativeAddressingArgument::Constant(
                        ip.get_virtual_address() as i128
                    );
                }
                _ => {}
            }
        }

        // calc mul operator
        while args.contains(&iceball::RelativeAddressingArgument::Operator(
            iceball::AddressingOperator::Mul,
        )) {
            let operator_index = args
                .iter()
                .position(|x| {
                    matches!(
                        x,
                        iceball::RelativeAddressingArgument::Operator(
                            iceball::AddressingOperator::Mul
                        )
                    )
                })
                .unwrap();
            let arg1 = extract_constant(&args[operator_index - 1]);
            let arg2 = extract_constant(&args[operator_index + 1]);
            args.insert(
                operator_index - 1,
                iceball::RelativeAddressingArgument::Constant(arg1 * arg2),
            );
            args.remove(operator_index);
            args.remove(operator_index);
        }

        // calc add/sub operator
        while args.contains(&iceball::RelativeAddressingArgument::Operator(
            iceball::AddressingOperator::Add,
        )) || args.contains(&iceball::RelativeAddressingArgument::Operator(
            iceball::AddressingOperator::Sub,
        )) {
            let operator_index = args
                .iter()
                .position(|x| {
                    matches!(
                        x,
                        iceball::RelativeAddressingArgument::Operator(
                            iceball::AddressingOperator::Add | iceball::AddressingOperator::Sub
                        )
                    )
                })
                .unwrap();
            let arg1 = extract_constant(&args[operator_index - 1]);
            let arg2 = extract_constant(&args[operator_index + 1]);
            args.insert(
                operator_index - 1,
                iceball::RelativeAddressingArgument::Constant(match args[operator_index] {
                    iceball::RelativeAddressingArgument::Operator(
                        iceball::AddressingOperator::Add,
                    ) => arg1 + arg2,
                    iceball::RelativeAddressingArgument::Operator(
                        iceball::AddressingOperator::Sub,
                    ) => arg1 - arg2,
                    _ => unreachable!(),
                }),
            );
            args.remove(operator_index);
            args.remove(operator_index);
            args.remove(operator_index);
        }

        // return
        debug_assert!(
            args.len() == 1,
            "Address computation not fully reduced: {:?}",
            args
        );
        let address = extract_constant(&args[0])
            .try_into()
            .expect("Negative address result");
        Address::from_virtual_address(&self.sections, address)
    }
}
