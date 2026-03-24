use super::{
    app::App,
    prompt,
    types::{
        OptimizationFocus, PromptKind, View, all_optimization_fields, optimization_field_count,
    },
};
use crate::{
    model::{EditorLayer, OptimizationSettings},
    worker::WorkerRequest,
};
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

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
        let handled = match self.current_view {
            View::Sections => self.handle_sections_key(key),
            View::Assembly => self.handle_output_key(key, EditorLayer::Assembly),
            View::Ir => self.handle_output_key(key, EditorLayer::Ir),
            View::Ast => self.handle_output_key(key, EditorLayer::Ast),
            View::Editor => self.handle_editor_key(key),
            View::Optimization => self.handle_optimization_key(key),
            View::Patch => self.handle_patch_key(key),
            View::Logs => self.handle_logs_key(key),
        };

        if handled {
            return;
        }

        let _ = self.handle_global_key(key);
    }

    fn handle_global_key(&mut self, key: KeyEvent) -> bool {
        if !key.modifiers.is_empty() {
            return false;
        }

        match key.code {
            KeyCode::Char('q') => {
                self.running = false;
                true
            }
            KeyCode::Char('1') => {
                self.set_view(View::Sections);
                true
            }
            KeyCode::Char('2') => {
                self.set_view(View::Assembly);
                true
            }
            KeyCode::Char('3') => {
                self.set_view(View::Ir);
                true
            }
            KeyCode::Char('4') => {
                self.set_view(View::Ast);
                true
            }
            KeyCode::Char('5') => {
                self.set_view(View::Editor);
                true
            }
            KeyCode::Char('6') => {
                self.set_view(View::Optimization);
                true
            }
            KeyCode::Char('7') => {
                self.set_view(View::Patch);
                true
            }
            KeyCode::Char('8') => {
                self.set_view(View::Logs);
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
            KeyCode::Char('d') => {
                self.start_decompile();
                true
            }
            KeyCode::Char('x') => {
                self.send_request(WorkerRequest::ExportPatch, "exporting patch");
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
                true
            }
            KeyCode::Char('s') if key.modifiers.is_empty() => {
                self.toggle_all_sections();
                true
            }
            KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.toggle_all_sections();
                true
            }
            KeyCode::Enter => {
                self.toggle_section(self.section_cursor);
                true
            }
            _ => false,
        }
    }

    fn handle_output_key(&mut self, key: KeyEvent, layer: EditorLayer) -> bool {
        let len = self.output_len(layer);
        if len == 0 {
            return false;
        }

        let cursor = self.output_cursor_mut(layer);
        match key.code {
            KeyCode::Up => {
                move_cursor_up(cursor, 1);
                self.update_hover();
                true
            }
            KeyCode::Char('k') if key.modifiers.is_empty() => {
                move_cursor_up(cursor, 1);
                self.update_hover();
                true
            }
            KeyCode::Down => {
                move_cursor_down(cursor, 1, len);
                self.update_hover();
                true
            }
            KeyCode::Char('j') if key.modifiers.is_empty() => {
                move_cursor_down(cursor, 1, len);
                self.update_hover();
                true
            }
            KeyCode::PageUp => {
                move_cursor_up(cursor, 10);
                self.update_hover();
                true
            }
            KeyCode::PageDown => {
                move_cursor_down(cursor, 10, len);
                self.update_hover();
                true
            }
            KeyCode::Home => {
                *cursor = 0;
                self.update_hover();
                true
            }
            KeyCode::End => {
                *cursor = len.saturating_sub(1);
                self.update_hover();
                true
            }
            KeyCode::Enter => {
                self.load_editor_from_current_row(layer);
                true
            }
            KeyCode::Char('e') if key.modifiers.is_empty() => {
                self.edit_current_row(layer);
                true
            }
            _ => false,
        }
    }

    fn handle_editor_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('e') if key.modifiers.is_empty() => {
                self.edit_loaded_draft();
                true
            }
            KeyCode::Char('r') if key.modifiers.is_empty() => {
                if let Some(target) = self.editor_target {
                    self.reload_editor(target);
                }
                true
            }
            KeyCode::Char('[') => {
                self.cycle_edit_position(false);
                true
            }
            KeyCode::Char(']') => {
                self.cycle_edit_position(true);
                true
            }
            KeyCode::Enter => {
                self.apply_edit();
                true
            }
            _ => false,
        }
    }

    fn handle_optimization_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Tab => {
                self.optimization_focus = self.optimization_focus.next();
                true
            }
            KeyCode::BackTab => {
                self.optimization_focus = self.optimization_focus.previous();
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
            _ => match self.optimization_focus {
                OptimizationFocus::Settings => self.handle_optimization_settings_key(key),
                OptimizationFocus::Scripts => self.handle_optimization_scripts_key(key),
                OptimizationFocus::Buffer => self.handle_optimization_buffer_key(key),
            },
        }
    }

    fn handle_optimization_settings_key(&mut self, key: KeyEvent) -> bool {
        let field_count = optimization_field_count();
        match key.code {
            KeyCode::Up => {
                move_cursor_up(&mut self.optimization_setting_cursor, 1);
                true
            }
            KeyCode::Char('k') if key.modifiers.is_empty() => {
                move_cursor_up(&mut self.optimization_setting_cursor, 1);
                true
            }
            KeyCode::Down => {
                move_cursor_down(&mut self.optimization_setting_cursor, 1, field_count);
                true
            }
            KeyCode::Char('j') if key.modifiers.is_empty() => {
                move_cursor_down(&mut self.optimization_setting_cursor, 1, field_count);
                true
            }
            KeyCode::PageUp => {
                move_cursor_up(&mut self.optimization_setting_cursor, 10);
                true
            }
            KeyCode::PageDown => {
                move_cursor_down(&mut self.optimization_setting_cursor, 10, field_count);
                true
            }
            KeyCode::Home => {
                self.optimization_setting_cursor = 0;
                true
            }
            KeyCode::End if field_count > 0 => {
                self.optimization_setting_cursor = field_count - 1;
                true
            }
            KeyCode::Char(' ') => {
                if let Some(field) = all_optimization_fields().nth(self.optimization_setting_cursor)
                {
                    let next = !(field.get)(&self.optimization.draft_settings);
                    (field.set)(&mut self.optimization.draft_settings, next);
                }
                true
            }
            KeyCode::Char('c') if key.modifiers.is_empty() => {
                self.apply_optimization_settings();
                true
            }
            KeyCode::Char('r') if key.modifiers.is_empty() => {
                self.optimization.draft_settings = OptimizationSettings::default();
                self.set_status("Restored optimization draft defaults");
                true
            }
            _ => false,
        }
    }

    fn handle_optimization_scripts_key(&mut self, key: KeyEvent) -> bool {
        let len = self.optimization.script_presets.len();
        match key.code {
            KeyCode::Up => {
                move_cursor_up(&mut self.optimization_script_cursor, 1);
                true
            }
            KeyCode::Char('k') if key.modifiers.is_empty() => {
                move_cursor_up(&mut self.optimization_script_cursor, 1);
                true
            }
            KeyCode::Down => {
                move_cursor_down(&mut self.optimization_script_cursor, 1, len);
                true
            }
            KeyCode::Char('j') if key.modifiers.is_empty() => {
                move_cursor_down(&mut self.optimization_script_cursor, 1, len);
                true
            }
            KeyCode::PageUp => {
                move_cursor_up(&mut self.optimization_script_cursor, 10);
                true
            }
            KeyCode::PageDown => {
                move_cursor_down(&mut self.optimization_script_cursor, 10, len);
                true
            }
            KeyCode::Home => {
                self.optimization_script_cursor = 0;
                true
            }
            KeyCode::End if len > 0 => {
                self.optimization_script_cursor = len - 1;
                true
            }
            KeyCode::Char(' ') => {
                if let Some(preset) = self
                    .optimization
                    .script_presets
                    .get_mut(self.optimization_script_cursor)
                {
                    preset.enabled = !preset.enabled;
                }
                true
            }
            KeyCode::Char('n') if key.modifiers.is_empty() => {
                self.open_path_prompt(
                    PromptKind::AddScriptPath,
                    "Add Script Path",
                    String::new(),
                    "Enter a .fb file path. Tab autocompletes.",
                );
                true
            }
            KeyCode::Char('x') if key.modifiers.is_empty() => {
                self.remove_script_preset(self.optimization_script_cursor);
                true
            }
            KeyCode::Enter => {
                self.load_script_preset_into_buffer(self.optimization_script_cursor);
                true
            }
            KeyCode::Char('c') if key.modifiers.is_empty() => {
                self.apply_optimization_settings();
                true
            }
            _ => false,
        }
    }

    fn handle_optimization_buffer_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('e') if key.modifiers.is_empty() => {
                self.open_prompt(
                    PromptKind::EditBuffer,
                    "Edit Buffer Script",
                    self.optimization.editor_buffer.clone(),
                    true,
                    "Ctrl+S saves the buffer. Esc cancels.",
                );
                true
            }
            KeyCode::Char('u') if key.modifiers.is_empty() => {
                self.apply_buffer_script();
                true
            }
            KeyCode::Char('c') if key.modifiers.is_empty() => {
                self.clear_applied_buffer_script();
                true
            }
            KeyCode::Char('o') if key.modifiers.is_empty() => {
                self.open_path_prompt(
                    PromptKind::LoadBufferPath,
                    "Load Buffer From Path",
                    self.optimization.editor_path.clone().unwrap_or_default(),
                    "Enter a .fb file path. Tab autocompletes.",
                );
                true
            }
            KeyCode::Char('s') if key.modifiers.is_empty() => {
                if let Some(path) = self.optimization.editor_path.clone() {
                    self.save_buffer_to_path(path);
                } else {
                    self.open_path_prompt(
                        PromptKind::SaveBufferPath,
                        "Save Buffer",
                        String::new(),
                        "Enter a path for the buffer script. Tab autocompletes.",
                    );
                }
                true
            }
            KeyCode::Char('S') => {
                self.open_path_prompt(
                    PromptKind::SaveBufferPath,
                    "Save Buffer As",
                    self.optimization.editor_path.clone().unwrap_or_default(),
                    "Enter a path for the buffer script. Tab autocompletes.",
                );
                true
            }
            _ => false,
        }
    }

    fn handle_patch_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Up => {
                move_cursor_up(&mut self.patch_scroll, 1);
                true
            }
            KeyCode::Char('k') if key.modifiers.is_empty() => {
                move_cursor_up(&mut self.patch_scroll, 1);
                true
            }
            KeyCode::Down => {
                self.patch_scroll = self.patch_scroll.saturating_add(1);
                true
            }
            KeyCode::Char('j') if key.modifiers.is_empty() => {
                self.patch_scroll = self.patch_scroll.saturating_add(1);
                true
            }
            KeyCode::PageUp => {
                move_cursor_up(&mut self.patch_scroll, 10);
                true
            }
            KeyCode::PageDown => {
                self.patch_scroll = self.patch_scroll.saturating_add(10);
                true
            }
            KeyCode::Home => {
                self.patch_scroll = 0;
                true
            }
            KeyCode::End => {
                self.patch_scroll = u16::MAX as usize;
                true
            }
            KeyCode::Char('s') if key.modifiers.is_empty() => {
                self.open_path_prompt(
                    PromptKind::SavePatchPath,
                    "Save Patch Preview",
                    String::new(),
                    "Enter a file path for the exported patch JSON. Tab autocompletes.",
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

        let mut keys = vec![("1-8", "views"), ("q", "quit")];
        match self.current_view {
            View::Sections => keys.extend([
                ("o", "open"),
                ("a", "analyze"),
                ("g", "analyze-all"),
                ("d", "decompile"),
                ("x", "export"),
                ("Up/Down/Pg", "move"),
                ("Home/End", "jump"),
                ("Space/Enter", "toggle"),
                ("s/Ctrl+A", "select-ready"),
            ]),
            View::Assembly | View::Ir | View::Ast => keys.extend([
                ("o", "open"),
                ("a", "analyze"),
                ("g", "analyze-all"),
                ("d", "decompile"),
                ("x", "export"),
                ("Up/Down/Pg", "move"),
                ("Home/End", "jump"),
                ("Enter", "load-editor"),
                ("e", "edit"),
            ]),
            View::Editor => keys.extend([
                ("o", "open"),
                ("a", "analyze"),
                ("g", "analyze-all"),
                ("d", "decompile"),
                ("x", "export"),
                ("e", "edit"),
                ("[ ]", "position"),
                ("Enter", "apply"),
                ("r", "reload"),
            ]),
            View::Optimization => match self.optimization_focus {
                OptimizationFocus::Settings => keys.extend([
                    ("o", "open"),
                    ("a", "analyze"),
                    ("g", "analyze-all"),
                    ("d", "decompile"),
                    ("x", "export"),
                    ("Tab", "focus"),
                    ("Up/Down/Pg", "move"),
                    ("Home/End", "jump"),
                    ("Space", "toggle"),
                    ("c", "apply"),
                    ("r", "defaults"),
                    ("L/W", "load-save"),
                ]),
                OptimizationFocus::Scripts => keys.extend([
                    ("o", "open"),
                    ("a", "analyze"),
                    ("g", "analyze-all"),
                    ("d", "decompile"),
                    ("Tab", "focus"),
                    ("Up/Down/Pg", "move"),
                    ("Home/End", "jump"),
                    ("Space", "toggle"),
                    ("n", "add"),
                    ("x", "remove"),
                    ("Enter", "load"),
                    ("c", "apply"),
                    ("L/W", "load-save"),
                ]),
                OptimizationFocus::Buffer => keys.extend([
                    ("a", "analyze"),
                    ("g", "analyze-all"),
                    ("d", "decompile"),
                    ("x", "export"),
                    ("Tab", "focus"),
                    ("e", "edit"),
                    ("u", "apply"),
                    ("c", "clear-applied"),
                    ("o", "load"),
                    ("s/S", "save"),
                    ("L/W", "load-save"),
                ]),
            },
            View::Patch => keys.extend([
                ("o", "open"),
                ("a", "analyze"),
                ("g", "analyze-all"),
                ("d", "decompile"),
                ("x", "export"),
                ("Up/Down/Pg", "scroll"),
                ("Home/End", "jump"),
                ("s", "save"),
            ]),
            View::Logs => keys.extend([
                ("o", "open"),
                ("a", "analyze"),
                ("g", "analyze-all"),
                ("d", "decompile"),
                ("x", "export"),
                ("Up/Down/Pg", "scroll"),
                ("Home/End", "jump"),
            ]),
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
