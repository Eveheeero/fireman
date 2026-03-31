# Firebat

GUI decompiler built on the [Fireball](../fireball/) engine with a node-based optimization pipeline. Uses [egui](https://github.com/emilk/egui) for cross-platform immediate-mode rendering.

## Architecture

Firebat uses a **node graph** as its primary interface. Users build decompilation pipelines by connecting nodes on a 2D canvas:

```
InputNode ──> OptNode 0 ──> OptNode 1 ──> PreviewNode
              (settings)    (settings)    (AST snapshot)
```

### Node Types

- **InputNode** — Loads a binary file via Fireball, analyzes sections, and produces the initial AST. Includes a file browser dialog.
- **OptNode** — Holds a full `OptimizationStore` (draft/applied settings, `.fb` script editor buffer) and caches the optimized `Arc<Ast>` output. Each OptNode independently configures which optimization passes to apply.
- **PreviewNode** — Passive read-only snapshot. Displays the AST at its point in the pipeline without modifying it. Can be placed anywhere to inspect intermediate results.

### Pipeline Execution

Nodes are processed sequentially following graph connections:

1. **InputNode** loads the binary and generates the raw AST
2. Each **OptNode** receives the preceding node's output AST and applies its optimization settings
3. **PreviewNode** passes data through unchanged, displaying a snapshot

The pipeline supports async execution via a background worker thread. AST data flows through the graph as `Arc<Ast>` to minimize cloning overhead.

## Toolbar

| Button           | Action                                      |
|------------------|---------------------------------------------|
| + Add Node       | Opens node type selector (Input/Opt/Preview) |
| Execute Pipeline | Runs the full pipeline sequentially          |
| Clear Graph      | Removes all nodes and connections             |

The node type selector includes a pass type dropdown for choosing which optimization pass a new OptNode should use.

## Graph Canvas

- **Pan**: Middle-click drag or Space + drag
- **Zoom**: Scroll wheel (0.1x -- 5.0x range)
- **Select**: Click a node
- **Drag**: Click and drag a node to reposition
- **Connect**: Click an output port, then click an input port
- **Delete**: Click the `x` button on a node

Connections are rendered as lines between node ports. Each node type has a distinct color:
- Input: Blue (`#0F6CBD`)
- Optimization: Cyan (`#038387`)
- Preview: Green (`#0F7B0F`)

## Optimization Passes

Each OptNode can enable one or more of 31 optimization passes:

| Pass                          | Description                                              |
|-------------------------------|----------------------------------------------------------|
| Constant Folding              | Evaluates constant expressions during optimization       |
| Control Flow Cleanup          | Removes structural noise before higher-level recovery    |
| Copy Propagation              | Eliminates temporary copies when values can be forwarded |
| Dead Store Elimination        | Removes writes that never affect later behavior          |
| Expression Inlining           | Inlines short temporary expressions into their uses      |
| Loop Analysis                 | Recovers loop constructs from CFG structure              |
| Parameter Analysis            | Infers function parameters from recovered usage          |
| Call Argument Analysis        | Propagates argument information into recovered calls     |
| Pattern Matching              | Runs `.fb` pattern scripts for AST rewriting             |
| Boolean Recovery              | Normalizes predicate-heavy code into boolean expressions |
| Switch Reconstruction         | Detects and prints switch-style control flow             |
| Lifetime Scoping              | Shrinks recovered variable lifetimes around real usage   |
| Signedness Inference          | Refines integer semantics from instruction behavior      |
| Name Recovery                 | Recovers variable and helper names when possible         |
| Early Return Normalization    | Prefers normalized early-return shapes in the AST        |
| Collapse Unused Variable      | Drops redundant variables that do not survive analysis   |
| Ternary Recovery              | Rebuilds ternary expressions from compact branches       |
| Operator Canonicalization     | Normalizes operator ordering for consistent comparison   |
| Magic Division Recovery       | Recovers division from magic-number multiply patterns    |
| Identity Simplification       | Simplifies identity operations like `x+0`, `x*1`        |
| Bit Trick Recognition         | Recognizes bit manipulation idioms                       |
| Cast Minimization             | Removes redundant type casts                             |
| Assertion Recovery            | Recovers assertion patterns from conditional aborts      |
| Do-While Recovery             | Recovers do-while loops from CFG                         |
| Clamp Recovery                | Recovers clamp/min/max patterns                          |
| Loop Cleanup                  | Cleans up loop structure after recovery                  |
| If-Conversion Reversal        | Reverses compiler if-conversion optimizations            |
| Anti-Debug AST Suppression    | Suppresses anti-debug code patterns in output            |
| Logging Suppression           | Suppresses logging boilerplate in output                 |
| Static Guard Suppression      | Suppresses static guard patterns in output               |
| Security Scaffold Suppression | Suppresses security scaffold patterns in output          |

## Floating Editors

Assembly, IR, and AST editors are available as floating windows for manual editing of decompiled output. Edits are applied via the background worker and regenerate downstream layers.

## Background Worker

All heavy operations (file loading, section analysis, decompilation, AST optimization, edits, patch export) run on a dedicated background thread communicating via mpsc channels. The UI polls for results each frame and shows a "Processing..." indicator while work is in progress.

## Persistence

Optimization settings are saved to `$XDG_CONFIG_HOME/firebat/settings.json` (defaults to `~/.config/firebat/settings.json`).

## Performance HUD

Press `F12` to toggle the performance HUD overlay showing:
- Average frame time (ms) and FPS
- p95 frame time
- Pending worker jobs
- Node and connection counts

## Dependencies

- **fireball** — Decompiler engine (workspace dependency)
- **eframe 0.34** / **egui 0.34** — GUI framework
- **rfd 0.17** — Native file dialogs
- **serde** / **serde_json** — Serialization
- **chrono** — Timestamps for logging
- **uuid** — Node IDs
- **image** — Icon loading

### Optional Features

- `keystone` — Assembly editing support
- `unicorn` — Emulation support
- `full` — Both features enabled
