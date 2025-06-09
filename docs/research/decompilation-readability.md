# Decompilation Readability Research

## Human-Readable Decompilation Principles

### 1. Cognitive Load Reduction

#### Variable Naming Strategies
```c
// Bad: Raw decompilation
int32_t v1, v2, v3;
v1 = *(int32_t*)(a1 + 0x10);
v2 = *(int32_t*)(a1 + 0x14);
v3 = v1 + v2;
*(int32_t*)(a1 + 0x18) = v3;

// Good: Context-aware naming
struct Point {
    int32_t x;      // +0x10
    int32_t y;      // +0x14
    int32_t sum;    // +0x18
};

point->sum = point->x + point->y;
```

#### Pattern-Based Improvements
```c
// Pattern: Array iteration
// Before:
int32_t i = 0;
loop_1000:
    if (i >= 100) goto end_1050;
    *(base + i * 4) = 0;
    i = i + 1;
    goto loop_1000;
end_1050:

// After:
for (int32_t i = 0; i < 100; i++) {
    array[i] = 0;
}
```

### 2. Structure Recognition

#### Common Patterns Database
```rust
pub struct PatternDatabase {
    // String operations
    strlen_pattern: Pattern,
    strcpy_pattern: Pattern,
    memset_pattern: Pattern,
    
    // Data structures
    linked_list_pattern: Pattern,
    binary_tree_pattern: Pattern,
    hash_table_pattern: Pattern,
    
    // Algorithms
    sort_pattern: Pattern,
    search_pattern: Pattern,
    crypto_pattern: Pattern,
}

pub struct Pattern {
    name: String,
    signature: Vec<InstructionPattern>,
    confidence_threshold: f64,
    replacement: HighLevelConstruct,
}
```

#### Structure Recovery Algorithm
```rust
fn recover_structures(blocks: Vec<BasicBlock>) -> Vec<Structure> {
    let mut structures = Vec::new();
    
    // Phase 1: Identify access patterns
    let access_patterns = analyze_memory_access(&blocks);
    
    // Phase 2: Cluster related accesses
    let clusters = cluster_accesses(access_patterns);
    
    // Phase 3: Infer structure layout
    for cluster in clusters {
        if let Some(structure) = infer_structure_from_cluster(cluster) {
            structures.push(structure);
        }
    }
    
    // Phase 4: Validate with type constraints
    validate_structures(&mut structures);
    
    structures
}
```

### 3. Control Flow Readability

#### Structured Control Flow Recovery
```c
// Level 1: Direct translation
if (condition) goto label1;
    statement1;
    goto label2;
label1:
    statement2;
label2:

// Level 2: Basic structuring
if (!condition) {
    statement1;
} else {
    statement2;
}

// Level 3: Pattern recognition
// Recognize as error checking pattern
if (error_condition) {
    handle_error();
    return ERROR_CODE;
}
proceed_normally();
```

#### Loop Canonicalization
```rust
enum LoopType {
    CountingFor {
        init: Statement,
        condition: Expression,
        increment: Expression,
    },
    WhileLoop {
        condition: Expression,
    },
    DoWhile {
        condition: Expression,
    },
    ForEach {
        iterator: Expression,
        collection: Expression,
    },
}

fn canonicalize_loop(cfg: &ControlFlowGraph) -> Option<LoopType> {
    // Identify loop header, body, and exit
    let header = find_loop_header(cfg)?;
    let body = find_loop_body(cfg, header)?;
    let exit = find_loop_exit(cfg, header)?;
    
    // Pattern match for loop type
    if let Some(counting) = try_extract_counting_loop(header, body, exit) {
        return Some(LoopType::CountingFor { 
            init: counting.init,
            condition: counting.condition,
            increment: counting.increment,
        });
    }
    
    // Try other patterns...
}
```

### 4. Type Recovery and Presentation

#### Meaningful Type Names
```c
// Before: Raw types
void* func1(void* a1, int a2) {
    int32_t* v1 = (int32_t*)a1;
    char* v2 = (char*)malloc(a2);
    // ...
}

// After: Inferred types
struct Buffer* allocate_buffer(struct Context* ctx, size_t size) {
    int32_t* refcount = &ctx->refcount;
    char* data = (char*)malloc(size);
    // ...
}
```

#### Type Propagation
```rust
struct TypeInferenceEngine {
    constraints: Vec<TypeConstraint>,
    solutions: HashMap<Variable, Type>,
}

impl TypeInferenceEngine {
    fn infer_types(&mut self, ir: &IR) -> TypeMap {
        // Generate constraints from operations
        for stmt in ir.statements() {
            match stmt {
                IR::Load(dst, src) => {
                    // dst has same type as dereferenced src
                    self.add_constraint(
                        TypeConstraint::Dereference(src.ty, dst.ty)
                    );
                }
                IR::Call(dst, func, args) => {
                    // Match against known function signatures
                    if let Some(sig) = lookup_signature(func) {
                        self.add_constraint(
                            TypeConstraint::FunctionCall(sig, dst.ty, args)
                        );
                    }
                }
                // ...
            }
        }
        
        // Solve constraint system
        self.solve_constraints()
    }
}
```

### 5. Expression Simplification

#### Arithmetic Simplification
```c
// Before: Compiler optimizations visible
result = ((x << 2) + x) << 1;  // x * 10
value = (y >> 31) & 1;          // sign bit
mask = ~(~0 << n);             // (1 << n) - 1

// After: Human-readable
result = x * 10;
value = (y < 0) ? 1 : 0;
mask = (1 << n) - 1;
```

#### Idiom Recognition
```rust
pub struct IdiomRecognizer {
    idioms: Vec<Idiom>,
}

pub struct Idiom {
    pattern: ExprPattern,
    meaning: String,
    replacement: Box<dyn Fn(&Bindings) -> Expression>,
}

impl IdiomRecognizer {
    fn new() -> Self {
        let mut idioms = Vec::new();
        
        // Swap without temp
        idioms.push(Idiom {
            pattern: pattern!("a ^= b; b ^= a; a ^= b;"),
            meaning: "swap values",
            replacement: Box::new(|bindings| {
                expr!("swap({a}, {b})", bindings)
            }),
        });
        
        // Absolute value
        idioms.push(Idiom {
            pattern: pattern!("(x >> 31) ^ (x + (x >> 31))"),
            meaning: "absolute value",
            replacement: Box::new(|bindings| {
                expr!("abs({x})", bindings)
            }),
        });
        
        Self { idioms }
    }
}
```

### 6. Comment Generation

#### Automatic Documentation
```c
/**
 * Function: process_data
 * Address: 0x401000
 * Calling Convention: cdecl
 * Confidence: High
 * 
 * Likely performs data validation and transformation.
 * Uses standard CRC32 algorithm for checksum.
 */
int32_t process_data(uint8_t* buffer, size_t length) {
    // Validate input parameters
    if (buffer == NULL || length == 0) {
        return ERROR_INVALID_PARAMETER;  // -1
    }
    
    // Calculate CRC32 checksum
    // [DECOMPILER]: Recognized CRC32 polynomial 0xEDB88320
    uint32_t crc = 0xFFFFFFFF;
    for (size_t i = 0; i < length; i++) {
        crc = crc32_table[(crc ^ buffer[i]) & 0xFF] ^ (crc >> 8);
    }
    
    return ~crc;
}
```

#### Uncertainty Annotations
```c
// Function purpose unclear
__uncertain("Complex arithmetic - possible encryption")
int32_t mystery_function(int32_t input) {
    // [DECOMPILER]: High cyclomatic complexity (15)
    // [DECOMPILER]: Possible obfuscation detected
    
    int32_t result = input;
    
    __confidence(low) {
        result = ((result * 0x343FD) + 0x269EC3) & 0x7FFFFFFF;
        result ^= (result >> 16);
        result *= 0x85EBCA6B;
    }
    
    return result;
}
```

### 7. Incremental Improvement

#### Progressive Enhancement Pipeline
```rust
pub struct ReadabilityPipeline {
    stages: Vec<Box<dyn EnhancementStage>>,
}

impl ReadabilityPipeline {
    pub fn new() -> Self {
        Self {
            stages: vec![
                Box::new(BasicStructuring),      // Gotos to if/while
                Box::new(LoopRecovery),          // Identify loop patterns
                Box::new(TypeInference),         // Recover types
                Box::new(NameGeneration),        // Meaningful names
                Box::new(PatternRecognition),    // Library patterns
                Box::new(ExpressionSimplifier),  // Simplify arithmetic
                Box::new(CommentGenerator),      // Add documentation
            ],
        }
    }
    
    pub fn enhance(&self, code: Code) -> Code {
        self.stages.iter().fold(code, |code, stage| {
            stage.enhance(code)
        })
    }
}
```

### 8. User Interaction

#### Interactive Refinement
```rust
pub struct InteractiveDecompiler {
    pub fn get_user_hint(&self, context: &Context) -> Option<Hint> {
        // Present ambiguous case to user
        match context.ambiguity {
            Ambiguity::UnknownType(var) => {
                println!("Variable {} has ambiguous type:", var);
                println!("1. int32_t");
                println!("2. float");
                println!("3. pointer");
                // Get user input
            }
            Ambiguity::LoopBounds(loop_id) => {
                println!("Loop {} has unclear bounds:", loop_id);
                // Show assembly, get user interpretation
            }
        }
    }
}
```

## Readability Metrics

### Objective Measurements
```rust
pub struct ReadabilityScore {
    /// Average line length
    pub line_length: f64,
    
    /// Maximum nesting depth
    pub nesting_depth: usize,
    
    /// Ratio of meaningful to generic names
    pub naming_quality: f64,
    
    /// Percentage of structured control flow
    pub structure_recovery: f64,
    
    /// Cyclomatic complexity
    pub complexity: usize,
    
    /// Type annotation coverage
    pub type_coverage: f64,
}

impl ReadabilityScore {
    pub fn overall(&self) -> f64 {
        // Weighted combination of factors
        0.20 * self.naming_quality +
        0.25 * self.structure_recovery +
        0.20 * self.type_coverage +
        0.15 * (1.0 / (1.0 + self.complexity as f64)) +
        0.10 * (1.0 / (1.0 + self.nesting_depth as f64)) +
        0.10 * (1.0 / (1.0 + (self.line_length - 80.0).abs() / 80.0))
    }
}
```

## Best Practices Summary

1. **Preserve Correctness**: Never sacrifice accuracy for readability
2. **Progressive Enhancement**: Multiple passes, each improving readability
3. **Pattern Recognition**: Build comprehensive pattern database
4. **Type Recovery**: Aggressive type inference with constraint solving
5. **Meaningful Names**: Context-aware naming based on usage patterns
6. **Clear Structure**: Recover high-level control flow constructs
7. **Documentation**: Generate helpful comments and annotations
8. **User Feedback**: Allow interactive refinement for ambiguous cases