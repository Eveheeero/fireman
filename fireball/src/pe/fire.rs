use super::PE;
use crate::core::Fire;

impl Fire for PE {
    fn from_path(path: &str) -> Result<PE, Box<dyn std::error::Error>> {
        let binary = std::fs::read(path)?;
        Ok(PE::new(Some(path.to_owned()), binary))
    }

    fn from_binary(binary: Vec<u8>) -> Result<PE, Box<dyn std::error::Error>> {
        Ok(PE::new(None, binary))
    }

    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    fn get_binary(&self) -> &Vec<u8> {
        &self.binary
    }
}
