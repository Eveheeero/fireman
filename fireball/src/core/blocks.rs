//! 프로그램을 분석한 결과로 나온 "Block"를 모아두는 구조체를 정의하는 모듈

use crate::core::{relation::DestinationType, Address, Block, Relation, RelationType};
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
    /// - `start_address: Address`: 블럭의 시작 주소
    /// - `end_address: Option<Address>`: 블럭의 마지막 인스트럭션의 주소
    /// - `connected_to: &[(Option<Address>, DestinationType, RelationType)]`: 이 블럭이 어떤 블럭과 연결되었는지
    /// - `name: Option<String>`: 블럭의 이름
    ///
    /// ### Returns
    /// - `Arc<Block>`: 생성된 블럭
    pub(crate) fn generate_block(
        &self,
        start_address: Address,
        end_address: Option<Address>,
        connected_to: &[(Option<Address>, DestinationType, RelationType)],
        name: Option<String>,
    ) -> Arc<Block> {
        /* 락 해제 전 관계 생성 (관계 생성중 저장소 접근 필요하기 떄문) */
        let connected_to: Vec<_> = connected_to
            .into_iter()
            .map(|connected_to| {
                let connected_block = connected_to
                    .0
                    .as_ref()
                    .and_then(|connected_to| self.find_from_start_address(connected_to));
                (connected_block, connected_to.1, connected_to.2)
            })
            .collect();

        /* 저장소의 락 해제 */
        let blocks_writer = &mut self.data.write().unwrap();

        /* 주어진 정보로 새 블록 생성 */
        let new_block = Block::new(blocks_writer.len(), name, start_address, end_address);

        for connected_to in connected_to {
            let connected_block = connected_to.0;
            let relation = Relation::new(
                new_block.get_id(),
                connected_block.as_ref().map(|x| x.get_id()),
                connected_to.1,
                connected_to.2,
            );
            new_block.add_connected_to(relation);
            if let Some(connected_block) = connected_block {
                connected_block.add_connected_from(relation);
            }
        }

        /* 새 블록을 저장소에 저장 */
        blocks_writer.insert(new_block.clone());

        /* 반환 */
        new_block
    }

    /// 주어진 주소를 시작주소로 가진 블럭을 반환한다.
    ///
    /// ### Arguments
    /// - `address: &Address`: 대상 주소
    ///
    /// ### Returns
    /// - `Option<Arc<Block>>`: 검출된 블럭
    pub(crate) fn find_from_start_address(&self, address: &Address) -> Option<Arc<Block>> {
        /* 저장소의 락 해제 */
        let blocks_reader = &self.data.read().unwrap();

        /* 저장소의 데이터에서 검사 */
        blocks_reader
            .iter()
            .find(|block| block.get_start_address() == address)
            .map(Arc::clone)
    }

    /// 주어진 주소를 포함하는 블럭을 반환한다.
    ///
    /// ### Arguments
    /// - `address: &Address`: 대상 주소
    ///
    /// ### Returns
    /// - `Vec<Arc<Block>>`: 검출된 블럭
    pub(crate) fn find_from_containing_address(&self, address: &Address) -> Vec<Arc<Block>> {
        /* 저장소의 락 해제 */
        let blocks_reader = &self.data.read().unwrap();

        /* 저장소의 데이터에서 검사 */
        blocks_reader
            .iter()
            .filter(|block| {
                block.get_start_address() <= address
                    && if block.get_end_address().is_some() {
                        block.get_end_address().unwrap() >= address
                    } else {
                        true
                    }
            })
            .map(Arc::clone)
            .collect()
    }
}
