//! Example demonstrating calling convention support

use fireball::arch::{
    ArchType, CallingConventionInfo, CallingConventionProvider, OperatingSystem, ParamLocation,
    ParamType, get_calling_convention_provider,
};
use fireball::ir::low_ir::CallConv;

fn main() {
    println!("=== Calling Convention Demo ===\n");

    // Demonstrate x86-64 System V calling convention (Linux)
    demo_x64_sysv();

    // Demonstrate x86-64 Windows calling convention
    demo_x64_win64();

    // Demonstrate ARM64 calling conventions
    demo_arm64();

    // Demonstrate parameter passing
    demo_parameter_passing();
}

fn demo_x64_sysv() {
    println!("--- x86-64 System V ABI (Linux) ---");

    let provider = get_calling_convention_provider(ArchType::X86_64, OperatingSystem::Linux);
    let info = provider.get_convention_info(CallConv::X86_64SysV);

    println!("Name: {}", info.name);
    println!("Integer argument registers: {:?}", info.int_arg_registers);
    println!("Float argument registers: {:?}", info.float_arg_registers);
    println!("Return registers: {:?}", info.return_registers);
    println!("Red zone size: {} bytes", info.red_zone_size);
    println!("Shadow space: {} bytes", info.shadow_space);
    println!();
}

fn demo_x64_win64() {
    println!("--- x86-64 Windows ---");

    let provider = get_calling_convention_provider(ArchType::X86_64, OperatingSystem::Windows);
    let info = provider.get_convention_info(CallConv::X86_64Win64);

    println!("Name: {}", info.name);
    println!("Integer argument registers: {:?}", info.int_arg_registers);
    println!("Float argument registers: {:?}", info.float_arg_registers);
    println!("Return registers: {:?}", info.return_registers);
    println!("Red zone size: {} bytes", info.red_zone_size);
    println!("Shadow space: {} bytes", info.shadow_space);
    println!();
}

fn demo_arm64() {
    println!("--- ARM64 AAPCS ---");

    let provider = get_calling_convention_provider(ArchType::Arm64, OperatingSystem::Linux);
    let info = provider.get_convention_info(CallConv::Arm64Aapcs);

    println!("Name: {}", info.name);
    println!("Integer argument registers: {:?}", info.int_arg_registers);
    println!("Float argument registers: {:?}", info.float_arg_registers);
    println!("Return registers: {:?}", info.return_registers);
    println!("Stack alignment: {} bytes", info.stack_alignment);
    println!();

    // Darwin variant
    println!("--- ARM64 AAPCS Darwin ---");
    let darwin_info = provider.get_convention_info(CallConv::Arm64AapcsDarwin);
    println!("Name: {}", darwin_info.name);
    println!("Red zone size: {} bytes", darwin_info.red_zone_size);
    println!();
}

fn demo_parameter_passing() {
    println!("--- Parameter Passing Demo ---");

    let provider = get_calling_convention_provider(ArchType::X86_64, OperatingSystem::Linux);
    let info = provider.get_convention_info(CallConv::X86_64SysV);

    let mut used_int_regs = 0;
    let mut used_float_regs = 0;
    let mut stack_offset = 0;

    // Simulate passing parameters to a function
    let params = vec![
        ("int1", ParamType::Integer),
        ("float1", ParamType::Float),
        ("int2", ParamType::Integer),
        ("double1", ParamType::Double),
        ("int3", ParamType::Integer),
        ("int4", ParamType::Integer),
        ("int5", ParamType::Integer),
        ("int6", ParamType::Integer),
        ("int7", ParamType::Integer), // This should go on stack
        ("struct1", ParamType::Aggregate { size: 24 }), // Large struct
    ];

    println!("Function parameters:");
    for (i, (name, param_type)) in params.iter().enumerate() {
        let location = info.get_param_location(
            i,
            *param_type,
            &mut used_int_regs,
            &mut used_float_regs,
            &mut stack_offset,
        );

        match location {
            ParamLocation::Register(reg) => {
                println!("  {}: {} -> register {}", i + 1, name, reg);
            }
            ParamLocation::FloatRegister(reg) => {
                println!("  {}: {} -> float register {}", i + 1, name, reg);
            }
            ParamLocation::Stack { offset } => {
                println!("  {}: {} -> stack offset {}", i + 1, name, offset);
            }
            ParamLocation::Split {
                registers,
                stack_bytes,
            } => {
                println!(
                    "  {}: {} -> split: registers {:?}, stack {} bytes",
                    i + 1,
                    name,
                    registers,
                    stack_bytes
                );
            }
        }
    }
    println!();

    // Show PreserveAll convention
    println!("--- PreserveAll Convention ---");
    let preserve_info = provider.get_convention_info(CallConv::PreserveAll);
    println!("Name: {}", preserve_info.name);
    println!("Caller-saved registers: {:?}", preserve_info.caller_saved);
    println!(
        "Callee-saved registers: {} registers total",
        preserve_info.callee_saved.len()
    );
    println!("(All registers are preserved by the callee)");
}
