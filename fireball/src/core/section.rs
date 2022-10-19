/// 섹션에 대한 정보가 들어있는 구조체
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub(crate) struct Section {
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

lazy_static::lazy_static! {
  /// 섹션 정보의 집합
  /// 가상주소(시작주소) : 섹션 정보
  static ref SECTIONS: std::sync::RwLock< std::collections::HashSet<Section>> = Default::default();
}

/// 섹션 정보를 빌드하는 함수
pub(crate) fn build_section(binary: &Vec<u8>) {
    let gl = goblin::Object::parse(binary).unwrap();
    let mut section_writer = SECTIONS.write().unwrap();

    match gl {
        goblin::Object::PE(gl) => {
            let sections = gl.sections;
            for section in sections {
                let name = section.name().unwrap().to_string();
                let real_name = if let Some(real_name) = section.real_name {
                    Some(real_name.to_string())
                } else {
                    None
                };
                let virtual_address = section.virtual_address as u64;
                let virtual_size = section.virtual_size as u64;
                let file_offset = section.pointer_to_raw_data as u64;
                let size_of_file = section.size_of_raw_data as u64;

                section_writer.insert(Section {
                    name,
                    real_name,
                    virtual_address,
                    virtual_size,
                    file_offset,
                    size_of_file,
                });
            }
        }
        _ => todo!(),
    }
}

/// 가상주소를 입력받아서 섹션 정보를 반환하는 함수
pub(crate) fn get_section_from_virtual_address(virtual_address: u64) -> Option<Section> {
    let section_reader = SECTIONS.read().unwrap();
    for section in section_reader.iter() {
        if section.virtual_address <= virtual_address
            && virtual_address <= section.virtual_address + section.virtual_size
        {
            return Some(section.clone());
        }
    }
    None
}

/// 파일 오프셋을 입력받아서 섹션 정보를 반환하는 함수
pub(crate) fn get_section_from_file_offset(file_offset: u64) -> Option<Section> {
    let section_reader = SECTIONS.read().unwrap();
    for section in section_reader.iter() {
        if section.file_offset <= file_offset
            && file_offset <= section.file_offset + section.size_of_file
        {
            return Some(section.clone());
        }
    }
    None
}

/// 섹션 이름을 받아 섹션 정보를 반환하는 함수
pub(crate) fn get_section_from_name(name: &str) -> Option<Section> {
    let section_reader = SECTIONS.read().unwrap();
    for section in section_reader.iter() {
        if section.name == name {
            return Some(section.clone());
        }
    }
    None
}
