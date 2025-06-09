# Comprehensive Deterministic Architecture for Fireman Decompiler

## Core Principle: Absolute Determinism

**The Golden Rule**: Given identical assembly bytes at identical addresses, the decompiler MUST produce byte-for-byte
identical output, regardless of:

- Machine architecture running the decompiler
- Available memory or CPU cores
- Previous decompilation runs
- Time of day or system load
- Thread scheduling
- Hash function implementations

## 1. Foundation: Deterministic Data Structures

### Banned Types (NEVER USE)

```rust
// ❌ NEVER use these types
HashMap<K, V>      // Non-deterministic iteration
HashSet<T>         // Non-deterministic iteration  
FxHashMap<K, V>    // Still non-deterministic
AHashMap<K, V>     // Still non-deterministic
IndexMap<K, V>     // Without explicit sorting

// ❌ NEVER use for ordering
Vec<T>             // When order matters
VecDeque<T>        // For work queues
LinkedList<T>      // Ever
```

### Required Types (ALWAYS USE)

```rust
// ✅ ALWAYS use these types
BTreeMap<K, V>     // Deterministic iteration
BTreeSet<T>        // Deterministic iteration
Vec<T>             // Only for inherently ordered data
&[T]               // For immutable sequences

// ✅ For specialized cases
SmallVec<[T; N]>   // With deterministic operations
ArrayVec<[T; N]>   // With deterministic operations
```

## 2. Address-Based Design

### Address Representation

```rust
/// Fixed 64-bit addresses for all architectures
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(pub u64);

impl Display for Address {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // ALWAYS use 16-digit hex with leading zeros
        write!(f, "{:016x}", self.0)
    }
}

// ❌ NEVER
let addr = ptr as usize;           // Platform-dependent
format!("{:x}", addr);             // Variable width

// ✅ ALWAYS  
let addr = Address(ptr as u64);   // Fixed size
format!("{:016x}", addr.0);        // Fixed width
```

### Deterministic Temporary Naming

```rust
pub struct DeterministicNamer {
    /// Counters per (address, purpose) pair - using BTreeMap
    counters: BTreeMap<(Address, &'static str), u32>,
}

impl DeterministicNamer {
    pub fn new_temp(&mut self, addr: Address, purpose: &'static str) -> String {
        let counter = self.counters.entry((addr, purpose)).or_insert(0);
        let current = *counter;
        *counter += 1;
        
        // Fixed format: purpose.address.counter
        format!("{}.{:016x}.{}", purpose, addr.0, current)
    }
    
    /// CRITICAL: Reset for each function
    pub fn reset(&mut self) {
        self.counters.clear();
    }
}

// Examples:
// load.0000000000401000.0
// add.0000000000401003.0  
// phi.0000000000401010.0
```

## 3. IR Generation Rules

### Canonical Operand Ordering

```rust
impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // 1. Constants come first (sorted by value)
            (Value::Constant(a), Value::Constant(b)) => a.cmp(b),
            (Value::Constant(_), _) => Ordering::Less,
            (_, Value::Constant(_)) => Ordering::Greater,
            
            // 2. Then globals (sorted by address)
            (Value::Global(a), Value::Global(b)) => a.cmp(b),
            (Value::Global(_), _) => Ordering::Less,
            (_, Value::Global(_)) => Ordering::Greater,
            
            // 3. Then locals (sorted by source, name, version)
            (Value::Local(a), Value::Local(b)) => {
                a.source_addr.cmp(&b.source_addr)
                    .then_with(|| a.name.cmp(&b.name))
                    .then_with(|| a.version.cmp(&b.version))
            }
            
            _ => unreachable!("All value types must be comparable"),
        }
    }
}

fn canonicalize_commutative(op: BinaryOp, lhs: Value, rhs: Value) -> (Value, Value) {
    if op.is_commutative() && lhs > rhs {
        (rhs, lhs)  // Swap to maintain canonical order
    } else {
        (lhs, rhs)
    }
}
```

### Block Processing Order

```rust
pub struct DeterministicCFGBuilder {
    /// Use BTreeSet for deterministic discovery
    work_queue: BTreeSet<Address>,
    discovered: BTreeSet<Address>,
    blocks: BTreeMap<Address, BasicBlock>,
}

impl DeterministicCFGBuilder {
    pub fn build(&mut self, entry: Address) -> ControlFlowGraph {
        self.work_queue.insert(entry);
        
        // ALWAYS process minimum address first
        while let Some(addr) = self.work_queue.pop_first() {
            if self.discovered.contains(&addr) {
                continue;
            }
            self.discovered.insert(addr);
            
            let block = self.analyze_block(addr);
            
            // Add successors in sorted order
            for succ in block.successors().iter().sorted() {
                self.work_queue.insert(*succ);
            }
            
            self.blocks.insert(addr, block);
        }
        
        ControlFlowGraph { 
            blocks: self.blocks,  // Already sorted by address
            entry,
        }
    }
}
```

### SSA Construction

```rust
pub struct DeterministicSSA {
    /// Version counters per base variable
    versions: BTreeMap<String, u32>,
    
    /// Dominance frontier computation
    dominance: DominanceInfo,
}

impl DeterministicSSA {
    pub fn construct(&mut self, cfg: &ControlFlowGraph) -> SSAForm {
        // 1. Compute dominance in deterministic order
        let rpo = self.reverse_post_order(cfg);
        
        // 2. Place phi functions deterministically
        let phi_locations = self.compute_phi_locations(&rpo);
        
        // 3. Rename variables in RPO
        for block_addr in &rpo {
            self.rename_block(cfg, *block_addr);
        }
        
        SSAForm { /* ... */ }
    }
    
    fn compute_phi_locations(&self, rpo: &[Address]) -> BTreeMap<Address, BTreeSet<Variable>> {
        let mut result = BTreeMap::new();
        
        // Process variables in sorted order
        let all_vars: BTreeSet<_> = self.collect_all_variables();
        
        for var in all_vars {
            let mut work_list: BTreeSet<_> = self.def_sites(&var);
            let mut phi_sites = BTreeSet::new();
            
            while let Some(site) = work_list.pop_first() {
                // Process dominance frontier in sorted order
                let df: BTreeSet<_> = self.dominance.frontier(site);
                
                for frontier_block in df {
                    if phi_sites.insert(frontier_block) {
                        work_list.insert(frontier_block);
                    }
                }
            }
            
            result.insert(var.source_addr, phi_sites);
        }
        
        result
    }
}
```

## 4. Multi-Level IR Design

### Three-Level IR Architecture

```rust
pub enum IRLevel {
    /// Direct instruction translation
    Low {
        instructions: Vec<LowIR>,
        metadata: InstructionMetadata,
    },
    
    /// Basic optimizations applied
    Medium {
        statements: Vec<MediumIR>,
        patterns: Vec<RecognizedPattern>,
        confidence: ConfidenceMap,
    },
    
    /// Near-source representation
    High {
        ast: Vec<HighIR>,
        types: TypeEnvironment,
        structures: RecoveredStructures,
    },
}

/// Each level maintains deterministic ordering
impl IRLevel {
    pub fn validate_determinism(&self) -> Result<(), DeterminismError> {
        match self {
            IRLevel::Low { instructions, .. } => {
                // Verify address ordering
                let addresses: Vec<_> = instructions.iter()
                    .map(|i| i.address)
                    .collect();
                
                if !addresses.windows(2).all(|w| w[0] <= w[1]) {
                    return Err(DeterminismError::UnorderedInstructions);
                }
            }
            // Similar validation for other levels
            _ => { /* ... */ }
        }
        Ok(())
    }
}
```

## 5. Zero-Copy Architecture

### Memory-Mapped Binary Access

```rust
pub struct DeterministicBinaryView {
    /// Memory-mapped file for zero-copy access
    mmap: Mmap,
    
    /// Sections in deterministic order
    sections: BTreeMap<Address, SectionView>,
    
    /// Metadata computed once
    metadata: BinaryMetadata,
}

impl DeterministicBinaryView {
    pub fn new(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        
        // Parse sections in deterministic order
        let sections = Self::parse_sections(&mmap)?;
        
        Ok(Self {
            mmap,
            sections,
            metadata: Self::compute_metadata(&sections),
        })
    }
    
    /// Zero-copy instruction iteration
    pub fn instructions_at(&self, addr: Address) -> InstructionIter<'_> {
        InstructionIter {
            data: self.section_data_at(addr),
            current: addr,
        }
    }
}
```

### Arena-Based IR Allocation

```rust
pub struct IRArena {
    /// Per-function allocation
    nodes: TypedArena<IRNode>,
    
    /// Bulk deallocation on drop
    _marker: PhantomData<IRNode>,
}

pub struct FunctionLifter<'arena> {
    arena: &'arena IRArena,
    namer: DeterministicNamer,
}

impl<'arena> FunctionLifter<'arena> {
    pub fn lift_function(&mut self, func: &Function) -> &'arena [IRNode] {
        // Reset namer for each function
        self.namer.reset();
        
        // Allocate all IR nodes in arena
        let mut nodes = Vec::new();
        
        for block in func.blocks_ordered() {
            let ir_block = self.lift_block(block);
            nodes.extend(ir_block);
        }
        
        // Move to arena for zero-copy access
        self.arena.alloc_slice(&nodes)
    }
}
```

## 6. Parallel Analysis (Deterministic)

### Function-Level Parallelism

```rust
pub struct DeterministicParallelAnalyzer {
    /// Thread pool with fixed size
    pool: ThreadPool,
    
    /// Results collected in deterministic order
    results: BTreeMap<Address, AnalysisResult>,
}

impl DeterministicParallelAnalyzer {
    pub fn analyze_all(&mut self, functions: &[Function]) -> Vec<AnalysisResult> {
        // Sort functions by address first
        let sorted_funcs: Vec<_> = functions.iter()
            .sorted_by_key(|f| f.address)
            .collect();
        
        // Process in parallel but collect in order
        let (tx, rx) = mpsc::channel();
        
        for func in sorted_funcs {
            let tx = tx.clone();
            let func = func.clone();
            
            self.pool.execute(move || {
                let result = analyze_function(&func);
                tx.send((func.address, result)).unwrap();
            });
        }
        
        drop(tx);
        
        // Collect results
        while let Ok((addr, result)) = rx.recv() {
            self.results.insert(addr, result);
        }
        
        // Return in deterministic order
        self.results.values().cloned().collect()
    }
}
```

## 7. Testing Determinism

### Comprehensive Test Suite

```rust
#[cfg(test)]
mod determinism_tests {
    use super::*;
    use sha2::{Sha256, Digest};
    
    #[test]
    fn test_absolute_determinism() {
        let test_binary = include_bytes!("../testdata/complex.exe");
        
        // Run 1000 times under different conditions
        let mut hashes = Vec::new();
        
        for i in 0..1000 {
            // Pollute memory differently
            let _garbage: Vec<_> = (0..i*1000)
                .map(|x| vec![x as u8; x % 1000])
                .collect();
            
            // Create fresh decompiler
            let decompiler = Decompiler::new();
            
            // Decompile
            let output = decompiler.decompile(test_binary);
            
            // Serialize entire output
            let serialized = bincode::serialize(&output).unwrap();
            
            // Hash the bytes
            let mut hasher = Sha256::new();
            hasher.update(&serialized);
            let hash = hasher.finalize();
            
            hashes.push(hash);
        }
        
        // ALL hashes must be identical
        let first = &hashes[0];
        for (i, hash) in hashes.iter().enumerate() {
            assert_eq!(first, hash, 
                "Run {} produced different output! CRITICAL BUG!", i);
        }
    }
    
    #[test]
    fn test_parallel_determinism() {
        let binary = load_test_binary();
        
        // Sequential run
        let sequential = {
            let d = Decompiler::new();
            d.decompile(&binary)
        };
        
        // Parallel run with different thread counts
        for num_threads in [1, 2, 4, 8, 16, 32] {
            let parallel = {
                let mut d = Decompiler::new();
                d.set_thread_count(num_threads);
                d.decompile(&binary)
            };
            
            assert_eq!(sequential, parallel,
                "Parallel with {} threads differs!", num_threads);
        }
    }
    
    #[test]
    fn test_cross_platform_determinism() {
        // This test would run in CI on different platforms
        let binary = load_test_binary();
        let output = Decompiler::new().decompile(&binary);
        
        // Compare with golden hash
        let hash = compute_hash(&output);
        let expected = "a1b2c3d4e5f6..."; // Pre-computed on reference platform
        
        assert_eq!(hash, expected, "Cross-platform determinism violated!");
    }
}
```

### Determinism Validation Tools

```rust
/// Run-time determinism checker
pub struct DeterminismValidator {
    seen_operations: BTreeMap<OperationKey, OperationResult>,
}

impl DeterminismValidator {
    pub fn validate_operation<T>(&mut self, 
        key: OperationKey, 
        compute: impl FnOnce() -> T
    ) -> T 
    where T: Eq + Clone + Debug
    {
        let result = compute();
        
        if let Some(previous) = self.seen_operations.get(&key) {
            assert_eq!(previous, &result,
                "Non-deterministic operation detected: {:?}", key);
        } else {
            self.seen_operations.insert(key, result.clone());
        }
        
        result
    }
}
```

## 8. Common Pitfalls and Solutions

### Pitfall: Floating Point in Analysis

```rust
// ❌ NEVER use floating point
let complexity = instrs as f64 / blocks as f64;

// ✅ Use fixed-point arithmetic
let complexity = (instrs * 1000) / blocks;  // 3 decimal places
```

### Pitfall: Time-Based Operations

```rust
// ❌ NEVER include timestamps
let comment = format!("Generated at {}", Utc::now());

// ✅ Use deterministic markers
let comment = format!("Generated from binary hash {}", binary_hash);
```

### Pitfall: Thread-Local State

```rust
// ❌ NEVER use thread IDs or thread-local storage
thread_local! {
    static COUNTER: RefCell<u32> = RefCell::new(0);
}

// ✅ Pass state explicitly
struct AnalysisState {
    counter: u32,
}
```

### Pitfall: Default Hash Functions

```rust
// ❌ NEVER rely on default hash behavior
let mut seen = HashSet::new();

// ✅ Use deterministic collections
let mut seen = BTreeSet::new();
```

## 9. Performance Without Breaking Determinism

### Safe Optimizations

```rust
// ✅ Parallel read, sequential write
let analyses: Vec<_> = functions.par_iter()
    .map(|f| analyze(f))
    .collect();

// Then sort results
let mut results: Vec<_> = analyses.into_iter()
    .map(|a| (a.address, a))
    .collect();
results.sort_by_key(|(addr, _)| *addr);

// ✅ Caching with deterministic keys
let cache: BTreeMap<CacheKey, CachedResult> = BTreeMap::new();

// ✅ SIMD with deterministic operations
use std::simd::*;
let opcodes = u8x32::from_slice(&bytes[offset..]);
let matches = opcodes.simd_eq(u8x32::splat(0x90));  // NOP detection
```

### Measurement and Validation

```rust
#[cfg(debug_assertions)]
macro_rules! assert_deterministic {
    ($expr:expr) => {{
        let result = $expr;
        if cfg!(feature = "validate-determinism") {
            DETERMINISM_VALIDATOR.validate(stringify!($expr), result)
        } else {
            result
        }
    }}
}
```

## 10. Determinism Checklist

Before any code change, verify:

- [ ] No HashMap/HashSet usage
- [ ] All collections use BTree variants
- [ ] Iteration order is explicit
- [ ] No floating point operations
- [ ] No time-dependent values
- [ ] No platform-specific sizes
- [ ] Fixed-width formatting for addresses
- [ ] Deterministic naming scheme
- [ ] Sorted processing order
- [ ] Tests include determinism validation

## Conclusion

Determinism is not a feature - it's the foundation of trustworthy decompilation. Every design decision must prioritize
deterministic behavior, even at the cost of some performance. Users must be able to trust that the same binary always
produces the same decompiled output.

Remember: **If it's not deterministic, it's a bug.**