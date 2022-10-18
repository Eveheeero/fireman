use super::PE;

use capstone::prelude::BuildsCapstone;

impl PE {
    /// 바이너리를 기반으로 PE 구조체를 생성한다.
    pub(crate) fn new(path: Option<String>, binary: Vec<u8>) -> Self {
        // 1. 캡스톤 객체 생성
        // 2. 바이너리에 이미 지정되어있는 정보 생성

        // 공통적으로 사용되는 객체 생성
        let gl = goblin::pe::PE::parse(&binary).unwrap();

        // 캡스톤 객체 생성
        let capstone = {
            // 바이너리를 기반으로 86x64인지 확인한다.
            let is_86 = !gl.is_64;

            // 캡스톤 객체 생성
            let capstone = capstone::Capstone::new()
                .x86()
                .mode(if is_86 {
                    capstone::arch::x86::ArchMode::Mode32
                } else {
                    capstone::arch::x86::ArchMode::Mode64
                })
                .build()
                .unwrap();

            Box::pin(capstone)
        };

        // 바이너리에 이미 지정되어있는 정보 생성
        let defined = Default::default();

        PE {
            path,
            binary,
            capstone,
            defined,
        }
    }
}
