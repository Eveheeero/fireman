# Fireman

![Logo](firebat\src-tauri\icons\icon.png)

Working In Progress (2025.05 Updated)

## Features & Plans

**Currently, There is no executable file.**

- [x] Generate IR Based Environment
- [X] Complete Instruction Parsing Routine
  - [X] X64
    - [X] Copy All Instruction Documents
    - [X] Complete Instruction Parsing Function
  - [ ] ARM
  - [ ] ...
- [ ] IR Based Analyzed Routine
  - [X] Single Block Variable Analysis (aka Data Flow Analysis)
    - [X] Reaching Definitions Analysis
    - [X] Liveness Analysis
  - [X] Control Flow Analysis
    - [ ] Complex Loop Analysis
  - [ ] Merged Block Variable Analysis
- [ ] Simulation Routine
- [ ] Generate C Code
- [ ] GUI decompiler
  - [ ] Inspect IR
  - [ ] Modify IR or Instruction
  - [ ] Generate C Code
  - [ ] Simulate With Memory / Register
  - [ ] Graph
- [ ] TUI decompiler
- [ ] CLI decompiler
- [ ] IR Pattern Matching Routine (to detect well-known library's function like msvc's memcpy)
- [ ] Optimizer
- [ ] Obfucasioner (possible?)

## Code style

### Comment Template (optional, to avoid typing Note, NOTE, NOTES, notes, ....)

- \#\#\# Arguments
- \#\#\# Returns
- \#\#\# Note
- \#\#\# Todo

## Source file order

### Module definition file

- submod declares
- submod use declares
- use declares

### Main source file order

- use declares
- source code
- impl block (when impl block is small)
- impl block declared mod declares (when impl block is large)
