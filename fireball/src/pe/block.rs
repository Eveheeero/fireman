use crate::{
    core::{Address, Block},
    pe::PE,
};
use std::sync::Arc;

impl PE {
    /// 파일 오프셋을 기준으로 블럭의 범위를 계산한다
    ///
    /// ### Arguments
    /// - `address: &Address` - 블럭의 시작
    ///
    /// ### Returns
    /// - `Arc<Block>` - 해당 주소로부터 계산된 블럭
    pub(crate) fn generate_block_from_address(&self, address: &Address) -> Arc<Block> {
        if let Some(block) = self.blocks.find_from_start_address(address) {
            return block;
        }
        let mut address = address.clone();
        let start_address = address.clone();
        let mut end_address = None;
        loop {
            let inst = self.parse_assem_count(&address, 1);
            if inst.is_err() || inst.as_ref().unwrap().len() == 0 {
                break;
            }
            debug_assert_eq!(inst.as_ref().unwrap().len(), 1);
            let inst = &inst.unwrap()[0].inner;
            if inst.statement.is_err() {
                break;
            }
            if inst.is_jcc() || inst.is_call() || inst.is_ret() {
                end_address = Some(address);
                break;
            }
            address += inst.bytes.as_ref().unwrap().len() as u64;
        }

        self.blocks.generate_block(start_address, end_address, None)
    }
}
