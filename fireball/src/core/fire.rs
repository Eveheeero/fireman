use std::sync::Arc;

use super::{Address, Block, Relation, Sections};

/// 파서 모듈에 대한 트레이트
pub trait Fire {
    /// 파일 경로를 기반으로 파서 객체를 생성한다.
    fn from_path(path: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    /// 바이너리를 기반으로 파서 객체를 생성한다.
    fn from_binary(binary: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    /// 파일 경로를 반환한다.
    fn get_path(&self) -> Option<String>;

    /// 바이너리를 반환한다.
    fn get_binary(&self) -> &Vec<u8>;

    /// 파일의 모든 내용을 분석한다.
    fn decom_all(&self) -> Result<(), Box<dyn std::error::Error>>;

    /// 엔트리포인트부터 분석한다.
    fn decom_from_entry(&self) -> Result<(), Box<dyn std::error::Error>>;

    /// 파일 오프셋을 기반으로 분석한다.
    fn decom_from_file_offset(&self, address: u64) -> Result<(), Box<dyn std::error::Error>>;

    /// 가상 주소를 기반으로 분석한다.
    fn decom_from_virtual_address(&self, address: u64) -> Result<(), Box<dyn std::error::Error>>;

    /// 모든 섹션 정보를 가져온다.
    fn get_sections(&self) -> Arc<Sections>;

    /// 파일 오프셋과 이어진 블록 정보를 가져온다.
    fn parse_block(&self, address: Address) -> (Arc<Block>, Option<Arc<Relation>>);
}
