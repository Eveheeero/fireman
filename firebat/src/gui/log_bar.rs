use crate::Firebat;
use eframe::egui;
use std::{
    collections::VecDeque,
    sync::{LazyLock, Mutex},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Maximum amount of log lines kept in memory.
const LOG_CAPACITY: usize = 1000;

static LOGS: LazyLock<Mutex<VecDeque<String>>> = LazyLock::new(|| Mutex::new(VecDeque::new()));

/// Height of the collapsed log bar in points.
pub const LOG_BAR_HEIGHT: f32 = 18.0;

/// Height of the expanded log overlay in points.
const LOG_EXPANDED_HEIGHT: f32 = 200.0;

/// Horizontal padding of the log text, in points.
const LOG_PADDING_X: f32 = 8.0;

#[derive(Debug, Default)]
pub struct LogBarData {
    pub expanded: bool,
}

/// Installs the tracing subscriber which feeds the log bar.
pub fn init_tracing() {
    tracing_subscriber::registry().with(LogCollector).init();
}

/// Crates whose events are shown in the log bar.
const LOG_TARGETS: [&str; 2] = ["fireball", "firebat"];

/// Lowest level shown in the log bar.
const LOG_LEVEL: tracing::Level = tracing::Level::DEBUG;

struct LogCollector;

fn is_collected(metadata: &tracing::Metadata<'_>) -> bool {
    if *metadata.level() > LOG_LEVEL {
        return false;
    }
    let target = metadata.target();
    LOG_TARGETS.iter().any(|scope| {
        target == *scope
            || target
                .strip_prefix(scope)
                .is_some_and(|r| r.starts_with("::"))
    })
}

impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for LogCollector {
    fn enabled(
        &self,
        metadata: &tracing::Metadata<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        is_collected(metadata)
    }

    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if !is_collected(event.metadata()) {
            return;
        }
        let mut visitor = MessageVisitor(String::new());
        event.record(&mut visitor);
        let time = chrono::Local::now().format("%H:%M:%S");
        let level = event.metadata().level();
        let mut logs = LOGS.lock().unwrap();
        while logs.len() >= LOG_CAPACITY {
            logs.pop_front();
        }
        logs.push_back(format!("[{time}] {level} {}", visitor.0));
    }
}

struct MessageVisitor(String);

impl tracing::field::Visit for MessageVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() != "message" {
            return;
        }
        if !self.0.is_empty() {
            self.0.push(' ');
        }
        self.0.push_str(&format!("{value:?}"));
    }
}

pub fn ui(app: &mut Firebat, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    let mut bar_rect = egui::Rect::NOTHING;

    egui::panel::Panel::bottom("log_bar")
        .exact_size(LOG_BAR_HEIGHT)
        .resizable(false)
        .frame(egui::Frame::NONE.fill(egui::Color32::from_gray(16)))
        .show(ui, |ui| {
            style(ui);
            bar_rect = ui.max_rect();

            let response = ui.interact(
                bar_rect,
                ui.id().with("log_bar_toggle"),
                egui::Sense::click(),
            );
            if response.clicked() {
                app.log_bar.expanded = !app.log_bar.expanded;
            }
            if response.hovered() {
                ui.painter()
                    .rect_filled(bar_rect, 0.0, egui::Color32::from_gray(32));
            }

            let logs = LOGS.lock().unwrap();
            let latest = logs.back().map(String::as_str).unwrap_or("");
            let text_pos = egui::pos2(bar_rect.left() + LOG_PADDING_X, bar_rect.center().y);
            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                latest,
                egui::TextStyle::Small.resolve(ui.style()),
                egui::Color32::from_gray(200),
            );
        });

    if app.log_bar.expanded {
        expanded(ui.ctx(), bar_rect);
    }
}

/// Draws the expanded log list as an overlay so that no other panel moves.
fn expanded(ctx: &egui::Context, bar_rect: egui::Rect) {
    let height = LOG_EXPANDED_HEIGHT.min(bar_rect.top() - 0.0);
    let rect = egui::Rect::from_min_max(
        egui::pos2(bar_rect.left(), bar_rect.top() - height),
        egui::pos2(bar_rect.right(), bar_rect.top()),
    );

    egui::Area::new(egui::Id::new("log_bar_expanded"))
        .order(egui::Order::Foreground)
        .fixed_pos(rect.min)
        .show(ctx, |ui| {
            ui.set_width(rect.width());
            ui.set_height(rect.height());
            egui::Frame::NONE
                .fill(egui::Color32::from_gray(16))
                .inner_margin(egui::Margin::symmetric(LOG_PADDING_X as i8, 4))
                .show(ui, |ui| {
                    style(ui);
                    egui::ScrollArea::vertical()
                        .stick_to_bottom(true)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for log in LOGS.lock().unwrap().iter() {
                                ui.label(
                                    egui::RichText::new(log)
                                        .color(egui::Color32::from_gray(200))
                                        .text_style(egui::TextStyle::Small),
                                );
                            }
                        });
                });
        });
}

fn style(ui: &mut egui::Ui) {
    let style = ui.style_mut();
    style.spacing.item_spacing = egui::vec2(0.0, 2.0);
    for font in style.text_styles.values_mut() {
        *font = egui::FontId::monospace(11.0);
    }
}
