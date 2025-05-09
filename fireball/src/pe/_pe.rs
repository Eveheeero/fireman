//! PE 구조체에 대한 구현이 담겨있는 모듈

use super::Pe;
use crate::core::{Address, Blocks, PreDefinedOffset, PreDefinedOffsets, Sections};
use capstone::prelude::BuildsCapstone;

impl Pe {
    /// 바이너리를 기반으로 PE 구조체를 생성한다.
    pub(crate) fn new(path: Option<String>, binary: Vec<u8>) -> Self {
        // 1. 섹션정보 생성
        // 2. 캡스톤 객체 생성
        // 3. 바이너리에 이미 지정되어있는 정보 생성

        // 공통적으로 사용되는 객체 생성
        let gl = goblin::pe::PE::parse(&binary).unwrap();

        // 바이너리 전체에 대한 섹션정보 생성
        let sections = Sections::new();
        sections.build_all(&binary);

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
        let defined = {
            let defined = PreDefinedOffsets::new();

            let imports = gl.imports;
            let exports = gl.exports;

            for import in imports {
                let name = import.name.to_string();
                let offset = import.offset as u64;

                defined.insert(PreDefinedOffset {
                    name,
                    address: Address::from_virtual_address(&sections, offset),
                });
            }

            for export in exports {
                let name = if let Some(name) = export.name {
                    name.to_string()
                } else {
                    format!("0x{:x}", export.offset.unwrap())
                };
                let offset = export.offset.unwrap() as u64;

                defined.insert(PreDefinedOffset {
                    name,
                    address: Address::from_virtual_address(&sections, offset),
                });
            }

            defined
        };

        Pe {
            entry: Address::from_virtual_address(&sections, gl.entry as u64),
            path,
            binary,
            capstone,
            defined,
            sections,
            blocks: Blocks::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn inspect_blocks(&self) -> std::sync::Arc<Blocks> {
        self.blocks.clone()
    }
    pub fn entry(&self) -> &Address {
        &self.entry
    }
}
