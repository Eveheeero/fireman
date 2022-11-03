use std::sync::Arc;

use super::PE;
use crate::core::{Address, Block, Relation};

impl PE {
    pub(super) fn _parse_block(&self, address: Address) -> (Arc<Block>, Option<Arc<Relation>>) {
        let section = self
            .sections
            .from_virtual_address(address.get_virtual_address())
            .unwrap();
        let block_start = address.clone();
        let block_end: Address;
        let relation: Option<Arc<Relation>>;
        let mut now_address = address;
        loop {
            let insts = self
                .parse_assem_count(now_address.clone(), 1)
                .expect("Disassemble Error!");
            let inst = &insts[0];
            match inst.mnemonic().unwrap() {
                "call" => {
                    let target = inst.op_str().unwrap();
                    let target_address = Address::from_virtual_address(
                        &self.sections,
                        target.parse::<u64>().unwrap(),
                    )
                    .unwrap();
                    relation = Some(Relation::new(now_address.clone(), target_address));
                    block_end = now_address;
                    break;
                }
                "jmp" => {}
                "ret" => {}
                _ =>
                    /* now_address = now_address + inst.len() */
                    {}
            }
        }
        let block = self
            .blocks
            .new_block(section, block_start, Some(block_end), None);
        return (block, relation);
    }
}
