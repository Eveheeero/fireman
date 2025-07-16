//! Module defining a structure that collects "Block"s resulting from program analysis

use crate::core::{
    Address, Block, Instruction, Relation, RelationType, Relations, relation::DestinationType,
};
use std::sync::Arc;

/// A structure that manages IR-level blocks
///
/// Use this structure to create new blocks and retrieve existing ones.
pub struct Blocks {
    /// Actual block data storage
    data: std::sync::RwLock<std::collections::HashSet<Arc<Block>>>,
    /// Block relations
    relations: Arc<Relations>,
}

#[derive(Clone)]
pub(crate) struct BlockRelationInformation {
    pub(crate) destination: Option<Address>,
    pub(crate) destination_type: DestinationType,
    pub(crate) relation_type: RelationType,
}

impl Blocks {
    /// Creates the block storage structure.
    ///
    /// ### Arguments
    /// - `relations: Arc<Relations>`: A structure that stores relations between blocks
    ///
    /// ### Returns
    /// - `Arc<Self>`: The created block store structure
    pub(crate) fn new(relations: Arc<Relations>) -> Arc<Self> {
        Arc::new(Self {
            data: Default::default(),
            relations,
        })
    }

    /// Generates a new block in the store.
    ///
    /// ### Arguments
    /// - `start_address: Address`: The start address of the block
    /// - `block_size: Option<u64>`: The size of the block
    /// - `connected_to: &[BlockRelationInformation]`: the relations to other blocks
    /// - `name: Option<String>`: The name of the block
    /// - `instructions: Arc<[Instruction]>`: The instructions of the block
    ///
    /// ### Returns
    /// - `Arc<Block>`: The generated block
    pub(crate) fn generate_block(
        &self,
        start_address: Address,
        block_size: Option<u64>,
        connected_to: &[BlockRelationInformation],
        name: Option<String>,
        instructions: Arc<[Instruction]>,
    ) -> Arc<Block> {
        /* Before acquiring the lock, check relations targeting this block */
        let connected_from: Vec<_> = {
            self.data
                .read()
                .unwrap()
                .iter()
                .flat_map(|block| block.get_connected_to().clone())
                .filter(|relation| relation.to().as_ref() == Some(&start_address))
                .collect()
        };

        /* Before acquiring the lock, create relations (requires access to the store during creation) */
        let connected_to: Vec<_> = connected_to
            .iter()
            .map(|connected_to| {
                let connected_block = connected_to
                    .destination
                    .as_ref()
                    .and_then(|connected_to| self.get_by_start_address(connected_to));
                (connected_to, connected_block)
            })
            .collect();

        /* Acquire the lock on the store */
        let blocks_writer = &mut self.data.write().unwrap();

        /* Create the new block with the provided information */
        let new_block = Block::new(
            blocks_writer.len(),
            name,
            start_address,
            block_size,
            instructions,
        );

        for connected_from in connected_from {
            new_block.add_connected_from(connected_from);
        }

        for (relation, connected_block) in connected_to {
            let connected_address = relation.destination.clone();
            let relation = Relation::new(
                new_block.get_id(),
                connected_address.clone(),
                relation.destination_type,
                relation.relation_type,
            );
            self.relations.add_relation(relation.clone());
            new_block.add_connected_to(relation.clone());
            if let Some(connected_block) = connected_block {
                connected_block.add_connected_from(relation);
            }
        }

        /* Insert the new block into the store */
        blocks_writer.insert(new_block.clone());

        /* Return the new block */
        new_block
    }

    /// Returns the block with the given start address.
    ///
    /// ### Arguments
    /// - `address: &Address`: The target address
    ///
    /// ### Returns
    /// - `Option<Arc<Block>>`: The found block, if any
    pub fn get_by_start_address(&self, address: &Address) -> Option<Arc<Block>> {
        /* Acquire a read lock on the store */
        let blocks_reader = &self.data.read().unwrap();

        /* Inspect the store's data */
        blocks_reader
            .iter()
            .find(|block| block.get_start_address() == address)
            .map(Arc::clone)
    }

    /// Returns the blocks containing the given address.
    ///
    /// ### Arguments
    /// - `address: &Address`: The target address
    ///
    /// ### Returns
    /// - `Vec<Arc<Block>>`: The found blocks
    pub fn get_by_containing_address(&self, address: &Address) -> Vec<Arc<Block>> {
        /* Acquire a read lock on the store */
        let blocks_reader = &self.data.read().unwrap();

        /* Inspect the store's data */
        blocks_reader
            .iter()
            .filter(|block| block.contains(address))
            .map(Arc::clone)
            .collect()
    }

    /// Retrieves a block by its ID.
    ///
    /// ### Arguments
    /// - `id: usize`: The block ID
    ///
    /// ### Returns
    /// - `Option<Arc<Block>>`: The block corresponding to the ID, if any
    pub fn get_by_block_id(&self, id: usize) -> Option<Arc<Block>> {
        /* Acquire a read lock on the store */
        let blocks_reader = &self.data.read().unwrap();

        /* Inspect the store's data */
        blocks_reader
            .iter()
            .find(|block| block.get_id() == id)
            .map(Arc::clone)
    }

    /// Returns all blocks.
    ///
    /// ### Returns
    /// - `Vec<Arc<Block>>`: All blocks
    pub fn get_all(&self) -> Vec<Arc<Block>> {
        /* Acquire a read lock on the store */
        let blocks_reader = &self.data.read().unwrap();
        blocks_reader.iter().map(Arc::clone).collect()
    }
}

impl std::fmt::Debug for BlockRelationInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockRelationInformation")
            .field(
                "destination",
                &self.destination.as_ref().map(|x| x.to_string()),
            )
            .field("destination_type", &self.destination_type)
            .field("relation_type", &self.relation_type)
            .finish()
    }
}
