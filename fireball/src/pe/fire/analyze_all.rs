use super::Pe;
use crate::{FireRaw, core::Block, prelude::DecompileError};
use std::{
    collections::HashSet,
    sync::{Arc, atomic::Ordering},
};

impl Pe {
    pub(super) fn _analyze_all(&self) -> Result<Vec<Arc<Block>>, DecompileError> {
        let entry = self.entry();
        let entry_va = entry.get_virtual_address();
        tracing::info!(entry_va, "Full-program block analysis started");

        let mut queue = Vec::new();
        queue.push(entry.clone());
        let mut visited = HashSet::new();
        let mut result = Vec::new();

        while let Some(address) = queue.pop() {
            if self.cancel_token.load(Ordering::Relaxed) {
                tracing::warn!(entry_va, "Full-program block analysis cancelled");
                return Err(DecompileError::Unknown(Some(
                    "analysis cancelled".to_string(),
                )));
            }

            let current_va = address.get_virtual_address();
            if !visited.insert(current_va) {
                continue;
            }
            let block = self.analyze_block(&address)?;
            result.push(block.clone());
            let mut discovered = block
                .get_connected_to()
                .iter()
                .filter_map(|relation| relation.to())
                .filter(|address| !visited.contains(&address.get_virtual_address()))
                .collect::<Vec<_>>();
            // Keep traversal logs deterministic even when relation storage order varies.
            discovered.sort_unstable_by_key(|address| address.get_virtual_address());
            for address in discovered.into_iter().rev() {
                queue.push(address);
            }
        }

        tracing::info!(
            entry_va,
            visited_blocks = visited.len(),
            analyzed_blocks = result.len(),
            pending_queue = queue.len(),
            "Full-program block analysis completed"
        );
        Ok(result)
    }
}
