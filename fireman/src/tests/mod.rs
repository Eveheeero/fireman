pub(super) fn hello_world_binary() -> &'static [u8] {
    include_bytes!("../../../fireball/tests/resources/hello_world.exe")
}

mod hello_world;
