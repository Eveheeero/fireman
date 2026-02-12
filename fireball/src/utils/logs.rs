struct CompactFormatter;

impl<S, N> tracing_subscriber::fmt::FormatEvent<S, N> for CompactFormatter
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    N: for<'a> tracing_subscriber::fmt::FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: tracing_subscriber::fmt::format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        let metadata = event.metadata();

        let level = match *metadata.level() {
            tracing::Level::TRACE => "T",
            tracing::Level::DEBUG => "D",
            tracing::Level::INFO => "I",
            tracing::Level::WARN => "W",
            tracing::Level::ERROR => "E",
        };

        let file = metadata.file().unwrap_or("unknown");
        let shortened_path = if let Some(stripped) = file.strip_prefix("fireball/src/") {
            format!("fb/{}", stripped)
        } else if let Some(stripped) = file.strip_prefix("fireman/src/") {
            format!("fm/{}", stripped)
        } else {
            file.to_string()
        };

        write!(writer, "{} {}: ", level, shortened_path)?;

        use tracing_subscriber::fmt::FormatFields as _;
        ctx.format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

pub fn test_init_with_log_file(log_file_path: &str) {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        use tracing_subscriber::util::SubscriberInitExt;
        let _ = test_log_subscriber_with_file(log_file_path).try_init();
    })
}

pub fn test_log_subscriber_with_file(log_file_path: &str) -> impl tracing::Subscriber {
    use tracing_subscriber::{Layer, prelude::__tracing_subscriber_SubscriberExt};

    let log_verbose = std::env::var("FIREMAN_VERBOSE_LOG")
        .map(|value| {
            let value = value.to_ascii_lowercase();
            matches!(value.as_str(), "1" | "true" | "yes" | "on")
        })
        .unwrap_or(false);
    let stdio_level = if log_verbose {
        tracing_subscriber::filter::LevelFilter::TRACE
    } else {
        tracing_subscriber::filter::LevelFilter::ERROR
    };

    std::fs::create_dir_all(std::path::Path::new(log_file_path).parent().unwrap()).unwrap();
    let file = std::fs::File::create(log_file_path).unwrap();
    let subscriber = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .event_format(CompactFormatter)
                .with_filter(stdio_level),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(file)
                .event_format(CompactFormatter),
        )
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_target("fireball", tracing::Level::TRACE),
        );
    subscriber
}
