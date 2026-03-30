//! Modules that implement the `Fire` trait for the `Elf` struct.

mod analyze_all;
mod analyze_block;
mod analyze_from_entry;
mod analyze_from_file_offset;
mod analyze_from_virtual_address;

use super::Elf;
use crate::{
    core::{Address, Block, Blocks, Fire, FireRaw, PreDefinedOffsets, Relations, Sections},
    ir::analyze::global_recovery::recover_globals,
    prelude::DecompileError,
};
use std::sync::Arc;

impl Fire for Elf {
    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    fn get_binary(&self) -> &Vec<u8> {
        &self.binary
    }

    fn decompile_all(&self) -> Result<String, DecompileError> {
        let blocks = self.analyze_all()?;
        self.seed_recovered_globals(&blocks);
        Ok(
            crate::ir::analyze::generate_ast_with_pre_defined_symbols(blocks, self.get_defined())?
                .optimize(None)?
                .print(None),
        )
    }

    fn decompile_from_entry(&self) -> Result<String, DecompileError> {
        let block = self.analyze_from_entry()?;
        self.seed_recovered_globals(std::slice::from_ref(&block));
        Ok(
            crate::ir::analyze::generate_ast_with_pre_defined_symbols([block], self.get_defined())?
                .optimize(None)?
                .print(None),
        )
    }

    fn decompile_from_file_offset(&self, address: u64) -> Result<String, DecompileError> {
        let block = self.analyze_from_file_offset(address)?;
        self.seed_recovered_globals(std::slice::from_ref(&block));
        Ok(
            crate::ir::analyze::generate_ast_with_pre_defined_symbols([block], self.get_defined())?
                .optimize(None)?
                .print(None),
        )
    }

    fn decompile_from_virtual_address(&self, address: u64) -> Result<String, DecompileError> {
        let block = self.analyze_from_virtual_address(address)?;
        self.seed_recovered_globals(std::slice::from_ref(&block));
        Ok(
            crate::ir::analyze::generate_ast_with_pre_defined_symbols([block], self.get_defined())?
                .optimize(None)?
                .print(None),
        )
    }
}

impl Elf {
    fn seed_recovered_globals(&self, blocks: &[Arc<Block>]) {
        let globals = recover_globals(blocks, self.sections.as_ref());
        if globals.is_empty() {
            return;
        }

        let existing_addrs = {
            let reader = self.defined.get_reader();
            reader
                .iter()
                .map(|item| item.address.get_virtual_address())
                .collect::<std::collections::HashSet<_>>()
        };

        let mut inserted = 0usize;
        for global in globals {
            if existing_addrs.contains(&global.address) {
                continue;
            }

            self.defined.insert(crate::core::PreDefinedOffset {
                address: Address::from_virtual_address(self.sections.as_ref(), global.address),
                name: format!("global_{:X}", global.address),
            });
            inserted += 1;
        }

        if inserted > 0 {
            tracing::debug!(
                inserted,
                "Recovered global variable names seeded into predefined offsets"
            );
        }
    }
}

impl FireRaw for Elf {
    fn analyze_all(&self) -> Result<Vec<Arc<Block>>, DecompileError> {
        self._analyze_all()
    }

    fn analyze_from_entry(&self) -> Result<Arc<Block>, DecompileError> {
        self._analyze_from_entry()
    }

    fn analyze_from_file_offset(&self, address: u64) -> Result<Arc<Block>, DecompileError> {
        self._analyze_from_file_offset(address)
    }

    fn analyze_from_virtual_address(&self, address: u64) -> Result<Arc<Block>, DecompileError> {
        self._analyze_from_virtual_address(address)
    }

    fn analyze_block(&self, address: &Address) -> Result<Arc<Block>, DecompileError> {
        self._analyze_block(address)
    }

    fn get_sections(&self) -> Arc<Sections> {
        self.sections.clone()
    }

    fn get_defined(&self) -> Arc<PreDefinedOffsets> {
        self.defined.clone()
    }

    fn get_blocks(&self) -> Arc<Blocks> {
        self.blocks.clone()
    }

    fn get_relations(&self) -> Arc<Relations> {
        self.relations.clone()
    }
}
