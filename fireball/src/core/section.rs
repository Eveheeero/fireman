/// 섹션에 대한 정보가 들어있는 구조체
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub(crate) struct Section {
    pub(crate) name: String,
    pub(crate) base_addr: usize,
}

lazy_static::lazy_static! {
  /// 섹션 정보의 집합
  /// 가상주소(시작주소) : 섹션 정보
  pub(crate) static ref SECTIONS: std::sync::RwLock< std::collections::HashMap<u64, Section>> = Default::default();
}

pub(crate) fn build_section(binary: &Vec<u8>) {
    let gl = goblin::Object::parse(binary).unwrap();
    let mut section_writer = SECTIONS.write().unwrap();

    match gl {
        goblin::Object::PE(gl) => {
            let sections = gl.sections;
            for section in sections {
                let virtual_address = section.virtual_address as u64;
                let name = section.name().unwrap().to_owned();
                let base_addr = section.virtual_address as usize;
                section_writer.insert(virtual_address, Section { name, base_addr });
            }
        }
        _ => todo!(),
    }
}
