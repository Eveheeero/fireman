# Fireman Project Overview

Fireman is a decompiler framework written in Rust, inspired by Snowman. It's designed to analyze and decompile binary
executables, particularly focusing on PE files.

## Purpose

- Binary analysis and decompilation
- Transforms machine code → IR (Intermediate Representation) → C-like code
- GUI for analyzing decompilation process

## Components

1. **fireman** - CLI executable
2. **fireball** - Core decompiler library (main logic)
3. **firebat** - Tauri-based GUI application
4. **iceball** - Disassembly library (WIP)
5. **dryice** - IR pattern matching framework (reserved)
6. **fireman_macro** - Procedural macros

## Tech Stack

- Core: Rust (workspace project)
- GUI: Tauri v2 + React + TypeScript + Vite
- Key libraries: goblin, capstone, pdb, cpp_demangle

## Current Status

- x86_64 instruction parsing ✓
- Basic IR generation ✓
- Control flow analysis ✓
- Basic C code generation ✓
- GUI for IR inspection ✓
- ARM support (planned)
- Advanced optimizations (planned)
