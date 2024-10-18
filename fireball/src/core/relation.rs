//! 분석중 나온 분기에 대한 연관관계를 정의하는 모듈

/// 코드 블럭과 다른 블럭과의 연결을 나타낸다. (jmp, call 등)
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Relation {
    /// 해당 연결의 출발 블럭 아이디
    from: usize,
    /// 해당 연결의 도착 블럭 아이디
    to: Option<usize>,
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
    Jcc,
    /// 한 블럭이 여러 블럭으로 나누어 진 경우
    Continued,
    /// 오프셋에 의해 연결 타겟이 달라지는 경우
    Dynamic,
}

impl Relation {
    /// 새로운 연결을 생성한다.
    ///
    /// ### Arguments
    /// - `from: usize`: 연결의 출발 블럭 아이디
    /// - `to: Option<usize>`: 연결의 도착 블럭 아이디
    /// - `relation_type: RelationType`: 연결의 타입
    ///
    /// ### Returns
    /// - `Self`: 새로 생성된 연결
    pub fn new(from: usize, to: Option<usize>, relation_type: RelationType) -> Self {
        Self {
            from,
            to,
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
    /// - `Option<usize>`: 연결의 도착 블럭 아이디
    pub fn to(&self) -> Option<usize> {
        self.to
    }

    pub fn relation_type(&self) -> &RelationType {
        &self.relation_type
    }
}
