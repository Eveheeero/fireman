# Fireman

![Logo](firebat/src-tauri/icons/icon.png)

A decompiler framework written in Rust, inspired by Snowman. Fireman analyzes and decompiles binary executables by
transforming machine code → IR (Intermediate Representation) → C-like code.

## Installation

To get started with Fireman, clone the repository with its submodules:

```bash
git clone https://github.com/your-username/fireman.git
cd fireman
git submodule init
git submodule update
```

## Building

Build the entire workspace:

```bash
cargo build -r
```

## Features & Roadmap

See [README-TODOS.md](README-TODOS.md) for the detailed features list and development roadmap.

## Code style

### Comment Template (optional, to avoid typing Note, NOTE, NOTES, notes, ....)

- \#\#\# Arguments
- \#\#\# Returns
- \#\#\# Note
- \#\#\# Todo
