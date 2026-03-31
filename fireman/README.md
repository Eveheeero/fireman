# Fireman TUI

Terminal-based interactive decompiler built on [ratatui](https://ratatui.rs). Provides a flexible multi-tab interface with a composable optimization pipeline for iterative AST refinement.

## Tab System

The interface is organized around four tab types:

### Input

Load binaries, analyze sections, and select functions to decompile. Sections can be analyzed individually by address or all at once. Toggling a section (checked/unchecked) triggers decompilation of all selected sections through the optimization pipeline.

### Logs

Scrollable application log with timestamps. Shows worker status, errors, and operation history. Capped at 256 entries. Useful for diagnosing decompilation issues or tracking pipeline progress.

### Opt (Optimization Stage)

Each Opt tab holds its own set of optimization pass settings and an inline `.fb` pattern script editor. The interface is split into two panels:

- **Settings (left)**: 32 optimization pass toggles arranged as radio buttons. Selecting one pass disables all others. Passes include IR analyzation, parameter analyzation, constant folding, copy propagation, expression inlining, dead store elimination, loop analyzation, ternary/boolean recovery, switch reconstruction, pattern matching, and more.
- **Script (right)**: Inline `.fb` pattern script editor. Scripts are written in the fireball DSL for AST pattern matching and rewriting. Scripts can be loaded from and saved to disk. When the buffer has content, an "Apply .fb script" checkbox appears in the settings panel.

Changes to settings are applied immediately and trigger re-decompilation through the entire pipeline.

### Preview

Read-only AST snapshot. Displays the decompiled output at a specific point in the pipeline without modifying it. The info panel shows which optimization stage (or raw decompile) produced the displayed result. Multiple Preview tabs can be inserted at any position in the pipeline simultaneously.

## Optimization Pipeline

The pipeline is the core architectural concept. It is a free-form sequence of **Opt** and **Preview** entries that the user builds interactively:

```
Raw AST --> Opt 0 (settings_0) --> Opt 1 (settings_1) --> Opt 2 (settings_2)
         \                      \                      \
       Preview                Preview                Preview
     (shows raw)          (shows Opt 0 out)        (shows Opt 1 out)
```

### Sequential Chaining

Each Opt stage receives the output AST of the preceding Opt stage as its input. If it is the first Opt in the pipeline, it receives the raw decompile result. This means Opt 1 refines Opt 0's output, not the original raw AST. This enables iterative, incremental optimization where each stage builds on the previous result.

### Cascade Invalidation

When an Opt stage's settings change, all downstream Opt outputs and Preview snapshots are cleared and re-computed automatically. This ensures the pipeline always reflects the current configuration. Changing Opt 0's settings will invalidate and re-run Opt 1, Opt 2, and all Preview snapshots that depend on them.

### Decompile Queue

Since the background worker processes one request at a time, optimization stages are queued and executed sequentially. The raw decompile runs first, then each Opt stage is processed in pipeline order. Progress is shown in the title bar busy indicator.

### Preview Transparency

Preview tabs do not participate in the optimization chain. They are passive observers that display the AST state at their position in the pipeline. Inserting or removing a Preview never affects the optimization results. They exist purely for inspection.

## Keyboard Reference

### Global (all tabs)

| Key     | Action                                 |
|---------|----------------------------------------|
| `1`-`9` | Jump to tab by number                  |
| `t`     | Next tab                               |
| `T`     | Previous tab                           |
| `n`     | Add a new Opt stage at end of pipeline |
| `p`     | Insert a Preview after current tab     |
| `c`     | Close current tab (not Input/Logs)     |
| `r`     | Reset pipeline to empty                |
| `q`     | Quit                                   |
| `?`     | Show license/about dialog              |

### Input Tab

| Key              | Action                         |
|------------------|--------------------------------|
| `o`              | Open binary (file browser)     |
| `a`              | Analyze a specific address     |
| `g`              | Analyze all sections           |
| `Up`/`Down`      | Move cursor                    |
| `PgUp`/`PgDn`    | Fast move (10 rows)            |
| `Home`/`End`     | Jump to start/end              |
| `Space`/`Enter`  | Toggle section and decompile   |
| `s` / `Ctrl+A`   | Toggle all analyzed sections   |

### Opt Tab -- Settings Focus

| Key          | Action                         |
|--------------|--------------------------------|
| `Tab`        | Switch focus to Script panel   |
| `Up`/`Down`  | Move cursor through passes     |
| `Space`      | Select optimization pass       |
| `r`          | Reset draft settings to none   |
| `L`          | Load saved config from disk    |
| `W`          | Save current config to disk    |

### Opt Tab -- Script Focus

| Key       | Action                         |
|-----------|--------------------------------|
| `Tab`     | Switch focus to Settings panel |
| Type      | Edit `.fb` pattern script      |
| `Ctrl+S`  | Save script to file            |
| `Ctrl+O`  | Load script from file          |

### Preview Tab

| Key            | Action            |
|----------------|-------------------|
| `Up`/`Down`    | Scroll AST        |
| `PgUp`/`PgDn`  | Fast scroll      |
| `Home`/`End`   | Jump to start/end |

### Logs Tab

| Key            | Action            |
|----------------|-------------------|
| `Up`/`Down`    | Scroll logs       |
| `PgUp`/`PgDn`  | Fast scroll      |
| `Home`/`End`   | Jump to start/end |

## Persistence

Optimization settings are saved to `$XDG_CONFIG_HOME/firebat/settings.json` (defaults to `~/.config/firebat/settings.json`). Use `W` to save and `L` to load from within any Opt tab. Each Opt stage can independently load or save its configuration. Startup optimization configs can also be passed via command-line arguments.

## Background Worker

All heavy operations (file loading, section analysis, decompilation, AST optimization) run on a dedicated background thread communicating via mpsc channels. The TUI polls for results at 50ms intervals and displays a busy indicator in the title bar while work is in progress. Only one operation can be active at a time -- subsequent requests are rejected until the current one completes.

## Optimization Passes

The following passes are available in each Opt stage:

| Pass                          | Description                                              |
|-------------------------------|----------------------------------------------------------|
| IR analyzation                | Builds the IR analysis layer used by later AST passes    |
| Parameter analyzation         | Infers function parameters from recovered usage          |
| Call argument analyzation     | Propagates argument information into recovered calls     |
| Name recovery                 | Recovers variable and helper names when possible         |
| Signedness inference          | Refines integer semantics from instruction behavior      |
| Constant folding              | Evaluates constant expressions during optimization       |
| Copy propagation              | Eliminates temporary copies when values can be forwarded |
| Expression inlining           | Inlines short temporary expressions into their uses      |
| Dead store elimination        | Removes writes that never affect later behavior          |
| Collapse unused variable      | Drops redundant variables that do not survive analysis   |
| Lifetime scoping              | Shrinks recovered variable lifetimes around real usage   |
| Control flow cleanup          | Removes structural noise before higher-level recovery    |
| Loop analyzation              | Recovers loop constructs from CFG structure              |
| Ternary recovery              | Rebuilds ternary expressions from compact branches       |
| Boolean recovery              | Normalizes predicate-heavy code into boolean expressions |
| Switch reconstruction         | Detects and prints switch-style control flow             |
| Early return normalization    | Prefers normalized early-return shapes in the AST        |
| Pattern matching              | Runs predefined and selected `.fb` pattern scripts       |
| Operator canonicalization     | Normalizes operator ordering for consistent comparison   |
| Magic division recovery       | Recovers division from magic-number multiply patterns    |
| Identity simplification       | Simplifies identity operations like `x+0`, `x*1`        |
| Bit trick recognition         | Recognizes bit manipulation idioms                       |
| Cast minimization             | Removes redundant type casts                             |
| Assertion recovery            | Recovers assertion patterns from conditional aborts      |
| Do-while recovery             | Recovers do-while loops from CFG                         |
| Clamp recovery                | Recovers clamp/min/max patterns                          |
| Loop cleanup                  | Cleans up loop structure after recovery                  |
| If-conversion reversal        | Reverses compiler if-conversion optimizations            |
| Anti-debug AST suppression    | Suppresses anti-debug code patterns in output            |
| Logging suppression           | Suppresses logging boilerplate in output                 |
| Static guard suppression      | Suppresses static guard patterns in output               |
| Security scaffold suppression | Suppresses security scaffold patterns in output          |
