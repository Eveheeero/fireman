use std::sync::Arc;

use super::Address;

/// 코드 블럭과 다른 블럭과의 연관을 나타낸다.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Relation {
    from: Address,
    to: Address,
    relation_type: RelationType,
}

/// 코드 블럭의 연관 타입을 나타낸다.
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum RelationType {
    Call,
    Jump,
    Jcc,
}

impl Relation {
    /// 새로운 연관을 생성한다.
    pub fn new(from: Address, to: Address, relation_type: RelationType) -> Arc<Self> {
        Arc::new(Self {
            from,
            to,
            relation_type,
        })
    }

    pub fn from(&self) -> &Address {
        &self.from
    }

    pub fn to(&self) -> &Address {
        &self.to
    }

    pub fn relation_type(&self) -> &RelationType {
        &self.relation_type
    }
}
