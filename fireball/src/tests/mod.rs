//! Test module

pub(super) fn hello_world_binary() -> &'static [u8] {
    include_bytes!("../../tests/resources/hello_world.exe")
}

mod optimizer_passes;
mod pe_error_paths;
mod pe_hello_world;
