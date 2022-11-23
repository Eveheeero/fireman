use std::sync::Arc;

use super::{Address, Block, Relation, Sections};

/// ## Main Decompile Trait
pub trait Fire {
    /// 파일 경로를 기반으로 파서 객체를 생성한다.
    ///
    /// ### Arguments
    /// - path - 읽어올 파일 경로
    fn from_path(path: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    /// 바이너리를 기반으로 파서 객체를 생성한다.
    ///
    /// ### Arguments
    /// - binary - 파싱할 바이너리 데이터
    fn from_binary(binary: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    /// 파일 경로를 반환한다.
    ///
    /// ### Returns
    /// - Option\<String\> - 파일 경로
    ///
    /// ### Note
    /// - 해당 함수는 아무런 추가적인 연산을 수행하지 않는다.
    fn get_path(&self) -> Option<String>;

    /// 바이너리를 반환한다.
    ///
    /// ### Returns
    /// - &Vec\<u8\> - 원본 파일 바이너리 데이터
    ///
    /// ### Note
    /// - 해당 함수는 아무런 추가적인 연산을 수행하지 않는다.
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
    ///
    /// ### Returns
    /// - Arc\<Sections\> - 구조체 내부에 저장되어있는 섹션 정보의 모음집
    ///
    /// ### Note
    /// - 해당 함수는 아무런 추가적인 연산을 수행하지 않는다.
    fn get_sections(&self) -> Arc<Sections>;

    /// 파일 오프셋과 이어진 블록 정보를 가져온다.
    ///
    /// ### Arguments
    /// - adress - 파싱할 블록의 시작 주소
    ///
    /// ### Returns
    /// - Arc\<Block\> - 파싱된 블록 객체
    ///
    /// ### Note
    /// - 해당 함수는 인자로 주어진 주소로부터 어떤 주소까지 점프 없이 수행되는지 파악 후 블록을 생성하여 반환한다.
    /// - 해당 함수의 내부에서, 블록이 어느 주소와 연결되어있는지 설정되며, 대상 주소에 해당하는 블록의 connected_from이 설정된다.
    fn parse_block(&self, address: Address) -> Arc<Block>;
}
