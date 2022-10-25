#![allow(dead_code)]

/// 섹션에 대한 정보가 들어있는 구조체
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Section {
    /// 섹션 식별코드
    pub(crate) id: usize,
    /// .text와 같은 이름
    pub(crate) name: String,
    /// 섹션의 이름
    pub(crate) real_name: Option<String>,
    /// 섹션의 가상 시작 주소
    /// 0x1000
    pub(crate) virtual_address: u64,
    /// 섹션의 가상 주소의 크기
    /// 0x1000 ~ 0x2000 까지의 주소를 가진 섹션이면 0x1000이 사이즈가 된다.
    pub(crate) virtual_size: u64,
    /// 섹션에 해당하는 파일의 오프셋
    pub(crate) file_offset: u64,
    /// 섹션에 해당하는 파일의 크기
    pub(crate) size_of_file: u64,
}
