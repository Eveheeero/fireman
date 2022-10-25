use std::sync::{Arc, RwLock, RwLockReadGuard};

use super::PreDefinedOffset;

/// 파일 내부에 지정되어있는 데이터에 대한 구조체가 들어있는 구조체
pub struct PreDefinedOffsets {
    data: RwLock<Vec<PreDefinedOffset>>,
}

impl PreDefinedOffsets {
    /// 미리 지정된 데이터를 저장하는 구조체를 생성한다.
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            data: Default::default(),
        })
    }

    pub(crate) fn insert(&self, data: PreDefinedOffset) {
        self.data.write().unwrap().push(data);
    }

    /// 내부 데이터의 반복자를 반환하는 함수
    pub fn get_reader(&self) -> RwLockReadGuard<Vec<PreDefinedOffset>> {
        self.data.read().unwrap()
    }
}
