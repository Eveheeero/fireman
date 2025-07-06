use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

pub fn init_log() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let file = std::fs::File::create("fireman.log").unwrap();
        let _ = tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .without_time()
                    .with_file(true)
                    .with_line_number(true)
                    .with_target(false)
                    .with_ansi(false)
                    .with_writer(file),
            )
            .with(
                tracing_subscriber::filter::Targets::new()
                    .with_target("fireball", tracing::Level::TRACE)
                    .with_target("fireman", tracing::Level::TRACE),
            )
            .try_init();
    });
}
