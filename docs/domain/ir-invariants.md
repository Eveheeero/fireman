# IR Invariants and Determinism Rules

## Core Invariants

### CRITICAL: Same Assembly → Same IR (Always!)

**The Golden Rule**: Given identical assembly bytes at identical addresses, the IR output must be byte-for-byte
identical, regardless of:

- Machine architecture running the decompiler
- Available memory or CPU cores
- Previous decompilation runs
- Time of day or system load
- Random number generator state
- Hash table implementation details

### 1. Instruction Address Mapping

**Invariant**: Every IR instruction traces back to exactly one source instruction address.

```rust
/// Every IR instruction maintains source mapping
pub struct InstructionMapping {
    /// Source assembly address
    source_addr: Address,
    
    /// Generated IR instructions (in order)
    ir_instructions: Vec<Instruction>,
}

/// Validation
fn validate_mapping(mapping: &[InstructionMapping]) -> Result<()> {
    // No address produces empty IR
    for m in mapping {
        assert!(!m.ir_instructions.is_empty(), 
            "Address {:x} produced no IR", m.source_addr);
    }
    
    // Addresses are monotonic
    let addresses: Vec<_> = mapping.iter().map(|m| m.source_addr).collect();
    assert!(addresses.windows(2).all(|w| w[0] < w[1]),
        "Addresses must be in ascending order");
    
    Ok(())
}
```

### 2. Deterministic Temporary Naming

**Invariant**: Temporary names are fully determined by their creation context.

```rust
/// Temporary naming scheme - MUST be deterministic
pub struct TempNamer {
    /// Counters per (address, purpose) pair
    counters: BTreeMap<(Address, &'static str), u32>,
}

impl TempNamer {
    /// Generate deterministic temporary name
    pub fn new_temp(&mut self, addr: Address, purpose: &'static str) -> LocalId {
        let key = (addr, purpose);
        let counter = self.counters.entry(key).or_insert(0);
        let current = *counter;
        *counter += 1;
        
        // Fixed format: purpose.address.counter
        let name = format!("{}.{:016x}.{}", purpose, addr, current);
        
        LocalId {
            name,
            version: 0,  // Will be set during SSA
            source: addr,
        }
    }
    
    /// CRITICAL: Reset for each function
    pub fn reset(&mut self) {
        self.counters.clear();
    }
}

// Examples (note: fixed-width hex addresses):
// load.0000000000401000.0  - first load temp at 0x401000
// load.0000000000401000.1  - second load temp at 0x401000  
// add.0000000000401003.0   - first add temp at 0x401003
```

### 3. Canonical Operand Ordering

**Invariant**: Commutative operations always order operands consistently.

```rust
/// Operand comparison for canonical ordering
impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Constants come first
            (Value::Constant(_), Value::Constant(_)) => {
                self.const_value().cmp(&other.const_value())
            }
            (Value::Constant(_), _) => Ordering::Less,
            (_, Value::Constant(_)) => Ordering::Greater,
            
            // Then globals
            (Value::Global(a), Value::Global(b)) => a.cmp(b),
            (Value::Global(_), Value::Local(_)) => Ordering::Less,
            (Value::Local(_), Value::Global(_)) => Ordering::Greater,
            
            // Then locals (by source address, then name)
            (Value::Local(a), Value::Local(b)) => {
                a.source.cmp(&b.source)
                    .then_with(|| a.name.cmp(&b.name))
                    .then_with(|| a.version.cmp(&b.version))
            }
            
            _ => panic!("Unexpected value comparison"),
        }
    }
}

/// Apply canonical ordering
fn canonicalize_binop(op: BinaryOp, lhs: Value, rhs: Value) -> (Value, Value) {
    if op.is_commutative() && lhs > rhs {
        (rhs, lhs)
    } else {
        (lhs, rhs)
    }
}
```

### 4. SSA Versioning Rules

**Invariant**: SSA versions are assigned in dominance order.

```rust
/// SSA version assignment
pub struct SSAVersioner {
    /// Next version for each base variable
    versions: BTreeMap<String, u32>,
    
    /// Stack of versions for each variable (for dominance)
    stacks: BTreeMap<String, Vec<u32>>,
}

impl SSAVersioner {
    /// Get new version (deterministic based on traversal order)
    pub fn new_version(&mut self, base_name: &str) -> u32 {
        let version = self.versions.entry(base_name.to_string())
            .and_modify(|v| *v += 1)
            .or_insert(1);
        
        self.stacks.entry(base_name.to_string())
            .or_default()
            .push(*version);
        
        *version
    }
    
    /// Process blocks in reverse post-order
    pub fn version_function(&mut self, func: &mut Function) {
        let rpo = reverse_post_order(&func.cfg);
        
        for block_id in rpo {
            self.version_block(func, block_id);
        }
    }
}
```

### 5. Type Consistency

**Invariant**: Types are explicit and consistent throughout IR.

```rust
/// Type checking invariants
pub struct TypeChecker {
    pub fn check_instruction(&self, inst: &Instruction) -> Result<()> {
        match inst {
            Instruction::BinOp { op, dst, lhs, rhs, ty } => {
                // Operand types must match instruction type
                let lhs_ty = self.value_type(lhs)?;
                let rhs_ty = self.value_type(rhs)?;
                
                assert_eq!(lhs_ty, *ty, "LHS type mismatch");
                assert_eq!(rhs_ty, *ty, "RHS type mismatch");
                
                // Result type matches operand type
                let result_ty = self.binop_result_type(*op, ty)?;
                assert_eq!(self.local_type(dst)?, result_ty);
            }
            
            Instruction::Cast { src_ty, dst_ty, .. } => {
                // Cast must be valid
                assert!(self.valid_cast(src_ty, dst_ty)?,
                    "Invalid cast from {:?} to {:?}", src_ty, dst_ty);
            }
            
            _ => { /* Check other instructions */ }
        }
        
        Ok(())
    }
}
```

## Strict Determinism Rules

### Rule 1: No Implementation-Defined Behavior (EVER!)

```rust
// FATAL BUG: HashMap iteration
let mut ops = HashMap::new();
for (k, v) in ops.iter() {  // THIS IS A CRITICAL BUG!
    process(k, v);
}

// REQUIRED: Always use BTreeMap
let mut ops = BTreeMap::new();
for (k, v) in ops.iter() {  // Deterministic iteration order
    process(k, v);
}

// ALSO BAD: HashSet for any ordered operation
let blocks: HashSet<BlockId> = discover_blocks();
for block in blocks {  // WRONG! Order varies
    analyze(block);
}

// REQUIRED: BTreeSet or sort first
let blocks: BTreeSet<BlockId> = discover_blocks();
for block in blocks {  // Always same order
    analyze(block);
}
```

### Rule 2: Explicit Undefined Values

```rust
/// Handle undefined values explicitly
fn lift_uninitialized_read(addr: Address, ty: Type) -> Instruction {
    let dst = new_temp(addr, ty, "undef");
    
    Instruction::Assign {
        dst,
        value: Value::Constant(Constant::Undef(ty)),
        source: addr,
    }
}
```

### Rule 3: Consistent Block Ordering

```rust
/// CRITICAL: Blocks must always be processed in same order
fn order_blocks(blocks: &BTreeMap<BlockId, BasicBlock>) -> Vec<BlockId> {
    // Already sorted by BTreeMap!
    blocks.keys().copied().collect()
}

/// When discovering blocks dynamically
struct DeterministicBlockDiscovery {
    /// Use BTreeSet for work queue
    work_queue: BTreeSet<Address>,
    seen: BTreeSet<Address>,
}

impl DeterministicBlockDiscovery {
    fn discover(&mut self, entry: Address) -> Vec<BasicBlock> {
        self.work_queue.insert(entry);
        let mut blocks = Vec::new();
        
        // ALWAYS take minimum address first
        while let Some(addr) = self.work_queue.pop_first() {
            if self.seen.contains(&addr) {
                continue;
            }
            self.seen.insert(addr);
            
            let block = self.analyze_block(addr);
            
            // Add successors in deterministic order
            for succ in block.successors() {
                self.work_queue.insert(succ);
            }
            
            blocks.push(block);
        }
        
        // Blocks are naturally in address order
        blocks
    }
}
```

### Rule 4: Normalized Constants

```rust
/// Normalize constant representation
fn normalize_constant(c: Constant) -> Constant {
    match c {
        Constant::Int { value, ty } => {
            // Normalize to smallest signed representation
            let normalized = match ty {
                Type::I8 => (value as i8) as i128,
                Type::I16 => (value as i16) as i128,
                Type::I32 => (value as i32) as i128,
                Type::I64 => (value as i64) as i128,
                _ => value,
            };
            Constant::Int { value: normalized, ty }
        }
        _ => c,
    }
}
```

### Rule 5: Stable Phi Node Ordering

```rust
/// Phi nodes MUST have deterministic predecessor ordering
fn create_phi(block_addr: Address, preds_and_values: Vec<(BlockId, Value)>, ty: Type) -> Instruction {
    // CRITICAL: Sort by block ID to ensure consistent ordering
    let mut sorted = preds_and_values;
    sorted.sort_by_key(|(pred, _)| *pred);
    
    let mut incoming = BTreeMap::new();
    for (pred, val) in sorted {
        incoming.insert(pred, val);
    }
    
    Instruction::Phi {
        dst: new_temp(block_addr, ty, "phi"),
        incoming,  // BTreeMap maintains order
        ty,
    }
}

/// SSA construction must be deterministic
impl SSABuilder {
    fn place_phi_functions(&mut self, cfg: &CFG) -> PhiPlacements {
        let mut placements = BTreeMap::new();
        
        // Process variables in sorted order
        let vars: BTreeSet<_> = self.all_variables.iter().cloned().collect();
        
        for var in vars {
            // Get definition sites in sorted order
            let def_sites: BTreeSet<_> = self.def_sites[&var].iter().cloned().collect();
            
            // Compute dominance frontier deterministically
            let mut work_list = def_sites;
            let mut phi_sites = BTreeSet::new();
            
            while let Some(site) = work_list.pop_first() {
                // Process frontiers in sorted order
                let frontiers: BTreeSet<_> = self.dominance_frontiers[&site]
                    .iter()
                    .cloned()
                    .collect();
                    
                for frontier in frontiers {
                    if phi_sites.insert(frontier) {
                        work_list.insert(frontier);
                    }
                }
            }
            
            placements.insert(var, phi_sites);
        }
        
        PhiPlacements(placements)
    }
}
```

## Testing Determinism

### Comprehensive Determinism Tests

```rust
#[test]
fn test_absolute_determinism() {
    let test_binaries = vec![
        include_bytes!("../testdata/simple.bin"),
        include_bytes!("../testdata/complex.bin"),
        include_bytes!("../testdata/obfuscated.bin"),
    ];
    
    for binary in test_binaries {
        // Generate IR 100 times under different conditions
        let mut ir_hashes = Vec::new();
        
        for i in 0..100 {
            // Pollute memory state differently each time
            let _garbage: Vec<_> = (0..i*1000)
                .map(|x| vec![x as u8; (x * 7) % 1000])
                .collect();
            
            // Create fresh lifter
            let lifter = IRLifter::new();
            
            // Generate IR
            let ir = lifter.lift(binary);
            
            // Serialize to bytes for exact comparison
            let ir_bytes = bincode::serialize(&ir).unwrap();
            
            // Hash the bytes
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(&ir_bytes);
            let hash = hasher.finalize();
            
            ir_hashes.push(hash);
        }
        
        // ALL hashes must be EXACTLY identical
        let first = &ir_hashes[0];
        for (i, hash) in ir_hashes.iter().enumerate() {
            assert_eq!(first, hash, 
                "Run {} produced different IR! This is a CRITICAL BUG!", i);
        }
    }
}

#[test]
fn test_determinism_across_threads() {
    use std::sync::Arc;
    use std::thread;
    
    let binary = Arc::new(include_bytes!("../testdata/multithread.bin").to_vec());
    
    // Lift in parallel from multiple threads
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let bin = binary.clone();
            thread::spawn(move || {
                let lifter = IRLifter::new();
                lifter.lift(&bin)
            })
        })
        .collect();
    
    // Collect results
    let results: Vec<_> = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    
    // All must be identical
    for i in 1..results.len() {
        assert_eq!(results[0], results[i],
            "Thread {} produced different IR!", i);
    }
}
```

### Differential Testing

```rust
#[test]
fn test_cross_platform_determinism() {
    let test_cases = load_test_binaries();
    
    for binary in test_cases {
        // Run on different configurations
        let results = vec![
            lift_with_config(&binary, Config::default()),
            lift_with_config(&binary, Config::minimal()),
            lift_with_config(&binary, Config::maximal()),
        ];
        
        // All must produce same IR
        assert!(results.windows(2).all(|w| w[0] == w[1]));
    }
}
```

### Chaos Testing

```rust
#[test]
fn test_determinism_under_memory_pressure() {
    let binary = load_test_binary();
    
    // Normal conditions
    let baseline = lift_binary(&binary);
    
    // Memory pressure
    let under_pressure = {
        let _bloat: Vec<_> = (0..1_000_000)
            .map(|i| vec![i as u8; 1000])
            .collect();
        lift_binary(&binary)
    };
    
    assert_eq!(baseline, under_pressure);
}
```

## Common Pitfalls That Break Determinism

### 1. Address Representation

```rust
// BUG: Platform-dependent size
let addr = ptr as usize;  // 32 or 64 bits!

// BUG: Formatting without width
format!("{:x}", addr)  // "1000" vs "401000"

// CORRECT: Fixed representation
let addr = Address(ptr as u64);
format!("{:016x}", addr.0)  // Always "0000000000401000"
```

### 2. Floating Point in Analysis

```rust
// BAD: Floating point is non-deterministic
let score = complexity as f64 / size as f64;

// GOOD: Use integer arithmetic
let score = (complexity * 1000) / size;  // Fixed-point
```

### 3. Hidden Non-Determinism

```rust
// BUG: Thread ID in names
let name = format!("temp_{:?}", thread::current().id());

// BUG: Timestamp in output
let comment = format!("Generated at {}", Utc::now());

// BUG: Random for "unique" IDs
let id = Uuid::new_v4();

// BUG: Pointer address in debug output
let debug = format!("{:p}", &some_struct);

// CORRECT: Only use deterministic inputs
let name = format!("temp_{:016x}_{}", inst_addr, counter);
let comment = format!("Generated from {:016x}", source_addr);
let id = DeterministicId::from_address(addr);
let debug = format!("{:#?}", some_struct);  // No addresses
```

### 4. Collection Pitfalls

```rust
// BUG: Vec order depends on insertion order
let mut funcs = Vec::new();
for f in discover_functions() {  // Random order!
    funcs.push(f);
}

// BUG: Collecting from HashMap
let items: Vec<_> = map.into_iter().collect();  // Random order!

// BUG: Using HashSet for work queue
let mut work = HashSet::new();
while let Some(item) = work.iter().next() {  // Random choice!
    process(item);
}

// CORRECT: Always maintain order
let mut funcs = BTreeSet::new();
for f in discover_functions() {
    funcs.insert(f);
}
let funcs: Vec<_> = funcs.into_iter().collect();  // Sorted!

// CORRECT: Sorted iteration
let mut items: Vec<_> = map.into_iter().collect();
items.sort_by_key(|(k, _)| k.clone());

// CORRECT: BTreeSet for deterministic work queue
let mut work = BTreeSet::new();
while let Some(item) = work.pop_first() {  // Always minimum
    process(item);
}
```

## Determinism Verification Checklist

### Data Structures

- [ ] **NO HashMap/HashSet** - Only BTreeMap/BTreeSet
- [ ] **NO Vec for unordered data** - Use BTreeSet
- [ ] **NO FxHashMap/AHashMap** - Still non-deterministic!
- [ ] **IndexMap only with sort** - Insertion order isn't enough

### Algorithms

- [ ] **All iterations sorted** - No "order doesn't matter"
- [ ] **Work queues deterministic** - BTreeSet, not Vec/VecDeque
- [ ] **Fixed processing order** - Entry points, blocks, edges
- [ ] **Stable sorts only** - sort_by_key, not sort_unstable

### Temporaries & Names

- [ ] **Address-based names** - temp_401000_0, not temp_0
- [ ] **Fixed-width formatting** - {:016x}, not {:x}
- [ ] **Counter reset per function** - No global counters
- [ ] **No pointer-based names** - No format!("{:p}", ptr)

### Constants & Values

- [ ] **Canonical integers** - Sign-extend consistently
- [ ] **No floating point** - Not even for "scores"
- [ ] **Normalized addresses** - 64-bit zero-extended
- [ ] **Fixed enum discriminants** - Not #[repr(C)]

### Control Flow

- [ ] **Sorted successor order** - Even for CFG edges
- [ ] **Deterministic work queue** - Pop minimum, not any
- [ ] **Fixed phi operand order** - Sort by predecessor
- [ ] **Stable block numbering** - Based on address

### Testing

- [ ] **Hash-based comparison** - Serialize and SHA256
- [ ] **1000x repetition test** - Same binary, same IR
- [ ] **Cross-platform test** - Linux/Windows/Mac identical
- [ ] **Parallel execution test** - Thread count doesn't matter
- [ ] **Memory pressure test** - Low memory = same IR

### Common Fixes

```rust
// Before (BUG)
let mut map = HashMap::new();

// After (FIXED)  
let mut map = BTreeMap::new();

// Before (BUG)
for func in functions {  // Random order

// After (FIXED)
for func in functions.iter().sorted_by_key(|f| f.address) {

// Before (BUG)
format!("t{}", counter++)

// After (FIXED)
format!("t_{:016x}_{}", inst_addr, local_counter++)
```

Remember: **Determinism is not optional - it's a CORE REQUIREMENT!**