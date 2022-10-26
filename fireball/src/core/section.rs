#![allow(dead_code)]

/// 섹션에 대한 정보가 들어있는 구조체
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
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

unsafe impl Send for Section {}

impl std::fmt::Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#X} - {:#X}",
            self.virtual_address,
            self.virtual_address + self.virtual_size
        )
    }
}

/// 테스트 모듈
#[cfg(test)]
mod tests {
    use super::Section;

    /// Section구조체를 Display로 출력했을 때 제대로 출력되는지 확인하기 위한 테스트
    #[test]
    fn display_test() {
        let section = Section {
            id: 10,
            name: String::from("test"),
            real_name: Some("TestSection".to_owned()),
            virtual_address: 0x1000,
            virtual_size: 0x2A00,
            file_offset: 0x30B0,
            size_of_file: 0x400C,
        };

        assert_eq!(format!("{}", section), "0x1000 - 0x3A00");
    }
}
