//! 디컴파일 공통 기능을 담당하는 "Fire"구조체에 대해 정의하는 모듈

use super::Sections;
use crate::{
    core::{Address, Block, Blocks, PreDefinedOffsets},
    prelude::DecompileError,
};
use std::sync::Arc;

/// ## Main Decompile Trait
/// 해당 Trait는 디컴파일러를 작성할 때, 해당라이브러리에서 필요한 기능들을 정의해준다.
pub trait Fire {
    /// 파일 경로를 반환한다.
    ///
    /// ### Returns
    /// - `Option<String>` - 파일 경로
    ///
    /// ### Note
    /// - 해당 함수는 아무런 추가적인 연산을 수행하지 않는다.
    fn get_path(&self) -> Option<String>;

    /// 바이너리를 반환한다.
    ///
    /// ### Returns
    /// - `&Vec<u8>` - 원본 파일 바이너리 데이터
    ///
    /// ### Note
    /// - 해당 함수는 아무런 추가적인 연산을 수행하지 않는다.
    fn get_binary(&self) -> &Vec<u8>;

    /// 파일의 전체 내용을 디컴파일한다.
    ///
    /// ### Returns
    /// - `Result<Vec<Arc<Block>>, DecompileError>` - 디컴파일 결과
    ///
    /// ### Todo
    /// 해당 방법은 난독화에 대비하여, 분석을 마친 후, 이미 분석한 오프셋에서 일정 오프셋을 이동시켜 제대로 된 인스트럭션이 나오는지 확인하는 등으로 검사를 수행할 예정이다.
    /// 엔트리부터 분석을 시작한 후, 분석이 끝난 오프셋에서 일정 오프셋을 이동시킨 후 decom_from_file_offset등을 이용해 분석?
    fn decom_all(&self) -> Result<Vec<Arc<Block>>, DecompileError>;

    /// 엔트리포인트부터 디컴파일을 수행한다.
    ///
    /// ### Returns
    /// - `Result<Arc<Block>, DecompileError>` - 디컴파일 결과
    fn decom_from_entry(&self) -> Result<Arc<Block>, DecompileError>;

    /// 주어진 파일 오프셋부터 블럭이 끝날 때까지 디컴파일을 수행한다.
    ///
    /// ### Arguments
    /// - `address: u64` - 분석을 시작할 파일 오프셋
    ///
    /// ### Returns
    /// - `Result<Arc<Block>, DecompileError>` - 디컴파일 결과
    fn decom_from_file_offset(&self, address: u64) -> Result<Arc<Block>, DecompileError>;

    /// 주어진 가상 주소부터 블럭이 끝날 때까지 디컴파일을 수행한다.
    ///
    /// ### Arguments
    /// - `address: u64` - 분석을 시작할 파일 오프셋
    ///
    /// ### Returns
    /// - `Result<Arc<Block>, DecompileError>` - 디컴파일 결과
    fn decom_from_virtual_address(&self, address: u64) -> Result<Arc<Block>, DecompileError>;

    /// 주어진 주소부터 블럭이 끝날 때까지 디컴파일을 수행한다.
    ///
    /// ### Arguments
    /// - `address: &Address` - 분석을 시작할 주소
    ///
    /// ### Returns
    /// - `Result<Arc<Block>, DecompileError>` - 디컴파일 결과
    fn decom_block(&self, address: &Address) -> Result<Arc<Block>, DecompileError>;

    /// 분석 후 나온 모든 섹션의 정보를 가져온다.
    ///
    /// ### Returns
    /// - `Arc<Sections>` - 디컴파일러 객체가 가지고 있는 섹션 정보를 관리하는 객체
    ///
    /// ### Note
    /// - 해당 함수는 아무런 추가적인 연산을 수행하지 않는다.
    fn get_sections(&self) -> Arc<Sections>;

    /// 미리 정의된 데이터를 가져온다.
    ///
    /// ### Returns
    /// - `Arc<PreDefinedOffsets>` - 바이너리 내부에 정의된 데이터
    fn get_defined(&self) -> Arc<PreDefinedOffsets>;
    /// 분석에 의해 생성된 데이터를 가져온다.
    ///
    /// ### Returns
    /// - `Arc<Blocks>` - 분석에 의해 생성된 블럭을 관리하는 객체
    fn get_blocks(&self) -> Arc<Blocks>;
}
