use crate::tui::{FiremanCtx, MutexCtx};
use fireball::core::FireRaw;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::i32;

pub fn handle_events(event: event::Event, ctx_: &MutexCtx) -> std::io::Result<bool> {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            _ if super::super::handle_focus_move(ctx_, key) => Ok(false),
            KeyCode::Esc => Ok(true),
            KeyCode::Enter => analyze_block(ctx_),
            KeyCode::Char(' ') => select_block(ctx_),
            KeyCode::Char('q') => {
                generate_ast(ctx_)?;
                Ok(false)
            }
            KeyCode::Char(c) => {
                ctx_.write()
                    .unwrap()
                    .main_context
                    .block_context
                    .input
                    .push(c);
                Ok(false)
            }
            KeyCode::Up => move_cursor(ctx_, -1),
            KeyCode::Down => move_cursor(ctx_, 1),
            KeyCode::PageDown => move_cursor(ctx_, 10),
            KeyCode::PageUp => move_cursor(ctx_, -10),
            KeyCode::End => move_cursor(ctx_, i32::MAX),
            KeyCode::Home => move_cursor(ctx_, i32::MIN),
            KeyCode::Backspace => {
                ctx_.write().unwrap().main_context.block_context.input.pop();
                Ok(false)
            }
            _ => Ok(false),
        },
        _ => Ok(false),
    }
}

pub fn select_block(ctx_: &MutexCtx) -> std::io::Result<bool> {
    let mut ctx = ctx_.write().unwrap();

    let Some(cursor) = ctx
        .main_context
        .block_context
        .list_cursor
        .map(|x| &mut ctx.main_context.block_context.list[x])
    else {
        return Ok(false);
    };

    if !cursor.analyzed {
        ctx.top_message = "Block not analyzed".to_string();
        return Ok(false);
    }
    cursor.selected = !cursor.selected;

    return Ok(false);
}

pub fn analyze_block(ctx_: &MutexCtx) -> std::io::Result<bool> {
    let mut ctx = ctx_.write().unwrap();
    let fireball = ctx.fireball();

    let input = &ctx.main_context.block_context.input;
    let analyzed_result;
    let cursor = ctx
        .main_context
        .block_context
        .list_cursor
        .map(|x| &ctx.main_context.block_context.list[x]);
    if input.is_empty() && cursor.is_none() {
        // check already analyzed
        if ctx.main_context.block_context.entry_analyzed {
            ctx.top_message = "Already Analyzed".to_string();
            return Ok(false);
        }

        // analyze
        let result = fireball.analyze_from_entry();
        if let Err(e) = result {
            ctx.top_message = e.to_string();
            return Ok(false);
        }
        analyzed_result = result.unwrap();
        ctx.main_context.block_context.entry_analyzed = true;
    } else if input.is_empty()
        && let Some(cursor) = cursor
    {
        // check already analyzed
        if cursor.analyzed {
            ctx.top_message = "Already Analyzed".to_string();
            return Ok(false);
        }

        // analyze
        let result =
            fireball.analyze_from_virtual_address(cursor.start_address.get_virtual_address());
        if let Err(e) = result {
            ctx.top_message = e.to_string();
            return Ok(false);
        }
        analyzed_result = result.unwrap();
    } else {
        let address = crate::utils::parse_address(input);
        if let Err(e) = address {
            ctx.top_message = e;
            return Ok(false);
        }
        let address = address.unwrap();

        // check already analyzed
        if let Some(analyzed_result) = ctx
            .main_context
            .block_context
            .list
            .iter()
            .find(|item| item.start_address.get_virtual_address() == address)
            && analyzed_result.analyzed
        {
            ctx.top_message = "Already Analyzed".to_string();
            return Ok(false);
        }

        // analyze
        let result = fireball.analyze_from_virtual_address(address);
        if let Err(e) = result {
            ctx.top_message = e.to_string();
            return Ok(false);
        }
        analyzed_result = result.unwrap();
    }

    // insert result
    let analyzed_start_address = analyzed_result.get_start_address().clone();
    // insert or update analyzed block
    let list_item = ctx
        .main_context
        .block_context
        .list_get_mut_or_insert(analyzed_start_address);
    list_item.analyzed = true;
    // insert connected blocks
    let connected_to = analyzed_result.get_connected_to();
    for connected_to in connected_to.iter() {
        let Some(connected_to_start_address) = connected_to.to() else {
            continue;
        };
        let _ = ctx
            .main_context
            .block_context
            .list_get_mut_or_insert(connected_to_start_address);
    }

    display_asm_ir(&mut ctx)?;
    Ok(false)
}

pub fn move_cursor(ctx_: &MutexCtx, vector: i32) -> std::io::Result<bool> {
    let mut ctx = ctx_.write().unwrap();
    let _fireball = ctx.fireball();

    let current: i32 = ctx
        .main_context
        .block_context
        .list_cursor
        .map(|x| i32::try_from(x).unwrap())
        .unwrap_or(-1);
    let result;

    if vector != 1 && current == -1 {
        // current unselected and input is pgdown or end
        result = vector;
    } else if vector == i32::MIN {
        result = 0;
    } else {
        result = current + vector;
    }

    let list_len = ctx.main_context.block_context.list.len();
    if result < 0 {
        ctx.main_context.block_context.list_cursor = None;
    } else if list_len == 0 {
        ctx.main_context.block_context.list_cursor = None;
    } else if result >= list_len as i32 {
        ctx.main_context.block_context.list_cursor = Some(list_len - 1);
    } else {
        ctx.main_context.block_context.list_cursor = Some(result as usize);
    }

    display_asm_ir(&mut ctx)?;
    Ok(false)
}

pub fn display_asm_ir(ctx: &mut FiremanCtx) -> std::io::Result<()> {
    let Some(cursor) = ctx
        .main_context
        .block_context
        .list_cursor
        .map(|x| &ctx.main_context.block_context.list[x])
    else {
        return Ok(());
    };

    if !cursor.analyzed {
        return Ok(());
    }

    let start_address = &cursor.start_address;
    let fireball = ctx.fireball();
    let block = fireball
        .get_blocks()
        .get_by_start_address(start_address)
        .unwrap();
    let ir = block.get_ir();
    let ir = ir.as_ref().unwrap();
    let instructions = ir.instructions();

    // display asm
    let asm_data = ctx
        .main_context
        .asm_context
        .data
        .entry(start_address.clone())
        .or_insert(super::super::asm_panel::Data::new(instructions.clone()));
    ctx.main_context.asm_context.list = asm_data.displayed.clone();

    // display ir
    let ir_data = ctx
        .main_context
        .ir_context
        .data
        .entry(start_address.clone())
        .or_insert(super::super::ir_panel::Data::new(
            instructions.clone(),
            ir.ir().to_vec(),
        ));
    ctx.main_context.ir_context.list = ir_data.displayed.clone();

    Ok(())
}

pub fn generate_ast(ctx_: &MutexCtx) -> std::io::Result<()> {
    let mut ctx = ctx_.write().unwrap();
    let fireball = ctx.fireball();

    let selected = ctx
        .main_context
        .block_context
        .list
        .iter()
        .filter(|x| x.selected)
        .collect::<Vec<_>>();
    if selected.is_empty() {
        ctx.top_message = "Select block first".to_string();
        return Ok(());
    }
    let key =
        super::super::ast_panel::Key(selected.iter().map(|x| &x.start_address).cloned().collect());

    match ctx.main_context.ast_context.data.get(&key) {
        Some(data) => {
            ctx.main_context.ast_context.list = data.displayed.clone();
        }
        None => {
            let blocks = fireball.get_blocks();
            let selected_blocks = selected
                .iter()
                .map(|x| &x.start_address)
                .map(|address| blocks.get_by_start_address(address).unwrap());
            let ast = fireball::ir::analyze::generate_ast(selected_blocks);
            if let Err(e) = ast {
                ctx.top_message = e.to_string();
                return Ok(());
            }
            let ast = ast.unwrap();
            let data = super::super::ast_panel::Data::new(ast);
            let displayed = data.displayed.clone();
            ctx.main_context.ast_context.data.insert(key, data);
            ctx.main_context.ast_context.list = displayed;
        }
    }

    Ok(())
}
