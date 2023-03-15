use std::sync::Arc;

use super::Address;

/// 코드 블럭과 다른 블럭과의 연결을 나타낸다. (jmp, call 등)
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Relation {
    /// 해당 연결의 출발점
    from: Address,
    /// 해당 연결의 도착점
    to: Address,
    /// 해당 연결의 타입
    relation_type: RelationType,
}

/// 코드 블럭의 연결 타입을 나타낸다.
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum RelationType {
    /// 해당 연결이 call 연결임을 나타낸다.
    Call,
    /// 해당 연결이 jmp 연결임을 나타낸다.
    Jump,
}

impl Relation {
    /// 새로운 연결을 생성한다.
    ///
    /// ### Arguments
    /// - `from: Address`: 연결의 출발점
    /// - `to: Address`: 연결의 도착점
    /// - `relation_type: RelationType`: 연결의 타입
    ///
    /// ### Returns
    /// - `Arc<Self>`: 새로 생성된 연결
    pub fn new(from: Address, to: Address, relation_type: RelationType) -> Arc<Self> {
        Arc::new(Self {
            from,
            to,
            relation_type,
        })
    }

    /// 연결의 시작 주소를 가져온다.
    ///
    /// ### Returns
    /// - `&Address`: 연결의 시작 주소
    pub fn from(&self) -> &Address {
        &self.from
    }

    /// 연결의 도착 주소를 가져온다.
    ///
    /// ### Returns
    /// - `&Address`: 연결의 도착 주소
    pub fn to(&self) -> &Address {
        &self.to
    }
}
