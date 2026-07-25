use crate::tui::{
    TuiApp,
    tab::{
        SelectTargetBlockData, SelectTargetBlockDataBlock, TuiTab, handle_del_tab, handle_new_tab,
        handle_turn_tab, refresh_decompile,
    },
};
use crossterm::event;
use fireball::core::{Block, FireRaw};
use ratatui::{Frame, prelude::*, widgets};
use std::sync::Arc;

pub fn draw(data: &mut SelectTargetBlockData, mut area: Rect, terminal: &mut Frame) {
    // render borer
    terminal.render_widget(widgets::Block::bordered(), area);
    area.x += 1;
    area.y += 1;
    area.height -= 2;
    area.width -= 2;

    // render input
    let mut input_area = area;
    input_area.height = 1;
    area.y += 1;
    area.height -= 1;
    let text = if !data.input.is_empty() {
        &data.input
    } else {
        "enter address to analyze"
    };
    terminal.render_widget(text::Text::raw(text), input_area);

    // render blocks
    terminal.render_stateful_widget(&data.blocks_list, area, &mut data.state);
}
pub fn handle_event(app: &mut TuiApp, event: event::Event) {
    if handle_turn_tab(app, &event) || handle_new_tab(app, &event) || handle_del_tab(app, &event) {
        return;
    }

    let current_tab_index = app.data.tab.current_tab_index;
    let current_tab = &mut app.data.tab.tabs[current_tab_index];
    let TuiTab::SelectTargetBlock(data) = current_tab else {
        unreachable!()
    };
    fn insert_block(data: &mut SelectTargetBlockData, block: Arc<Block>) {
        let start_address = block.get_start_address().get_virtual_address();
        let block_size = block.get_block_size();
        let end_address = block_size.map(|x| start_address + x);
        let connected_to = block
            .get_connected_to()
            .iter()
            .filter_map(|x| x.to())
            .map(|x| x.get_virtual_address())
            .collect::<Vec<_>>();
        if let Some(block) = data
            .blocks
            .iter_mut()
            .find(|block| block.start_address == start_address)
        {
            block.end_address = end_address;
            block.analyzed = true;
            block.selected = true;
        } else {
            data.blocks.push(SelectTargetBlockDataBlock {
                start_address,
                end_address,
                analyzed: true,
                selected: true,
            });
        }
        for connected_to in connected_to {
            let exist = data
                .blocks
                .iter()
                .find(|block| block.start_address == connected_to)
                .is_some();
            if !exist {
                data.blocks.push(SelectTargetBlockDataBlock {
                    start_address: connected_to,
                    end_address: None,
                    analyzed: false,
                    selected: false,
                });
            }
        }
    }

    // handle address input. 0~9 ,x, backspace
    if let Some(event) = event.as_key_press_event()
        && matches!(event.code.as_char(), Some('0'..'9' | 'x' | 'X'))
        && let Some(c) = event.code.as_char()
    {
        data.input.push(c);
    }
    if let Some(event) = event.as_key_press_event()
        && event.code == event::KeyCode::Backspace
    {
        data.input.pop();
    }

    // handle cursor, analyze, select
    if let Some(event) = event.as_key_press_event() {
        match event.code {
            event::KeyCode::Up => {
                let selected = data.state.selected();
                if selected.is_some() {
                    let selected_inner = selected.unwrap();
                    if selected_inner < 1 {
                        data.state.select(None);
                    } else {
                        data.state.select(Some(selected_inner - 1));
                    }
                }
            }
            event::KeyCode::Down => {
                let selected = data.state.selected();
                let target = if selected.is_none() {
                    0
                } else {
                    selected.unwrap() + 1
                };
                data.state
                    .select(Some(target.min(data.blocks_list.len() - 1)));
            }
            event::KeyCode::PageUp => {
                let selected = data.state.selected();
                if selected.is_some() {
                    let selected_inner = selected.unwrap();
                    if selected_inner < 3 {
                        data.state.select(None);
                    } else {
                        data.state.select(Some(selected_inner - 3));
                    }
                }
            }
            event::KeyCode::PageDown => {
                let selected = data.state.selected();
                let target = if selected.is_none() {
                    3
                } else {
                    selected.unwrap() + 3
                };
                data.state
                    .select(Some(target.min(data.blocks_list.len() - 1)));
            }
            event::KeyCode::Char(' ') => {
                if let Some(selected) = data.state.selected()
                    && data.blocks[selected].analyzed
                {
                    data.blocks[selected].selected ^= true;
                    refresh_list(data);
                    refresh_decompile(app);
                }
            }
            event::KeyCode::Char('a' | 'A') if is_select_all_modifier(event) => {
                if data
                    .blocks
                    .iter()
                    .filter(|i| i.analyzed)
                    .all(|i| i.selected)
                {
                    for i in &mut data.blocks {
                        if i.analyzed {
                            i.selected = false;
                        }
                    }
                } else {
                    for i in &mut data.blocks {
                        if i.analyzed {
                            i.selected = true;
                        }
                    }
                }
                refresh_list(data);
                refresh_decompile(app);
            }
            event::KeyCode::Enter | event::KeyCode::Char('a') => {
                let cursor = data.state.selected();
                if let Some(cursor) = cursor {
                    let current = &data.blocks[cursor];
                    if let Ok(block) = app
                        .fireball
                        .as_ref()
                        .unwrap()
                        .analyze_from_virtual_address(current.start_address)
                    {
                        insert_block(data, block);
                        refresh_list(data);
                        refresh_decompile(app);
                    }
                } else {
                    if data.input.is_empty()
                        && let Ok(block) = app.fireball.as_ref().unwrap().analyze_from_entry()
                    {
                        insert_block(data, block);
                        refresh_list(data);
                        refresh_decompile(app);
                    } else {
                        let address = &data.input;
                        let address = str_to_address(&address);
                        if let Some(address) = address
                            && let Ok(block) = app
                                .fireball
                                .as_ref()
                                .unwrap()
                                .analyze_from_virtual_address(address)
                        {
                            insert_block(data, block);
                            refresh_list(data);
                            refresh_decompile(app);
                        }
                    }
                }
            }
            event::KeyCode::Char('A') => {
                if let Ok(blocks) = app.fireball.as_ref().unwrap().analyze_all() {
                    for block in blocks {
                        insert_block(data, block);
                    }
                    refresh_list(data);
                    refresh_decompile(app);
                }
            }
            _ => {}
        }
    }
}

fn is_select_all_modifier(event: event::KeyEvent) -> bool {
    #[cfg(target_os = "macos")]
    {
        return event.modifiers == event::KeyModifiers::SUPER;
    }
    event.modifiers == event::KeyModifiers::CONTROL
}

fn refresh_list(data: &mut SelectTargetBlockData) {
    let list: Vec<_> = data
        .blocks
        .iter()
        .map(|x| {
            format!(
                "{} 0x{:06x}",
                if x.selected {
                    "[v]"
                } else if x.analyzed {
                    "[ ]"
                } else {
                    "   "
                },
                x.start_address
            )
        })
        .collect();
    data.blocks_list =
        widgets::List::new(list).highlight_style(style::Style::new().fg(style::Color::Blue));
}

fn str_to_address(s: &str) -> Option<u64> {
    let s = s
        .strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .unwrap_or(s);
    u64::from_str_radix(s, 16).ok().or_else(|| s.parse().ok())
}
