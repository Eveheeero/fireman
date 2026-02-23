use super::Pe;
use crate::{FireRaw, core::Block, prelude::DecompileError};
use std::{collections::HashSet, sync::Arc};

impl Pe {
    pub(super) fn _analyze_all(&self) -> Result<Vec<Arc<Block>>, DecompileError> {
        let entry = self.entry();
        let mut queue = Vec::new();
        queue.push(entry.clone());
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        while let Some(address) = queue.pop() {
            let current_va = address.get_virtual_address();
            if !visited.insert(current_va) {
                continue;
            }
            let block = self.analyze_block(&address)?;
            result.push(block.clone());
            let connected_to = block.get_connected_to();
            for connected_to in connected_to.iter() {
                if let Some(address) = connected_to.to() {
                    if visited.contains(&address.get_virtual_address()) {
                        continue;
                    }
                    queue.push(address);
                }
            }
        }
        Ok(result)
    }
}
