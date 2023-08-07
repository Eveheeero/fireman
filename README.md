# Fireman

Working In Progress

top is main goal, lower is subs

- [x] Generate IR Based Environment
- [ ] Complete Instruction Parsing Routine
  - [ ] X64
    - [ ] Copy All Instruction Documents
    - [ ] Complete Instruction Parsing Function
  - [ ] ARM
  - [ ] ...
- [ ] IR to C like code
  - [ ] ...
- [ ] GUI based decompiler
- [ ] TUI based decompiler
- [ ] Command line decompiler
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
