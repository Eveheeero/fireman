# IR Invariants and Determinism Rules

## Core Invariants

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
/// Temporary naming scheme
pub struct TempNamer {
    /// Counters per address
    counters: BTreeMap<Address, BTreeMap<&'static str, u32>>,
}

impl TempNamer {
    /// Generate deterministic temporary name
    pub fn new_temp(&mut self, addr: Address, purpose: &'static str) -> LocalId {
        let counter = self.counters
            .entry(addr)
            .or_default()
            .entry(purpose)
            .or_insert(0);
        
        let name = format!("{}.{:x}.{}", purpose, addr, counter);
        *counter += 1;
        
        LocalId {
            name,
            version: 0,  // Will be set during SSA
            source: addr,
        }
    }
}

// Examples:
// load.1000.0  - first load temp at 0x1000
// load.1000.1  - second load temp at 0x1000  
// add.1003.0   - first add temp at 0x1003
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

## Determinism Rules

### Rule 1: No Implementation-Defined Behavior

```rust
// BAD: Depends on HashMap iteration order
let mut ops = HashMap::new();
for (k, v) in ops.iter() {  // Non-deterministic!
    process(k, v);
}

// GOOD: Use BTreeMap or sort
let mut ops = BTreeMap::new();
for (k, v) in ops.iter() {  // Deterministic!
    process(k, v);
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
/// Always process blocks in address order
fn order_blocks(blocks: &HashMap<BlockId, BasicBlock>) -> Vec<BlockId> {
    let mut ids: Vec<_> = blocks.keys().copied().collect();
    ids.sort_by_key(|id| id.0);  // BlockId(Address)
    ids
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
/// Phi nodes ordered by predecessor block
fn create_phi(predecessors: &[BlockId], values: &[Value], ty: Type) -> Instruction {
    let mut incoming = BTreeMap::new();
    
    for (pred, val) in predecessors.iter().zip(values) {
        incoming.insert(*pred, val.clone());
    }
    
    Instruction::Phi {
        dst: new_temp(addr, ty, "phi"),
        incoming,  // BTreeMap ensures deterministic iteration
        ty,
    }
}
```

## Testing Determinism

### Property-Based Tests

```rust
#[test]
fn prop_deterministic_lifting() {
    proptest!(|(bytes in vec(any::<u8>(), 1..100))| {
        // Lift same bytes multiple times
        let results: Vec<_> = (0..10)
            .map(|_| lift_bytes(&bytes))
            .collect();
        
        // All results must be identical
        for i in 1..results.len() {
            prop_assert_eq!(&results[0], &results[i]);
        }
    });
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

## Common Pitfalls

### 1. Address Truncation

```rust
// BAD: Loses high bits on 32-bit systems
let addr = ptr as usize;

// GOOD: Use proper address type
let addr = Address::from_ptr(ptr);
```

### 2. Floating Point in Analysis

```rust
// BAD: Floating point is non-deterministic
let score = complexity as f64 / size as f64;

// GOOD: Use integer arithmetic
let score = (complexity * 1000) / size;  // Fixed-point
```

### 3. Random Number Generation

```rust
// BAD: Non-deterministic
let temp_id = rand::random::<u32>();

// GOOD: Deterministic from context
let temp_id = hash_combine(addr, counter);
```

### 4. Time-Based Decisions

```rust
// BAD: Time-dependent
if elapsed > Duration::from_secs(1) {
    use_fast_path();
}

// GOOD: Complexity-based
if complexity > THRESHOLD {
    use_fast_path();
}
```

## Verification Checklist

- [ ] All collections use deterministic types (BTreeMap, IndexMap)
- [ ] No floating-point arithmetic in core logic
- [ ] All temporaries named by address + counter
- [ ] Commutative operations canonicalized
- [ ] Block processing in address order
- [ ] No system-dependent behavior (time, random, allocation)
- [ ] Explicit handling of undefined values
- [ ] All source addresses preserved in IR
- [ ] SSA versions assigned deterministically
- [ ] Constants normalized to canonical form