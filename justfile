# Fireman Build Recipes
# Cross-platform: Linux (bash/zsh), Windows (PowerShell via pwsh)
# PGO via cargo-pgo: `cargo install cargo-pgo`

set windows-shell := ["powershell", "-NoProfile", "-Command"]

# ── Bench ────────────────────────────────────────────────────────────

# Run benchmarks and open HTML report
[unix]
bench-open:
    cargo bench
    xdg-open target/criterion/report/index.html 2>/dev/null || open target/criterion/report/index.html 2>/dev/null || true

[windows]
bench-open:
    cargo bench
    Start-Process "target\criterion\report\index.html"

# ── PGO (cargo-pgo) ─────────────────────────────────────────────────

# Full PGO build using tests as profiling
pgo:
    cargo pgo test
    cargo pgo optimize

# Full PGO+LTO build using tests (slower compile, best runtime perf)
[unix]
pgo-lto:
    CARGO_PROFILE_RELEASE_LTO=fat CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 cargo pgo test
    CARGO_PROFILE_RELEASE_LTO=fat CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 cargo pgo optimize

[windows]
pgo-lto:
    $env:CARGO_PROFILE_RELEASE_LTO = "fat"; $env:CARGO_PROFILE_RELEASE_CODEGEN_UNITS = "1"; cargo pgo test
    $env:CARGO_PROFILE_RELEASE_LTO = "fat"; $env:CARGO_PROFILE_RELEASE_CODEGEN_UNITS = "1"; cargo pgo optimize

# Check PGO/BOLT environment readiness
pgo-info:
    cargo pgo info

# Clean PGO/BOLT artifacts
pgo-clean:
    cargo pgo clean

# ── Utilities ────────────────────────────────────────────────────────

# Clean all build artifacts
clean:
    cargo clean
