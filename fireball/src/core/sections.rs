use std::sync::Arc;

use super::Section;

/// 모든 섹션의 데이터가 담겨있는 구조체
pub struct Sections {
    /// 섹션 정보의 집합
    /// 가상주소(시작주소) : 섹션 정보
    data: std::sync::RwLock<std::collections::HashSet<Arc<Section>>>,
}

impl Sections {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            data: Default::default(),
        })
    }

    /// 섹션 정보를 빌드하는 함수
    ///
    /// 바이너리 파일의 모든 바이트를 읽어 섹션 정보를 로드해 저장한다.
    pub(crate) fn build_all(&self, binary: &[u8]) {
        let gl = goblin::Object::parse(binary).unwrap();
        let section_writer = &mut self.data.write().unwrap();

        match gl {
            goblin::Object::PE(gl) => {
                let sections = gl.sections;
                for section in sections {
                    let id = section_writer.len();
                    let name = section.name().unwrap().to_string();
                    let real_name = section.real_name;
                    let virtual_address = section.virtual_address as u64;
                    let virtual_size = section.virtual_size as u64;
                    let file_offset = section.pointer_to_raw_data as u64;
                    let size_of_file = section.size_of_raw_data as u64;

                    section_writer.insert(Arc::new(Section {
                        id,
                        name,
                        real_name,
                        virtual_address,
                        virtual_size,
                        file_offset,
                        size_of_file,
                    }));
                }
            }
            _ => todo!(),
        }
    }

    /// 가상주소를 입력받아서 섹션 정보를 반환하는 함수
    pub(crate) fn from_virtual_address(&self, virtual_address: u64) -> Option<Arc<Section>> {
        let section_reader = &self.data.read().unwrap();
        // 모든 섹션에 대한 검사
        for section in section_reader.iter() {
            // 가상주소에 대한 섹션의 시작과 끝
            let section_start_virtual = section.virtual_address;
            let section_end_virtual = section.virtual_address + section.virtual_size;

            // 가상주소가 섹션의 범위 안에 있으면 섹션 정보를 반환
            if section_start_virtual <= virtual_address && virtual_address < section_end_virtual {
                return Some(section.clone());
            }
        }
        None
    }

    /// 파일 오프셋을 입력받아서 해당 오프셋이 속한 섹션 정보를 반환하는 함수
    pub(crate) fn from_file_offset(&self, file_offset: u64) -> Option<Arc<Section>> {
        let section_reader = &self.data.read().unwrap();
        // 모든 섹션에 대한 검사
        for section in section_reader.iter() {
            // 파일 오프셋에 대한 섹션의 시작과 끝
            let section_start_file = section.file_offset;
            let section_end_file = section.file_offset + section.size_of_file;

            // 파일 오프셋이 섹션의 범위 안에 있으면 섹션 정보를 반환
            if section_start_file <= file_offset && file_offset < section_end_file {
                return Some(section.clone());
            }
        }
        None
    }

    /// 섹션 이름을 받아, 해당 이름을 가진 섹션 정보를 반환하는 함수
    pub(crate) fn from_name(&self, name: &str) -> Option<Arc<Section>> {
        let section_reader = &self.data.read().unwrap();
        // 모든 섹션에 대한 검사
        for section in section_reader.iter() {
            // 섹션 이름이 일치하면 섹션 정보를 반환
            if section.name == name {
                return Some(section.clone());
            }
        }
        None
    }
}
