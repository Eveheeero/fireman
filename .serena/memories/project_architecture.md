# Fireman Architecture Overview

## Core Architecture Flow

```
Binary File → PE Parser → Disassembler → IR Generation → Analysis → C Generation
                                              ↓
                                        GUI Visualization
```

## Key Components

### fireball (Core Library)

- **pe/**: PE file parsing and analysis
    - Entry point detection
    - Section handling
    - Import/export processing

- **arch/**: Architecture-specific code
    - x86_64/: Instruction parsing and analysis
    - (Future: ARM support)

- **ir/**: Intermediate Representation
    - Statement-based IR
    - Control flow graph
    - Data flow analysis
    - IR to C conversion

- **core/**: Core data structures
    - Fire (main analyzer)
    - Block, Instruction, Section
    - Relations between components

### firebat (GUI)

- Tauri v2 application
- React-based UI
- Displays:
    - Assembly view
    - IR view
    - Section information
    - C code output

### iceball (Disassembly)

- Work in progress
- Architecture documentation extractor
- Will provide better disassembly capabilities

## Key Interfaces

- `Fire`: Main decompiler interface
- `Block`: Basic block of instructions
- `IR::Statement`: IR representation
- `CAbstractSyntaxTree`: C code generation

## Data Flow

1. Binary loaded via goblin
2. Sections identified and mapped
3. Instructions disassembled (capstone)
4. Basic blocks created
5. IR generated from instructions
6. Analysis passes run on IR
7. C code generated from optimized IR
