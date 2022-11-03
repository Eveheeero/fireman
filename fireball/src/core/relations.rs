use std::sync::Arc;

use super::Relation;

/// 코드 블럭에 대한 여러 연관 블럭을 저장하는 구조체
#[derive(Debug)]
pub struct Relations {
    data: Vec<Relation>,
}

impl Relations {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            data: Default::default(),
        })
    }
}
