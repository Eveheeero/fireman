# Pattern Storage Format Specification

This document describes the format for storing decompiler patterns used by Fireball's Medium IR pattern recognition
system.

## Overview

Patterns are stored in human-readable text files with specific extensions:

- `.pat` - Library function patterns
- `.idiom` - Common code idioms
- `.arch` - Architecture-specific patterns

## Directory Structure

```
patterns/
├── libraries/
│   ├── libc_common.pat
│   ├── win32_common.pat
│   └── cpp_stdlib.pat
├── idioms/
│   ├── loops.idiom
│   ├── string_ops.idiom
│   └── memory_patterns.idiom
└── architectures/
    ├── x86_64/
    │   ├── prologue.arch
    │   └── calling_conventions.arch
    └── arm/
        ├── prologue.arch
        └── thumb_patterns.arch
```

## Library Pattern Format (.pat)

Library patterns describe known library functions with their signatures and behaviors.

### Syntax

```
FUNCTION: library::function_name
  RETURNS: type
  PARAM: name type
  PARAM: name type
  CONVENTION: calling_convention
  BEHAVIOR: behavior_type
  ATTRIBUTES: [optional attributes]

FUNCTION: library::another_function
  ...
```

### Example

```
FUNCTION: libc::malloc
  RETURNS: void*
  PARAM: size u64
  CONVENTION: cdecl
  BEHAVIOR: modifies_memory
  ATTRIBUTES: heap_allocator

FUNCTION: libc::strlen
  RETURNS: u64
  PARAM: str char*
  CONVENTION: cdecl
  BEHAVIOR: pure
  ATTRIBUTES: null_terminated_string
```

### Type Notation

- Primitive types: `void`, `bool`, `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `f32`, `f64`
- Pointer types: `type*` (e.g., `char*`, `void*`)
- Array types: `type[size]` (e.g., `int[10]`)
- Const qualifier: `const type` (e.g., `const char*`)

### Calling Conventions

- `cdecl` - C calling convention
- `stdcall` - Windows stdcall
- `fastcall` - Fast calling convention
- `thiscall` - C++ this calling convention
- `vectorcall` - Vector calling convention
- `sysv` - System V AMD64 ABI
- `ms64` - Microsoft x64 calling convention

### Behaviors

- `pure` - No side effects
- `modifies_memory` - Modifies memory (heap, stack, or global)
- `io_read` - Performs input operations
- `io_write` - Performs output operations
- `system_call` - Makes system calls
- `throws` - Can throw exceptions

## Idiom Pattern Format (.idiom)

Idiom patterns describe common programming patterns and code structures.

### Syntax

```
IDIOM: idiom_name
  DESCRIPTION: Human-readable description
  CONFIDENCE_BOOST: +20
  PATTERN:
    <pattern specification>
  END_PATTERN
```

### Example

```
IDIOM: strlen_loop
  DESCRIPTION: String length calculation loop
  CONFIDENCE_BOOST: +20
  PATTERN:
    LOOP_HEADER:
      INIT: ptr = string_start
      CONDITION: *ptr != 0
      INCREMENT: ptr++
    LOOP_BODY:
      # Empty or counter increment
    RESULT: ptr - string_start
  END_PATTERN

IDIOM: null_check_pattern
  DESCRIPTION: Null pointer check before use
  CONFIDENCE_BOOST: +15
  PATTERN:
    IF_CONDITION: ptr == NULL
    THEN_BRANCH:
      RETURN: error_value
    ELSE_BRANCH:
      USE: *ptr
  END_PATTERN
```

## Architecture Pattern Format (.arch)

Architecture-specific patterns for instruction sequences.

### Syntax

```
ARCH_PATTERN: pattern_name
  ARCHITECTURE: x86_64|arm|arm64|riscv
  DESCRIPTION: Description
  SEQUENCE:
    instruction_pattern
    instruction_pattern
    ...
  END_SEQUENCE
```

### Example

```
ARCH_PATTERN: function_prologue
  ARCHITECTURE: x86_64
  DESCRIPTION: Standard function prologue
  SEQUENCE:
    push rbp
    mov rbp, rsp
    sub rsp, <frame_size>
  END_SEQUENCE

ARCH_PATTERN: pic_got_load
  ARCHITECTURE: x86_64
  DESCRIPTION: Position-independent code GOT access
  SEQUENCE:
    lea rax, [rip + <offset>]
    mov <reg>, [rax + <got_offset>]
  END_SEQUENCE
```

### Instruction Pattern Syntax

- Exact match: `push rbp`
- Register wildcard: `<reg>` matches any register
- Immediate wildcard: `<imm>` matches any immediate value
- Memory wildcard: `[<mem>]` matches any memory operand
- Optional: `?mov rax, rbx` (instruction may or may not be present)
- Alternatives: `push rbp|push ebp` (either instruction matches)

## Pattern Matching Rules

### Confidence Scoring

- Base confidence starts at 50%
- Each matched element increases confidence
- Pattern-specific boosts are applied
- Final confidence is capped at 100%

### Precedence

1. Architecture-specific patterns (highest priority)
2. Library function patterns
3. Idiom patterns (lowest priority)

### Wildcards and Variables

- `<name>` - Captures a value and binds it to a variable
- `<name:type>` - Type-constrained capture (e.g., `<size:u32>`)
- `<name=value>` - Captures and checks equality
- `<name~pattern>` - Captures and matches against regex pattern

## Pattern Composition

Patterns can reference other patterns:

```
IDIOM: error_handling_pattern
  DESCRIPTION: Common error handling structure
  PATTERN:
    result = CALL_PATTERN: library_function
    IF_CONDITION: result < 0
    THEN_BRANCH:
      INCLUDE_PATTERN: cleanup_and_return
  END_PATTERN
```

## Metadata and Versioning

Each pattern file should include metadata:

```
# Pattern File Metadata
# Version: 1.0
# Author: Fireball Team
# Date: 2024-01-01
# Platform: generic|windows|linux|macos
# Architecture: any|x86_64|arm|arm64
```

## Pattern Testing

Pattern files can include test cases:

```
TEST_CASE: malloc_detection
  INPUT_ASM:
    push rdi
    mov edi, 0x100
    call malloc
    test rax, rax
  EXPECTED_MATCH: libc::malloc
  EXPECTED_CONFIDENCE: 95
END_TEST
```

## Best Practices

1. **Keep patterns focused** - One pattern per concept
2. **Use meaningful names** - Pattern names should be self-documenting
3. **Document confidence boosts** - Explain why a pattern increases confidence
4. **Test patterns** - Include test cases for complex patterns
5. **Version control** - Track pattern changes over time
6. **Platform awareness** - Mark platform-specific patterns clearly

## Future Extensions

- Support for C++ template patterns
- Machine learning confidence adjustments
- Pattern inheritance and composition
- Performance hints for pattern matching
- Integration with debugging information
