//! Tests for calling convention support

use fireball::arch::{
    ArchType, OperatingSystem, ParamLocation, ParamType, get_calling_convention_provider,
};
use fireball::ir::low_ir::CallConv;

#[test]
fn test_x86_calling_conventions() {
    let provider = get_calling_convention_provider(ArchType::X86, OperatingSystem::Windows);

    // Test cdecl
    let cdecl = provider.get_convention_info(CallConv::C);
    assert_eq!(cdecl.name, "cdecl");
    assert!(cdecl.int_arg_registers.is_empty());
    assert!(cdecl.caller_cleanup);

    // Test stdcall
    let stdcall = provider.get_convention_info(CallConv::X86Stdcall);
    assert_eq!(stdcall.name, "stdcall");
    assert!(!stdcall.caller_cleanup);

    // Test fastcall
    let fastcall = provider.get_convention_info(CallConv::X86Fastcall);
    assert_eq!(fastcall.name, "fastcall");
    assert_eq!(fastcall.int_arg_registers, vec!["ecx", "edx"]);

    // Test thiscall
    let thiscall = provider.get_convention_info(CallConv::X86Thiscall);
    assert_eq!(thiscall.name, "thiscall");
    assert_eq!(thiscall.int_arg_registers, vec!["ecx"]);

    // Test vectorcall
    let vectorcall = provider.get_convention_info(CallConv::X86Vectorcall);
    assert_eq!(vectorcall.name, "vectorcall");
    assert_eq!(vectorcall.float_arg_registers.len(), 6);
}

#[test]
fn test_x64_sysv_convention() {
    let provider = get_calling_convention_provider(ArchType::X86_64, OperatingSystem::Linux);
    let info = provider.get_convention_info(CallConv::X86_64SysV);

    assert_eq!(info.name, "System V AMD64 ABI");
    assert_eq!(info.int_arg_registers.len(), 6);
    assert_eq!(info.int_arg_registers[0], "rdi");
    assert_eq!(info.int_arg_registers[1], "rsi");
    assert_eq!(info.float_arg_registers.len(), 8);
    assert_eq!(info.red_zone_size, 128);
    assert_eq!(info.shadow_space, 0);
    assert_eq!(info.stack_alignment, 16);
}

#[test]
fn test_x64_win64_convention() {
    let provider = get_calling_convention_provider(ArchType::X86_64, OperatingSystem::Windows);
    let info = provider.get_convention_info(CallConv::X86_64Win64);

    assert_eq!(info.name, "Microsoft x64");
    assert_eq!(info.int_arg_registers.len(), 4);
    assert_eq!(info.int_arg_registers[0], "rcx");
    assert_eq!(info.int_arg_registers[1], "rdx");
    assert_eq!(info.shadow_space, 32);
    assert_eq!(info.red_zone_size, 0);
}

#[test]
fn test_arm_calling_conventions() {
    // ARM32
    let arm32_provider = get_calling_convention_provider(ArchType::Arm32, OperatingSystem::Linux);
    let aapcs = arm32_provider.get_convention_info(CallConv::ArmAapcs);
    assert_eq!(aapcs.name, "AAPCS (ARM)");
    assert_eq!(aapcs.int_arg_registers, vec!["r0", "r1", "r2", "r3"]);

    let aapcs_vfp = arm32_provider.get_convention_info(CallConv::ArmAapcsVfp);
    assert_eq!(aapcs_vfp.name, "AAPCS-VFP (ARM)");
    assert_eq!(aapcs_vfp.float_arg_registers.len(), 16);

    // ARM64
    let arm64_provider = get_calling_convention_provider(ArchType::Arm64, OperatingSystem::Linux);
    let aapcs64 = arm64_provider.get_convention_info(CallConv::Arm64Aapcs);
    assert_eq!(aapcs64.name, "AAPCS64");
    assert_eq!(aapcs64.int_arg_registers.len(), 8);
    assert_eq!(aapcs64.int_arg_registers[0], "x0");

    let darwin = arm64_provider.get_convention_info(CallConv::Arm64AapcsDarwin);
    assert_eq!(darwin.name, "AAPCS64 Darwin");
    assert_eq!(darwin.red_zone_size, 128);
}

#[test]
fn test_preserve_conventions() {
    let provider = get_calling_convention_provider(ArchType::X86_64, OperatingSystem::Linux);

    let preserve_all = provider.get_convention_info(CallConv::PreserveAll);
    assert_eq!(preserve_all.name, "preserve_all");
    assert!(preserve_all.caller_saved.is_empty());
    assert!(preserve_all.callee_saved.len() > 20); // Many registers preserved

    let preserve_most = provider.get_convention_info(CallConv::PreserveMost);
    assert_eq!(preserve_most.name, "preserve_most");
    assert!(!preserve_most.caller_saved.is_empty());
    assert!(preserve_most.callee_saved.len() > 8);
}

#[test]
fn test_parameter_locations() {
    let provider = get_calling_convention_provider(ArchType::X86_64, OperatingSystem::Linux);
    let info = provider.get_convention_info(CallConv::X86_64SysV);

    let mut used_int = 0;
    let mut used_float = 0;
    let mut stack_offset = 0;

    // First integer parameter should go in rdi
    let loc1 = info.get_param_location(
        0,
        ParamType::Integer,
        &mut used_int,
        &mut used_float,
        &mut stack_offset,
    );
    assert_eq!(loc1, ParamLocation::Register("rdi"));
    assert_eq!(used_int, 1);

    // First float parameter should go in xmm0
    let loc2 = info.get_param_location(
        1,
        ParamType::Float,
        &mut used_int,
        &mut used_float,
        &mut stack_offset,
    );
    assert_eq!(loc2, ParamLocation::FloatRegister("xmm0"));
    assert_eq!(used_float, 1);

    // After using all int registers, should go to stack
    used_int = 6; // All registers used
    let loc3 = info.get_param_location(
        7,
        ParamType::Integer,
        &mut used_int,
        &mut used_float,
        &mut stack_offset,
    );
    match loc3 {
        ParamLocation::Stack { offset } => assert_eq!(offset, 0),
        _ => panic!("Expected stack location"),
    }

    // Small aggregate might use registers
    used_int = 0;
    used_float = 0;
    stack_offset = 0;
    let loc4 = info.get_param_location(
        0,
        ParamType::Aggregate { size: 16 },
        &mut used_int,
        &mut used_float,
        &mut stack_offset,
    );
    match loc4 {
        ParamLocation::Split {
            registers,
            stack_bytes,
        } => {
            assert_eq!(registers.len(), 2); // 16 bytes = 2 registers
            assert_eq!(stack_bytes, 0);
        }
        _ => panic!("Expected split location"),
    }
}

#[test]
fn test_default_conventions() {
    // Linux defaults
    let linux_x64 = get_calling_convention_provider(ArchType::X86_64, OperatingSystem::Linux);
    assert_eq!(linux_x64.get_default_convention(), CallConv::X86_64SysV);

    let linux_x86 = get_calling_convention_provider(ArchType::X86, OperatingSystem::Linux);
    assert_eq!(linux_x86.get_default_convention(), CallConv::C);

    // Windows defaults
    let win_x64 = get_calling_convention_provider(ArchType::X86_64, OperatingSystem::Windows);
    assert_eq!(win_x64.get_default_convention(), CallConv::X86_64Win64);

    let win_x86 = get_calling_convention_provider(ArchType::X86, OperatingSystem::Windows);
    assert_eq!(win_x86.get_default_convention(), CallConv::X86Stdcall);
}

#[test]
fn test_supported_conventions() {
    let x86_provider = get_calling_convention_provider(ArchType::X86, OperatingSystem::Linux);
    assert!(x86_provider.is_supported(CallConv::C));
    assert!(x86_provider.is_supported(CallConv::X86Stdcall));
    assert!(x86_provider.is_supported(CallConv::X86Fastcall));
    assert!(!x86_provider.is_supported(CallConv::X86_64SysV));

    let x64_provider = get_calling_convention_provider(ArchType::X86_64, OperatingSystem::Linux);
    assert!(x64_provider.is_supported(CallConv::X86_64SysV));
    assert!(x64_provider.is_supported(CallConv::PreserveAll));
    assert!(!x64_provider.is_supported(CallConv::X86Stdcall));

    let arm64_provider = get_calling_convention_provider(ArchType::Arm64, OperatingSystem::Linux);
    assert!(arm64_provider.is_supported(CallConv::Arm64Aapcs));
    assert!(arm64_provider.is_supported(CallConv::Arm64AapcsDarwin));
    assert!(!arm64_provider.is_supported(CallConv::X86_64SysV));
}
