//! PE파일에 대한 구조체가 담겨있는 모듈

mod _pe;
mod asm;
mod block;
mod fire;
mod fmt;

use crate::core::{Address, Blocks, PreDefinedOffsets, Relations, Sections};
use std::{pin::Pin, sync::Arc};

/// PE파일 파서
pub struct Pe {
    /// 엔트리 주소
    entry: Address,
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
    /// 블럭에 대한 정보를 담고 있는 데이터
    blocks: Arc<Blocks>,
    /// 블럭의 연결에 대한 정보를 담고 잇는 데이터
    relations: Arc<Relations>,
}
