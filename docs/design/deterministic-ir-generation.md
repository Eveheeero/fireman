# Deterministic IR Generation Design

## Core Principle

**Same assembly input MUST always produce the same IR output**, regardless of:
- Analysis order
- System state
- Previous decompilation runs
- Memory addresses
- Hash map iteration order

## Implementation Strategy

### Critical Rule: Every Conversion Must Be Deterministic

**ABSOLUTE REQUIREMENT**: Given the same assembly input, every intermediate step and final IR output must be
byte-for-byte identical across:

- Different runs
- Different machines
- Different memory states
- Different thread scheduling
- Different allocator states

### 1. Deterministic Data Structures

```rust
use indexmap::{IndexMap, IndexSet};
use std::collections::BTreeMap;

/// Use ordered collections for deterministic iteration
pub struct DeterministicIR {
    /// Ordered by instruction address
    instructions: BTreeMap<Address, Instruction>,
    
    /// Preserve block discovery order
    blocks: IndexMap<BlockId, BasicBlock>,
    
    /// Deterministic register allocation
    registers: IndexMap<String, RegisterId>,
    
    /// Stable temporary naming
    temp_counter: u32,
}

/// Block IDs are address-based for stability
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockId(pub Address);

/// Registers have canonical names
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegisterId {
    /// Architecture register name (e.g., "rax", "r0")
    pub name: &'static str,
    /// Sub-register index (e.g., 0 for full, 1 for low half)
    pub sub: u8,
}
```

### 2. Deterministic Temporary Generation

```rust
/// CRITICAL: Temporary names must be 100% deterministic
pub struct DeterministicTempGen {
    /// Counter per (instruction_address, purpose) pair
    counters: BTreeMap<(Address, &'static str), u32>,
}

impl DeterministicTempGen {
    pub fn new_temp(&mut self, addr: Address, purpose: &'static str) -> TempVar {
        // ALWAYS use instruction address + purpose + counter
        let key = (addr, purpose);
        let counter = self.counters.entry(key).or_insert(0);
        *counter += 1;
        
        // Deterministic name format
        TempVar {
            name: format!("t_{:016x}_{}_{}", addr, purpose, counter - 1),
            source_addr: addr,
            purpose,
            index: counter - 1,
        }
    }
    
    /// Reset counters for each function to ensure isolation
    pub fn reset(&mut self) {
        self.counters.clear();
    }
}

// Example usage:
// MOV RAX, [RBX + 8] at address 0x401000 generates:
// t_0000000000401000_addr_calc_0  (for RBX + 8)
// t_0000000000401000_load_value_0  (for loaded value)
// t_0000000000401000_assign_0      (for assignment)
```

### 3. Address-Based Ordering

```rust
/// All IR elements are strictly ordered by address
impl IRGenerator {
    pub fn generate(&self, instructions: &[Instruction]) -> IR {
        // Sort instructions by address first
        let mut sorted_instructions: Vec<_> = instructions.to_vec();
        sorted_instructions.sort_by_key(|inst| inst.address);
        
        let mut ir = IR::new();
        
        // Process in deterministic order
        for inst in sorted_instructions {
            let statements = self.lift_instruction(&inst);
            ir.add_statements(inst.address, statements);
        }
        
        ir
    }
}

/// Temporary variables are named based on their creation address
impl TempGenerator {
    pub fn new_temp(&mut self, addr: Address, ty: Type) -> Temp {
        // Use address and counter for deterministic naming
        let name = format!("t_{:x}_{}", addr, self.get_counter(addr));
        Temp { name, ty }
    }
    
    fn get_counter(&mut self, addr: Address) -> u32 {
        let counter = self.counters.entry(addr).or_insert(0);
        *counter += 1;
        *counter - 1
    }
}
```

### 4. Deterministic Instruction Lifting

```rust
/// Every instruction type has EXACTLY ONE lifting pattern
pub struct DeterministicLifter {
    temp_gen: DeterministicTempGen,
    
    /// Fixed lifting rules - no random choices
    lifting_rules: BTreeMap<Opcode, LiftingRule>,
}

impl DeterministicLifter {
    /// MOV instruction - deterministic lifting
    pub fn lift_mov(&mut self, inst: &X86Inst, addr: Address) -> Vec<IRStatement> {
        let mut statements = Vec::new();
        
        match (&inst.dst, &inst.src) {
            // REG <- REG
            (Operand::Reg(dst), Operand::Reg(src)) => {
                statements.push(IRStatement::Assign {
                    dst: self.reg_to_var(dst),
                    src: self.reg_to_var(src),
                    addr,
                });
            }
            
            // REG <- IMM
            (Operand::Reg(dst), Operand::Imm(imm)) => {
                statements.push(IRStatement::Assign {
                    dst: self.reg_to_var(dst),
                    src: IRValue::Const(self.normalize_immediate(*imm)),
                    addr,
                });
            }
            
            // REG <- MEM
            (Operand::Reg(dst), Operand::Mem(mem)) => {
                // ALWAYS calculate address first
                let addr_temp = self.temp_gen.new_temp(addr, "addr_calc");
                statements.push(self.calculate_address(mem, addr_temp, addr));
                
                // ALWAYS load to temp first
                let load_temp = self.temp_gen.new_temp(addr, "load_value");
                statements.push(IRStatement::Load {
                    dst: load_temp.clone(),
                    addr: IRValue::Var(addr_temp),
                    size: inst.size,
                    source: addr,
                });
                
                // ALWAYS assign temp to register
                statements.push(IRStatement::Assign {
                    dst: self.reg_to_var(dst),
                    src: IRValue::Var(load_temp),
                    addr,
                });
            }
            
            // MEM <- REG
            (Operand::Mem(mem), Operand::Reg(src)) => {
                // ALWAYS same pattern: calc addr, then store
                let addr_temp = self.temp_gen.new_temp(addr, "addr_calc");
                statements.push(self.calculate_address(mem, addr_temp, addr));
                
                statements.push(IRStatement::Store {
                    addr: IRValue::Var(addr_temp),
                    value: self.reg_to_var(src),
                    size: inst.size,
                    source: addr,
                });
            }
            
            _ => panic!("Unhandled MOV variant at {:x} - MUST handle all cases", addr),
        }
        
        statements
    }
    
    /// Deterministic address calculation
    fn calculate_address(&mut self, mem: &MemOperand, dst: TempVar, addr: Address) -> IRStatement {
        match mem {
            MemOperand::BaseDisp { base, disp } => {
                if *disp == 0 {
                    // Just copy base
                    IRStatement::Assign {
                        dst,
                        src: self.reg_to_var(base),
                        addr,
                    }
                } else {
                    // ALWAYS add in same order: base + disp
                    IRStatement::BinOp {
                        op: BinOp::Add,
                        dst,
                        left: self.reg_to_var(base),
                        right: IRValue::Const(self.normalize_immediate(*disp)),
                        addr,
                    }
                }
            }
            
            MemOperand::BaseIndexScale { base, index, scale, disp } => {
                // ALWAYS compute in same order: base + (index * scale) + disp
                let scaled = self.temp_gen.new_temp(addr, "scaled_index");
                
                // ... deterministic computation
            }
            
            _ => panic!("All memory operand types must be handled"),
        }
    }
}
```

### 5. Canonical IR Generation

```rust
/// Each instruction type has exactly one IR translation
pub trait InstructionLifter {
    fn lift(&self, inst: &Instruction, addr: Address) -> Vec<Statement>;
}

/// x86_64 Example
impl InstructionLifter for X86Lifter {
    fn lift(&self, inst: &Instruction, addr: Address) -> Vec<Statement> {
        match &inst.opcode {
            // MOV reg, reg
            Opcode::MOV { dst, src } if dst.is_reg() && src.is_reg() => {
                vec![Statement::Assign {
                    dst: self.reg_to_ir(dst),
                    src: self.reg_to_ir(src),
                    addr,
                }]
            }
            
            // MOV reg, [mem]
            Opcode::MOV { dst, src } if dst.is_reg() && src.is_mem() => {
                let temp_addr = self.new_temp(addr, Type::Ptr);
                let temp_val = self.new_temp(addr, dst.size().to_type());
                
                vec![
                    Statement::Assign {
                        dst: temp_addr.clone(),
                        src: self.mem_to_address(src, addr),
                        addr,
                    },
                    Statement::Load {
                        dst: temp_val.clone(),
                        src: temp_addr,
                        size: dst.size(),
                        addr,
                    },
                    Statement::Assign {
                        dst: self.reg_to_ir(dst),
                        src: temp_val,
                        addr,
                    },
                ]
            }
            
            // Every case must be explicitly handled
            _ => panic!("Unhandled instruction at {:x}: {:?}", addr, inst),
        }
    }
}
```

### 6. Deterministic Control Flow

```rust
/// Control flow must be discovered in same order every time
pub struct DeterministicCFGBuilder {
    /// Process addresses in sorted order
    work_queue: BTreeSet<Address>,
    
    /// Track what we've seen
    visited: BTreeSet<Address>,
    
    /// Deterministic block IDs
    block_ids: BTreeMap<Address, BlockId>,
}

impl DeterministicCFGBuilder {
    pub fn build_cfg(&mut self, entry: Address) -> ControlFlowGraph {
        self.work_queue.insert(entry);
        let mut cfg = ControlFlowGraph::new();
        
        // ALWAYS process addresses in sorted order
        while let Some(addr) = self.work_queue.pop_first() {
            if self.visited.contains(&addr) {
                continue;
            }
            
            self.visited.insert(addr);
            
            // Assign deterministic block ID
            let block_id = self.get_or_create_block_id(addr);
            
            // Analyze block
            let block = self.analyze_block(addr);
            
            // Add successors in deterministic order
            match &block.terminator {
                Terminator::Branch(target) => {
                    self.work_queue.insert(*target);
                    cfg.add_edge(block_id, self.get_or_create_block_id(*target));
                }
                
                Terminator::ConditionalBranch { true_target, false_target, .. } => {
                    // ALWAYS add in same order: true branch first
                    self.work_queue.insert(*true_target);
                    self.work_queue.insert(*false_target);
                    
                    cfg.add_edge(block_id, self.get_or_create_block_id(*true_target));
                    cfg.add_edge(block_id, self.get_or_create_block_id(*false_target));
                }
                
                Terminator::Switch { default, cases, .. } => {
                    // Add default first
                    self.work_queue.insert(*default);
                    cfg.add_edge(block_id, self.get_or_create_block_id(*default));
                    
                    // Add cases in sorted order
                    for (_, target) in cases.iter() {  // BTreeMap iterates in order
                        self.work_queue.insert(*target);
                        cfg.add_edge(block_id, self.get_or_create_block_id(*target));
                    }
                }
                
                _ => {} // Return, unreachable, etc.
            }
            
            cfg.add_block(block_id, block);
        }
        
        cfg
    }
    
    fn get_or_create_block_id(&mut self, addr: Address) -> BlockId {
        *self.block_ids.entry(addr)
            .or_insert_with(|| BlockId(addr))
    }
}
```

### 7. Stable Block Splitting

```rust
/// Deterministic basic block boundaries
pub struct BlockBuilder {
    /// Sort targets for consistent ordering
    pub fn find_block_boundaries(&self, instructions: &[Instruction]) -> BTreeSet<Address> {
        let mut boundaries = BTreeSet::new();
        
        // Entry point is always a boundary
        if let Some(first) = instructions.first() {
            boundaries.insert(first.address);
        }
        
        for inst in instructions {
            match inst.opcode {
                // Unconditional jump
                Opcode::JMP { target } => {
                    boundaries.insert(target);
                    if let Some(next) = self.next_instruction_address(inst) {
                        boundaries.insert(next);
                    }
                }
                
                // Conditional jump
                Opcode::Jcc { target, .. } => {
                    boundaries.insert(target);
                    if let Some(next) = self.next_instruction_address(inst) {
                        boundaries.insert(next);
                    }
                }
                
                // Call (if not tail call)
                Opcode::CALL { .. } => {
                    if let Some(next) = self.next_instruction_address(inst) {
                        boundaries.insert(next);
                    }
                }
                
                // Return
                Opcode::RET => {
                    if let Some(next) = self.next_instruction_address(inst) {
                        boundaries.insert(next);
                    }
                }
                
                _ => {}
            }
        }
        
        boundaries
    }
}
```

### 5. Deterministic SSA Construction

```rust
/// SSA form with deterministic phi placement
pub struct SSABuilder {
    /// Dominance frontier calculation is deterministic
    pub fn insert_phi_nodes(&self, cfg: &ControlFlowGraph) -> PhiNodes {
        let mut phi_nodes = BTreeMap::new();
        
        // Process variables in sorted order
        let variables: BTreeSet<_> = self.collect_all_variables(cfg)
            .into_iter()
            .collect();
        
        for var in variables {
            let mut work_list: BTreeSet<_> = self.blocks_defining(var)
                .into_iter()
                .collect();
            
            let mut has_phi = BTreeSet::new();
            
            while let Some(block) = work_list.pop_first() {
                for frontier_block in &self.dominance_frontiers[&block] {
                    if !has_phi.contains(frontier_block) {
                        phi_nodes.entry(*frontier_block)
                            .or_insert_with(BTreeSet::new)
                            .insert(var.clone());
                        
                        has_phi.insert(*frontier_block);
                        work_list.insert(*frontier_block);
                    }
                }
            }
        }
        
        PhiNodes(phi_nodes)
    }
    
    /// Rename variables deterministically
    pub fn rename_variables(&mut self, cfg: &ControlFlowGraph) -> SSAForm {
        // Process blocks in reverse post-order (deterministic)
        let rpo = self.reverse_post_order(cfg);
        
        for block_id in rpo {
            self.rename_block(block_id);
        }
        
        self.ssa_form.clone()
    }
}
```

### 6. Canonicalization Rules

```rust
/// Canonical form for IR operations
pub struct IRCanonicalizer {
    /// Normalize commutative operations
    pub fn canonicalize(&self, stmt: Statement) -> Statement {
        match stmt {
            Statement::BinOp { op, left, right, dst, addr } => {
                match op {
                    // Commutative ops: smaller operand first
                    BinOp::Add | BinOp::Mul | BinOp::And | BinOp::Or | BinOp::Xor => {
                        let (left, right) = self.order_operands(left, right);
                        Statement::BinOp { op, left, right, dst, addr }
                    }
                    
                    // Non-commutative: keep original order
                    _ => Statement::BinOp { op, left, right, dst, addr }
                }
            }
            
            // Normalize immediate values
            Statement::Assign { dst, src: Operand::Immediate(val), addr } => {
                Statement::Assign {
                    dst,
                    src: Operand::Immediate(self.normalize_immediate(val)),
                    addr,
                }
            }
            
            _ => stmt,
        }
    }
    
    /// Order operands deterministically
    fn order_operands(&self, a: Operand, b: Operand) -> (Operand, Operand) {
        use std::cmp::Ordering;
        
        match self.compare_operands(&a, &b) {
            Ordering::Less | Ordering::Equal => (a, b),
            Ordering::Greater => (b, a),
        }
    }
    
    fn compare_operands(&self, a: &Operand, b: &Operand) -> Ordering {
        match (a, b) {
            (Operand::Immediate(x), Operand::Immediate(y)) => x.cmp(y),
            (Operand::Register(x), Operand::Register(y)) => x.cmp(y),
            (Operand::Temp(x), Operand::Temp(y)) => x.cmp(y),
            // Immediates come first, then registers, then temps
            (Operand::Immediate(_), _) => Ordering::Less,
            (_, Operand::Immediate(_)) => Ordering::Greater,
            (Operand::Register(_), Operand::Temp(_)) => Ordering::Less,
            (Operand::Temp(_), Operand::Register(_)) => Ordering::Greater,
        }
    }
}
```

### 9. Deterministic Constant Handling

```rust
/// All constants must have canonical representation
pub struct ConstantNormalizer {
    /// Normalize all integers to canonical form
    pub fn normalize_integer(&self, value: i128, size: Size) -> i128 {
        match size {
            Size::Byte => (value as i8) as i128,      // Sign extend from 8 bits
            Size::Word => (value as i16) as i128,     // Sign extend from 16 bits
            Size::Dword => (value as i32) as i128,    // Sign extend from 32 bits
            Size::Qword => (value as i64) as i128,    // Sign extend from 64 bits
            _ => value,
        }
    }
    
    /// Normalize floating point to bit representation
    pub fn normalize_float(&self, value: f64) -> u64 {
        // ALWAYS use bit representation, never float comparison
        value.to_bits()
    }
    
    /// Canonical representation for addresses
    pub fn normalize_address(&self, addr: u64) -> u64 {
        // Zero-extend to 64 bits, no sign extension
        addr & 0xFFFF_FFFF_FFFF_FFFF
    }
}

/// Deterministic immediate operand handling
impl DeterministicLifter {
    fn normalize_immediate(&self, imm: i64) -> IRValue {
        // ALWAYS use canonical representation
        let normalized = self.normalizer.normalize_integer(imm as i128, self.current_size);
        
        IRValue::Const(Constant::Int {
            value: normalized,
            size: self.current_size,
            // ALWAYS include signedness hint
            is_signed: normalized < 0,
        })
    }
}
```

### 10. Testing Determinism

```rust
#[cfg(test)]
mod determinism_tests {
    use super::*;
    use sha2::{Sha256, Digest};
    
    /// Test exact byte-for-byte determinism
    #[test]
    fn test_exact_determinism() {
        let test_cases = vec![
            // Same instruction, different memory states
            vec![Instruction::mov(0x1000, Reg::RAX, Reg::RBX)],
            
            // Complex addressing modes
            vec![Instruction::mov(0x2000, Reg::RAX, Mem::base_index_scale(
                Reg::RBX, Reg::RCX, 8, 0x10
            ))],
            
            // Control flow
            vec![
                Instruction::cmp(0x3000, Reg::RAX, Imm(0)),
                Instruction::jz(0x3003, 0x3010),
            ],
        ];
        
        for test_asm in test_cases {
            // Generate IR 1000 times with different conditions
            let mut hashes = Vec::new();
            
            for i in 0..1000 {
                // Pollute memory state
                let _garbage: Vec<_> = (0..i).map(|x| vec![x as u8; x % 100]).collect();
                
                // Generate with fresh generator
                let mut generator = IRGenerator::new();
                let ir = generator.generate(&test_asm);
                
                // Serialize to bytes for exact comparison
                let bytes = bincode::serialize(&ir).unwrap();
                
                // Hash the bytes
                let mut hasher = Sha256::new();
                hasher.update(&bytes);
                let hash = hasher.finalize();
                
                hashes.push(hash);
            }
            
            // All hashes must be EXACTLY identical
            let first_hash = &hashes[0];
            for (i, hash) in hashes.iter().enumerate().skip(1) {
                assert_eq!(first_hash, hash,
                    "IR generation produced different output on iteration {} for {:?}",
                    i, test_asm
                );
            }
        }
    }
    
    #[test]
    fn test_deterministic_with_different_allocation_patterns() {
        // Test with different heap states
        let assembly = create_test_assembly();
        
        // Force different allocation patterns
        let ir1 = {
            let _allocations: Vec<_> = (0..1000).map(|i| vec![i; 100]).collect();
            IRGenerator::new().generate(&assembly)
        };
        
        let ir2 = {
            // Different allocation pattern
            drop(vec![0u8; 1_000_000]);
            IRGenerator::new().generate(&assembly)
        };
        
        assert_eq!(ir1, ir2, "IR generation depends on memory allocation");
    }
}
```

### 11. Deterministic Optimization Passes

```rust
/// Even optimizations must be deterministic
pub struct DeterministicOptimizer {
    /// Apply passes in fixed order
    passes: Vec<Box<dyn OptimizationPass>>,
}

impl DeterministicOptimizer {
    pub fn optimize(&self, mut ir: IR) -> IR {
        // ALWAYS apply passes in same order
        for (pass_idx, pass) in self.passes.iter().enumerate() {
            let before_hash = self.hash_ir(&ir);
            
            ir = pass.apply(ir);
            
            let after_hash = self.hash_ir(&ir);
            
            // Verify determinism
            self.verify_deterministic(pass_idx, &before_hash, &after_hash);
        }
        
        ir
    }
}

/// Example: Constant folding must be deterministic
pub struct DeterministicConstantFolder;

impl OptimizationPass for DeterministicConstantFolder {
    fn apply(&self, ir: IR) -> IR {
        let mut new_statements = Vec::new();
        
        for stmt in ir.statements {
            match stmt {
                IRStatement::BinOp { op, dst, left: Const(a), right: Const(b), addr } => {
                    // ALWAYS fold in same way
                    let result = match op {
                        BinOp::Add => self.safe_add(a, b),
                        BinOp::Sub => self.safe_sub(a, b),
                        BinOp::Mul => self.safe_mul(a, b),
                        // ... handle all ops
                    };
                    
                    new_statements.push(IRStatement::Assign {
                        dst,
                        src: IRValue::Const(result),
                        addr,
                    });
                }
                _ => new_statements.push(stmt),
            }
        }
        
        IR { statements: new_statements, ..ir }
    }
    
    /// Arithmetic must handle overflow deterministically
    fn safe_add(&self, a: Constant, b: Constant) -> Constant {
        match (a, b) {
            (Constant::Int { value: a, size }, Constant::Int { value: b, .. }) => {
                // Use wrapping arithmetic for determinism
                let result = a.wrapping_add(b);
                let normalized = self.normalize_for_size(result, size);
                Constant::Int { value: normalized, size, is_signed: false }
            }
            _ => panic!("Type mismatch in constant folding"),
        }
    }
}
```

### 12. Hash-Independent Implementation

```rust
/// Avoid HashMap for any ordering-sensitive operations
pub struct DeterministicAnalyzer {
    /// Use BTreeMap instead of HashMap
    symbol_table: BTreeMap<String, Symbol>,
    
    /// Use IndexMap when insertion order matters
    functions: IndexMap<Address, Function>,
    
    /// Sort before processing when using HashMap
    pub fn analyze_with_hashmap(&self, data: &HashMap<Key, Value>) -> Result {
        // Convert to sorted vec first
        let mut sorted: Vec<_> = data.iter().collect();
        sorted.sort_by_key(|(k, _)| *k);
        
        for (key, value) in sorted {
            self.process(key, value)?;
        }
        
        Ok(self.result())
    }
}
```

## Verification Strategies

### 1. Snapshot Testing
```rust
#[test]
fn test_ir_snapshot() {
    let binary = include_bytes!("../testdata/sample.bin");
    let ir = decompile_to_ir(binary);
    
    // Compare against golden snapshot
    insta::assert_yaml_snapshot!(ir);
}
```

### 2. Round-Trip Testing
```rust
#[test]
fn test_ir_round_trip() {
    let original_asm = load_test_assembly();
    let ir = lift_to_ir(&original_asm);
    let regenerated_asm = lower_from_ir(&ir);
    
    assert_equivalent_behavior(&original_asm, &regenerated_asm);
}
```

### 3. Differential Testing
```rust
#[test]
fn test_determinism_across_platforms() {
    let results = vec![
        run_on_platform("linux-x64"),
        run_on_platform("windows-x64"),
        run_on_platform("macos-arm64"),
    ];
    
    // All platforms must produce identical IR
    assert!(results.windows(2).all(|w| w[0] == w[1]));
}
```

## Absolute Rules for Determinism

### MUST DO:

1. **Use BTreeMap/BTreeSet everywhere** - NEVER HashMap/HashSet
2. **Sort all collections before iteration** - No exceptions
3. **Base everything on addresses** - Instructions addresses are your anchor
4. **Normalize all constants** - Same value, same representation
5. **Fixed order for everything** - Operands, blocks, edges, passes
6. **Test with hash comparison** - Byte-for-byte identical
7. **Reset state between functions** - No cross-function pollution
8. **Handle all cases explicitly** - No default/random behavior

### MUST NOT:

1. **No floating point operations** - Use integer arithmetic only
2. **No system-dependent behavior** - No timestamps, random, thread IDs
3. **No pointer-based ordering** - Addresses change between runs
4. **No lazy randomness** - Even "doesn't matter" cases must be deterministic
5. **No hash-based collections** - Unless wrapped with sorted iteration
6. **No external state** - Each function lifts in isolation

### Testing Checklist:

- [ ] Run same binary 1000 times - identical IR?
- [ ] Run on different machines - identical IR?
- [ ] Run with different memory pressure - identical IR?
- [ ] Run with different thread counts - identical IR?
- [ ] Run after random allocations - identical IR?
- [ ] Serialize IR and compare SHA256 - identical?

### Common Determinism Bugs:

```rust
// BUG: HashMap iteration order
let mut map = HashMap::new();
for (k, v) in map.iter() { }  // NON-DETERMINISTIC!

// FIX: Use BTreeMap or sort
let mut map = BTreeMap::new();
for (k, v) in map.iter() { }  // Deterministic

// BUG: Temporary counter not isolated
static COUNTER: AtomicU32 = AtomicU32::new(0);
let temp = format!("t{}", COUNTER.fetch_add(1));  // NON-DETERMINISTIC!

// FIX: Address-based + local counter
let temp = format!("t_{:x}_{}", inst_addr, local_counter);

// BUG: Floating point comparison
if score > 0.5 { }  // NON-DETERMINISTIC!

// FIX: Integer comparison
if score_int > 500 { }  // Deterministic
```

Remember: **If two runs produce different IR for the same input, it's a CRITICAL BUG!**