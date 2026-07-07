mod help;
mod navigate_input;
mod tab;

use crate::{JsonPreset, TuiArgs};
use crossterm::event;
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
    tabs: Vec<TuiTab>,
    optimization_config: fireball::abstract_syntax_tree::AstOptimizationConfig,
    print_config: fireball::abstract_syntax_tree::AstPrintConfig,
}
struct TuiTab {
    index: usize,
    kind: TuiTabKind,
}
enum TuiTabKind {}

#[derive(PartialEq)]
enum TuiState {
    Init,
    NavigateInput,
    Tab(usize),
    Help(/* previous state */ Box<TuiState>),
    Exit,
}

impl TuiApp {
    fn new() -> Self {
        Self {
            state: TuiState::Init,
            fireball: None,
            tabs: Vec::new(),
            optimization_config: Default::default(),
            print_config: Default::default(),
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
            Some(input) => fireball::Fireball::from_path(&input).ok(),
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
        let mut optimization_config = json_preset
            .optimization_config
            .to_fireball_optimization_config();
        for path in custom_script
            .into_iter()
            .chain(json_preset.custom_script.into_iter())
        {
            let content = std::fs::read_to_string(&path);
            match content {
                Ok(content) => {
                    optimization_config
                        .pattern_matching
                        .push(AstPattern::new(path, content));
                }
                Err(_) => {}
            }
        }
        let print_config = json_preset.print_config.to_fireball_print_config();
        self.optimization_config = optimization_config;
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
        todo!()
    }
    fn handle_event(&mut self, event: event::Event) {
        todo!()
    }
}
