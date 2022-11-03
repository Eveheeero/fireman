use std::sync::Arc;

use super::PE;
use crate::core::{Address, Block, Relation};

impl PE {
    pub(super) fn _parse_block(&self, address: Address) -> (Arc<Block>, Option<Arc<Relation>>) {
        let block_start = address;
        let block_end: Address;
        let relation: Arc<Relation>;
        let mut now_address = address;
        loop {
            let inst = self
                .parse_assem_count(now_address.clone(), 1)
                .expect("Disassemble Error!")[0];
            match inst.mnemonic().unwrap() {
                "call" => {
                    let target = inst.op_str().unwrap();
                    let target_address = Address::from_virtual_address(
                        &self.sections,
                        target.parse::<u64>().unwrap(),
                    )
                    .unwrap();
                    relation = Relation::new(now_address, target_address);
                    block_end = now_address;
                    break;
                }
                "jmp" => {}
                "ret" => {}
                _ => {}
            }
        }
        todo!("Block관련 구조체 개선 필요");
        return;
    }
}
