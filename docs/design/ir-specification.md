# Fireman Multi-Level IR Specification

## Design Goals

1. **ABSOLUTELY DETERMINISTIC**: Same assembly bytes at same addresses MUST produce byte-for-byte identical IR at all
   levels
2. **Type-Safe**: Strongly typed with explicit casts
3. **SSA-Based**: Static Single Assignment for analysis
4. **Architecture-Neutral**: Abstract away ISA details
5. **Analysis-Friendly**: Easy to pattern match and transform
6. **Progressive Refinement**: Three-level IR (Low → Medium → High) with increasing semantic understanding
7. **ML-Enhanced**: Integrates confidence scores and ML suggestions without breaking determinism

### Determinism is Non-Negotiable

**Critical Requirement**: The IR generation process must be 100% deterministic. This means:

- Same input bytes → Same IR output (byte-for-byte)
- No dependence on: system time, memory state, thread scheduling, hash functions
- All operations must have defined, stable ordering
- Testing must verify exact binary equality of IR output

## Multi-Level IR Architecture

### Overview

The Fireman decompiler uses a progressive three-level IR design that gradually transforms low-level assembly into
high-level constructs while maintaining absolute determinism at every stage.

### Low IR (Direct Translation Layer)

**Purpose**: Preserve all assembly semantics with 100% fidelity

**Characteristics**:

- One-to-one mapping with assembly instructions
- All CPU flags and side effects explicitly represented
- No simplification or optimization
- Confidence: Always 100% (direct translation)

**Example**:

```rust
// x86_64: mov rax, [rbp-8]
%addr.401000.0 = sub i64 %rbp.401000.0, 8
%load.401000.0 = load i64* %addr.401000.0
%rax.401003.0 = copy %load.401000.0
%flags.401003.0 = flags_unchanged  // mov doesn't affect flags
```

### Medium IR (Pattern Recognition Layer)

**Purpose**: Identify patterns and abstract common idioms

**Characteristics**:

- Pattern-based abstractions (loops, function calls, switches)
- Basic type inference and propagation
- Dead code and unreachable path identification
- Confidence tracking per pattern (0.0 - 1.0)
- ML Enhancement: XGBoost models for pattern detection

**Pattern Database Integration**:

```rust
struct PatternMatch {
    pattern_id: PatternId,        // Deterministic pattern ID
    confidence: f32,              // Pattern match confidence
    library: Option<String>,      // e.g., "glibc", "msvcrt"
    function: Option<String>,     // e.g., "malloc", "printf"
    source_hash: [u8; 32],       // For cache validation
}
```

**Example**:

```rust
// Recognized as function prologue
%prologue.401000 = pattern:function_entry {
    saved_regs: [rbp, rbx],
    frame_size: 32,
    confidence: 0.95
}

// Recognized as loop header
%loop.401100 = pattern:counted_loop {
    counter: %rcx.401100.0,
    limit: %rax.401100.0,
    confidence: 0.87
}
```

### High IR (Source-Like Representation)

**Purpose**: Generate near-source code representation

**Characteristics**:

- High-level constructs (if/while/for/switch/struct)
- Recovered types with confidence scores
- ML-suggested variable names (optional, cached)
- Source-like expressions and idioms
- Function signatures with calling conventions

**ML Enhancement Points**:

- **CodeLlama 7B**: Variable naming, comment generation (optional)
- **Cloud LLMs**: Complex pattern analysis (on-demand, fully cached)
- **Pattern DB**: Pre-analyzed library function signatures

**Example**:

```rust
// High-level struct recovery
struct Point {              // confidence: 0.92
    int32_t x;             // offset 0, confidence: 0.95
    int32_t y;             // offset 4, confidence: 0.95
}

// Function with recovered signature
int32_t calculate_distance(Point* p1, Point* p2) {  // confidence: 0.88
    int32_t dx = p1->x - p2->x;  // ML suggested: "dx" (confidence: 0.75)
    int32_t dy = p1->y - p2->y;  // ML suggested: "dy" (confidence: 0.75)
    return dx * dx + dy * dy;
}
```

### IR Level Transitions

```rust
/// Low → Medium IR transformation
pub struct LowToMediumTransform {
    pattern_db: PatternDatabase,
    ml_models: Option<MLModels>,
}

impl LowToMediumTransform {
    pub fn transform(&mut self, low_ir: &LowIRModule) -> MediumIRModule {
        let mut medium = MediumIRModule::new();
        
        // Phase 1: Pattern detection (deterministic)
        for (addr, func) in &low_ir.functions {
            let patterns = self.detect_patterns(func);
            medium.add_function(addr, self.apply_patterns(func, patterns));
        }
        
        // Phase 2: ML enhancement (optional, cached)
        if let Some(ml) = &self.ml_models {
            medium = ml.enhance_medium_ir(medium);
        }
        
        medium
    }
}

/// Medium → High IR transformation
pub struct MediumToHighTransform {
    type_recovery: TypeRecoveryEngine,
    ml_naming: Option<MLNamingEngine>,
}

impl MediumToHighTransform {
    pub fn transform(&mut self, medium_ir: &MediumIRModule) -> HighIRModule {
        let mut high = HighIRModule::new();
        
        // Phase 1: Structure recovery
        let structures = self.type_recovery.recover_types(medium_ir);
        
        // Phase 2: Control flow recovery
        for (addr, func) in &medium_ir.functions {
            let high_func = self.recover_control_flow(func, &structures);
            high.add_function(addr, high_func);
        }
        
        // Phase 3: ML naming (optional, cached)
        if let Some(naming) = &self.ml_naming {
            high = naming.suggest_names(high);
        }
        
        high
    }
}
```

### ML Integration Layer

All ML operations maintain determinism through:

1. **Fixed Seeds**: All models use deterministic initialization
2. **Content Hashing**: Cache keys based on input content hash
3. **Fallback Logic**: Graceful degradation when ML unavailable
4. **Version Tracking**: Model versions in output metadata

```rust
pub struct MLEnhancement {
    // Local models (always available)
    xgboost: XGBoostModels,
    
    // Optional models (cached)
    llm_cache: BTreeMap<ContentHash, LLMResult>,
    
    // Pattern database
    pattern_db: PatternDatabase,
}

impl MLEnhancement {
    /// Deterministic enhancement with caching
    pub fn enhance(&mut self, ir: IRLevel, content: &[u8]) -> EnhancedIR {
        let content_hash = self.hash_content(content);
        
        // Check cache first
        if let Some(cached) = self.llm_cache.get(&content_hash) {
            return cached.clone();
        }
        
        // Run enhancement
        let enhanced = match ir {
            IRLevel::Low => self.enhance_low(content),
            IRLevel::Medium => self.enhance_medium(content),
            IRLevel::High => self.enhance_high(content),
        };
        
        // Cache result
        self.llm_cache.insert(content_hash, enhanced.clone());
        enhanced
    }
}
```

### Confidence Tracking

Every transformation tracks confidence scores:

```rust
pub struct ConfidenceInfo {
    /// Overall confidence (0.0 - 1.0)
    pub overall: f32,
    
    /// Per-component confidence
    pub components: BTreeMap<String, f32>,
    
    /// Source of confidence assessment
    pub source: ConfidenceSource,
}

pub enum ConfidenceSource {
    /// Direct translation (always 1.0)
    Direct,
    
    /// Pattern matching with score
    Pattern { pattern_id: PatternId, score: f32 },
    
    /// ML model prediction
    MLModel { model: String, version: String, score: f32 },
    
    /// Heuristic analysis
    Heuristic { rule: String, score: f32 },
}
```

## IR Structure

### Core Types

```rust
/// IR types with explicit sizes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    /// Void type
    Void,
    
    /// Boolean (1 bit)
    Bool,
    
    /// Integers with explicit bit width
    I1, I8, I16, I32, I64, I128,
    
    /// Floating point
    F32, F64, F80,
    
    /// Pointer with optional pointee type
    Pointer(Option<Box<Type>>),
    
    /// Fixed-size array
    Array(Box<Type>, usize),
    
    /// Structure (ordered fields)
    Struct(Vec<Type>),
    
    /// Function type
    Function {
        ret: Box<Type>,
        params: Vec<Type>,
        varargs: bool,
    },
    
    /// Unknown type (for initial lifting)
    Unknown,
}

impl Type {
    /// Get size in bytes
    pub const fn size(&self) -> Option<usize> {
        match self {
            Type::Void => Some(0),
            Type::Bool | Type::I1 => Some(1),
            Type::I8 => Some(1),
            Type::I16 => Some(2),
            Type::I32 | Type::F32 => Some(4),
            Type::I64 | Type::F64 | Type::Pointer(_) => Some(8),
            Type::I128 => Some(16),
            Type::F80 => Some(10),
            Type::Array(elem, count) => elem.size().map(|s| s * count),
            Type::Struct(fields) => {
                let mut size = 0;
                for field in fields {
                    size += field.size()?;
                }
                Some(size)
            }
            _ => None,
        }
    }
}
```

### Values

```rust
/// IR values (operands) - with deterministic ordering
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    /// Constant values (always ordered first)
    Constant(Constant),
    
    /// Global variable
    Global(GlobalId),
    
    /// Local SSA variable  
    Local(LocalId),
    
    /// Function reference
    Function(FunctionId),
    
    /// Basic block label
    Label(BlockId),
}

/// CRITICAL: Deterministic ordering for canonical forms
impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        use Value::*;
        match (self, other) {
            // Constants always come first
            (Constant(a), Constant(b)) => a.cmp(b),
            (Constant(_), _) => Ordering::Less,
            (_, Constant(_)) => Ordering::Greater,
            
            // Then globals (by ID)
            (Global(a), Global(b)) => a.cmp(b),
            (Global(_), _) => Ordering::Less,
            (_, Global(_)) => Ordering::Greater,
            
            // Then functions (by address)
            (Function(a), Function(b)) => a.cmp(b),
            (Function(_), _) => Ordering::Less,
            (_, Function(_)) => Ordering::Greater,
            
            // Then locals (by source addr, name, version)
            (Local(a), Local(b)) => a.cmp(b),
            (Local(_), Label(_)) => Ordering::Less,
            (Label(_), Local(_)) => Ordering::Greater,
            
            // Finally labels (by block address)
            (Label(a), Label(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Constant {
    /// Integer constant with type
    Int { value: i128, ty: Type },
    
    /// Floating point constant
    Float { bits: u64, ty: Type },
    
    /// Null pointer
    Null(Type),
    
    /// Undefined value
    Undef(Type),
    
    /// Aggregate constant
    Aggregate(Vec<Constant>),
}

/// Local SSA variable - deterministic naming
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalId {
    /// Source instruction address (primary key)
    pub source: Address,
    
    /// Purpose/type (e.g., "load", "addr", "result")
    pub purpose: &'static str,
    
    /// Index for same purpose at same address
    pub index: u32,
    
    /// SSA version (assigned during SSA construction)
    pub version: u32,
}

/// Deterministic ordering: by address, then purpose, then index
impl Ord for LocalId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source.cmp(&other.source)
            .then_with(|| self.purpose.cmp(&other.purpose))
            .then_with(|| self.index.cmp(&other.index))
            .then_with(|| self.version.cmp(&other.version))
    }
}

impl PartialOrd for LocalId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for LocalId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Deterministic format: %purpose_address_index.version
        if self.version == 0 {
            write!(f, "%{}_{:016x}_{}", self.purpose, self.source, self.index)
        } else {
            write!(f, "%{}_{:016x}_{}.{}", self.purpose, self.source, self.index, self.version)
        }
    }
}

/// Helper for creating temporaries deterministically
pub struct TempAllocator {
    counters: BTreeMap<(Address, &'static str), u32>,
}

impl TempAllocator {
    pub fn new() -> Self {
        Self { counters: BTreeMap::new() }
    }
    
    pub fn new_temp(&mut self, addr: Address, purpose: &'static str) -> LocalId {
        let key = (addr, purpose);
        let index = self.counters.entry(key).or_insert(0);
        let current = *index;
        *index += 1;
        
        LocalId {
            source: addr,
            purpose,
            index: current,
            version: 0,  // Will be set during SSA
        }
    }
    
    /// MUST reset between functions
    pub fn reset(&mut self) {
        self.counters.clear();
    }
}
```

### Instructions

```rust
/// IR instructions - deterministic and canonical
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    /// Binary operation: %dst = op %lhs, %rhs
    BinOp {
        op: BinaryOp,
        dst: LocalId,
        lhs: Value,
        rhs: Value,
        ty: Type,
    },
    
    /// Unary operation: %dst = op %src
    UnOp {
        op: UnaryOp,
        dst: LocalId,
        src: Value,
        ty: Type,
    },
    
    /// Memory load: %dst = load ty* %ptr
    Load {
        dst: LocalId,
        ptr: Value,
        ty: Type,
        align: Option<u32>,
        volatile: bool,
    },
    
    /// Memory store: store ty %val, ty* %ptr
    Store {
        val: Value,
        ptr: Value,
        ty: Type,
        align: Option<u32>,
        volatile: bool,
    },
    
    /// Type cast: %dst = cast op %src to ty
    Cast {
        op: CastOp,
        dst: LocalId,
        src: Value,
        src_ty: Type,
        dst_ty: Type,
    },
    
    /// Function call: %dst = call fn(%args...)
    Call {
        dst: Option<LocalId>,
        func: Value,
        args: Vec<(Value, Type)>,
        conv: CallConv,
    },
    
    /// PHI node: %dst = phi [%val1, %bb1], [%val2, %bb2], ...
    Phi {
        dst: LocalId,
        incoming: BTreeMap<BlockId, Value>,
        ty: Type,
    },
    
    /// Select: %dst = select %cond, %true_val, %false_val
    Select {
        dst: LocalId,
        cond: Value,
        true_val: Value,
        false_val: Value,
        ty: Type,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinaryOp {
    // Arithmetic (canonical order for commutative ops)
    Add, Sub, Mul, SDiv, UDiv, SRem, URem,
    
    // Bitwise (canonical order)
    And, Or, Xor, Shl, LShr, AShr,
    
    // Comparison (not commutative)
    Eq, Ne, Slt, Sle, Sgt, Sge, Ult, Ule, Ugt, Uge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnaryOp {
    Neg,    // Arithmetic negation
    Not,    // Bitwise NOT
    FNeg,   // Floating-point negation
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CastOp {
    // Integer casts
    Trunc,    // Truncate to smaller integer
    ZExt,     // Zero extend
    SExt,     // Sign extend
    
    // Pointer casts
    PtrToInt,
    IntToPtr,
    
    // Float casts
    FPTrunc,
    FPExt,
    FPToUI,
    FPToSI,
    UIToFP,
    SIToFP,
    
    // Bitcast (no value change)
    Bitcast,
}
```

### Terminators

```rust
/// Block terminators - control flow
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Terminator {
    /// Return: ret ty %val
    Return(Option<(Value, Type)>),
    
    /// Unconditional branch: br label %dest
    Branch(BlockId),
    
    /// Conditional branch: br %cond, label %true, label %false
    CondBranch {
        cond: Value,
        true_dest: BlockId,
        false_dest: BlockId,
    },
    
    /// Switch: switch %val, label %default [val1, label1], ...
    Switch {
        value: Value,
        default: BlockId,
        cases: BTreeMap<Constant, BlockId>,
    },
    
    /// Indirect branch: indirectbr %addr, [%bb1, %bb2, ...]
    IndirectBranch {
        addr: Value,
        destinations: BTreeSet<BlockId>,
    },
    
    /// Unreachable
    Unreachable,
}
```

### Basic Blocks and Functions

```rust
/// Basic block with deterministic ordering
#[derive(Debug, Clone)]
pub struct BasicBlock {
    /// Block identifier (address-based)
    pub id: BlockId,
    
    /// PHI nodes (MUST be sorted by destination variable)
    pub phis: Vec<Instruction>,
    
    /// Regular instructions (in address order)
    pub instructions: Vec<Instruction>,
    
    /// Block terminator
    pub terminator: Terminator,
}

impl BasicBlock {
    /// Ensure deterministic PHI ordering
    pub fn sort_phis(&mut self) {
        self.phis.sort_by_key(|phi| {
            match phi {
                Instruction::Phi { dst, .. } => dst.clone(),
                _ => panic!("Non-PHI in PHI list"),
            }
        });
    }
    
    /// Verify block is in canonical form
    pub fn verify_determinism(&self) -> Result<(), String> {
        // Check PHIs are sorted
        for window in self.phis.windows(2) {
            if let (Instruction::Phi { dst: dst1, .. }, Instruction::Phi { dst: dst2, .. }) = (&window[0], &window[1]) {
                if dst1 >= dst2 {
                    return Err(format!("PHIs not sorted: {:?} >= {:?}", dst1, dst2));
                }
            }
        }
        
        // Check instructions are in address order
        let mut last_addr = Address(0);
        for inst in &self.instructions {
            let addr = inst.source_address();
            if addr < last_addr {
                return Err(format!("Instructions not in address order"));
            }
            last_addr = addr;
        }
        
        Ok(())
    }
}

/// Function representation
#[derive(Debug, Clone)]
pub struct Function {
    /// Function name/address
    pub id: FunctionId,
    
    /// Function signature
    pub signature: FunctionType,
    
    /// Entry block
    pub entry: BlockId,
    
    /// All blocks (ordered by address)
    pub blocks: BTreeMap<BlockId, BasicBlock>,
    
    /// Local variable types
    pub locals: BTreeMap<LocalId, Type>,
}

/// Module - collection of functions
#[derive(Debug, Clone)]
pub struct Module {
    /// Target architecture
    pub target: TargetInfo,
    
    /// Global variables (ordered)
    pub globals: BTreeMap<GlobalId, Global>,
    
    /// Functions (ordered by address)
    pub functions: BTreeMap<FunctionId, Function>,
    
    /// External functions
    pub externals: BTreeMap<String, FunctionType>,
}
```

## Lifting Rules - Deterministic Implementation

### x86_64 to IR

```rust
/// CRITICAL: Every lift operation must be deterministic
pub struct DeterministicX86Lifter {
    temp_alloc: TempAllocator,
    
    /// Fixed patterns for each instruction
    patterns: BTreeMap<OpcodeClass, LiftPattern>,
}

impl DeterministicX86Lifter {
    /// MOV instruction - fully deterministic
    fn lift_mov(&mut self, inst: &X86Inst, addr: Address) -> Vec<Instruction> {
        let mut result = Vec::new();
        
        match (&inst.operands[0], &inst.operands[1]) {
            // Pattern: MOV reg, reg
            (Operand::Reg(dst), Operand::Reg(src)) => {
                result.push(Instruction::Assign {
                    dst: self.reg_to_local(dst, addr),
                    value: Value::Local(self.reg_to_local(src, addr)),
                    source_addr: addr,
                });
            }
            
            // Pattern: MOV reg, imm
            (Operand::Reg(dst), Operand::Imm(imm)) => {
                result.push(Instruction::Assign {
                    dst: self.reg_to_local(dst, addr),
                    value: Value::Constant(self.normalize_immediate(*imm, inst.size)),
                    source_addr: addr,
                });
            }
            
            // Pattern: MOV reg, [mem]
            (Operand::Reg(dst), Operand::Mem(mem)) => {
                // Step 1: Calculate address (deterministic)
                let addr_temp = self.temp_alloc.new_temp(addr, "addr");
                result.extend(self.calc_mem_address(mem, addr_temp.clone(), addr));
                
                // Step 2: Load from memory
                let load_temp = self.temp_alloc.new_temp(addr, "load");
                result.push(Instruction::Load {
                    dst: load_temp.clone(),
                    ptr: Value::Local(addr_temp),
                    ty: self.size_to_type(inst.size),
                    align: None,
                    volatile: false,
                });
                
                // Step 3: Move to register
                result.push(Instruction::Assign {
                    dst: self.reg_to_local(dst, addr),
                    value: Value::Local(load_temp),
                    source_addr: addr,
                });
            }
            
            // Pattern: MOV [mem], reg
            (Operand::Mem(mem), Operand::Reg(src)) => {
                // Step 1: Calculate address
                let addr_temp = self.temp_alloc.new_temp(addr, "addr");
                result.extend(self.calc_mem_address(mem, addr_temp.clone(), addr));
                
                // Step 2: Store to memory
                result.push(Instruction::Store {
                    val: Value::Local(self.reg_to_local(src, addr)),
                    ptr: Value::Local(addr_temp),
                    ty: self.size_to_type(inst.size),
                    align: None,
                    volatile: false,
                });
            }
            
            _ => panic!("Unhandled MOV at {:016x}: {:?}", addr, inst),
        }
        
        result
    }
    
    /// ADD instruction (canonicalized)
    fn lift_add(&mut self, dst: &Operand, src: &Operand, addr: Address) -> Vec<Instruction> {
        let dst_ty = self.operand_type(dst);
        let (dst_val, mut pre1) = self.operand_to_value(dst, addr);
        let (src_val, mut pre2) = self.operand_to_value(src, addr);
        
        let result = self.new_temp(addr, dst_ty, "add_result");
        
        let mut insts = vec![];
        insts.extend(pre1);
        insts.extend(pre2);
        
        // Canonical form: smaller operand first for Add
        let (lhs, rhs) = self.canonicalize_operands(dst_val, src_val);
        
        insts.push(Instruction::BinOp {
            op: BinaryOp::Add,
            dst: result.clone(),
            lhs,
            rhs,
            ty: dst_ty,
        });
        
        // Store result back
        let (dst_loc, mut post) = self.location_to_lvalue(dst, addr);
        insts.extend(post);
        insts.push(self.create_store(dst_loc, Value::Local(result), addr));
        
        // Update flags deterministically
        insts.extend(self.update_flags_add(dst_val, src_val, addr));
        
        insts
    }
}
```

## Canonicalization

### Canonical Forms

1. **Commutative operations**: Smaller operand first (by Value ordering)
2. **Constant folding**: Always fold compile-time constants
3. **Type consistency**: Explicit casts for all type changes
4. **SSA form**: Each variable assigned exactly once
5. **Block ordering**: By entry address
6. **PHI ordering**: By predecessor block ID

### Example Canonicalization

```rust
// Input (non-canonical):
%t1 = add i32 %x, 5
%t2 = add i32 10, %t1

// Output (canonical):
%t1 = add i32 5, %x      // constant first
%t2 = add i32 10, %t1    // already canonical
```

## Validation and Determinism Verification

```rust
pub struct IRValidator {
    pub fn validate_module(&self, module: &Module) -> Result<(), ValidationError> {
        // Check determinism first
        self.validate_determinism(module)?;
        
        // Check all functions
        for (id, func) in &module.functions {
            self.validate_function(func)?;
        }
        
        // Check all references resolve
        self.validate_references(module)?;
        
        Ok(())
    }
    
    fn validate_determinism(&self, module: &Module) -> Result<(), ValidationError> {
        // Verify functions are in address order
        let addresses: Vec<_> = module.functions.keys().map(|f| f.address).collect();
        for window in addresses.windows(2) {
            if window[0] >= window[1] {
                return Err(ValidationError::NonDeterministicOrder(
                    "Functions not in address order".into()
                ));
            }
        }
        
        // Verify each function
        for (_, func) in &module.functions {
            // Blocks in address order
            let block_addrs: Vec<_> = func.blocks.keys().map(|b| b.0).collect();
            for window in block_addrs.windows(2) {
                if window[0] >= window[1] {
                    return Err(ValidationError::NonDeterministicOrder(
                        format!("Blocks not in order in function {:?}", func.id)
                    ));
                }
            }
            
            // Verify each block
            for (_, block) in &func.blocks {
                block.verify_determinism()
                    .map_err(|e| ValidationError::NonDeterministicBlock(e))?;
            }
        }
        
        Ok(())
    }
    
    fn validate_function(&self, func: &Function) -> Result<(), ValidationError> {
        // Entry block exists
        if !func.blocks.contains_key(&func.entry) {
            return Err(ValidationError::MissingEntryBlock);
        }
        
        // All blocks reachable
        let reachable = self.compute_reachable_blocks(func);
        if reachable.len() != func.blocks.len() {
            return Err(ValidationError::UnreachableBlocks);
        }
        
        // SSA property
        self.validate_ssa(func)?;
        
        // Type consistency
        self.validate_types(func)?;
        
        Ok(())
    }
}
```

## Usage Example

```rust
// Lifting assembly to IR
let mut module = Module::new(TargetInfo::x86_64());
let mut lifter = X86Lifter::new();

for function in binary.functions() {
    let ir_func = lifter.lift_function(function);
    module.functions.insert(ir_func.id, ir_func);
}

// Validate IR
let validator = IRValidator::new();
validator.validate_module(&module)?;

// Optimize IR (preserves determinism)
let optimizer = IROptimizer::new();
let optimized = optimizer.optimize(module);

// Generate C code
let generator = CCodeGenerator::new();
let c_code = generator.generate(&optimized);
```

## Absolute Determinism Guarantees

### Core Requirements

1. **NO HashMaps/HashSets**: Only BTreeMap, BTreeSet everywhere
2. **Address-based everything**: All IDs, names, ordering based on instruction addresses
3. **Canonical operand ordering**: Commutative ops always order operands the same way
4. **Deterministic temp names**: Format: `purpose_address_index`
5. **Sorted EVERYTHING**: Every iteration, every collection, every operation
6. **No floating-point**: Not even for "scores" or "metrics"
7. **No system dependencies**: No time, thread IDs, random numbers, pointer addresses

### Verification

```rust
/// Test that IR generation is deterministic
#[test]
fn test_ir_determinism() {
    let binary = include_bytes!("../tests/sample.bin");
    
    // Generate 1000 times
    let results: Vec<_> = (0..1000).map(|i| {
        // Different memory pressure each time
        let _mem: Vec<_> = (0..i*1000).map(|x| vec![x as u8; x % 1000]).collect();
        
        // Fresh lifter each time
        let mut lifter = create_lifter();
        let ir = lifter.lift(binary);
        
        // Serialize for byte comparison
        bincode::serialize(&ir).unwrap()
    }).collect();
    
    // All must be IDENTICAL
    let first = &results[0];
    for (i, result) in results.iter().enumerate() {
        assert_eq!(first.len(), result.len(), "Different sizes at iteration {}", i);
        assert_eq!(first, result, "Different IR at iteration {}", i);
    }
}
```

### Common Violations

```rust
// WRONG: HashMap iteration
for (k, v) in hashmap { }  // Order varies!

// RIGHT: BTreeMap or sort
for (k, v) in btreemap { }  // Always same order

// WRONG: Counter without context
static COUNTER: AtomicU32 = AtomicU32::new(0);
let id = COUNTER.fetch_add(1);  // Non-deterministic!

// RIGHT: Address-based counter
let id = format!("{}_{:016x}_{}", purpose, addr, local_counter);

// WRONG: Time or random
let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

// RIGHT: Deterministic seed
let seed = hash_of_input_bytes;
```

Remember: **If the same assembly produces different IR, it's a CRITICAL BUG!**