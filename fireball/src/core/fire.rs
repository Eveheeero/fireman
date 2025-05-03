//! 디컴파일 공통 기능을 담당하는 "Fire"구조체에 대해 정의하는 모듈

use super::Sections;
use crate::{
    core::{Address, PreDefinedOffsets},
    prelude::{DecompileError, IoError},
};
use std::sync::Arc;

/// ## Main Decompile Trait
/// 해당 Trait는 디컴파일러를 작성할 때, 해당라이브러리에서 필요한 기능들을 정의해준다.
pub trait Fire {
    /// 파일 경로를 기반으로 파서 객체를 생성한다.
    ///
    /// ### Arguments
    /// - `path: &str` - 읽어올 파일 경로
    ///
    /// ### Returns
    /// - `Result<Self, IoError>` - 파일을 읽는것을 성공할 시 파서 객체를 반환하며, 파일을 읽을 수 없을 시 에러를 반환한다.
    fn from_path(path: &str) -> Result<Self, IoError>
    where
        Self: Sized;

    /// 바이너리를 기반으로 파서 객체를 생성한다.
    ///
    /// ### Arguments
    /// - `binary: Vec<u8>` - 파싱할 바이너리 데이터
    ///
    /// ### Returns
    /// - `Result<Self, IoError>` - 파서 객체를 반환하며, 항상 성공한다.
    fn from_binary(binary: Vec<u8>) -> Result<Self, IoError>
    where
        Self: Sized;

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
    /// - `Result<(), DecompileError>` - 디컴파일에 실패할 시 에러를 반환한다.
    ///
    /// ### Todo
    /// 해당 방법은 난독화에 대비하여, 분석을 마친 후, 이미 분석한 오프셋에서 일정 오프셋을 이동시켜 제대로 된 인스트럭션이 나오는지 확인하는 등으로 검사를 수행할 예정이다.
    /// 엔트리부터 분석을 시작한 후, 분석이 끝난 오프셋에서 일정 오프셋을 이동시킨 후 decom_from_file_offset등을 이용해 분석?
    fn decom_all(&self) -> Result<(), DecompileError>;

    /// 엔트리포인트부터 디컴파일을 수행한다.
    ///
    /// ### Returns
    /// - `Result<(), DecompileError>` - 디컴파일에 실패할 시 에러를 반환한다.
    fn decom_from_entry(&self) -> Result<(), DecompileError>;

    /// 주어진 파일 오프셋부터 블럭이 끝날 때까지 디컴파일을 수행한다.
    ///
    /// ### Arguments
    /// - `address: u64` - 분석을 시작할 파일 오프셋
    ///
    /// ### Returns
    /// - `Result<(), DecompileError>` - 디컴파일에 실패할 시 에러를 반환한다.
    fn decom_from_file_offset(&self, address: u64) -> Result<(), DecompileError>;

    /// 주어진 가상 주소부터 블럭이 끝날 때까지 디컴파일을 수행한다.
    ///
    /// ### Arguments
    /// - `address: u64` - 분석을 시작할 파일 오프셋
    ///
    /// ### Returns
    /// - `Result<(), DecompileError>` - 디컴파일에 실패할 시 에러를 반환한다.
    fn decom_from_virtual_address(&self, address: u64) -> Result<(), DecompileError>;

    /// 주어진 주소부터 블럭이 끝날 때까지 디컴파일을 수행한다.
    ///
    /// ### Arguments
    /// - `address: &Address` - 분석을 시작할 주소
    ///
    /// ### Returns
    /// - `Result<(), DecompileError>` - 디컴파일에 실패할 시 에러를 반환한다.
    fn decom_block(&self, address: &Address) -> Result<(), DecompileError>;

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
}
