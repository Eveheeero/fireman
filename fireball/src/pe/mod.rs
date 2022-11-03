use std::{pin::Pin, sync::Arc};

use crate::core::{PreDefinedOffsets, Relations, Sections};

/// PE파일 파서
pub struct PE {
    /// 파일 경로
    path: Option<String>,
    /// 바이너리
    binary: Vec<u8>,
    /// 캡스톤 엔진
    capstone: Pin<Box<capstone::Capstone>>,

    /// 파일 내부에서 이미 지정된 데이터
    defined: Arc<PreDefinedOffsets>,
    /// 섹션에 대한 정보를 담고 있는 데이터
    sections: Arc<Sections>,
    /// 여러 섹션에 대한 연관 정보를 담고 있는 데이터
    relations: Arc<Relations>,
}

/// PE 구조체에 대한 구현이 담겨있는 모듈
mod _pe;

/// 코어 트레이트에 대한 구현이 담겨있는 모듈
mod fire;

/// 출력에 대한 구현이 담겨있는 모듈
mod fmt;

/// 어셈블리 파싱 모듈
mod asm;
