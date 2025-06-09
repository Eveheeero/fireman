# Decompiler Libraries and Tools: Production Guide

## Quick Reference: What to Use When

| Task                   | Recommended Library | Why                                | Alternative         |
|------------------------|---------------------|------------------------------------|---------------------|
| **Disassembly**        | Capstone            | Fast, multi-arch, production-ready | iced-x86 (x86 only) |
| **Binary Parsing**     | LIEF                | Feature-rich, actively maintained  | goblin (lighter)    |
| **CFG/Analysis**       | petgraph            | Mature graph algorithms            | egg (rewriting)     |
| **Constraint Solving** | Z3                  | Industry standard, powerful        | boolector           |
| **Pattern Matching**   | aho-corasick        | Blazing fast                       | regex (flexibility) |
| **GUI**                | egui                | Pure Rust, fast                    | tauri (web tech)    |
| **Parallel**           | rayon               | Drop-in parallelism                | tokio (async)       |

## Rust Libraries for Decompiler Development

### Binary Parsing

#### `object` - Universal Object File Library
```toml
object = "0.32"
```
```rust
use object::{Object, ObjectSection};

let file = object::File::parse(&*data)?;
for section in file.sections() {
    println!("{}: {:x}", section.name()?, section.address());
}
```

#### Production Binary Parsing Options

##### `LIEF` - Industry Standard

```toml
lief = "0.13"
```

```rust
use lief::{Binary, PE, ELF};

// LIEF provides unified API across formats
let binary = Binary::parse( & buffer) ?;

// Access common properties
let entry = binary.entrypoint();
let sections = binary.sections();

// Format-specific features
match binary {
Binary::PE(pe) => {
// Access imports, exports, resources
for import in pe.imports() {
println ! ("Import: {} from {}", import.name(), import.library());
}
}
Binary::ELF(elf) => {
// Access symbols, relocations, dynamics
for symbol in elf.symbols() {
println ! ("Symbol: {} @ 0x{:x}", symbol.name(), symbol.value());
}
}
_ => {}
}
```

##### `goblin` - Lightweight Alternative
```toml
goblin = "0.7"
```
```rust
use goblin::{Object, Hint};

// Faster parsing with less features
let hint = Hint::default ();
let obj = Object::parse_with_hint( & buffer, hint) ?;

match obj {
Object::PE(pe) => {
// Basic PE parsing
for section in & pe.sections {
println ! ("{}: 0x{:x}", section.name()?, section.virtual_address);
}
}
Object::Elf(elf) => {
// Basic ELF parsing
for sym in & elf.syms {
println ! ("{}: 0x{:x}", elf.strtab[sym.st_name], sym.st_value);
}
}
_ => {}
}
```

**Industry Practice**: Use LIEF for full-featured analysis, goblin for fast initial parsing.

#### `capstone` - Multi-Architecture Disassembler
```toml
capstone = "0.11"
```
```rust
use capstone::prelude::*;

// Production setup with error handling
let cs = Capstone::new()
    .x86()
    .mode(arch::x86::ArchMode::Mode64)
    .syntax(arch::x86::ArchSyntax::Intel)
.detail(true)  // Enable detail mode for operand info
    .build()?;

// Efficient batch disassembly
let insns = cs.disasm_all(code, addr) ?;
for insn in insns.iter() {
// Access detailed operand information
let detail = cs.insn_detail( & insn)?;
for op in detail.arch_detail().operands() {
match op.op_type {
arch::x86::X86OperandType::Reg(reg) => {
println ! ("Register: {:?}", reg);
}
arch::x86::X86OperandType::Imm(imm) => {
println ! ("Immediate: 0x{:x}", imm);
}
_ => {}
}
}
}
```

**Industry Tip**: Always enable detail mode for production decompilers - you'll need operand types, sizes, and access
modes.

#### `bad64` - ARM64 Disassembler
```toml
bad64 = "0.1"
```
```rust
use bad64::{decode, Instruction};

match decode(bytes, addr)? {
    Instruction::AddSubImmediate { rd, rn, imm, .. } => {
        println!("add {}, {}, #{}", rd, rn, imm);
    }
    // ...
}
```

### Analysis

#### `petgraph` - Graph Algorithms
```toml
petgraph = "0.6"
```
```rust
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::{dominators, has_path_connecting};

let mut cfg = DiGraph::new();
let entry = cfg.add_node(BasicBlock::entry());
let exit = cfg.add_node(BasicBlock::exit());
cfg.add_edge(entry, exit, EdgeType::Fallthrough);

let dom = dominators::simple_fast(&cfg, entry);
```

#### `egg` - E-Graphs for Optimization
```toml
egg = "0.9"
```
```rust
use egg::{*, rewrite as rw};

define_language! {
    enum IRLang {
        "+" = Add([Id; 2]),
        "*" = Mul([Id; 2]),
        Num(i32),
    }
}

let rules: &[Rewrite<IRLang, ()>] = &[
    rw!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
    rw!("commute-mul"; "(* ?a ?b)" => "(* ?b ?a)"),
];
```

### Parallel Processing

#### `rayon` - Data Parallelism
```toml
rayon = "1.8"
```
```rust
use rayon::prelude::*;

functions.par_iter()
    .map(|func| analyze_function(func))
    .collect::<Vec<_>>()
```

#### `crossbeam` - Concurrent Programming
```toml
crossbeam = "0.8"
```
```rust
use crossbeam::channel;
use crossbeam::epoch;

let (sender, receiver) = channel::unbounded();
crossbeam::scope(|s| {
    s.spawn(|_| producer(sender));
    s.spawn(|_| consumer(receiver));
}).unwrap();
```

### Type Systems and Inference

#### `ena` - Union-Find for Type Inference
```toml
ena = "0.14"
```
```rust
use ena::unify::{InPlaceUnificationTable, UnifyKey};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct TypeVar(u32);

impl UnifyKey for TypeVar {
    type Value = Option<Type>;
    // ...
}
```

### Symbolic Execution

#### `z3` - SMT Solver for Type Recovery
```toml
z3 = "0.12"
```
```rust
use z3::{ast, Config, Context, Solver, SatResult};

// Production type constraint solving
struct TypeConstraintSolver {
    ctx: Context,
    solver: Solver,
}

impl TypeConstraintSolver {
    fn new() -> Self {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);

        // Set timeout for production use
        solver.set_params(&ctx.solver_params().timeout(5000)); // 5 seconds

        Self { ctx, solver }
    }

    fn solve_pointer_arithmetic(&mut self) -> Result<HashMap<String, Type>> {
        // Example: ptr + offset must be valid
        let ptr = ast::BV::new_const(&self.ctx, "ptr", 64);
        let offset = ast::BV::new_const(&self.ctx, "offset", 64);
        let result = ptr.bvadd(&offset);

        // Add constraints
        self.solver.assert(&ptr.bvuge(&ast::BV::from_u64(&self.ctx, 0x1000, 64)));
        self.solver.assert(&result.bvult(&ast::BV::from_u64(&self.ctx, 0xFFFF_0000, 64)));

        match self.solver.check() {
            SatResult::Sat => {
                let model = self.solver.get_model().unwrap();
                // Extract type information from model
                Ok(self.extract_types(&model))
            }
            SatResult::Unsat => Err("Constraints unsatisfiable".into()),
            SatResult::Unknown => Err("Solver timeout".into()),
        }
    }
}
```

**Industry Secret**: Always set timeouts on SMT solvers - some constraints can take forever to solve.

### Code Generation

#### `inkwell` - LLVM Bindings
```toml
inkwell = { version = "0.3", features = ["llvm15-0"] }
```
```rust
use inkwell::context::Context;
use inkwell::builder::Builder;

let context = Context::create();
let module = context.create_module("decompiled");
let builder = context.create_builder();
```

### Testing and Fuzzing

#### `proptest` - Property Testing
```toml
proptest = "1.4"
```
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_decode_encode(bytes in prop::collection::vec(any::<u8>(), 1..16)) {
        if let Ok(inst) = decode(&bytes) {
            assert_eq!(encode(&inst), bytes);
        }
    }
}
```

#### `arbitrary` - Fuzzing Support
```toml
arbitrary = { version = "1.3", features = ["derive"] }
```
```rust
#[derive(Arbitrary, Debug)]
struct Instruction {
    opcode: u8,
    operands: Vec<Operand>,
}
```

### Performance

#### `criterion` - Benchmarking
```toml
[dev-dependencies]
criterion = "0.5"
```
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_cfg(c: &mut Criterion) {
    c.bench_function("cfg_construction", |b| {
        b.iter(|| build_cfg(black_box(&function)))
    });
}
```

#### `pprof` - CPU Profiling
```toml
[dependencies]
pprof = { version = "0.13", features = ["flamegraph"] }
```
```rust
let guard = pprof::ProfilerGuardBuilder::default()
    .frequency(1000)
    .blocklist(&["libc", "libgcc", "pthread"])
    .build()?;

// Run decompilation

if let Ok(report) = guard.report().build() {
    let file = File::create("flamegraph.svg")?;
    report.flamegraph(&mut BufWriter::new(file))?;
}
```

## External Tools

### Decompiler References

#### IDA Pro SDK
- Industry standard for reverse engineering
- Extensive plugin API
- Pattern matching with FLIRT

#### Ghidra
- Open-source decompiler framework
- P-Code intermediate representation
- Extensive analysis capabilities

#### Binary Ninja
- Modern API design
- Medium Level IL (MLIL)
- Cloud-based collaboration

### Analysis Tools

#### angr - Binary Analysis Framework
```python
import angr

proj = angr.Project('binary')
cfg = proj.analyses.CFGFast()
func = cfg.functions[0x401000]
```

#### Miasm - Reverse Engineering Framework
```python
from miasm.analysis.binary import Container
from miasm.analysis.machine import Machine

container = Container.from_stream(open('binary', 'rb'))
machine = Machine(container.arch)
```

#### Radare2 - Unix-like RE Framework
```bash
r2 -A binary
afl  # List functions
pdf @ main  # Disassemble main
```

## Production Patterns from Industry

### Pattern Matching at Scale

```rust
use aho_corasick::{AhoCorasick, PatternID};

// How commercial decompilers really work
struct PatternDatabase {
    // Aho-Corasick for simultaneous pattern matching
    ac: AhoCorasick,
    patterns: Vec<KnownPattern>,
}

struct KnownPattern {
    id: PatternID,
    name: &'static str,
    library: &'static str,
    confidence: f32,
}

impl PatternDatabase {
    fn build() -> Self {
        // Real patterns from libc, libstdc++, etc.
        let patterns = vec![
            // memcpy implementations
            (b"\x48\x89\xf8\x48\x89\xd1\xf3\xa4\xc3", "memcpy_rep_movsb", "libc"),
            (b"\x48\x89\xf8\x48\x89\xf2\x48\x89\xce", "memcpy_avx", "libc"),

            // strlen variants
            (b"\x48\x89\xf8\x48\x89\xc1\x48\xff\xc1\x80\x39\x00", "strlen_simple", "libc"),

            // C++ vtable calls
            (b"\x48\x8b\x07\xff\x50", "virtual_call", "libstdc++"),
        ];

        let ac = AhoCorasick::builder()
            .match_kind(aho_corasick::MatchKind::LeftmostFirst)
            .build(patterns.iter().map(|(p, _, _)| p))
            .unwrap();

        Self { ac, patterns: /* ... */ }
    }

    fn identify(&self, code: &[u8]) -> Vec<Match> {
        self.ac.find_overlapping_iter(code)
            .map(|m| Match {
                pattern: &self.patterns[m.pattern()],
                offset: m.start(),
                length: m.len(),
            })
            .collect()
    }
}
```

### Incremental Analysis Pipeline

```rust
// How IDA Pro and similar tools provide instant feedback
struct IncrementalAnalyzer {
    // Results available at different stages
    quick_results: Arc<RwLock<QuickAnalysis>>,
    detailed_results: Arc<RwLock<DetailedAnalysis>>,
    deep_results: Arc<RwLock<DeepAnalysis>>,
}

impl IncrementalAnalyzer {
    fn analyze(&self, binary: Binary) {
        let quick = self.quick_results.clone();
        let detailed = self.detailed_results.clone();
        let deep = self.deep_results.clone();

        // Stage 1: Immediate results (< 100ms)
        rayon::spawn(move || {
            let analysis = QuickAnalysis {
                entry_points: find_entry_points(&binary),
                strings: extract_strings_fast(&binary),
                imports: parse_imports(&binary),
            };
            *quick.write() = analysis;
        });

        // Stage 2: Detailed analysis (< 5s)
        rayon::spawn(move || {
            let analysis = DetailedAnalysis {
                functions: identify_functions(&binary),
                cfg: build_initial_cfg(&binary),
                types: infer_basic_types(&binary),
            };
            *detailed.write() = analysis;
        });

        // Stage 3: Deep analysis (background)
        rayon::spawn(move || {
            let analysis = DeepAnalysis {
                decompiled: full_decompilation(&binary),
                patterns: match_all_patterns(&binary),
                data_structures: recover_structures(&binary),
            };
            *deep.write() = analysis;
        });
    }
}
```

### Visitor Pattern for IR
```rust
trait IRVisitor {
    fn visit_load(&mut self, dst: Register, addr: Expression);
    fn visit_store(&mut self, addr: Expression, src: Register);
    fn visit_binop(&mut self, op: BinOp, dst: Register, lhs: Operand, rhs: Operand);
}

struct TypeInferenceVisitor {
    types: HashMap<Register, Type>,
}

impl IRVisitor for TypeInferenceVisitor {
    fn visit_load(&mut self, dst: Register, addr: Expression) {
        if let Some(ptr_type) = self.infer_type(&addr) {
            self.types.insert(dst, ptr_type.pointed_type());
        }
    }
}
```

### Builder Pattern for AST
```rust
struct ASTBuilder<'a> {
    arena: &'a Bump,
}

impl<'a> ASTBuilder<'a> {
    fn function(&self) -> FunctionBuilder<'a> {
        FunctionBuilder::new(self.arena)
    }
    
    fn expr(&self) -> ExprBuilder<'a> {
        ExprBuilder::new(self.arena)
    }
}
```

### Strategy Pattern for Architectures
```rust
trait ArchStrategy {
    fn decode(&self, bytes: &[u8]) -> Result<Instruction>;
    fn lift(&self, inst: &Instruction) -> Vec<IR>;
    fn lower(&self, ir: &[IR]) -> Vec<Instruction>;
}

struct DecompilerEngine {
    strategy: Box<dyn ArchStrategy>,
}
```

## Industry Best Practices

### 1. Memory-Mapped Everything

```rust
use memmap2::MmapOptions;
use std::fs::File;

// Don't load 1GB binaries into RAM
struct MappedBinary {
    mmap: Mmap,
    parsed: OnceCell<ParsedBinary>,
}

impl MappedBinary {
    fn new(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        Ok(Self { mmap, parsed: OnceCell::new() })
    }

    fn parse(&self) -> &ParsedBinary {
        self.parsed.get_or_init(|| {
            parse_binary(&self.mmap)
        })
    }
}
```

### 2. Cache Design for Decompilers

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

// Multi-level caching strategy
struct DecompilerCache {
    // L1: Hot functions (fast, small)
    hot_functions: LruCache<Address, Arc<DecompiledFunction>>,

    // L2: All functions (medium)
    all_functions: DashMap<Address, Arc<DecompiledFunction>>,

    // L3: Disk cache (slow, unlimited)
    disk_cache: DiskCache,
}

impl DecompilerCache {
    fn new() -> Self {
        Self {
            hot_functions: LruCache::new(NonZeroUsize::new(100).unwrap()),
            all_functions: DashMap::new(),
            disk_cache: DiskCache::new(".decompiler_cache"),
        }
    }

    fn get_or_decompile(&mut self, addr: Address) -> Arc<DecompiledFunction> {
        // Check L1
        if let Some(func) = self.hot_functions.get(&addr) {
            return func.clone();
        }

        // Check L2
        if let Some(func) = self.all_functions.get(&addr) {
            self.hot_functions.put(addr, func.clone());
            return func.clone();
        }

        // Check L3
        if let Some(func) = self.disk_cache.get(addr) {
            let func = Arc::new(func);
            self.all_functions.insert(addr, func.clone());
            self.hot_functions.put(addr, func.clone());
            return func;
        }

        // Decompile and cache at all levels
        let func = Arc::new(decompile_function(addr));
        self.disk_cache.put(addr, &func);
        self.all_functions.insert(addr, func.clone());
        self.hot_functions.put(addr, func.clone());
        func
    }
}
```

### 3. Error Recovery Strategy

```rust
// Production decompilers never panic
#[derive(Debug)]
enum DecompileError {
    Recoverable(String),
    Partial(PartialResult),
    Fatal(String),
}

struct RobustDecompiler;

impl RobustDecompiler {
    fn decompile(&self, func: &Function) -> DecompileResult {
        // Try advanced analysis
        match self.advanced_decompile(func) {
            Ok(code) => return DecompileResult::Success(code),
            Err(DecompileError::Fatal(e)) => {
                log::error!("Fatal error: {}", e);
                return DecompileResult::Failure(e);
            }
            Err(_) => {} // Try next method
        }

        // Fall back to simpler analysis
        match self.basic_decompile(func) {
            Ok(code) => return DecompileResult::Partial(code),
            Err(_) => {} // Try next method
        }

        // Last resort: annotated assembly
        DecompileResult::Assembly(self.annotate_assembly(func))
    }
}
```

### 4. Real Performance Tips

1. **Profile First**: "I think it's slow" != measured bottleneck
2. **Batch Operations**: Process 1000 functions, not 1 function 1000 times
3. **SIMD Pattern Matching**: 10x faster than byte-by-byte
4. **Parallel by Default**: Modern CPUs have 8+ cores
5. **Lazy Everything**: Don't analyze until user looks at it
6. **Cache Invalidation**: The hardest problem - use content hashing
7. **Memory Pools**: Reuse allocations for hot paths

### 5. Testing with Real Binaries

```rust
// Integration test with real software
#[test]
fn test_real_world_binaries() {
    let test_binaries = vec![
        ("coreutils/ls", 500),      // Expected functions
        ("openssl/libssl.so", 2000),
        ("firefox/firefox", 50000),
    ];

    for (binary, expected_funcs) in test_binaries {
        let result = decompile(binary);
        assert!(result.functions.len() > expected_funcs * 0.9);
        assert!(result.success_rate() > 0.8);
    }
}
```