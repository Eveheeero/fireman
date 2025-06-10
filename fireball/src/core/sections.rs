//! Module that contains structure that contains all section information

use super::Section;
use std::sync::Arc;

/// Struct that holding all section's information.
/// Data in this struct is immutable, only can write when start analysis. (By build_all method)
#[derive(Default)]
pub struct Sections {
    /// Set of section information
    data: std::sync::RwLock<std::collections::HashSet<Arc<Section>>>,
}

impl Sections {
    /// Create new Sections struct.
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            data: Default::default(),
        })
    }

    /// Build all section struct from binary.
    /// This method is only called when start analysis.
    pub(crate) fn build_all(&self, binary: &[u8]) {
        // Parse the binary using goblin
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
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn from_virtual_address(&self, virtual_address: u64) -> Option<Arc<Section>> {
        let section_reader = &self.data.read().unwrap();
        // Iterate over all sections
        for section in section_reader.iter() {
            // Calculate section start and end for virtual address
            let section_start_virtual = section.virtual_address;
            let section_end_virtual = section.virtual_address + section.virtual_size;

            // Return the section information if the virtual address is within the section's range
            if section_start_virtual <= virtual_address && virtual_address < section_end_virtual {
                return Some(section.clone());
            }
        }
        None
    }

    /// Function that return section by file offset.
    /// It calcs section's from and to offset, and return section if file offset is in range.
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn from_file_offset(&self, file_offset: u64) -> Option<Arc<Section>> {
        let section_reader = &self.data.read().unwrap();
        // Iterate over all sections
        for section in section_reader.iter() {
            // Calculate section start and end for file offset
            let section_start_file = section.file_offset;
            let section_end_file = section.file_offset + section.size_of_file;

            // Return the section information if the file offset is within the section's range
            if section_start_file <= file_offset && file_offset < section_end_file {
                return Some(section.clone());
            }
        }
        None
    }

    /// Function that return section by section name.
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn from_name(&self, name: &str) -> Option<Arc<Section>> {
        let section_reader = &self.data.read().unwrap();
        // Iterate over all sections
        for section in section_reader.iter() {
            // Return the section information if the section name matches
            if section.name == name {
                return Some(section.clone());
            }
        }
        None
    }

    /// Add a section to the sections collection
    pub fn add_section(&self, section: Section) -> usize {
        let mut section_writer = self.data.write().unwrap();
        let id = section_writer.len();
        section_writer.insert(Arc::new(section));
        id
    }
}
