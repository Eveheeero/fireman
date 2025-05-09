//! 분석중 나온 분기에 대한 연관관계를 정의하는 모듈

use crate::core::Address;

/// 코드 블럭과 다른 블럭과의 연결을 나타낸다. (jmp, call 등)
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Relation {
    /// 해당 연결의 출발 블럭 아이디
    from: usize,
    /// 해당 연결의 도착 주소
    to: Option<Address>,
    /// 해당 연결의 도착 주소 타입
    destination_type: DestinationType,
    /// 해당 연결의 타입
    relation_type: RelationType,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum DestinationType {
    /// 정적 주소
    Static,
    /// 동적 주소
    Dynamic,
}

/// 코드 블럭의 연결 타입을 나타낸다.
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum RelationType {
    /// 해당 연결이 call 연결임을 나타낸다.
    Call,
    /// 해당 연결이 jmp 연결임을 나타낸다.
    Jump,
    Jcc,
    /// 한 블럭이 여러 블럭으로 나누어 진 경우
    Continued,
    /// 해당 연결이 ret 연결임을 나타낸다.
    Return,
}

impl Relation {
    /// 새로운 연결을 생성한다.
    ///
    /// ### Arguments
    /// - `from: usize`: 연결의 출발 블럭 아이디
    /// - `to: Option<Address>`: 연결의 도착 주소
    /// - `destination_type: DestinationType`: 연결의 도착 주소 타입
    /// - `relation_type: RelationType`: 연결의 타입
    ///
    /// ### Returns
    /// - `Self`: 새로 생성된 연결
    pub fn new(
        from: usize,
        to: Option<Address>,
        destination_type: DestinationType,
        relation_type: RelationType,
    ) -> Self {
        Self {
            from,
            to,
            destination_type,
            relation_type,
        }
    }

    /// 연결의 시작 블럭을 가져온다.
    ///
    /// ### Returns
    /// - `usize`: 연결의 시작 블럭 아이디
    pub fn from(&self) -> usize {
        self.from
    }

    /// 연결의 도착 블럭을 가져온다.
    ///
    /// ### Returns
    /// - `Option<Address>`: 연결의 도착 주소
    pub fn to(&self) -> Option<Address> {
        self.to.clone()
    }

    pub fn destination_type(&self) -> &DestinationType {
        &self.destination_type
    }
    pub fn relation_type(&self) -> &RelationType {
        &self.relation_type
    }
}
