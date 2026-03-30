use super::hello_world_elf_binary;
use crate::{
    core::{Fire, FireRaw},
    elf::Elf,
    utils::test_log_subscriber_with_file,
};
use tracing::Dispatch;

#[test]
fn elf_hello_world_load() {
    let subscriber = test_log_subscriber_with_file("logs/fireball_elf_hello_world.log");

    tracing::dispatcher::with_default(&Dispatch::new(subscriber), || {
        let binary = hello_world_elf_binary();
        let elf = Elf::from_binary(binary.to_vec()).unwrap();
        dbg!(&elf);
    });
}

#[test]
fn elf_hello_world_via_fireball_enum() {
    let subscriber =
        test_log_subscriber_with_file("logs/fireball_elf_hello_world_fireball_enum.log");

    tracing::dispatcher::with_default(&Dispatch::new(subscriber), || {
        let binary = hello_world_elf_binary();
        let fireball = crate::Fireball::from_binary(binary.to_vec()).unwrap();
        // Should detect as ELF
        assert!(matches!(fireball, crate::Fireball::Elf(_)));
    });
}

#[test]
fn elf_hello_world_entry_analysis() {
    let subscriber =
        test_log_subscriber_with_file("logs/fireball_elf_hello_world_entry_analysis.log");

    tracing::dispatcher::with_default(&Dispatch::new(subscriber), || {
        let binary = hello_world_elf_binary();
        let elf = Elf::from_binary(binary.to_vec()).unwrap();
        let block = elf.analyze_from_entry().unwrap();
        // The entry block should have at least one instruction
        assert!(
            !block.get_instructions().is_empty(),
            "Entry block should contain instructions"
        );
    });
}

#[test]
fn elf_hello_world_decompile_from_entry() {
    let subscriber =
        test_log_subscriber_with_file("logs/fireball_elf_hello_world_decompile_entry.log");

    tracing::dispatcher::with_default(&Dispatch::new(subscriber), || {
        let binary = hello_world_elf_binary();
        let elf = Elf::from_binary(binary.to_vec()).unwrap();
        let result = elf.decompile_from_entry().unwrap();
        assert!(!result.is_empty(), "Decompiled output should be non-empty");
    });
}

#[test]
fn elf_hello_world_sections_loaded() {
    let subscriber = test_log_subscriber_with_file("logs/fireball_elf_hello_world_sections.log");

    tracing::dispatcher::with_default(&Dispatch::new(subscriber), || {
        let binary = hello_world_elf_binary();
        let elf = Elf::from_binary(binary.to_vec()).unwrap();
        let sections = elf.get_sections();
        let all = sections.all();
        // A statically-linked ELF should have at least .text
        assert!(
            !all.is_empty(),
            "ELF should have at least one loadable section"
        );
        // Verify that .text section exists and is executable
        let text_section = all.iter().find(|s| s.name == ".text");
        assert!(text_section.is_some(), "ELF should have a .text section");
        assert!(
            text_section.unwrap().is_executable(),
            ".text section should be executable"
        );
    });
}
