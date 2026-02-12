use fireball::{core::Fire, pe::Pe};

fn test_init() {
    use tracing_subscriber::{
        Layer, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
    };

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

            // INFO -> I
            let level = match *metadata.level() {
                tracing::Level::TRACE => "T",
                tracing::Level::DEBUG => "D",
                tracing::Level::INFO => "I",
                tracing::Level::WARN => "W",
                tracing::Level::ERROR => "E",
            };

            // fireball/src/main.rs -> fb/main.rs
            let file = metadata.file().unwrap_or("unknown");
            let shortened_path = if let Some(stripped) = file.strip_prefix("fireball/src/") {
                format!("fb/{}", stripped)
            } else {
                file.to_string()
            };

            let line = metadata.line().unwrap_or(0);

            // I fb/main.rs:10
            write!(writer, "{} {}:{}: ", level, shortened_path, line)?;

            use tracing_subscriber::fmt::FormatFields as _;
            ctx.format_fields(writer.by_ref(), event)?;
            writeln!(writer)
        }
    }

    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let file = std::fs::File::create("fireball_outside.log").unwrap();
        let _ = tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .without_time()
                    .with_file(true)
                    .with_line_number(true)
                    .with_target(false)
                    .event_format(CompactFormatter)
                    .with_filter(stdio_level),
            )
            .with(
                tracing_subscriber::fmt::layer()
                    .without_time()
                    .with_file(true)
                    .with_line_number(true)
                    .with_target(false)
                    .with_ansi(false)
                    .with_writer(file)
                    .event_format(CompactFormatter),
            )
            .with(
                tracing_subscriber::filter::Targets::new()
                    .with_target("fireball", tracing::Level::TRACE),
            )
            .try_init();
    });
}
fn get_binary() -> &'static [u8] {
    include_bytes!("resources/hello_world.exe")
}

#[test]
fn hello_world() {
    test_init();
    let binary = get_binary();

    let pe = Pe::from_binary(binary.to_vec()).unwrap();
    let result = pe.decompile_all().unwrap();
    println!("{}", &result);
    std::fs::write("hello_world_result.log", &result).unwrap();
    assert!(true);
}
