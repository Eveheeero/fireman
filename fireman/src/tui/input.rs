use super::{
    app::App,
    prompt,
    types::{
        all_optimization_fields, optimization_field_count, OptimizationFocus, PromptKind, TabType,
    },
};
use crate::{model::OptimizationSettings, worker::WorkerRequest};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

impl App {
    pub(crate) fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return;
            }
            if self.prompt.is_some() {
                self.handle_prompt_key(key);
                return;
            }
            self.handle_key(key);
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if self.show_license {
            self.show_license = false;
            return;
        }
        let handled = match self.tabs.current_tab_type() {
            Some(TabType::Input) => self.handle_sections_key(key),
            Some(TabType::Result) => self.handle_result_key(key),
            Some(TabType::Pick) => self.handle_pick_key(key),
            Some(TabType::Logs) => self.handle_logs_key(key),
            None => false,
        };
        if handled {
            return;
        }
        let _ = self.handle_global_key(key);
    }

    fn handle_global_key(&mut self, key: KeyEvent) -> bool {
        if key.modifiers.intersects(
            KeyModifiers::CONTROL
                | KeyModifiers::ALT
                | KeyModifiers::META
                | KeyModifiers::SUPER
                | KeyModifiers::HYPER,
        ) {
            return false;
        }
        match key.code {
            KeyCode::Char('q') => {
                self.running = false;
                true
            }
            // Tab navigation - numbers go to specific tabs
            KeyCode::Char('1') => {
                self.tabs.goto_tab(0);
                true
            }
            KeyCode::Char('2') => {
                self.tabs.goto_tab(1);
                true
            }
            KeyCode::Char('3') => {
                self.tabs.goto_tab(2);
                true
            }
            KeyCode::Char('4') => {
                self.tabs.goto_tab(3);
                true
            }
            KeyCode::Char('5') => {
                self.tabs.goto_tab(4);
                true
            }
            KeyCode::Char('6') => {
                self.tabs.goto_tab(5);
                true
            }
            KeyCode::Char('7') => {
                self.tabs.goto_tab(6);
                true
            }
            KeyCode::Char('8') => {
                self.tabs.goto_tab(7);
                true
            }
            KeyCode::Char('9') => {
                self.tabs.goto_tab(8);
                true
            }
            // Tab management
            KeyCode::Char('t') => {
                self.tabs.next_tab();
                true
            }
            KeyCode::Char('T') => {
                self.tabs.prev_tab();
                true
            }
            KeyCode::Char('c') => {
                self.remove_pipeline_entry();
                true
            }
            KeyCode::Char('n') => {
                self.add_pipeline_stage();
                true
            }
            KeyCode::Char('r') => {
                // Reset pipeline and tabs to default
                self.tabs.reset();
                self.pipeline = vec![super::types::PipelineEntry::Result(
                    super::types::ResultTabState::new(),
                )];
                self.log("Reset tabs to default");
                true
            }
            KeyCode::Char('?') => {
                self.show_license = true;
                true
            }
            _ => false,
        }
    }

    fn handle_sections_key(&mut self, key: KeyEvent) -> bool {
        let len = self.known_sections.len();
        match key.code {
            KeyCode::Up => {
                move_cursor_up(&mut self.section_cursor, 1);
                true
            }
            KeyCode::Char('k') if key.modifiers.is_empty() => {
                move_cursor_up(&mut self.section_cursor, 1);
                true
            }
            KeyCode::Down => {
                move_cursor_down(&mut self.section_cursor, 1, len);
                true
            }
            KeyCode::Char('j') if key.modifiers.is_empty() => {
                move_cursor_down(&mut self.section_cursor, 1, len);
                true
            }
            KeyCode::PageUp => {
                move_cursor_up(&mut self.section_cursor, 10);
                true
            }
            KeyCode::PageDown => {
                move_cursor_down(&mut self.section_cursor, 10, len);
                true
            }
            KeyCode::Home => {
                self.section_cursor = 0;
                true
            }
            KeyCode::End if len > 0 => {
                self.section_cursor = len - 1;
                true
            }
            KeyCode::Char(' ') => {
                self.toggle_section(self.section_cursor);
                self.start_decompile();
                true
            }
            KeyCode::Char('s') if key.modifiers.is_empty() => {
                self.toggle_all_sections();
                self.start_decompile();
                true
            }
            KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.toggle_all_sections();
                self.start_decompile();
                true
            }
            KeyCode::Enter => {
                self.toggle_section(self.section_cursor);
                self.start_decompile();
                true
            }
            KeyCode::Char('o') => {
                self.open_path_prompt(
                    PromptKind::OpenFile,
                    "Open Binary",
                    self.opened_path.clone().unwrap_or_default(),
                    "Enter file path and press Enter. Tab autocompletes. Esc cancels.",
                );
                true
            }
            KeyCode::Char('a') => {
                self.open_prompt(
                    PromptKind::AnalyzeAddress,
                    "Analyze Address",
                    self.pending_analysis_address.clone().unwrap_or_default(),
                    false,
                    "Enter an address. Leave empty to analyze the entry point.",
                );
                true
            }
            KeyCode::Char('g') => {
                self.send_request(WorkerRequest::AnalyzeAllSections, "analyzing all sections");
                true
            }
            _ => false,
        }
    }

    fn handle_result_key(&mut self, key: KeyEvent) -> bool {
        let len = self
            .current_result_state()
            .and_then(|a| a.outputs.as_ref())
            .map(|o| o.ast.len())
            .unwrap_or(0);
        if len == 0 && !matches!(key.code, KeyCode::Char('d')) {
            return false;
        }
        let Some(res) = self.current_result_state_mut() else {
            return false;
        };
        match key.code {
            KeyCode::Up => {
                move_cursor_up(&mut res.cursor, 1);
                true
            }
            KeyCode::Char('k') if key.modifiers.is_empty() => {
                move_cursor_up(&mut res.cursor, 1);
                true
            }
            KeyCode::Down => {
                move_cursor_down(&mut res.cursor, 1, len);
                true
            }
            KeyCode::Char('j') if key.modifiers.is_empty() => {
                move_cursor_down(&mut res.cursor, 1, len);
                true
            }
            KeyCode::PageUp => {
                move_cursor_up(&mut res.cursor, 10);
                true
            }
            KeyCode::PageDown => {
                move_cursor_down(&mut res.cursor, 10, len);
                true
            }
            KeyCode::Home => {
                res.cursor = 0;
                true
            }
            KeyCode::End => {
                res.cursor = len.saturating_sub(1);
                true
            }
            KeyCode::Char('d') if key.modifiers.is_empty() => {
                let _ = res;
                self.start_decompile();
                true
            }
            _ => false,
        }
    }

    fn handle_pick_key(&mut self, key: KeyEvent) -> bool {
        let Some(focus) = self.current_pick_state().map(|o| o.focus) else {
            return false;
        };
        match key.code {
            KeyCode::Tab => {
                if let Some(pick) = self.current_pick_state_mut() {
                    pick.focus = pick.focus.next();
                }
                true
            }
            KeyCode::BackTab => {
                if let Some(pick) = self.current_pick_state_mut() {
                    pick.focus = pick.focus.previous();
                }
                true
            }
            KeyCode::Char('L') => {
                self.load_saved_optimization_store();
                true
            }
            KeyCode::Char('W') => {
                self.save_current_optimization_store();
                true
            }
            _ => match focus {
                OptimizationFocus::Settings => self.handle_pick_settings_key(key),
                OptimizationFocus::Script => self.handle_pick_script_key(key),
            },
        }
    }

    fn handle_pick_settings_key(&mut self, key: KeyEvent) -> bool {
        let base_field_count = optimization_field_count();
        let has_buffer = self
            .current_pick_state()
            .map(|o| !o.store.editor_buffer.trim().is_empty())
            .unwrap_or(false);
        let field_count = if has_buffer {
            base_field_count + 1
        } else {
            base_field_count
        };
        let Some(pick) = self.current_pick_state_mut() else {
            return false;
        };
        match key.code {
            KeyCode::Up => {
                move_cursor_up(&mut pick.setting_cursor, 1);
                true
            }
            KeyCode::Char('k') if key.modifiers.is_empty() => {
                move_cursor_up(&mut pick.setting_cursor, 1);
                true
            }
            KeyCode::Down => {
                move_cursor_down(&mut pick.setting_cursor, 1, field_count);
                true
            }
            KeyCode::Char('j') if key.modifiers.is_empty() => {
                move_cursor_down(&mut pick.setting_cursor, 1, field_count);
                true
            }
            KeyCode::PageUp => {
                move_cursor_up(&mut pick.setting_cursor, 10);
                true
            }
            KeyCode::PageDown => {
                move_cursor_down(&mut pick.setting_cursor, 10, field_count);
                true
            }
            KeyCode::Home => {
                pick.setting_cursor = 0;
                true
            }
            KeyCode::End if field_count > 0 => {
                pick.setting_cursor = field_count - 1;
                true
            }
            KeyCode::Char(' ') => {
                let cursor = pick.setting_cursor;
                let is_script_entry = has_buffer && cursor == base_field_count;
                if is_script_entry {
                    pick.store.fb_script_enabled = !pick.store.fb_script_enabled;
                    if pick.store.fb_script_enabled {
                        pick.store.applied_buffer_script =
                            if pick.store.editor_buffer.trim().is_empty() {
                                None
                            } else {
                                Some(pick.store.editor_buffer.clone())
                            };
                    } else {
                        pick.store.applied_buffer_script = None;
                    }
                    let _ = pick;
                    self.redecompile_last_selection();
                } else if let Some(field) = all_optimization_fields().nth(cursor) {
                    let current = (field.get)(&pick.store.draft_settings);
                    if current {
                        (field.set)(&mut pick.store.draft_settings, false);
                    } else {
                        let all_fields: Vec<_> = all_optimization_fields().collect();
                        for f in &all_fields {
                            (f.set)(&mut pick.store.draft_settings, false);
                        }
                        (field.set)(&mut pick.store.draft_settings, true);
                        pick.store.editor_buffer.clear();
                        pick.store.fb_script_enabled = false;
                        pick.store.applied_buffer_script = None;
                    }
                    let _ = pick;
                    self.apply_optimization_settings();
                }
                true
            }
            KeyCode::Char('r') if key.modifiers.is_empty() => {
                pick.store.draft_settings = OptimizationSettings::default();
                let _ = pick;
                self.set_status("Restored optimization draft defaults");
                true
            }
            _ => false,
        }
    }

    fn handle_pick_script_key(&mut self, key: KeyEvent) -> bool {
        let Some(pick) = self.current_pick_state_mut() else {
            return false;
        };
        match key.code {
            KeyCode::Tab => {
                pick.focus = OptimizationFocus::Settings;
                true
            }
            KeyCode::Char(ch)
                if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT =>
            {
                prompt::insert_char(&mut pick.store.editor_buffer, &mut pick.script_cursor, ch);
                true
            }
            KeyCode::Enter => {
                prompt::insert_char(
                    &mut pick.store.editor_buffer,
                    &mut pick.script_cursor,
                    '\n',
                );
                true
            }
            KeyCode::Backspace => {
                prompt::delete_prev_char(
                    &mut pick.store.editor_buffer,
                    &mut pick.script_cursor,
                );
                true
            }
            KeyCode::Delete => {
                prompt::delete_next_char(
                    &mut pick.store.editor_buffer,
                    &mut pick.script_cursor,
                );
                true
            }
            KeyCode::Left => {
                prompt::move_cursor_left(&pick.store.editor_buffer, &mut pick.script_cursor);
                true
            }
            KeyCode::Right => {
                prompt::move_cursor_right(&pick.store.editor_buffer, &mut pick.script_cursor);
                true
            }
            KeyCode::Home => {
                let buf = &pick.store.editor_buffer;
                let cursor = pick.script_cursor;
                let line_start = buf[..cursor].rfind('\n').map(|p| p + 1).unwrap_or(0);
                pick.script_cursor = line_start;
                true
            }
            KeyCode::End => {
                let buf = &pick.store.editor_buffer;
                let cursor = pick.script_cursor;
                let line_end = buf[cursor..]
                    .find('\n')
                    .map(|p| cursor + p)
                    .unwrap_or(buf.len());
                pick.script_cursor = line_end;
                true
            }
            KeyCode::Up => {
                let buf = &pick.store.editor_buffer;
                let cursor = pick.script_cursor;
                let line_start = buf[..cursor].rfind('\n').map(|p| p + 1).unwrap_or(0);
                let col = cursor - line_start;
                if line_start > 0 {
                    let prev_line_end = line_start - 1;
                    let prev_line_start = buf[..prev_line_end]
                        .rfind('\n')
                        .map(|p| p + 1)
                        .unwrap_or(0);
                    let prev_line_len = prev_line_end - prev_line_start;
                    pick.script_cursor = prev_line_start + col.min(prev_line_len);
                }
                true
            }
            KeyCode::Down => {
                let buf = &pick.store.editor_buffer;
                let cursor = pick.script_cursor;
                let line_start = buf[..cursor].rfind('\n').map(|p| p + 1).unwrap_or(0);
                let col = cursor - line_start;
                if let Some(nl_pos) = buf[cursor..].find('\n') {
                    let next_line_start = cursor + nl_pos + 1;
                    let next_line_end = buf[next_line_start..]
                        .find('\n')
                        .map(|p| next_line_start + p)
                        .unwrap_or(buf.len());
                    let next_line_len = next_line_end - next_line_start;
                    pick.script_cursor = next_line_start + col.min(next_line_len);
                }
                true
            }
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let path = pick.store.editor_path.clone();
                let _ = pick;
                if let Some(path) = path {
                    self.save_buffer_to_path(path);
                } else {
                    self.open_path_prompt(
                        PromptKind::SaveBufferPath,
                        "Save Script",
                        String::new(),
                        "Enter a path for the script. Tab autocompletes.",
                    );
                }
                true
            }
            KeyCode::Char('o') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let editor_path = pick.store.editor_path.clone().unwrap_or_default();
                let _ = pick;
                self.open_path_prompt(
                    PromptKind::LoadBufferPath,
                    "Load Script",
                    editor_path,
                    "Enter a .fb file path. Tab autocompletes.",
                );
                true
            }
            _ => false,
        }
    }

    fn handle_logs_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Up => {
                move_cursor_up(&mut self.log_scroll, 1);
                true
            }
            KeyCode::Char('k') if key.modifiers.is_empty() => {
                move_cursor_up(&mut self.log_scroll, 1);
                true
            }
            KeyCode::Down => {
                self.log_scroll = self.log_scroll.saturating_add(1);
                true
            }
            KeyCode::Char('j') if key.modifiers.is_empty() => {
                self.log_scroll = self.log_scroll.saturating_add(1);
                true
            }
            KeyCode::PageUp => {
                move_cursor_up(&mut self.log_scroll, 10);
                true
            }
            KeyCode::PageDown => {
                self.log_scroll = self.log_scroll.saturating_add(10);
                true
            }
            KeyCode::Home => {
                self.log_scroll = 0;
                true
            }
            KeyCode::End => {
                self.log_scroll = u16::MAX as usize;
                true
            }
            _ => false,
        }
    }

    fn handle_prompt_key(&mut self, key: KeyEvent) {
        let Some(prompt_ref) = self.prompt.as_mut() else {
            return;
        };
        let has_browser = prompt_ref.file_browser.is_some();
        match key.code {
            KeyCode::Esc => {
                self.prompt = None;
                self.set_status("Prompt cancelled");
            }
            KeyCode::Tab if has_browser => {
                let completed = prompt_ref
                    .file_browser
                    .as_ref()
                    .and_then(|browser| browser.complete_path(&prompt_ref.text));
                if let Some(new_path) = completed {
                    prompt_ref.text = new_path;
                    prompt_ref.cursor = prompt_ref.text.len();
                    if let Some(browser) = prompt_ref.file_browser.as_mut() {
                        browser.update_from_path(&prompt_ref.text);
                    }
                }
            }
            KeyCode::Up if has_browser => {
                if let Some(browser) = prompt_ref.file_browser.as_mut() {
                    browser.move_up();
                }
            }
            KeyCode::Down if has_browser => {
                if let Some(browser) = prompt_ref.file_browser.as_mut() {
                    browser.move_down();
                }
            }
            KeyCode::PageUp if has_browser => {
                if let Some(browser) = prompt_ref.file_browser.as_mut() {
                    browser.move_page_up();
                }
            }
            KeyCode::PageDown if has_browser => {
                if let Some(browser) = prompt_ref.file_browser.as_mut() {
                    browser.move_page_down();
                }
            }
            KeyCode::Enter if !prompt_ref.multiline => {
                self.submit_prompt();
            }
            KeyCode::Enter => {
                prompt::insert_char(&mut prompt_ref.text, &mut prompt_ref.cursor, '\n');
            }
            KeyCode::Backspace => {
                prompt::delete_prev_char(&mut prompt_ref.text, &mut prompt_ref.cursor);
                if let Some(browser) = prompt_ref.file_browser.as_mut() {
                    browser.update_from_path(&prompt_ref.text);
                }
            }
            KeyCode::Delete => {
                prompt::delete_next_char(&mut prompt_ref.text, &mut prompt_ref.cursor);
                if let Some(browser) = prompt_ref.file_browser.as_mut() {
                    browser.update_from_path(&prompt_ref.text);
                }
            }
            KeyCode::Left => {
                prompt::move_cursor_left(&prompt_ref.text, &mut prompt_ref.cursor);
            }
            KeyCode::Right => {
                prompt::move_cursor_right(&prompt_ref.text, &mut prompt_ref.cursor);
            }
            KeyCode::Home => {
                prompt_ref.cursor = 0;
            }
            KeyCode::End => {
                prompt_ref.cursor = prompt_ref.text.len();
            }
            KeyCode::Char('s')
                if prompt_ref.multiline && key.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                self.submit_prompt();
            }
            KeyCode::Char(ch) => {
                prompt::insert_char(&mut prompt_ref.text, &mut prompt_ref.cursor, ch);
                if let Some(browser) = prompt_ref.file_browser.as_mut() {
                    browser.update_from_path(&prompt_ref.text);
                }
            }
            _ => {}
        }
    }

    pub(crate) fn keybindings(&self) -> Vec<(&'static str, &'static str)> {
        if let Some(prompt) = &self.prompt {
            return if prompt.file_browser.is_some() {
                vec![
                    ("Esc", "cancel"),
                    ("Tab", "autocomplete"),
                    ("Up/Down", "browse"),
                    ("PgUp/PgDn", "page"),
                    ("Enter", "submit"),
                ]
            } else if prompt.multiline {
                vec![
                    ("Esc", "cancel"),
                    ("Ctrl+S", "save buffer"),
                    ("Enter", "newline"),
                ]
            } else {
                vec![
                    ("Esc", "cancel"),
                    ("Enter", "submit"),
                    ("Left/Right", "move cursor"),
                ]
            };
        }
        let mut keys = vec![
            ("1-9", "goto tab"),
            ("t", "next"),
            ("T", "prev"),
            ("n", "+stage"),
            ("c", "close"),
            ("r", "reset"),
            ("q", "quit"),
            ("?", "license"),
        ];
        match self.tabs.current_tab_type() {
            Some(TabType::Input) => keys.extend([
                ("o", "open binary"),
                ("a", "analyze address"),
                ("g", "analyze all sections"),
                ("Up/Dn", "move cursor"),
                ("PgUp/Dn", "fast move"),
                ("Home/End", "jump"),
                ("Space/Enter", "toggle section"),
                ("s", "select all ready"),
                ("Ctrl+A", "select all ready"),
            ]),
            Some(TabType::Result) => keys.extend([
                ("Up/Dn", "move cursor"),
                ("PgUp/Dn", "fast move"),
                ("Home/End", "jump"),
                ("d", "re-decompile"),
            ]),
            Some(TabType::Pick) => {
                let focus = self
                    .current_pick_state()
                    .map(|o| o.focus)
                    .unwrap_or(OptimizationFocus::Settings);
                match focus {
                    OptimizationFocus::Settings => keys.extend([
                        ("Tab", "switch focus"),
                        ("Up/Dn", "move cursor"),
                        ("Space", "select pass"),
                        ("r", "reset to none"),
                        ("L", "load config"),
                        ("W", "save config"),
                    ]),
                    OptimizationFocus::Script => keys.extend([
                        ("Tab", "switch focus"),
                        ("Type", "edit script"),
                        ("Backspace", "delete"),
                        ("Ctrl+S", "save file"),
                        ("Ctrl+O", "load file"),
                    ]),
                }
            }
            Some(TabType::Logs) => keys.extend([
                ("Up/Dn", "scroll logs"),
                ("PgUp/Dn", "fast scroll"),
                ("Home/End", "jump"),
            ]),
            None => {}
        }
        keys
    }
}

fn move_cursor_up(cursor: &mut usize, amount: usize) {
    *cursor = (*cursor).saturating_sub(amount);
}
fn move_cursor_down(cursor: &mut usize, amount: usize, len: usize) {
    if len == 0 {
        *cursor = 0;
        return;
    }
    *cursor = (*cursor).saturating_add(amount).min(len - 1);
}
