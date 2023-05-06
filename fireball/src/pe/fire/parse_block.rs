use std::sync::Arc;

use super::PE;
use crate::{
    core::{Address, Block, InstructionHistory},
    prelude::{trace, BlockParsingError},
};

impl PE {
    /// ### Todo
    /// - jmp, je, jle외에도 모든 형태의 분기문에 대한 처리 필요
    /// - 점프한 주소가 범위를 벗어났을때 중단하는 처리 필요
    pub(super) fn _parse_block(
        &self,
        address: Address,
        _history: &mut InstructionHistory,
    ) -> Result<Arc<Block>, BlockParsingError> {
        trace!("블럭 파싱 시작");
        trace!("블럭 시작 주소: {}", address.get_virtual_address());

        todo!()
    }
}
