use std::sync::Arc;

use super::{Address, Block, Section};

/// 블록들을 가지고 있는 구조체
pub struct Blocks {
    data: std::sync::RwLock<std::collections::HashSet<Arc<Block>>>,
}

impl Blocks {
    /// 블록들을 가지고 있는 구조체를 생성한다.
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            data: Default::default(),
        })
    }

    /// 새 블록을 추가한다.
    /// 새로 만들어진 블록은 해당 구조체 내부에 저장되며, 복사본을 반환한다.
    pub(crate) fn generate_block(
        &self,
        section: Arc<Section>,
        start_address_virtual: Address,
        end_address_virtual: Option<Address>,
        name: Option<String>,
    ) -> Arc<Block> {
        /* 저장소의 락 해제 */
        let blocks_writer = &mut self.data.write().unwrap();

        /* 주어진 정보로 새 블록 생성 */
        let new_block = Block::new(
            blocks_writer.len(),
            name,
            start_address_virtual,
            end_address_virtual,
            section,
        );

        /* 새 블록을 저장소에 저장 */
        blocks_writer.insert(new_block.clone());

        /* 반환 */
        new_block
    }

    /// 시작지점이 N인 블록 객체를 반환한다.
    pub(crate) fn find_from_start_address(&self, address: Address) -> Option<Arc<Block>> {
        /* 저장소의 락 해제 */
        let blocks_reader = &self.data.read().unwrap();

        /* 저장소의 데이터에서 검사 */
        blocks_reader
            .iter()
            .find(|block| block.get_start_address_virtual() == &address)
            .map(Arc::clone)
    }
}
