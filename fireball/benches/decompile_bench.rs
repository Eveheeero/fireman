use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use fireball::{Fire, Fireball};
#[cfg(unix)]
use pprof::criterion::{Output, PProfProfiler};
use std::{hint::black_box, time::Duration};

fn benchmark_decompile_from_entry(c: &mut Criterion) {
    let binary = include_bytes!("../tests/resources/hello_world.exe").to_vec();

    c.bench_function("fireball/decompile_from_entry/hello_world", |b| {
        b.iter_batched(
            || {
                Fireball::from_binary(binary.clone())
                    .expect("failed to create Fireball from binary")
            },
            |fireball| {
                let decompiled = fireball
                    .decompile_from_entry()
                    .expect("decompile_from_entry must succeed");
                black_box(decompiled);
            },
            BatchSize::SmallInput,
        );
    });
}

#[cfg(unix)]
fn benchmark_config() -> Criterion {
    Criterion::default()
        .sample_size(30)
        .warm_up_time(Duration::from_secs(2))
        .measurement_time(Duration::from_secs(10))
        .with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)))
}

#[cfg(not(unix))]
fn benchmark_config() -> Criterion {
    Criterion::default()
        .sample_size(30)
        .warm_up_time(Duration::from_secs(2))
        .measurement_time(Duration::from_secs(10))
}

criterion_group! {
    name = fireball_decompile_benches;
    config = benchmark_config();
    targets = benchmark_decompile_from_entry
}
criterion_main!(fireball_decompile_benches);
