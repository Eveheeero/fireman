use super::MachO;
use crate::{BinaryKind, FireRaw, core::Block, prelude::DecompileError};
use std::{
    collections::HashSet,
    sync::{Arc, atomic::Ordering},
};

impl MachO {
    pub(super) fn _analyze_all(&self) -> Result<Vec<Arc<Block>>, DecompileError> {
        let seeds = self.collect_analysis_seeds();
        if seeds.is_empty() {
            return Err(DecompileError::NoEntryPoint);
        }

        let first_va = seeds.first().map(|a| a.get_virtual_address()).unwrap_or(0);
        tracing::info!(
            entry_va = first_va,
            seed_count = seeds.len(),
            kind = ?self.kind,
            "Full-program block analysis started"
        );

        let mut queue: Vec<_> = seeds;
        let mut visited = HashSet::new();
        let mut result = Vec::new();

        while let Some(address) = queue.pop() {
            if self.cancel_token.load(Ordering::Relaxed) {
                tracing::warn!("Full-program block analysis cancelled");
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
            discovered.sort_unstable_by_key(|address| address.get_virtual_address());
            for address in discovered.into_iter().rev() {
                queue.push(address);
            }
        }

        tracing::info!(
            visited_blocks = visited.len(),
            analyzed_blocks = result.len(),
            "Full-program block analysis completed"
        );
        Ok(result)
    }

    /// Collect starting addresses for BFS traversal based on binary kind.
    fn collect_analysis_seeds(&self) -> Vec<crate::core::Address> {
        use crate::core::Address;

        match self.kind {
            BinaryKind::Executable => {
                vec![self.entry.clone()]
            }
            BinaryKind::SharedLibrary | BinaryKind::ObjectFile | BinaryKind::DebugInfo => {
                let mut seeds = Vec::new();
                let mut seen = HashSet::new();

                // Use symbols as seeds
                let reader = self.defined.get_reader();
                for def in reader.iter() {
                    let va = def.address.get_virtual_address();
                    if va != 0 && seen.insert(va) {
                        seeds.push(def.address.clone());
                    }
                }

                // Include entry if valid
                if self.entry.get_virtual_address() != 0 {
                    let va = self.entry.get_virtual_address();
                    if seen.insert(va) {
                        seeds.push(self.entry.clone());
                    }
                }

                // Fallback: executable section starts
                if seeds.is_empty() {
                    for section in self.sections.all() {
                        if section.is_executable() && seen.insert(section.virtual_address) {
                            seeds.push(Address::from_virtual_address(
                                &self.sections,
                                section.virtual_address,
                            ));
                        }
                    }
                }

                seeds
            }
        }
    }
}
