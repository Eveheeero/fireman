use std::sync::{Arc, RwLock, RwLockReadGuard};

use super::{Address, Relation, Section};

/// 분석할 코드 블럭
#[derive(Debug)]
pub struct Block {
    /// 블럭의 아이디
    id: usize,
    /// 블럭의 이름
    name: Option<String>,
    /// 블럭의 시작
    start_address_virtual: Address,
    /// 블럭의 끝
    end_address_virtual: Option<Address>,
    /// 현재 블럭과 연관되어있는 블럭들을 담은 벡터
    connected_from: RwLock<Vec<Arc<Relation>>>,
    /// 현재 블럭과 연관된 블럭들을 담은 벡터
    connected_to: RwLock<Vec<Arc<Relation>>>,
    /// 블럭의 섹션
    section: Arc<Section>,
}

impl Block {
    pub(super) fn new(
        id: usize,
        name: Option<String>,
        start_address_virtual: Address,
        end_address_virtual: Option<Address>,
        section: Arc<Section>,
    ) -> Arc<Self> {
        Arc::new(Self {
            id,
            name,
            start_address_virtual,
            end_address_virtual,
            connected_from: Default::default(),
            connected_to: Default::default(),
            section,
        })
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn get_start_address_virtual(&self) -> &Address {
        &self.start_address_virtual
    }

    pub fn get_end_address_virtual(&self) -> Option<&Address> {
        self.end_address_virtual.as_ref()
    }

    pub fn get_connected_from(&self) -> RwLockReadGuard<Vec<Arc<Relation>>> {
        self.connected_from.read().unwrap()
    }

    pub fn get_connected_to(&self) -> RwLockReadGuard<Vec<Arc<Relation>>> {
        self.connected_to.read().unwrap()
    }

    pub fn get_section(&self) -> &Arc<Section> {
        &self.section
    }

    pub(crate) fn add_connected_from(&self, relation: Arc<Relation>) {
        self.connected_from.write().unwrap().push(relation);
    }

    pub(crate) fn add_connected_to(&self, relation: Arc<Relation>) {
        self.connected_to.write().unwrap().push(relation);
    }
}

impl std::hash::Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for Block {
    fn assert_receiver_is_total_eq(&self) {
        self.id.assert_receiver_is_total_eq();
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
