//! Test module

pub(super) fn hello_world_binary() -> &'static [u8] {
    include_bytes!("../../tests/resources/hello_world.exe")
}

mod embedded_parity;
mod optimizer_passes;
mod pattern_matching_files;
mod pe_error_paths;
mod pe_hello_world;
