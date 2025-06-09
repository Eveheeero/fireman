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

### C++ Coding Style Detection

#### Style Classification

```rust
enum CppStyle {
    ModernCpp20,          // C++20/23 with concepts, ranges, coroutines
    ModernCpp17,          // C++17 with structured bindings, std::optional
    ModernCpp14,          // C++14 with auto return types, generic lambdas
    ModernCpp11,          // C++11 with move semantics, lambdas
    LegacyCpp03,          // Pre-C++11 traditional style
    CWithClasses,         // C-style with minimal C++ features
    BoostHeavy,          // Heavy Boost library usage
    QtStyle,             // Qt framework patterns
    EmbeddedCpp,         // Embedded/restricted C++ (no exceptions/RTTI)
    GameEngine,          // Game engine patterns (ECS, custom allocators)
}

struct StyleIndicators {
    smart_pointers: Vec<SmartPointerType>,
    stl_usage: HashMap<String, usize>,
    namespace_patterns: Vec<String>,
    naming_conventions: NamingStyle,
    memory_management: MemoryStyle,
}
```

### Modern C++ Features

#### Template Metaprogramming
```rust
struct TemplateInstance {
    template_name: String,
    type_params: Vec<Type>,
    instantiation_addr: Address,
    mangled_name: String,
    sfinae_patterns: Vec<SFINAEPattern>,
}

// Advanced template patterns
enum TemplatePattern {
    // Variadic templates
    VariadicTemplate {
        param_pack: String,
        expansion_pattern: String,
    },
    // SFINAE (Substitution Failure Is Not An Error)
    SFINAE {
        enable_if_conditions: Vec<Condition>,
        decltype_expressions: Vec<Expression>,
    },
    // Concepts (C++20)
    Concept {
        name: String,
        requirements: Vec<Requirement>,
    },
    // CRTP (Curiously Recurring Template Pattern)
    CRTP {
        base_template: String,
        derived_class: String,
    },
}

// Recover template parameters from mangled names
fn demangle_template(mangled: &str) -> TemplateInstance {
    // Parse Itanium ABI mangling
    // Handle nested templates, parameter packs
    // Extract template arguments and constraints
}
```

#### Modern Memory Management

```rust
enum SmartPointerType {
    UniquePtr { deleter: Option<Address> },
    SharedPtr { control_block: Address },
    WeakPtr { control_block: Address },
    IntrusivePtr { refcount_offset: usize },
}

struct MemoryPattern {
    // RAII patterns
    raii_constructors: Vec<Constructor>,
    raii_destructors: Vec<Destructor>,
    
    // Move semantics
    move_constructors: Vec<MoveConstructor>,
    move_assignments: Vec<MoveAssignment>,
    
    // Custom allocators
    allocator_types: Vec<AllocatorType>,
    memory_pools: Vec<MemoryPool>,
}
```

#### Lambda and Closure Analysis

```rust
struct LambdaClosure {
    capture_list: Vec<CaptureItem>,
    function_ptr: Address,
    closure_type: ClosureType,
    mutable_flag: bool,
}

enum CaptureItem {
    ByValue { var_name: String, type_: Type },
    ByReference { var_name: String, type_: Type },
    ByMove { var_name: String, type_: Type },
    This { class_ptr: Address },
    StarThis { class_copy: Type },
}

// Generic lambdas (C++14)
struct GenericLambda {
    template_params: Vec<AutoParam>,
    body: Address,
}
```

#### Coroutines (C++20)

```rust
struct Coroutine {
    promise_type: Type,
    initial_suspend: Address,
    final_suspend: Address,
    yield_points: Vec<YieldPoint>,
    state_size: usize,
}

struct CoroutineFrame {
    promise: Address,
    resume_point: usize,
    locals: Vec<LocalVar>,
    temporaries: Vec<TempVar>,
}
```

#### Ranges and Views (C++20)

```rust
enum RangeAdapter {
    Filter { predicate: Address },
    Transform { function: Address },
    Take { count: usize },
    Drop { count: usize },
    Join,
    Split { delimiter: Value },
    Reverse,
}

struct RangePipeline {
    source: RangeSource,
    adapters: Vec<RangeAdapter>,
    materialization: Option<ContainerType>,
}
```

#### Virtual Function Tables and Polymorphism
```rust
struct CppClass {
    vtable: Address,
    rtti: Option<RTTIInfo>,
    base_classes: Vec<BaseClass>,
    virtual_bases: Vec<VirtualBase>,
    members: Vec<Member>,
    
    // Modern features
    deleted_functions: Vec<Function>,
    defaulted_functions: Vec<Function>,
    final_specifier: bool,
    override_specifiers: Vec<Function>,
}

struct VTableLayout {
    offset_to_top: isize,
    rtti_ptr: Address,
    virtual_funcs: Vec<VirtualFunc>,
    
    // Multiple inheritance
    primary_base: Option<BaseClass>,
    secondary_bases: Vec<BaseClass>,
    virtual_base_offsets: Vec<(Type, isize)>,
}

// Devirtualization opportunities
struct DevirtAnalysis {
    final_classes: Vec<Type>,
    sealed_hierarchies: Vec<ClassHierarchy>,
    monomorphic_calls: Vec<CallSite>,
}
```

#### Exception Handling
```rust
struct ExceptionHandler {
    try_start: Address,
    try_end: Address,
    catch_blocks: Vec<CatchBlock>,
    cleanup: Option<Address>,
    noexcept_flag: bool,
}

struct CatchBlock {
    type_info: Address,
    handler: Address,
    catch_all: bool,    // catch(...)
    rethrow_sites: Vec<Address>,
}

// Modern error handling patterns
enum ErrorHandling {
    Traditional { exceptions: Vec<ExceptionType> },
    Expected { success_type: Type, error_type: Type },
    Optional { value_type: Type },
    ErrorCode { enum_type: Type },
}
```

#### STL Container Recognition
```rust
enum STLContainer {
    // Sequence containers
    Vector { 
        begin: Address,
        end: Address,
        capacity: Address,
        allocator: Option<AllocatorType>,
    },
    Array {
        data: Address,
        size: usize,
    },
    Deque {
        map: Address,
        start: DequeIterator,
        finish: DequeIterator,
    },
    List {
        head: Address,
        size: usize,
        node_type: Type,
    },
    
    // Associative containers
    Map {
        root: Address,
        size: usize,
        comparator: Address,
        key_type: Type,
        value_type: Type,
    },
    UnorderedMap {
        buckets: Address,
        bucket_count: usize,
        hasher: Address,
        key_equal: Address,
    },
    
    // Container adaptors
    Stack { container: Box<STLContainer> },
    Queue { container: Box<STLContainer> },
    PriorityQueue {
        container: Box<STLContainer>,
        comparator: Address,
    },
    
    // Strings
    String {
        data: StringRep,
        size: usize,
        capacity: usize,
    },
    StringView {
        data: Address,
        size: usize,
    },
    
    // C++17/20 containers
    Optional { has_value: bool, storage: Address },
    Variant { index: usize, storage: Address },
    Any { type_info: Address, storage: Address },
    Span { data: Address, size: usize },
}

// Small string optimization patterns
enum StringRep {
    Small([u8; 24]),     // SSO buffer size varies
    Large(Address),
}
```

#### Modern C++ Idioms

```rust
struct ModernIdioms {
    // std::forward and perfect forwarding
    perfect_forwarding: Vec<ForwardingRef>,
    
    // Fold expressions (C++17)
    fold_expressions: Vec<FoldExpr>,
    
    // Structured bindings (C++17)
    structured_bindings: Vec<BindingPattern>,
    
    // If-init statements (C++17)
    if_init_statements: Vec<IfInit>,
    
    // Constexpr evaluation
    constexpr_functions: Vec<ConstexprFunc>,
    consteval_functions: Vec<ConstevalFunc>,
    
    // Modules (C++20)
    module_interfaces: Vec<ModuleInterface>,
    module_implementations: Vec<ModuleImpl>,
}
```

#### Library-Specific Patterns

##### Boost Library Patterns

```rust
struct BoostPatterns {
    // Boost smart pointers (pre-C++11)
    boost_shared_ptr: Vec<Address>,
    boost_scoped_ptr: Vec<Address>,

    // Boost type traits
    enable_if_patterns: Vec<EnableIf>,

    // Boost Spirit parsers
    spirit_grammars: Vec<Grammar>,

    // Boost Asio
    asio_handlers: Vec<AsyncHandler>,
}
```

##### Qt Framework Patterns

```rust
struct QtPatterns {
    // Qt meta-object system
    q_objects: Vec<QObject>,
    moc_data: Vec<MetaObjectData>,

    // Signal/slot connections
    signal_emissions: Vec<SignalEmit>,
    slot_invocations: Vec<SlotInvoke>,

    // Qt containers
    qstring_usage: Vec<QString>,
    qvector_usage: Vec<QVector>,
}
```

##### Game Engine Patterns

```rust
struct GameEnginePatterns {
    // Entity-Component-System
    entities: Vec<Entity>,
    components: Vec<Component>,
    systems: Vec<System>,

    // Custom memory management
    frame_allocators: Vec<FrameAllocator>,
    object_pools: Vec<ObjectPool>,

    // SIMD operations
    simd_intrinsics: Vec<SIMDOp>,
    vectorized_loops: Vec<VectorizedLoop>,
}
```

### Legacy C++ Patterns

#### Pre-Standard C++ (Pre-1998)

```rust
struct PreStandardCpp {
    // Old-style headers
    iostream_h: bool,           // <iostream.h> instead of <iostream>
    no_namespaces: bool,        // No std namespace
    
    // Non-standard features
    far_near_pointers: Vec<FarNearPtr>,  // DOS/Win16 era
    huge_pointers: Vec<HugePtr>,         // Segmented memory
    
    // Borland/Microsoft extensions
    __fastcall: Vec<Function>,
    __cdecl: Vec<Function>,
    __stdcall: Vec<Function>,
    
    // Pre-standard containers
    proprietary_strings: Vec<ProprietaryString>,
    custom_collections: Vec<CustomCollection>,
}

enum FarNearPtr {
    Near { segment: u16, offset: u16 },
    Far { segment: u16, offset: u16 },
    Huge { normalized_addr: u32 },
}
```

#### C++98/03 Legacy Patterns

```rust
struct LegacyCpp03 {
    // Manual memory management
    raw_new_delete: Vec<RawAllocation>,
    array_new_delete: Vec<ArrayAllocation>,
    
    // auto_ptr usage (deprecated)
    auto_ptrs: Vec<AutoPtr>,
    
    // Function objects instead of lambdas
    functors: Vec<Functor>,
    bind1st_bind2nd: Vec<BinderUsage>,
    
    // Old-style loops
    explicit_iterators: Vec<IteratorLoop>,
    index_loops: Vec<IndexLoop>,
}

struct RawAllocation {
    new_site: Address,
    delete_site: Option<Address>,  // May be missing (leak)
    type_allocated: Type,
}

struct AutoPtr {
    // Deprecated C++98 smart pointer
    release_calls: Vec<Address>,
    reset_calls: Vec<Address>,
    // Dangerous copy semantics
    ownership_transfers: Vec<Address>,
}
```

#### Old-Style Function Declarations

```rust
struct OldFunctionStyles {
    // K&R style parameters
    kr_style: Vec<KRFunction>,
    
    // Old exception specifications
    throw_specs: Vec<ThrowSpec>,
    
    // C-style varargs
    varargs_functions: Vec<VarArgsFunc>,
    
    // Old-style casts
    c_style_casts: Vec<CStyleCast>,
}

struct KRFunction {
    // int func(a, b)
    // int a;
    // char* b;
    // { ... }
    name: String,
    param_names: Vec<String>,
    param_declarations: Vec<ParamDecl>,
}

struct ThrowSpec {
    // void func() throw(std::bad_alloc, MyException)
    function: Address,
    allowed_exceptions: Vec<Type>,
    empty_throw: bool,  // throw()
}
```

#### Pre-STL Container Patterns

```rust
enum PreSTLContainer {
    // MFC containers
    CArray { element_type: Type },
    CList { element_type: Type },
    CMap { key_type: Type, value_type: Type },
    CString,
    
    // ATL containers
    CAtlArray { element_type: Type },
    CAtlList { element_type: Type },
    CAtlMap { key_type: Type, value_type: Type },
    
    // Rogue Wave containers
    RWCString,
    RWTValVector { element_type: Type },
    RWTPtrVector { element_type: Type },
    
    // Home-grown containers
    CustomVector {
        data_ptr: Address,
        size_field: Address,
        capacity_field: Address,
        growth_strategy: GrowthStrategy,
    },
}
```

#### Old COM Patterns

```rust
struct COMPatterns {
    // IUnknown implementation
    iunknown_vtable: VTable,
    queryinterface: Address,
    addref: Address,
    release: Address,
    
    // COM object patterns
    com_objects: Vec<COMObject>,
    
    // ATL patterns
    atl_com_maps: Vec<ATLComMap>,
    atl_object_maps: Vec<ATLObjectMap>,
    
    // Smart pointers
    com_ptrs: Vec<CComPtr>,
    variant_usage: Vec<Variant>,
}

struct COMObject {
    clsid: [u8; 16],  // GUID
    interfaces: Vec<Interface>,
    ref_count: Address,
    aggregation: Option<Address>,
}
```

#### Old Exception Patterns

```rust
struct OldExceptionPatterns {
    // SEH (Structured Exception Handling) on Windows
    seh_handlers: Vec<SEHHandler>,

    // setjmp/longjmp error handling
    setjmp_buffers: Vec<JmpBuf>,
    longjmp_sites: Vec<Address>,

    // Error codes
    hresult_usage: Vec<HResult>,
    errno_checking: Vec<ErrnoCheck>,

    // MFC exceptions
    cexception_throws: Vec<MFCException>,
}

struct SEHHandler {
    try_level: i32,
    filter: Address,
    handler: Address,
}
```

#### Legacy String Handling

```rust
struct LegacyStringPatterns {
    // C-style strings
    strcpy_usage: Vec<StrcpyCall>,
    strcat_usage: Vec<StrcatCall>,
    sprintf_usage: Vec<SprintfCall>,

    // Fixed buffers
    stack_buffers: Vec<FixedBuffer>,

    // TCHAR and Unicode transitions
    tchar_usage: Vec<TCharUsage>,

    // BSTR (Basic String) for COM
    bstr_allocations: Vec<BSTRAlloc>,

    // CString (MFC)
    cstring_operations: Vec<CStringOp>,
}

struct FixedBuffer {
    size: usize,
    overflow_potential: bool,
    bounds_checking: bool,
}
```

#### Old-Style Type Punning

```rust
struct TypePunning {
    // Union-based type punning
    union_punning: Vec<UnionPun>,

    // Reinterpret casts
    reinterpret_casts: Vec<ReinterpretCast>,

    // Aliasing violations
    strict_aliasing_violations: Vec<AliasingViolation>,

    // Endianness assumptions
    endian_dependent_code: Vec<EndianAssumption>,
}

struct UnionPun {
    union_type: Type,
    write_member: String,
    read_member: String,
}
```

#### Windows-Specific Legacy

```rust
struct WindowsLegacy {
    // Win32 API patterns
    window_procedures: Vec<WndProc>,
    message_crackers: Vec<MessageCracker>,

    // Hungarian notation
    hungarian_prefixes: HashMap<String, HungarianType>,

    // Resource management
    gdi_objects: Vec<GDIObject>,
    kernel_handles: Vec<KernelHandle>,

    // DLL patterns
    dllmain: Option<Address>,
    exported_functions: Vec<ExportedFunc>,
    delay_loaded_dlls: Vec<DelayLoadDLL>,
}

enum HungarianType {
    Boolean,        // b or f prefix
    ByteChar,       // ch prefix
    Integer,        // i or n prefix
    Long,           // l prefix
    Pointer,        // p or lp prefix
    String,         // sz or str prefix
    Handle,         // h prefix
    Word,           // w prefix
    DWord,          // dw prefix
}
```

#### Pre-Template Generic Programming

```rust
struct PreTemplateGenerics {
    // Macro-based generics
    macro_templates: Vec<MacroTemplate>,

    // void* based generics
    void_ptr_containers: Vec<VoidPtrContainer>,

    // Preprocessor metaprogramming
    pp_metaprograms: Vec<PreprocessorMeta>,

    // Type-unsafe callbacks
    function_pointers: Vec<UnsafeFuncPtr>,
}

struct MacroTemplate {
    // #define DECLARE_LIST(Type) \
    //   class Type##List { ... }
    macro_name: String,
    expansions: Vec<MacroExpansion>,
}
```

#### Build System Artifacts

```rust
struct BuildArtifacts {
    // Precompiled headers
    pch_usage: bool,
    stdafx_pattern: bool,

    // Export tables
    def_file_exports: Vec<Export>,
    declspec_dllexport: Vec<Address>,

    // Name mangling styles
    mangling_scheme: ManglingScheme,
}

enum ManglingScheme {
    Itanium,        // GCC, Clang
    MSVC,           // Microsoft
    Borland,        // Borland C++
    Watcom,         // Watcom C++
    Legacy,         // Pre-standard
}
```

## 4. C Language Decompilation

### Old-Style C Patterns

#### K&R C (Pre-ANSI)

```rust
struct KandRC {
    // K&R function declarations
    kr_functions: Vec<KRFunctionDecl>,

    // Implicit int
    implicit_int_declarations: Vec<ImplicitInt>,

    // Old-style function pointers
    untyped_func_ptrs: Vec<Address>,

    // No function prototypes
    missing_prototypes: Vec<Function>,

    // Default int return type
    implicit_returns: Vec<Function>,
}

struct KRFunctionDecl {
    // main(argc, argv)
    // int argc;
    // char **argv;
    // { ... }
    name: String,
    params: Vec<String>,
    param_decls: Vec<(String, Type)>,
}

struct ImplicitInt {
    // register i;  // implicitly int
    // unsigned j;  // implicitly unsigned int
    location: Address,
    actual_type: Type,
}
```

#### Classic C Idioms

```rust
struct ClassicCIdioms {
    // String manipulation
    null_terminated_strings: Vec<CString>,
    string_buffers: Vec<CharBuffer>,

    // Manual memory management
    malloc_free_pairs: Vec<MallocFreePair>,
    alloca_usage: Vec<AllocaCall>,

    // Bit manipulation
    bit_fields: Vec<BitField>,
    bit_masks: Vec<BitMask>,

    // Macro heavy code
    macro_constants: Vec<MacroConstant>,
    macro_functions: Vec<MacroFunction>,
}

struct CharBuffer {
    declaration: Address,
    size: usize,
    stack_allocated: bool,
    overflow_checks: Vec<BoundsCheck>,
}
```

#### Pre-C89 Patterns

```rust
struct PreC89 {
    // No void type
    empty_param_lists: Vec<Function>,  // f() instead of f(void)

    // No const/volatile
    missing_qualifiers: bool,

    // No void pointers
    char_ptr_as_generic: Vec<CharPtrUsage>,

    // External linkage by default
    implicit_extern: Vec<Symbol>,

    // No standard library
    custom_implementations: Vec<StandardFunction>,
}
```

#### Unsafe C Patterns

```rust
struct UnsafeCPatterns {
    // Buffer overflows
    gets_usage: Vec<GetsCall>,
    strcpy_unchecked: Vec<StrcpyCall>,
    sprintf_unchecked: Vec<SprintfCall>,

    // Format string vulnerabilities
    printf_var_format: Vec<FormatStringVuln>,

    // Integer overflows
    unchecked_arithmetic: Vec<ArithmeticOp>,

    // Uninitialized variables
    uninitialized_reads: Vec<UninitRead>,

    // Double frees
    double_frees: Vec<DoubleFree>,
    use_after_free: Vec<UseAfterFree>,
}

struct FormatStringVuln {
    call_site: Address,
    format_source: FormatSource,
    controlled_by_user: bool,
}
```

#### Old-Style Type Systems

```rust
struct OldTypeSystem {
    // typedef abuse
    typedef_chains: Vec<TypedefChain>,

    // Opaque pointers
    void_ptr_casts: Vec<VoidPtrCast>,

    // Type punning via unions
    union_casts: Vec<UnionCast>,

    // Struct hack (flexible array member)
    struct_hacks: Vec<StructHack>,
}

struct StructHack {
    // Pre-C99 flexible array member
    // struct s {
    //     int len;
    //     char data[1];  // Actually variable length
    // };
    struct_type: Type,
    fake_array_member: String,
    allocation_pattern: AllocationPattern,
}
```

### Modern C Patterns

#### C99 Features

```rust
struct C99Features {
    // Variable length arrays
    vlas: Vec<VLA>,

    // Designated initializers
    designated_inits: Vec<DesignatedInit>,

    // Compound literals
    compound_literals: Vec<CompoundLiteral>,

    // Inline functions
    inline_functions: Vec<InlineFunc>,

    // Restrict pointers
    restrict_ptrs: Vec<RestrictPtr>,

    // _Bool type
    bool_usage: Vec<BoolUsage>,

    // Long long support
    long_long_usage: Vec<LongLong>,
}

struct VLA {
    array_decl: Address,
    size_expr: Expression,
    stack_allocation: Address,
}

struct DesignatedInit {
    // struct point p = { .x = 1, .y = 2 };
    // int a[10] = { [0] = 1, [5] = 2 };
    target_type: Type,
    designators: Vec<Designator>,
}
```

#### C11 Features

```rust
struct C11Features {
    // Static assertions
    static_asserts: Vec<StaticAssert>,

    // Generic selections
    generic_selections: Vec<GenericSelection>,

    // Anonymous unions/structs
    anonymous_members: Vec<AnonymousMember>,

    // Aligned allocation
    aligned_alloc_calls: Vec<AlignedAlloc>,

    // Thread local storage
    thread_locals: Vec<ThreadLocal>,

    // Atomic operations
    atomics: Vec<AtomicOp>,
}

struct GenericSelection {
    // _Generic(x, int: 1, float: 2.0, default: 0)
    control_expr: Expression,
    associations: Vec<(Type, Expression)>,
    default_expr: Option<Expression>,
}

struct AtomicOp {
    operation: AtomicOperation,
    memory_order: MemoryOrder,
    address: Address,
}
```

#### C17/C18 Updates

```rust
struct C17Features {
    // Mostly bug fixes and clarifications
    // No major new features

    // Better VLA support
    vla_improvements: Vec<VLAUsage>,

    // Clarified atomics
    atomic_clarifications: Vec<AtomicPattern>,
}
```

#### C23 Features

```rust
struct C23Features {
    // Type inference
    auto_declarations: Vec<AutoDecl>,

    // typeof operator
    typeof_usage: Vec<TypeofUsage>,

    // Binary literals
    binary_literals: Vec<BinaryLiteral>,

    // Digit separators
    digit_separators: Vec<DigitSeparator>,

    // Attributes
    attributes: Vec<Attribute>,

    // BitInt types
    bit_ints: Vec<BitInt>,

    // nullptr constant
    nullptr_usage: Vec<NullptrUsage>,
}

struct BitInt {
    width: usize,
    signed: bool,
    usage_sites: Vec<Address>,
}
```

#### Modern C Idioms

```rust
struct ModernCIdioms {
    // Safe string handling
    strlcpy_strlcat: Vec<SafeStringOp>,
    snprintf_usage: Vec<SnprintfCall>,

    // Bounds checking
    explicit_bounds: Vec<BoundsCheck>,

    // Error handling patterns
    error_codes: Vec<ErrorCodePattern>,
    result_types: Vec<ResultType>,

    // Resource management
    cleanup_attributes: Vec<CleanupAttr>,
    raii_style: Vec<RAIIPattern>,
}

struct ErrorCodePattern {
    // Modern error handling
    // if ((err = function()) != 0) { handle_error(err); }
    error_type: Type,
    check_pattern: CheckPattern,
    error_propagation: PropagationStyle,
}
```

#### Compiler Extensions

```rust
struct CompilerExtensions {
    // GCC extensions
    gcc_attributes: Vec<GCCAttribute>,
    gcc_builtins: Vec<GCCBuiltin>,
    statement_expressions: Vec<StmtExpr>,

    // Clang extensions
    clang_attributes: Vec<ClangAttribute>,
    clang_builtins: Vec<ClangBuiltin>,

    // MSVC extensions
    msvc_intrinsics: Vec<MSVCIntrinsic>,
    msvc_pragmas: Vec<MSVCPragma>,

    // Common extensions
    inline_assembly: Vec<InlineAsm>,
    computed_gotos: Vec<ComputedGoto>,
}

struct GCCAttribute {
    name: String,  // packed, aligned, noreturn, etc.
    arguments: Vec<Value>,
    target: AttributeTarget,
}
```

#### Security-Focused C

```rust
struct SecureC {
    // Secure coding standard compliance
    cert_c_compliance: Vec<CERTRule>,
    misra_c_compliance: Vec<MISRARule>,

    // Bounds checking interfaces
    bounds_checking_interfaces: Vec<BoundsCheckingCall>,

    // Safe arithmetic
    overflow_checks: Vec<OverflowCheck>,

    // Memory safety
    sanitizer_annotations: Vec<SanitizerAnnotation>,
}

struct CERTRule {
    rule_id: String,  // e.g., "STR31-C"
    description: String,
    violations: Vec<Address>,
}
```

#### Embedded C Patterns

```rust
struct EmbeddedC {
    // Hardware register access
    volatile_registers: Vec<VolatileReg>,
    memory_mapped_io: Vec<MMIO>,

    // Interrupt handlers
    isr_functions: Vec<ISR>,

    // Fixed-point arithmetic
    fixed_point_ops: Vec<FixedPointOp>,

    // Bit manipulation
    bit_banding: Vec<BitBanding>,

    // Resource constraints
    no_heap_allocation: bool,
    static_allocation_only: bool,
}

struct VolatileReg {
    address: Address,
    register_name: String,
    access_width: usize,
    read_only: bool,
    write_only: bool,
}
```

#### Performance-Oriented C

```rust
struct PerformanceC {
    // Loop optimizations
    unrolled_loops: Vec<UnrolledLoop>,
    vectorized_loops: Vec<VectorizedLoop>,

    // Cache optimizations
    cache_aligned_data: Vec<CacheAligned>,
    prefetch_hints: Vec<Prefetch>,

    // Branch prediction hints
    likely_unlikely: Vec<BranchHint>,

    // SIMD intrinsics
    sse_intrinsics: Vec<SSEIntrinsic>,
    avx_intrinsics: Vec<AVXIntrinsic>,
    neon_intrinsics: Vec<NEONIntrinsic>,
}

struct BranchHint {
    condition: Expression,
    likely: bool,
    hint_type: HintType,  // __builtin_expect, [[likely]], etc.
}
```

#### C Library Detection

```rust
struct CLibraryPatterns {
    // Standard library versions
    libc_version: LibCVersion,

    // Common libraries
    openssl_patterns: Vec<OpenSSLPattern>,
    zlib_patterns: Vec<ZlibPattern>,
    sqlite_patterns: Vec<SQLitePattern>,

    // Framework patterns
    gtk_patterns: Vec<GTKPattern>,
    sdl_patterns: Vec<SDLPattern>,
}

enum LibCVersion {
    Glibc { version: String },
    Musl { version: String },
    Bionic { version: String },
    MSVCRT { version: String },
    Custom { name: String },
}
```

## 5. C# / .NET Decompilation

### C# Language Evolution

#### C# 1.0-2.0 (Classic)

```rust
struct CSharpClassic {
    // Basic OOP
    classes: Vec<ClassDef>,
    interfaces: Vec<InterfaceDef>,
    delegates: Vec<DelegateDef>,
    events: Vec<EventDef>,

    // Properties
    properties: Vec<PropertyDef>,
    indexers: Vec<IndexerDef>,

    // Generics (C# 2.0)
    generic_types: Vec<GenericType>,
    generic_methods: Vec<GenericMethod>,

    // Iterators (C# 2.0)
    yield_statements: Vec<YieldStatement>,
    iterator_blocks: Vec<IteratorBlock>,
}

struct PropertyDef {
    name: String,
    type_: Type,
    getter: Option<MethodDef>,
    setter: Option<MethodDef>,
    backing_field: Option<FieldDef>,
}

struct IteratorBlock {
    method: MethodDef,
    state_machine: StateMachine,
    yield_points: Vec<YieldPoint>,
}
```

#### C# 3.0-5.0 (Modern Features)

```rust
struct CSharpModern {
    // LINQ
    linq_queries: Vec<LinqQuery>,
    lambda_expressions: Vec<LambdaExpr>,
    expression_trees: Vec<ExpressionTree>,

    // Anonymous types
    anonymous_types: Vec<AnonymousType>,

    // Extension methods
    extension_methods: Vec<ExtensionMethod>,

    // Async/Await (C# 5.0)
    async_methods: Vec<AsyncMethod>,
    await_expressions: Vec<AwaitExpr>,

    // Dynamic (C# 4.0)
    dynamic_operations: Vec<DynamicOp>,
}

struct LinqQuery {
    // from x in collection
    // where x.Property > 5
    // select new { x.Name, x.Value }
    clauses: Vec<LinqClause>,
    source: Expression,
    projection: Expression,
}

struct AsyncMethod {
    method_def: MethodDef,
    state_machine: AsyncStateMachine,
    await_points: Vec<AwaitPoint>,
    task_type: TaskType,
}
```

#### C# 6.0-7.3 (Syntax Sugar)

```rust
struct CSharpSyntaxSugar {
    // Expression-bodied members
    expression_bodied: Vec<ExpressionBodiedMember>,

    // String interpolation
    interpolated_strings: Vec<InterpolatedString>,

    // Pattern matching
    pattern_matches: Vec<PatternMatch>,

    // Tuples
    value_tuples: Vec<ValueTuple>,
    tuple_deconstruction: Vec<Deconstruction>,

    // Local functions
    local_functions: Vec<LocalFunction>,

    // Ref returns/locals
    ref_returns: Vec<RefReturn>,
    ref_locals: Vec<RefLocal>,
}

struct PatternMatch {
    expression: Expression,
    patterns: Vec<Pattern>,
}

enum Pattern {
    Type { type_: Type, variable: Option<String> },
    Constant { value: ConstValue },
    Var { variable: String },
    When { pattern: Box<Pattern>, condition: Expression },
}
```

#### C# 8.0-9.0 (Nullable & Patterns)

```rust
struct CSharpNullablePatterns {
    // Nullable reference types
    nullable_annotations: Vec<NullableAnnotation>,
    null_checks: Vec<NullCheck>,

    // Advanced patterns
    switch_expressions: Vec<SwitchExpression>,
    property_patterns: Vec<PropertyPattern>,
    positional_patterns: Vec<PositionalPattern>,

    // Default interface methods
    interface_implementations: Vec<DefaultInterfaceImpl>,

    // Records (C# 9.0)
    records: Vec<RecordDef>,
    with_expressions: Vec<WithExpr>,

    // Init-only properties
    init_properties: Vec<InitOnlyProperty>,
}

struct RecordDef {
    name: String,
    parameters: Vec<Parameter>,
    properties: Vec<PropertyDef>,
    deconstructor: Option<MethodDef>,
    equality_members: EqualityMembers,
}

struct SwitchExpression {
    target: Expression,
    arms: Vec<SwitchArm>,
}
```

#### C# 10.0-12.0 (Latest Features)

```rust
struct CSharpLatest {
    // Global usings
    global_usings: Vec<GlobalUsing>,
    implicit_usings: Vec<ImplicitUsing>,

    // File-scoped namespaces
    file_scoped_namespaces: Vec<FileScopedNamespace>,

    // Pattern improvements
    list_patterns: Vec<ListPattern>,
    extended_property_patterns: Vec<ExtendedPropertyPattern>,

    // Generic math (C# 11.0)
    static_abstracts: Vec<StaticAbstract>,
    generic_math_constraints: Vec<GenericMathConstraint>,

    // Required members (C# 11.0)
    required_members: Vec<RequiredMember>,

    // Primary constructors (C# 12.0)
    primary_constructors: Vec<PrimaryConstructor>,

    // Collection expressions (C# 12.0)
    collection_expressions: Vec<CollectionExpr>,
}

struct StaticAbstract {
    interface: InterfaceDef,
    static_members: Vec<StaticMember>,
    operators: Vec<OperatorDef>,
}
```

### IL (Intermediate Language) Patterns

#### IL Instruction Patterns

```rust
struct ILPatterns {
    // Stack manipulation
    stack_ops: Vec<StackOp>,

    // Method calls
    call_instructions: Vec<CallInstruction>,

    // Object creation
    newobj_instructions: Vec<NewObjInstruction>,

    // Field access
    field_access: Vec<FieldAccess>,

    // Exception handling
    exception_blocks: Vec<ExceptionBlock>,
}

enum CallInstruction {
    Call { method: MethodRef },           // Static call
    Callvirt { method: MethodRef },       // Virtual call
    Calli { signature: CallSig },         // Indirect call
    Constrained { type_: Type },          // Constrained virtual call
}

struct ExceptionBlock {
    try_start: ILOffset,
    try_end: ILOffset,
    handlers: Vec<ExceptionHandler>,
}

enum ExceptionHandler {
    Catch { type_: Type, handler: ILOffset },
    Filter { filter: ILOffset, handler: ILOffset },
    Finally { handler: ILOffset },
    Fault { handler: ILOffset },
}
```

#### Compiler-Generated Code

```rust
struct CompilerGenerated {
    // Auto-properties backing fields
    backing_fields: Vec<BackingField>,

    // Anonymous type implementations
    anonymous_implementations: Vec<AnonymousImpl>,

    // Closure classes
    closure_classes: Vec<ClosureClass>,

    // State machines
    async_state_machines: Vec<AsyncStateMachine>,
    iterator_state_machines: Vec<IteratorStateMachine>,

    // Display classes (for closures)
    display_classes: Vec<DisplayClass>,
}

struct ClosureClass {
    name: String,  // <>c__DisplayClass0_0
    captured_vars: Vec<CapturedVariable>,
    lambda_methods: Vec<LambdaMethod>,
}

struct AsyncStateMachine {
    state_field: FieldDef,
    move_next_method: MethodDef,
    set_state_machine: MethodDef,
    awaiter_fields: Vec<AwaiterField>,
}
```

### .NET Framework Patterns

#### Framework Detection

```rust
struct DotNetFramework {
    // Target framework
    target_framework: TargetFramework,

    // Assembly references
    framework_assemblies: Vec<AssemblyRef>,
    nuget_packages: Vec<PackageRef>,

    // Runtime features
    runtime_version: RuntimeVersion,
}

enum TargetFramework {
    NetFramework { version: String },     // .NET Framework 4.x
    NetCore { version: String },          // .NET Core 1.x-3.x
    Net { version: String },              // .NET 5+
    NetStandard { version: String },      // .NET Standard
    Mono { version: String },             // Mono runtime
}
```

#### Common Framework Patterns

```rust
struct FrameworkPatterns {
    // ASP.NET patterns
    aspnet_mvc: Vec<MVCPattern>,
    aspnet_core: Vec<AspNetCorePattern>,

    // WPF/WinForms
    wpf_bindings: Vec<WPFBinding>,
    winforms_controls: Vec<WinFormsControl>,

    // Entity Framework
    ef_contexts: Vec<DbContext>,
    ef_queries: Vec<EFQuery>,

    // Dependency Injection
    di_containers: Vec<DIContainer>,
    service_registrations: Vec<ServiceRegistration>,
}

struct MVCPattern {
    controllers: Vec<ControllerClass>,
    actions: Vec<ActionMethod>,
    views: Vec<ViewRef>,
    models: Vec<ModelClass>,
}
```

### Attribute Analysis

```rust
struct AttributeAnalysis {
    // Security attributes
    security_attributes: Vec<SecurityAttribute>,

    // Serialization attributes
    serialization_attrs: Vec<SerializationAttr>,

    // Compiler attributes
    compiler_attrs: Vec<CompilerAttribute>,

    // Custom attributes
    custom_attrs: Vec<CustomAttribute>,
}

enum SecurityAttribute {
    SecurityCritical,
    SecuritySafeCritical,
    SecurityTransparent,
    AllowPartiallyTrustedCallers,
}

enum CompilerAttribute {
    CompilerGenerated,
    DebuggerHidden,
    DebuggerStepThrough,
    AsyncStateMachine { type_: Type },
    IteratorStateMachine { type_: Type },
}
```

### Obfuscation Detection

```rust
struct ObfuscationPatterns {
    // Name obfuscation
    obfuscated_names: Vec<ObfuscatedName>,

    // Control flow obfuscation
    cf_obfuscation: Vec<ControlFlowObfuscation>,

    // String encryption
    encrypted_strings: Vec<EncryptedString>,

    // Anti-tampering
    integrity_checks: Vec<IntegrityCheck>,

    // Packing
    packed_assemblies: Vec<PackedAssembly>,
}

struct ObfuscatedName {
    original_hint: Option<String>,
    obfuscated: String,
    name_pattern: NamePattern,
}

enum NamePattern {
    Random,
    Incremental,
    Unicode,
    Unprintable,
}
```

### Decompilation Optimizations

#### Pattern Recognition

```rust
struct CSharpPatternRecognition {
    // Using statements
    using_patterns: Vec<UsingPattern>,

    // Lock statements
    lock_patterns: Vec<LockPattern>,

    // Foreach loops
    foreach_patterns: Vec<ForeachPattern>,

    // LINQ method chains
    linq_chains: Vec<LinqChain>,
}

struct UsingPattern {
    // using (var resource = new Resource())
    // Translates from try-finally with Dispose
    resource_acquisition: Expression,
    body: Block,
    dispose_call: MethodCall,
}

struct ForeachPattern {
    // Recognizes GetEnumerator/MoveNext/Current pattern
    collection: Expression,
    enumerator_var: LocalVar,
    loop_var: LocalVar,
    body: Block,
}
```

#### Code Reconstruction

```rust
struct CodeReconstruction {
    // Auto-property detection
    auto_properties: Vec<AutoPropertyReconstruction>,

    // String interpolation reconstruction
    string_interpolations: Vec<StringInterpolationReconstruction>,

    // Pattern matching reconstruction
    pattern_reconstructions: Vec<PatternReconstruction>,

    // LINQ query syntax
    linq_syntax_reconstruction: Vec<LinqSyntaxReconstruction>,
}

struct AutoPropertyReconstruction {
    property: PropertyDef,
    backing_field: FieldDef,
    getter_pattern: GetterPattern,
    setter_pattern: Option<SetterPattern>,
}
```

### Platform-Specific Patterns

#### Unity Patterns

```rust
struct UnityPatterns {
    // MonoBehaviour scripts
    mono_behaviours: Vec<MonoBehaviour>,

    // Coroutines
    coroutines: Vec<Coroutine>,

    // Unity-specific attributes
    unity_attributes: Vec<UnityAttribute>,

    // Asset references
    asset_refs: Vec<AssetReference>,
}

struct MonoBehaviour {
    class_def: ClassDef,
    lifecycle_methods: Vec<LifecycleMethod>,
    serialized_fields: Vec<SerializedField>,
}
```

#### Xamarin Patterns

```rust
struct XamarinPatterns {
    // Platform-specific code
    platform_specific: Vec<PlatformSpecific>,

    // Dependency service
    dependency_services: Vec<DependencyService>,

    // XAML bindings
    xaml_bindings: Vec<XAMLBinding>,
}
```

## 6. Java/JVM Bytecode

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