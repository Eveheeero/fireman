//! Demo of Low IR generation from a simple binary

use fireball::arch::x86_64::lifter::X64Lifter;
use fireball::core::FireRaw;
use fireball::pe::Pe;

fn main() {
    // Load test binary
    let binary_path = "fireball/tests/resources/hello_world.exe";
    let binary_data = std::fs::read(binary_path).expect("Failed to read hello_world.exe");

    // Create Pe instance and analyze
    let fire = Pe::from_binary(binary_data).unwrap();
    fire.analyze_from_entry().unwrap();

    // Get first block
    let blocks = fire.get_blocks().get_all();
    if blocks.is_empty() {
        println!("No blocks found!");
        return;
    }

    let first_block = &blocks[0];
    println!(
        "Converting block at address: {}",
        first_block.get_start_address()
    );

    // Get IR block
    let ir_guard = first_block.get_ir();
    let ir_block = match &*ir_guard {
        Some(ir) => ir,
        None => {
            println!("No IR available for block");
            return;
        }
    };

    // Create lifter and convert to Low IR
    let mut lifter = X64Lifter::new();
    match lifter.lift_block(ir_block, first_block.get_start_address().clone()) {
        Ok(module) => {
            println!("\n=== Low IR Module ===");
            println!(
                "Target: {}-bit {} ({})",
                module.target.bits,
                module.target.arch,
                match module.target.endian {
                    fireball::ir::low_ir::Endianness::Little => "little-endian",
                    fireball::ir::low_ir::Endianness::Big => "big-endian",
                }
            );

            println!("\nFunctions: {}", module.functions.len());

            for (func_id, function) in &module.functions {
                println!("\nFunction {:016x}:", func_id.0);
                println!("  Entry block: {:016x}", function.entry.0);
                println!("  Blocks: {}", function.blocks.len());
                println!("  Locals: {}", function.locals.len());

                for (block_id, block) in &function.blocks {
                    println!("\n  Block {:016x}:", block_id.0);
                    println!("    Instructions: {}", block.instructions.len());

                    // Show first few instructions
                    for (i, inst) in block.instructions.iter().take(5).enumerate() {
                        println!("      [{}] {:?}", i, inst);
                    }

                    if block.instructions.len() > 5 {
                        println!(
                            "      ... {} more instructions",
                            block.instructions.len() - 5
                        );
                    }

                    println!("    Terminator: {:?}", block.terminator);
                }
            }
        }
        Err(e) => {
            println!("Failed to lift block: {}", e);
        }
    }
}
