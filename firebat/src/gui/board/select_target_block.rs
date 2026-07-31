use crate::{
    Firebat,
    gui::board::{BoardWindow, BoardWindowKind},
};
use fireball::core::{Block, FireRaw};
use std::sync::Arc;

pub const WINDOW_ID: &str = "select_target_block";

/// State of the window which picks the blocks to decompile.
#[derive(Default)]
pub struct SelectTargetBlockData {
    input: String,
    blocks: Vec<SelectTargetBlockDataBlock>,
}

pub struct SelectTargetBlockDataBlock {
    start_address: u64,
    end_address: Option<u64>,
    analyzed: bool,
    selected: bool,
}

/// Creates the window which picks the blocks to decompile.
pub fn window(pos: egui::Pos2) -> BoardWindow {
    BoardWindow::new(
        WINDOW_ID,
        "Select Target Block",
        pos,
        BoardWindowKind::SelectTargetBlock(SelectTargetBlockData::default()),
    )
    .closable(false)
}

pub fn ui(app: &mut Firebat, data: &mut SelectTargetBlockData, ui: &mut egui::Ui) {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.add(
            egui::TextEdit::singleline(&mut data.input)
                .hint_text("address")
                .desired_width(70.0),
        );
        if ui.button("Analyze").clicked() {
            analyze_input(app, data);
            changed = true;
        }
    });
    ui.horizontal(|ui| {
        if ui.button("Entry").clicked() {
            analyze_entry(app, data);
            changed = true;
        }
        if ui.button("All").clicked() {
            analyze_all(app, data);
            changed = true;
        }
    });

    ui.separator();

    let mut requested = None;
    for block in data.blocks.iter_mut() {
        ui.horizontal(|ui| {
            let label = match block.end_address {
                Some(end) => format!("0x{:06x} - 0x{:06x}", block.start_address, end),
                None => format!("0x{:06x}", block.start_address),
            };
            if block.analyzed {
                changed |= ui.checkbox(&mut block.selected, label).changed();
            } else {
                ui.label(label);
                if ui.small_button("analyze").clicked() {
                    requested = Some(block.start_address);
                }
            }
        });
    }

    if let Some(address) = requested {
        analyze_address(app, data, address);
        changed = true;
    }

    if changed {
        let blocks = data
            .blocks
            .iter()
            .filter(|it| it.analyzed && it.selected)
            .map(|it| it.start_address)
            .collect();
        app.board.pipeline.set_blocks(blocks);
    }
}

fn analyze_input(app: &mut Firebat, data: &mut SelectTargetBlockData) {
    let Some(address) = str_to_address(&data.input) else {
        tracing::warn!("invalid address: {}", data.input);
        return;
    };
    analyze_address(app, data, address);
}

fn analyze_address(app: &mut Firebat, data: &mut SelectTargetBlockData, address: u64) {
    let Some(fireball) = app.fireball.as_ref() else {
        return;
    };
    match fireball.analyze_from_virtual_address(address) {
        Ok(block) => insert_block(data, block),
        Err(error) => tracing::warn!("analyze failed at 0x{address:x}: {error:?}"),
    }
}

fn analyze_entry(app: &mut Firebat, data: &mut SelectTargetBlockData) {
    let Some(fireball) = app.fireball.as_ref() else {
        return;
    };
    match fireball.analyze_from_entry() {
        Ok(block) => insert_block(data, block),
        Err(error) => tracing::warn!("analyze from entry failed: {error:?}"),
    }
}

fn analyze_all(app: &mut Firebat, data: &mut SelectTargetBlockData) {
    let Some(fireball) = app.fireball.as_ref() else {
        return;
    };
    match fireball.analyze_all() {
        Ok(blocks) => {
            for block in blocks {
                insert_block(data, block);
            }
        }
        Err(error) => tracing::warn!("analyze all failed: {error:?}"),
    }
}

fn insert_block(data: &mut SelectTargetBlockData, block: Arc<Block>) {
    let start_address = block.get_start_address().get_virtual_address();
    let end_address = block.get_block_size().map(|size| start_address + size);
    let connected_to: Vec<_> = block
        .get_connected_to()
        .iter()
        .filter_map(|it| it.to())
        .map(|it| it.get_virtual_address())
        .collect();

    if let Some(known) = data
        .blocks
        .iter_mut()
        .find(|it| it.start_address == start_address)
    {
        known.end_address = end_address;
        known.analyzed = true;
        known.selected = true;
    } else {
        data.blocks.push(SelectTargetBlockDataBlock {
            start_address,
            end_address,
            analyzed: true,
            selected: true,
        });
    }

    for address in connected_to {
        if data.blocks.iter().any(|it| it.start_address == address) {
            continue;
        }
        data.blocks.push(SelectTargetBlockDataBlock {
            start_address: address,
            end_address: None,
            analyzed: false,
            selected: false,
        });
    }

    tracing::debug!("analyzed block at 0x{start_address:x}");
}

fn str_to_address(s: &str) -> Option<u64> {
    let s = s.trim();
    let hex = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X"));
    match hex {
        Some(hex) => u64::from_str_radix(hex, 16).ok(),
        None => u64::from_str_radix(s, 16).ok().or_else(|| s.parse().ok()),
    }
}
