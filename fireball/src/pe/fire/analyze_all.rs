use super::Pe;
use crate::{core::Block, prelude::DecompileError, Fire};
use std::sync::Arc;

impl Pe {
    pub(super) fn _analyze_all(&self) -> Result<Vec<Arc<Block>>, DecompileError> {
        let entry = self.entry();
        let mut queue = Vec::new();
        queue.push(entry.clone());
        let mut result = Vec::new();
        while let Some(address) = queue.pop() {
            let block = self.analyze_block(&address)?;
            result.push(block.clone());
            let connected_to = block.get_connected_to();
            for connected_to in connected_to.iter() {
                if let Some(address) = connected_to.to() {
                    queue.push(address);
                }
            }
        }
        Ok(result)
    }
}
