use std::sync::Arc;

use super::Address;

/// 코드 블럭과 다른 블럭과의 연관을 나타낸다.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Relation {
    from: Address,
    to: Address,
}

impl Relation {
    /// 새로운 연관을 생성한다.
    pub fn new(from: Address, to: Address) -> Arc<Self> {
        Arc::new(Self { from, to })
    }
}
