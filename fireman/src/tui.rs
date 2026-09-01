mod help;
mod navigate_input;
mod tab;

use crate::{JsonPreset, JsonPresetOptimizationConfig, TuiArgs};
use crossterm::{
    event,
    event::{KeyCode, KeyModifiers},
};
use fireball::{Fireball, pattern_matching::AstPattern};
use ratatui::Frame;

pub fn tui(args: TuiArgs) {
    let mut terminal = ratatui::init();
    let mut app = TuiApp::new();
    app.init(args);
    let result = app.run(&mut terminal);
    ratatui::restore();
    result.unwrap();
}

struct TuiApp {
    state: TuiState,
    fireball: Option<Fireball>,
    optimizations: Vec<fireball::abstract_syntax_tree::AstOptimizationKind>,
    print_config: fireball::abstract_syntax_tree::AstPrintConfig,
    data: TuiData<'static>,
}
#[derive(Default)]
struct TuiData<'tui> {
    navigate_input: navigate_input::TuiNavigateInputData<'tui>,
    tab: tab::TuiTabData<'tui>,
}

#[derive(PartialEq)]
enum TuiState {
    TempState,
    Init,
    NavigateInput,
    Tab(usize),
    Help { previous_state: Box<TuiState> },
    Exit,
}

impl TuiApp {
    fn new() -> Self {
        Self {
            state: TuiState::Init,
            fireball: None,
            optimizations: Default::default(),
            print_config: Default::default(),
            data: Default::default(),
        }
    }
    fn init(&mut self, args: TuiArgs) {
        let TuiArgs {
            input,
            custom_script,
            json: json_preset_path,
        } = args;

        // input
        let fireball = match input {
            Some(input) => {
                self.data.navigate_input.input = input.clone();
                fireball::Fireball::from_path(&input).ok()
            }
            None => None,
        };
        if fireball.is_some() {
            self.fireball = fireball;
            self.state = TuiState::Tab(0);
        } else {
            self.state = TuiState::NavigateInput;
        }

        // json
        let json_preset: JsonPreset = if let Some(json_preset_path) = json_preset_path
            && let Ok(json_preset) = std::fs::read_to_string(&json_preset_path)
            && let Ok(json_preset) = serde_json::from_str(&json_preset)
        {
            json_preset
        } else {
            Default::default()
        };
        let mut optimizations: Vec<_> = json_preset
            .optimizations
            .into_iter()
            .map(JsonPresetOptimizationConfig::to_fireball_optimization)
            .collect();
        for path in custom_script
            .into_iter()
            .chain(json_preset.custom_script.into_iter())
        {
            let content = std::fs::read_to_string(&path);
            match content {
                Ok(content) => {
                    optimizations.push(
                        fireball::abstract_syntax_tree::AstOptimizationKind::PatternMatching(
                            Box::new(AstPattern::new(path, content)),
                        ),
                    );
                }
                Err(_) => {}
            }
        }
        let print_config = json_preset.print_config.to_fireball_print_config();
        self.optimizations = optimizations;
        self.print_config = print_config;
    }
    fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
        while self.state != TuiState::Exit {
            terminal.draw(|frame| self.draw(frame))?;
            if event::poll(std::time::Duration::from_millis(50))? {
                let event = event::read()?;
                self.handle_event(event);
            }
        }
        Ok(())
    }

    fn draw(&mut self, terminal: &mut Frame) {
        match self.state {
            TuiState::TempState => {}
            TuiState::Init => {}
            TuiState::NavigateInput => navigate_input::draw(self, terminal),
            TuiState::Tab(_) => tab::draw(self, terminal),
            TuiState::Help { .. } => help::draw(self, terminal),
            TuiState::Exit => {}
        }
    }
    fn handle_event(&mut self, event: event::Event) {
        // handle interrupt
        if event.is_key_press()
            && let Some(event) = event.as_key_press_event()
            && event.modifiers == KeyModifiers::CONTROL
            && event.code == KeyCode::Char('c')
        {
            self.state = TuiState::Exit;
            return;
        }
        if handle_help(self, &event) {
            return;
        }

        match self.state {
            TuiState::TempState => {}
            TuiState::Init => {}
            TuiState::NavigateInput => navigate_input::handle_event(self, event),
            TuiState::Tab(_) => tab::handle_event(self, event),
            TuiState::Help { .. } => help::handle_event(self, event),
            TuiState::Exit => {}
        }
    }
}

/// ### Returns
/// bool -> true if handled
fn handle_help(app: &mut TuiApp, event: &event::Event) -> bool {
    if event.is_key_press()
        && let Some(event) = event.as_key_press_event()
        && event.code == KeyCode::F(1)
    {
        match &mut app.state {
            TuiState::Help { previous_state } => {
                let mut temp = TuiState::TempState;
                std::mem::swap(&mut temp, previous_state);
                app.state = temp;
                return true;
            }
            state => {
                let mut old_state = TuiState::TempState;
                std::mem::swap(&mut old_state, state);
                app.state = TuiState::Help {
                    previous_state: Box::new(old_state),
                };
            }
        }

        return true;
    }
    false
}
