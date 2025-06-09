# Language-Specific Decompilation Strategies

## 1. Rust Decompilation

### Rust Binary Characteristics

#### Memory Management Patterns
```rust
// Ownership tracking in assembly
struct RustMemoryPattern {
    // Box allocations
    box_alloc: Vec<AllocCall>,        // __rust_alloc calls
    box_dealloc: Vec<DeallocCall>,    // __rust_dealloc calls
    
    // Reference counting (Arc/Rc)
    refcount_ops: Vec<RefCountOp>,
    
    // Slice metadata
    slice_bounds_checks: Vec<BoundsCheck>,
    
    // Drop glue
    drop_impls: HashMap<TypeId, DropGlue>,
}

// Identifying Rust patterns
fn identify_rust_allocation(inst: &Instruction) -> Option<AllocType> {
    match inst {
        Call { target, args, .. } => {
            if target.contains("__rust_alloc") {
                Some(AllocType::Box(args[0])) // size argument
            } else if target.contains("__rust_realloc") {
                Some(AllocType::Vec(args[0], args[1])) // ptr, new_size
            } else {
                None
            }
        }
        _ => None,
    }
}
```

#### Trait Objects and Dynamic Dispatch
```rust
struct TraitObject {
    data_ptr: Address,
    vtable_ptr: Address,
}

struct VTable {
    drop_glue: Address,
    size: usize,
    align: usize,
    methods: Vec<Address>,
}

// Reconstruct trait hierarchy
fn analyze_vtable(vtable_addr: Address) -> VTable {
    // Read vtable structure
    // First 3 entries: drop, size, align
    // Remaining: trait methods
}
```

#### Error Handling Patterns
```rust
enum RustResult<T, E> {
    Ok(T),
    Err(E),
}

// Assembly pattern for Result
// Discriminant in lower bits or separate field
struct ResultPattern {
    discriminant_offset: usize,
    ok_variant: u8,
    err_variant: u8,
}

// Panic handling
struct PanicInfo {
    panic_handler: Address,
    unwind_tables: Vec<UnwindInfo>,
    landing_pads: Vec<Address>,
}
```

#### Async/Await State Machines
```rust
// Async function state machine
struct AsyncStateMachine {
    states: Vec<State>,
    transitions: Vec<StateTransition>,
    poll_fn: Address,
    future_size: usize,
}

// Generator transformation
enum GeneratorState {
    Start,
    Suspend(usize),    // Yield point
    Complete,
    Panicked,
}
```

### Rust-Specific Optimizations

#### Zero-Cost Abstractions
```rust
// Iterator chains compiled to loops
struct IteratorPattern {
    // Original: vec.iter().map(f).filter(g).collect()
    // Compiled: Tight loop with inlined closures
    loop_bounds: (Address, Address),
    inlined_closures: Vec<InlinedClosure>,
}

// Const generics
struct ConstGeneric {
    value: ConstValue,
    usage_sites: Vec<Address>,
}
```

## 2. Go Decompilation

### Go Binary Structure

#### Runtime Structures
```rust
struct GoRuntime {
    // Go runtime structures
    g0: Address,              // Initial goroutine
    m0: Address,              // Initial thread
    runtime_procs: usize,     // GOMAXPROCS
    
    // Function metadata
    pclntab: PCLineTable,     // PC to line mapping
    functab: Vec<Function>,   // Function table
    filetab: Vec<String>,     // Source files
}

struct GoFunction {
    entry: Address,
    name: String,
    args: usize,              // Argument size
    locals: usize,            // Local variable size
    pcsp: Vec<(PC, SP)>,      // PC to SP delta
    pcfile: Vec<(PC, File)>,  // PC to file
    pcline: Vec<(PC, Line)>,  // PC to line
}
```

#### Goroutine Management
```rust
// Goroutine spawn pattern
// runtime.newproc(siz, fn, args...)
struct GoroutineSpawn {
    size: usize,
    function: Address,
    arguments: Vec<Value>,
}

// Channel operations
enum ChannelOp {
    MakeChan { size: usize, elem_type: Type },
    Send { chan: Register, value: Register },
    Recv { chan: Register, dest: Register },
    Select { cases: Vec<SelectCase> },
}
```

#### Interface Reconstruction
```rust
struct GoInterface {
    itab: InterfaceTable,
    data: Address,
}

struct InterfaceTable {
    inter: InterfaceType,
    type_: ConcreteType,
    methods: Vec<Method>,
}

// Recover interface types
fn reconstruct_interface(iface_addr: Address) -> GoInterface {
    // Read itab pointer and data pointer
    // Decode method table
}
```

### Go-Specific Patterns

#### Defer Mechanism
```rust
struct DeferRecord {
    fn_addr: Address,
    args: Vec<Value>,
    panic_link: Option<Address>,
}

// Defer stack tracking
fn track_defer_stack(g: &Goroutine) -> Vec<DeferRecord> {
    // Walk linked list of defer records
    // Extract function and arguments
}
```

#### Slice Operations
```rust
struct GoSlice {
    data: Address,
    len: usize,
    cap: usize,
}

// Slice manipulation patterns
enum SliceOp {
    Make { len: usize, cap: usize },
    Append { slice: GoSlice, elems: Vec<Value> },
    Copy { dst: GoSlice, src: GoSlice },
    SubSlice { start: usize, end: Option<usize> },
}
```

## 3. C++ Decompilation

### Modern C++ Features

#### Template Instantiation
```rust
struct TemplateInstance {
    template_name: String,
    type_params: Vec<Type>,
    instantiation_addr: Address,
    mangled_name: String,
}

// Recover template parameters from mangled names
fn demangle_template(mangled: &str) -> TemplateInstance {
    // Parse Itanium ABI mangling
    // Extract template arguments
}
```

#### Virtual Function Tables
```rust
struct CppClass {
    vtable: Address,
    rtti: Option<RTTIInfo>,
    base_classes: Vec<BaseClass>,
    virtual_bases: Vec<VirtualBase>,
    members: Vec<Member>,
}

struct VTableLayout {
    offset_to_top: isize,
    rtti_ptr: Address,
    virtual_funcs: Vec<VirtualFunc>,
}
```

#### Exception Handling
```rust
struct ExceptionHandler {
    try_start: Address,
    try_end: Address,
    catch_blocks: Vec<CatchBlock>,
    cleanup: Option<Address>,
}

struct CatchBlock {
    type_info: Address,
    handler: Address,
}
```

#### STL Container Recognition
```rust
enum STLContainer {
    Vector { 
        begin: Address,
        end: Address,
        capacity: Address,
    },
    Map {
        root: Address,
        size: usize,
        comparator: Address,
    },
    String {
        data: StringRep,
        size: usize,
        capacity: usize,
    },
}

// Small string optimization
enum StringRep {
    Small([u8; 16]),
    Large(Address),
}
```

## 4. Java/JVM Bytecode

### JVM Class Structure
```rust
struct JavaClass {
    magic: u32,              // 0xCAFEBABE
    version: (u16, u16),     // Major, minor
    constant_pool: ConstantPool,
    access_flags: AccessFlags,
    this_class: ClassRef,
    super_class: ClassRef,
    interfaces: Vec<ClassRef>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    attributes: Vec<Attribute>,
}
```

### Bytecode Patterns
```rust
enum JVMPattern {
    // Synchronized blocks
    MonitorEnter(ObjectRef),
    MonitorExit(ObjectRef),
    
    // Try-with-resources
    TryWithResources {
        resources: Vec<AutoCloseable>,
        body: BytecodeRange,
        cleanup: BytecodeRange,
    },
    
    // Lambda expressions
    InvokeDynamic {
        bootstrap: BootstrapMethod,
        name: String,
        descriptor: String,
    },
}
```

## 5. .NET/CLR Decompilation

### CLR Metadata
```rust
struct CLRAssembly {
    metadata_root: MetadataRoot,
    type_defs: Vec<TypeDef>,
    method_defs: Vec<MethodDef>,
    field_defs: Vec<FieldDef>,
    assembly_refs: Vec<AssemblyRef>,
}

struct ILMethod {
    header: MethodHeader,
    locals: Vec<LocalVar>,
    instructions: Vec<ILInstruction>,
    exception_handlers: Vec<ExceptionHandler>,
}
```

### Generic Instantiation
```rust
struct GenericInstance {
    generic_def: TypeDef,
    type_args: Vec<Type>,
    method_instances: Vec<MethodInstance>,
}
```

## 6. Python Bytecode

### Python Object Model
```rust
struct PyCodeObject {
    co_name: String,
    co_code: Vec<u8>,          // Bytecode
    co_consts: Vec<PyObject>,  // Constants
    co_names: Vec<String>,     // Names
    co_varnames: Vec<String>,  // Local variables
    co_freevars: Vec<String>,  // Free variables
    co_cellvars: Vec<String>,  // Cell variables
}
```

### Python Patterns
```rust
enum PythonPattern {
    // List comprehension
    ListComp {
        iterator: Expr,
        filter: Option<Expr>,
        transform: Expr,
    },
    
    // Generator expression
    Generator {
        yield_points: Vec<Address>,
        state_machine: StateMachine,
    },
    
    // Decorator pattern
    Decorator {
        wrapper: Function,
        wrapped: Function,
    },
}
```

## 7. JavaScript/V8

### Hidden Classes
```rust
struct HiddenClass {
    properties: Vec<Property>,
    transitions: HashMap<String, HiddenClass>,
    prototype: Option<Address>,
}

struct V8Object {
    hidden_class: Address,
    properties: Vec<Value>,
    elements: Vec<Value>,
}
```

### JIT Optimization Patterns
```rust
enum V8Optimization {
    // Inline caching
    InlineCache {
        site: Address,
        cached_class: HiddenClass,
        cached_offset: usize,
    },
    
    // Hidden class transitions
    MapTransition {
        from: HiddenClass,
        to: HiddenClass,
        property: String,
    },
}
```

## 8. WebAssembly

### WASM Module Structure
```rust
struct WasmModule {
    types: Vec<FuncType>,
    imports: Vec<Import>,
    funcs: Vec<Function>,
    tables: Vec<Table>,
    mems: Vec<Memory>,
    globals: Vec<Global>,
    exports: Vec<Export>,
    data: Vec<DataSegment>,
}
```

### WASM to High-Level
```rust
// Structured control flow reconstruction
fn reconstruct_control_flow(func: &WasmFunction) -> ControlFlow {
    // WASM has structured control flow
    // No arbitrary jumps
    match instruction {
        Block { .. } => StructuredBlock,
        Loop { .. } => StructuredLoop,
        If { .. } => StructuredIf,
        _ => Linear,
    }
}
```

## Language Detection Heuristics

### Binary Signatures
```rust
struct LanguageSignature {
    runtime_symbols: Vec<String>,
    calling_conventions: Vec<CallingConvention>,
    memory_patterns: Vec<MemoryPattern>,
    string_formats: Vec<StringFormat>,
}

const RUST_SIGNATURE: LanguageSignature = LanguageSignature {
    runtime_symbols: vec!["__rust_alloc", "__rust_panic", "core::fmt"],
    calling_conventions: vec![CallingConvention::Rust],
    memory_patterns: vec![MemoryPattern::DropGlue],
    string_formats: vec![StringFormat::UTF8],
};
```

### Confidence Scoring
```rust
fn detect_language(binary: &Binary) -> Vec<(Language, f64)> {
    let mut scores = HashMap::new();
    
    // Symbol analysis
    for symbol in binary.symbols() {
        update_scores(&mut scores, analyze_symbol(symbol));
    }
    
    // Pattern matching
    for pattern in extract_patterns(binary) {
        update_scores(&mut scores, match_pattern(pattern));
    }
    
    // Sort by confidence
    scores.into_iter()
        .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
        .collect()
}
```