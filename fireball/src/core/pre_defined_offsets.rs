//! 프로그램에 대한 "PreDefinedOffset"를 모아두는 구조체를 정의하는 모듈

use super::PreDefinedOffset;
use std::sync::{Arc, RwLock, RwLockReadGuard};

/// 파일 내부에 미리 지정되어있는 정보를 관리하는 구조체
pub struct PreDefinedOffsets {
    /// 내부 데이터
    data: RwLock<Vec<PreDefinedOffset>>,
}

impl PreDefinedOffsets {
    /// 미리 지정된 데이터를 저장하는 구조체를 생성한다.
    ///
    /// ### Returns
    /// - `Arc<Self>`: 미리 지정되어있는 정보를 관리하는 컨테이너
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            data: Default::default(),
        })
    }

    /// 미리 지정된 정보를 저장소에 추가한다.
    ///
    /// ### Arguments
    /// - `data: PreDefinedOffset`: 파일 내부에 미리 지정되어있는 "주소에 대한 정보"
    pub(crate) fn insert(&self, data: PreDefinedOffset) {
        self.data.write().unwrap().push(data);
    }

    /// 내부 데이터의 리더를 가져온다.
    ///
    /// ### Returns
    /// - `RwLockReadGuard<Vec<PreDefinedOffset>>`: 내부 데이터의 리더
    pub fn get_reader(&self) -> RwLockReadGuard<Vec<PreDefinedOffset>> {
        self.data.read().unwrap()
    }
}
