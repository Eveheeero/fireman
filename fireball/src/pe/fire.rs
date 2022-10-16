use capstone::prelude::BuildsCapstone;

use super::PE;
use crate::core::Fire;

impl Fire for PE {
    fn from_path(path: &str) -> Result<PE, Box<dyn std::error::Error>> {
        let binary = std::fs::read(path)?;
        let is_x64 = goblin::pe::PE::parse(binary.as_slice())?.is_64;
        let capstone = if is_x64 {
            capstone::Capstone::new()
                .x86()
                .mode(capstone::arch::x86::ArchMode::Mode64)
                .build()
                .unwrap()
        } else {
            capstone::Capstone::new()
                .x86()
                .mode(capstone::arch::x86::ArchMode::Mode32)
                .build()
                .unwrap()
        };

        Ok(PE {
            path: Some(path.to_string()),
            binary,
            capstone: Box::pin(capstone),
            defined: Default::default(),
        })
    }

    fn from_binary(binary: Vec<u8>) -> Result<PE, Box<dyn std::error::Error>> {
        let is_x64 = goblin::pe::PE::parse(binary.as_slice())?.is_64;
        let capstone = if is_x64 {
            capstone::Capstone::new()
                .x86()
                .mode(capstone::arch::x86::ArchMode::Mode64)
                .build()
                .unwrap()
        } else {
            capstone::Capstone::new()
                .x86()
                .mode(capstone::arch::x86::ArchMode::Mode32)
                .build()
                .unwrap()
        };

        Ok(PE {
            path: None,
            binary,
            capstone: Box::pin(capstone),
            defined: Default::default(),
        })
    }

    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    fn get_binary(&self) -> &Vec<u8> {
        &self.binary
    }
}
