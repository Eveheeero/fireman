# Unicorn Engine Rust Crate Summary

## Overview

**unicorn_engine** version 2.1.3 provides Rust bindings for the Unicorn CPU emulator engine, a lightweight, multi-platform, multi-architecture CPU emulator framework.

- **License**: GPL-2.0
- **Repository**: https://github.com/unicorn-engine/unicorn
- **Documentation Coverage**: 6.05% of the crate is documented

## Core Components

### Main Emulator Interface

#### `Unicorn<'a, D>` Struct
The primary emulator instance that provides the main interface for CPU emulation.

**Key Methods:**
- `Unicorn::new(arch: Arch, mode: Mode)` - Create new emulator instance
- `emu_start(begin, until, timeout, count)` - Start emulation
- `emu_stop()` - Stop emulation
- `mem_map(address, size, perms)` - Map memory regions
- `mem_read/mem_write` - Memory operations
- `reg_read/reg_write` - Register operations

### Architecture Support

The crate supports multiple CPU architectures through enums and register definitions:

#### Supported Architectures (via `Arch` enum)
- **x86/x86_64**: Intel/AMD processors
- **ARM**: ARM32 processors with various CPU models
- **ARM64**: ARM64/AArch64 processors
- **MIPS**: MIPS32/64 processors
- **PowerPC**: PPC32/64 processors
- **RISC-V**: RISC-V 32/64 processors
- **SPARC**: SPARC32/64 processors
- **M68K**: Motorola 68000 series
- **S390X**: IBM System/390 Extended
- **TriCore**: Infineon TriCore

#### CPU Models
Each architecture provides specific CPU model enums:
- `ArmCpuModel`: 33 variants (Cortex-M0/M3/M4/M7/M33, Cortex-A7/A8/A9/A15, etc.)
- `Arm64CpuModel`: A53, A57, A72 variants
- `X86CpuModel`: Various x86 processor models
- And similar enums for other architectures

#### Register Definitions
Architecture-specific register enums:
- `RegisterX86`: x86/x86_64 registers (EAX, EBX, RIP, etc.)
- `RegisterARM`: ARM registers (R0-R15, CPSR, etc.)
- `RegisterARM64`: ARM64 registers (X0-X30, SP, PC, etc.)
- Similar register sets for all supported architectures

### Memory Management

#### Memory Operations
- `mem_map()`: Map memory regions with specified permissions
- `mem_unmap()`: Unmap memory regions
- `mem_read()/mem_write()`: Read/write memory at specified addresses
- `mem_regions()`: Get list of mapped memory regions
- `mem_protect()`: Change memory permissions

#### MMIO Support
- `mmio_map()`: Map memory-mapped I/O regions with callbacks
- `mmio_map_ro()`: Map read-only MMIO regions
- Support for custom read/write callbacks

### Hook System

The emulator provides extensive hooking capabilities for monitoring and modifying execution:

#### Hook Types
- **Code hooks**: Monitor instruction execution
- **Memory hooks**: Monitor memory access (read/write)
- **Block hooks**: Monitor basic block execution
- **Interrupt hooks**: Handle interrupts
- **Invalid instruction hooks**: Handle invalid opcodes

#### x86-Specific Hooks
- **IN/OUT hooks**: Monitor I/O port access
- **System call hooks**: Monitor SYSCALL/SYSENTER instructions
- `InsnSysX86` enum: SYSCALL (699), SYSENTER (700)

#### Hook Management
- `add_*_hook()` methods: Add various types of hooks
- `remove_hook(hook_id)`: Remove previously added hooks
- `UcHookId`: Handle for managing hooks

### Context Management

#### CPU Context Operations
- `Context` struct: Represents saved CPU state
- `context_alloc()`: Allocate empty context
- `context_save()`: Save current CPU state
- `context_restore()`: Restore saved CPU state
- `context_init()`: Allocate and initialize context with current state

### Error Handling

- `uc_error` enum: Comprehensive error codes for various failure conditions
- All operations return `Result<T, uc_error>` for proper error handling

### Constants and Utilities

#### Timing Constants
- `SECOND_SCALE`: Time scale for second-based timeouts
- `MILISECOND_SCALE`: Time scale for millisecond-based timeouts

#### Version Information
- `VERSION_MAJOR/MINOR/PATCH`: Library version constants
- `API_MAJOR/MINOR`: API version constants

## Key Features

### Multi-Architecture Emulation
- Single unified API across all supported architectures
- Architecture-specific optimizations and features
- Consistent interface for register and memory operations

### Performance-Oriented Design
- Lightweight emulation engine
- Efficient hook system for monitoring
- Context save/restore for fast state management

### Safety and Control
- Memory protection and permissions
- Controlled execution with timeouts and instruction limits
- Comprehensive error reporting

### Extensibility
- Rich hook system for custom behavior
- MMIO support for hardware emulation
- Plugin-friendly architecture

## Usage Pattern

```rust
use unicorn_engine::unicorn_const::{Arch, Mode, Permission, SECOND_SCALE};
use unicorn_engine::{Unicorn, RegisterARM};

// Create emulator instance
let mut emu = Unicorn::new(Arch::ARM, Mode::LITTLE_ENDIAN)?;

// Map memory
emu.mem_map(0x1000, 0x4000, Permission::ALL)?;

// Write code to memory
let code = [0x17, 0x00, 0x40, 0xe2]; // sub r0, #23
emu.mem_write(0x1000, &code)?;

// Set initial register values
emu.reg_write(RegisterARM::R0, 123)?;

// Execute code
emu.emu_start(0x1000, 0x1004, 10 * SECOND_SCALE, 1000)?;

// Read result
let result = emu.reg_read(RegisterARM::R0)?;
```

## Integration Notes

### Dependencies
- `bitflags ^2.3.3`: For flag operations
- `libc ^0.2`: C library bindings
- Build dependencies: `cc`, `cmake`, `pkg-config`

### Platform Support
- Primary platform: x86_64-unknown-linux-gnu
- Cross-platform support through Unicorn engine backend

### FFI Layer
- Extensive FFI bindings in `ffi` module
- Direct access to underlying Unicorn C library
- Type-safe Rust wrappers over C functions

This crate provides a comprehensive, type-safe Rust interface to the powerful Unicorn CPU emulator, making it suitable for reverse engineering, malware analysis, dynamic analysis, and educational purposes.
