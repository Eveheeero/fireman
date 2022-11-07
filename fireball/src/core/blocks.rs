use std::sync::Arc;

use super::{Address, Block, Section};

pub struct Blocks {
    data: std::sync::RwLock<std::collections::HashSet<Arc<Block>>>,
}

impl Blocks {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            data: Default::default(),
        })
    }

    pub(crate) fn new_block(
        &self,
        section: Arc<Section>,
        start_address_virtual: Address,
        end_address_virtual: Option<Address>,
        name: Option<String>,
    ) -> Arc<Block> {
        let blocks_writer = &mut self.data.write().unwrap();

        let new_block = Block::new(
            blocks_writer.len(),
            name,
            start_address_virtual,
            end_address_virtual,
            section,
        );

        blocks_writer.insert(new_block.clone());

        new_block
    }

    pub(crate) fn find_from_address(&self, address: Address) -> Option<Arc<Block>> {
        let blocks_reader = &self.data.read().unwrap();

        blocks_reader
            .iter()
            .find(|block| block.get_start_address_virtual() == &address)
            .map(Arc::clone)
    }
}
