# Deterministic IR Generation Design

## Core Principle

**Same assembly input MUST always produce the same IR output**, regardless of:
- Analysis order
- System state
- Previous decompilation runs
- Memory addresses
- Hash map iteration order

## Implementation Strategy

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

### 2. Address-Based Ordering

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

### 3. Canonical IR Generation

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

### 4. Stable Block Splitting

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

### 7. Testing Determinism

```rust
#[cfg(test)]
mod determinism_tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_deterministic_ir_generation() {
        let assembly = vec![
            Instruction::new(0x1000, Opcode::MOV { dst: Reg::RAX, src: Reg::RBX }),
            Instruction::new(0x1003, Opcode::ADD { dst: Reg::RAX, src: Imm(1) }),
            Instruction::new(0x1006, Opcode::JZ { target: 0x1010 }),
        ];
        
        // Generate IR multiple times
        let mut results = Vec::new();
        for _ in 0..100 {
            let generator = IRGenerator::new();
            let ir = generator.generate(&assembly);
            results.push(ir);
        }
        
        // All results must be identical
        for i in 1..results.len() {
            assert_eq!(results[0], results[i], 
                "IR generation was non-deterministic on iteration {}", i);
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

### 8. Hash-Independent Implementation

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

## Best Practices

1. **Always use deterministic collections** (BTreeMap, IndexMap, BTreeSet)
2. **Sort before iterating** when order matters
3. **Use addresses as primary keys** for stability
4. **Avoid floating-point** in IR generation
5. **Make temp names address-based**, not counter-based
6. **Test with different heap states** to catch allocation-dependent behavior
7. **Use canonical forms** for commutative operations
8. **Document any intentional non-determinism** (e.g., optimization heuristics)