# Fireman

![Logo](firebat/src-tauri/icons/icon.png)

Working In Progress (2025.06 Updated)

## Features & Plans

**Currently, There is only IR Based Analyzed Routine.**

- [x] Generate IR Based Environment
- [X] Complete Instruction Parsing Routine
  - [X] X64
    - [X] Copy All Instruction Documents
    - [X] Complete Instruction Parsing Function
  - [ ] ARM
  - [ ] ...
- [X] IR Based Analyzed Routine
  - [X] Single Block Variable Analysis (aka Data Flow Analysis)
    - [X] Reaching Definitions Analysis
    - [X] Liveness Analysis
  - [X] Control Flow Analysis
    - [ ] Complex Loop Analysis
  - [X] Merged Block Variable Analysis
- [ ] Simulation Routine
- [X] Generate C like Code
  - [ ] Optimization
- [ ] GUI decompiler
  - [X] Inspect IR
  - [ ] Modify IR or Instruction
  - [X] Generate C like Code
  - [ ] Simulate With Memory / Register
- [ ] TUI decompiler
- [ ] CLI decompiler
- [ ] IR Pattern Matching Routine (to detect well-known library's function like msvc's memcpy)
- [ ] Optimizer
- [ ] Deobfucasioner (possible?)

## Code style

### Comment Template (optional, to avoid typing Note, NOTE, NOTES, notes, ....)

- \#\#\# Arguments
- \#\#\# Returns
- \#\#\# Note
- \#\#\# Todo
