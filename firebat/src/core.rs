use crate::model::{Assembly, DecompileResult, Ir, KnownSectionData};
use fireball::{
    Fireball,
    core::{Address, Block, FireRaw},
};
use std::sync::Arc;

#[derive(Default)]
pub(crate) struct FirebatCore {
    path: Option<String>,
    fireball: Option<Fireball>,
}

impl FirebatCore {
    fn fireball(&self) -> Result<&Fireball, String> {
        self.fireball
            .as_ref()
            .ok_or_else(|| "Fireball is None".to_string())
    }

    pub(crate) fn open_file(&mut self, path: &str) -> Result<(), String> {
        self.path = Some(path.to_owned());
        let fireball = Fireball::from_path(path).map_err(|error| error.to_string())?;
        self.fireball = Some(fireball);
        Ok(())
    }

    pub(crate) fn analyze_section(&self, address: &str) -> Result<Vec<KnownSectionData>, String> {
        if address.is_empty() {
            return self.analyze_section_from_entry();
        }
        let parsed_address = parse_address(address)?;
        self.analyze_section_from_address(parsed_address)
    }

    pub(crate) fn analyze_all_sections(&self) -> Result<Vec<KnownSectionData>, String> {
        let fireball = self.fireball()?;
        let analyzed = fireball.analyze_all().map_err(|error| error.to_string())?;
        Ok(block_to_result(analyzed))
    }

    fn analyze_section_from_address(&self, address: u64) -> Result<Vec<KnownSectionData>, String> {
        let fireball = self.fireball()?;
        let result = fireball
            .analyze_from_virtual_address(address)
            .map_err(|error| error.to_string())?;
        Ok(block_to_result([result]))
    }

    fn analyze_section_from_entry(&self) -> Result<Vec<KnownSectionData>, String> {
        let fireball = self.fireball()?;
        let result = fireball
            .analyze_from_entry()
            .map_err(|error| error.to_string())?;
        Ok(block_to_result([result]))
    }

    pub(crate) fn decompile_sections(
        &self,
        start_addresses: Vec<u64>,
    ) -> Result<DecompileResult, String> {
        let fireball = self.fireball()?;
        let mut assembly = Vec::new();
        let mut irs = Vec::new();
        let blocks = fireball.get_blocks();
        let sections = fireball.get_sections();
        let target_blocks = start_addresses
            .iter()
            .map(|&address| Address::from_virtual_address(&sections, address))
            .filter_map(|address| blocks.get_by_start_address(&address))
            .collect::<Vec<_>>();

        let mut assembly_index = 0;
        for target_block in &target_blocks {
            let start_address = target_block.get_start_address().get_virtual_address();
            let ir = target_block.get_ir();
            let Some(ir) = ir.as_ref() else {
                continue;
            };

            for (instruction, ir) in ir.instructions().iter().zip(ir.ir()) {
                assembly_index += 1;
                assembly.push(Assembly {
                    index: assembly_index,
                    parents_start_address: start_address,
                    data: instruction.to_string(),
                });
                let Some(statements) = ir.statements.as_ref() else {
                    continue;
                };
                for statement in statements.iter() {
                    irs.push(Ir {
                        parents_assembly_index: assembly_index,
                        data: statement.to_string(),
                    });
                }
            }
        }

        let decompiled = fireball::ir::analyze::generate_ast(target_blocks)
            .map_err(|error| error.to_string())?
            .optimize(None)
            .map_err(|error| error.to_string())?
            .print(None);

        Ok(DecompileResult {
            assembly,
            ir: irs,
            decompiled,
        })
    }
}

pub(crate) fn parse_address(address: &str) -> Result<u64, String> {
    let address = address.trim();
    if let Ok(address) = address.parse::<u64>() {
        return Ok(address);
    }

    let address = if address.starts_with("0x") || address.starts_with("0X") {
        &address[2..]
    } else {
        address
    };

    if let Ok(address) = u64::from_str_radix(address, 16) {
        return Ok(address);
    }

    Err("Invalid Address".to_string())
}

fn block_to_result(blocks: impl IntoIterator<Item = Arc<Block>>) -> Vec<KnownSectionData> {
    let mut result = Vec::new();
    for block in blocks {
        let start_address = block.get_start_address().get_virtual_address();
        let known = KnownSectionData {
            end_address: block.get_block_size().map(|size| start_address + size),
            start_address,
            analyzed: true,
        };
        result.retain(|item: &KnownSectionData| item.start_address != known.start_address);
        result.push(known);

        let reader = block.get_connected_to();
        for relation in reader.iter() {
            let Some(to) = relation.to() else {
                continue;
            };
            let target = to.get_virtual_address();
            if result.iter().any(|item| item.start_address == target) {
                continue;
            }

            result.push(KnownSectionData {
                start_address: target,
                end_address: None,
                analyzed: false,
            });
        }
    }
    result
}

pub(crate) fn build_line_ranges(text: &str) -> Vec<(usize, usize)> {
    if text.is_empty() {
        return vec![(0, 0)];
    }

    let mut ranges = Vec::new();
    let mut line_start = 0;
    for (idx, ch) in text.char_indices() {
        if ch == '\n' {
            ranges.push((line_start, idx));
            line_start = idx + ch.len_utf8();
        }
    }
    if line_start <= text.len() {
        ranges.push((line_start, text.len()));
    }
    if ranges.is_empty() {
        ranges.push((0, text.len()));
    }
    ranges
}
