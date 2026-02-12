//! Module containing commonly used `use` statements and utilities

#![allow(unused_imports)]

pub(crate) use crate::utils::error::{
    FireballError, decompile_error::DecompileError, disassemble_error::DisassembleError,
    io_error::IoError,
};
pub(crate) use tracing::{debug, error, info, trace, warn};

pub(crate) type BitBox = bitvec::prelude::BitBox<usize>;
pub(crate) type BitSlice = bitvec::prelude::BitSlice<usize>;

#[cfg(test)]
pub(crate) fn test_init() {
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
        let file = std::fs::File::create("fireball.log").unwrap();
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

#[cfg(test)]
mod tests {
    /// Tests the storage order of bits in the bitvec implementation (Msb0)
    /// Includes methods for data manipulation and casting.
    #[test]
    fn bit_align_test_msb0() {
        /* Converting to BitPtr and using Cast is unstable, so this operation must be used. */
        let mut bits = bitvec::prelude::BitVec::<usize, bitvec::order::Msb0>::new();
        bits.resize(8, false);
        let mut bits = bits.into_boxed_bitslice();
        bits.set(7, true);
        let data = bits[0..8].iter().fold(0, |acc, x| (acc << 1) | *x as u8);
        assert_eq!(data, 0b00000001)
    }

    /// Tests the storage order of bits in the bitvec implementation (Lsb0)
    /// Includes methods for data manipulation and casting.
    #[test]
    fn bit_align_test_lsb0() {
        /* Converting to BitPtr and using Cast is unstable, so this operation must be used. */
        let mut bits = bitvec::prelude::BitVec::<usize>::new();
        bits.resize(8, false);
        let mut bits = bits.into_boxed_bitslice();
        bits.set(7, true);
        let data = bits[0..8]
            .iter()
            .rev()
            .fold(0, |acc, x| (acc << 1) | *x as u8);
        assert_eq!(data, 0b10000000)
    }
}
