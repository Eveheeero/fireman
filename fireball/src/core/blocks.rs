//! 프로그램을 분석한 결과로 나온 "Block"를 모아두는 구조체를 정의하는 모듈

use super::{Address, Block, Section};
use std::sync::Arc;

/// 어셈블리 단위의 블럭들을 관리하는 구조체
///
/// 해당 구조체를 이용해 블럭을 생성하고, 이미 존재하는 블럭을 가져온다.
pub struct Blocks {
    /// 블럭들의 실제 데이터
    data: std::sync::RwLock<std::collections::HashSet<Arc<Block>>>,
}

impl Blocks {
    /// 블럭 저장소 구조체를 생성한다.
    ///
    /// ### Returns
    /// - `Arc<Self>`: 생성된 블럭 저장소 구조체
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            data: Default::default(),
        })
    }

    /// 저장소에 새 블럭을 생성한다.
    ///
    /// ### Arguments
    /// - `section: Arc<Section>`: 블럭이 속한 섹션
    /// - `start_address_virtual: Address`: 블럭의 시작 주소
    /// - `end_address_virtual: Option<Address>`: 블럭의 끝 주소
    /// - `name: Option<String>`: 블럭의 이름
    ///
    /// ### Returns
    /// - `Arc<Block>`: 생성된 블럭
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

    /// 주어진 주소를 시작주소로 가진 블럭을 반환한다.
    ///
    /// ### Arguments
    /// - `address: Address`: 대상 주소
    ///
    /// ### Returns
    /// - `Option<Arc<Block>>`: 검출된 블럭
    pub(crate) fn find_from_start_address(&self, address: Address) -> Option<Arc<Block>> {
        /* 저장소의 락 해제 */
        let blocks_reader = &self.data.read().unwrap();

        /* 저장소의 데이터에서 검사 */
        blocks_reader
            .iter()
            .find(|block| block.get_start_address() == &address)
            .map(Arc::clone)
    }
}
