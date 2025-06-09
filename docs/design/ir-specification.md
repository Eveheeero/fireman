# Fireman IR Specification

## Design Goals

1. **Deterministic**: Same input always produces same IR
2. **Type-Safe**: Strongly typed with explicit casts
3. **SSA-Based**: Static Single Assignment for analysis
4. **Architecture-Neutral**: Abstract away ISA details
5. **Analysis-Friendly**: Easy to pattern match and transform

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
/// IR values (operands)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    /// Constant values
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

/// Local SSA variable
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalId {
    /// Base name (e.g., "rax", "temp")
    pub name: String,
    
    /// SSA version (0 for non-SSA)
    pub version: u32,
    
    /// Source instruction address
    pub source: Address,
}

impl Display for LocalId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.version == 0 {
            write!(f, "%{}", self.name)
        } else {
            write!(f, "%{}.{}", self.name, self.version)
        }
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
    
    /// PHI nodes (must come first)
    pub phis: Vec<Instruction>,
    
    /// Regular instructions (deterministic order)
    pub instructions: Vec<Instruction>,
    
    /// Block terminator
    pub terminator: Terminator,
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

## Lifting Rules

### x86_64 to IR

```rust
/// Deterministic x86_64 instruction lifting
impl X86Lifter {
    /// MOV instruction
    fn lift_mov(&mut self, dst: &Operand, src: &Operand, addr: Address) -> Vec<Instruction> {
        let (src_val, mut pre) = self.operand_to_value(src, addr);
        let (dst_loc, mut post) = self.location_to_lvalue(dst, addr);
        
        pre.append(&mut post);
        pre.push(self.create_store(dst_loc, src_val, addr));
        pre
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

## Validation

```rust
pub struct IRValidator {
    pub fn validate_module(&self, module: &Module) -> Result<(), ValidationError> {
        // Check all functions
        for (id, func) in &module.functions {
            self.validate_function(func)?;
        }
        
        // Check all references resolve
        self.validate_references(module)?;
        
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

## Determinism Guarantees

1. **No HashMaps in IR**: Only BTreeMap, IndexMap
2. **Address-based IDs**: All identifiers derived from addresses
3. **Canonical operand ordering**: Consistent for commutative ops
4. **Deterministic temp names**: Based on source address + counter
5. **Sorted collections**: All iterations in defined order
6. **No floating-point arithmetic**: During IR construction
7. **Explicit undefined behavior**: Mark as Unknown/Undef