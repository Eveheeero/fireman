//! Module that contains structure that contains all section information
//!
//!
//! 프로그램의 모든 섹션 정보를 가지고 있는 구조체를 정의하는 모듈

use super::Section;
use std::sync::Arc;

/// Struct that holding all section's information.
/// Data in this struct is immutable, only can write when start analysis. (By build_all method)
///
///
/// 모든 섹션을 담고 있는 구조체
/// 해당 구조체 안에 있는 내용은 분석을 시작할 때 생성되며, 이후에는 변경되지 않는다. (build_all 메소드에서만 생성된다.)
pub struct Sections {
    /// 섹션 정보의 집합
    /// 가상주소(시작주소) : 섹션 정보
    data: std::sync::RwLock<std::collections::HashSet<Arc<Section>>>,
}

impl Sections {
    /// Create new Sections struct.
    ///
    ///
    /// 새로운 Sections 구조체를 생성한다.
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            data: Default::default(),
        })
    }

    /// Build all section struct from binary.
    /// This method is only called when start analysis.
    ///
    ///
    /// 바이너리를 파싱해 섹션 구조체를 생성하는 함수
    /// 분석을 시작할 때 한 번만 호출된다.
    pub(crate) fn build_all(&self, binary: &[u8]) {
        // golbin으로부터 모든 바이너리를 파싱해 섹션 정보를 가져온다.
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

    /// Function that return section by virtual address.
    /// It calcs section's from and to address, and return section if virtual address is in range.
    ///
    ///
    /// 가상주소를 입력받아서 섹션 정보를 반환하는 함수
    /// 섹션의 시작주소와 끝주소를 계산하여, 가상주소가 해당 범위에 속하면 섹션 정보를 반환한다.
    #[allow(clippy::wrong_self_convention)]
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

    /// Function that return section by file offset.
    /// It calcs section's from and to offset, and return section if file offset is in range.
    ///
    ///
    /// 파일 오프셋을 입력받아서 해당 오프셋이 속한 섹션 정보를 반환하는 함수
    /// 섹션의 파일 시작 오프셋과 끝 오프셋을 계산하여, 파일 오프셋이 해당 범위에 속하면 섹션 정보를 반환한다.
    #[allow(clippy::wrong_self_convention)]
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

    /// Function that return section by section name.
    ///
    ///
    /// 섹션 이름을 받아, 해당 이름을 가진 섹션 정보를 반환하는 함수
    #[allow(clippy::wrong_self_convention)]
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
