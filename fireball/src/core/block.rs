use std::sync::Arc;

use super::{Address, Relation, Section};

/// 분석할 코드 블럭
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct Block {
    /// 블럭의 아이디
    id: usize,
    /// 블럭의 이름
    name: Option<String>,
    /// 블럭의 시작
    start_address_virtual: Address,
    /// 블럭의 끝
    end_address_virtual: Option<Address>,
    /// 현재 블럭과 연관되어있는 블럭들을 담은 벡터
    connected_from: Vec<Relation>,
    /// 현재 블럭과 연관된 블럭들을 담은 벡터
    connected_to: Vec<Relation>,
    /// 블럭의 섹션
    section: Arc<Section>,
}

lazy_static::lazy_static! {
    static ref BLOCKS: std::sync::RwLock<std::collections::HashSet<Arc<Block>>> = Default::default();
}

impl Block {
    pub(crate) fn new(
        section: Arc<Section>,
        start_address_virtual: Address,
        end_address_virtual: Option<Address>,
    ) -> Arc<Self> {
        let mut blocks_writer = BLOCKS.write().unwrap();

        let new_block = Block {
            id: blocks_writer.len(),
            name: None,
            start_address_virtual,
            end_address_virtual,
            connected_from: Default::default(),
            connected_to: Default::default(),
            section,
        };

        let new_block = Arc::new(new_block);

        blocks_writer.insert(new_block.clone());

        new_block
    }
}
