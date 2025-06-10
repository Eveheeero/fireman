# C Code Generation Fix Implementation Guide

## Problem Analysis

### Root Causes

1. **Missing Terminators**: `Pattern::LowIR` doesn't include terminator information, causing missing return statements
2. **Poor Variable Names**: LocalId information is not properly propagated, resulting in `param_0`, `result_0` names
3. **Incomplete Pattern Handling**: High IR generator assumes high-level patterns but doesn't handle Low IR fallbacks
   properly

## Phase 1: Critical Fixes (Week 1)

### Day 1-2: Fix Terminator Handling

#### Step 1: Enhance Pattern::LowIR Structure

```rust
// In medium_ir/mod.rs
pub enum Pattern {
    LowIR {
        instructions: Vec<low_ir::Instruction>,
        terminator: Option<low_ir::Terminator>,  // ADD THIS
        block_id: low_ir::BlockId,               // ADD THIS
        confidence: Confidence,
    },
    // ... other patterns
}
```

#### Step 2: Update Pattern Creation

```rust
// In medium_ir/analyzer.rs
fn create_low_ir_pattern(block: &low_ir::BasicBlock) -> Pattern {
    Pattern::LowIR {
        instructions: block.instructions.clone(),
        terminator: Some(block.terminator.clone()),  // Include terminator
        block_id: block.id.clone(),
        confidence: Confidence(100),
    }
}
```

#### Step 3: Handle Terminators in High IR

```rust
// In high_ir/mod.rs
fn convert_low_ir_instructions(
    &mut self,
    instructions: &[low_ir::Instruction],
    terminator: Option<&low_ir::Terminator>,  // Add parameter
) -> Vec<Statement> {
    let mut statements = Vec::new();

    // Convert regular instructions
    for inst in instructions {
        // ... existing code
    }

    // Handle terminator
    if let Some(term) = terminator {
        match term {
            low_ir::Terminator::Return(Some((value, ty))) => {
                let expr = self.convert_low_ir_value(value);
                statements.push(Statement::Return(Some(expr)));
            }
            low_ir::Terminator::Return(None) => {
                statements.push(Statement::Return(None));
            }
            low_ir::Terminator::Branch(target) => {
                statements.push(Statement::Goto(format!("block_{:x}", target.0)));
            }
            low_ir::Terminator::CondBranch { cond, true_dest, false_dest } => {
                let cond_expr = self.convert_low_ir_value(cond);
                // Generate if-goto pattern (will be optimized later)
                statements.push(Statement::If {
                    condition: cond_expr,
                    then_branch: Box::new(Statement::Goto(format!("block_{:x}", true_dest.0))),
                    else_branch: Some(Box::new(Statement::Goto(format!("block_{:x}", false_dest.0)))),
                });
            }
            // Handle other terminators
            _ => {}
        }
    }

    statements
}
```

### Day 3-4: Fix Variable Naming

#### Step 1: Improve Name Generation

```rust
// In high_ir/mod.rs
fn convert_local_id_to_name(&mut self, local_id: &low_ir::LocalId) -> String {
    // Use purpose field directly if meaningful
    let base_name = if !local_id.purpose.is_empty() && local_id.purpose != "temp" {
        local_id.purpose.to_string()
    } else {
        // Generate context-based name
        match self.current_context {
            Context::LoopCounter => "i".to_string(),
            Context::Condition => "cond".to_string(),
            Context::Result => "result".to_string(),
            _ => format!("var_{}", self.name_gen.next_id())
        }
    };

    // Only add suffix if there are conflicts
    if self.name_conflicts.contains(&base_name) {
        format!("{}_{}", base_name, local_id.version)
    } else {
        base_name
    }
}
```

#### Step 2: Parameter Name Mapping

```rust
// In high_ir/mod.rs
fn generate_parameters(&mut self, sig: &medium_ir::FunctionSignature) -> Vec<Parameter> {
    sig.parameters
        .iter()
        .enumerate()
        .map(|(i, (hint, ty))| {
            let name = if !hint.is_empty() && hint != "param" {
                hint.clone()
            } else {
                // Use common parameter names based on type and position
                match (i, ty) {
                    (0, TypeRef::Primitive(PrimitiveType::I32)) => "argc".to_string(),
                    (1, TypeRef::Pointer(_)) => "argv".to_string(),
                    _ => format!("arg{}", i)
                }
            };
            Parameter {
                name,
                ty: self.type_inference.convert_type(ty),
            }
        })
        .collect()
}
```

### Day 5: Integration Testing

#### Test Cases

```rust
// tests/c_generation_fixes.rs

#[test]
fn test_simple_return() {
    // Input: function that adds two numbers and returns
    let c_code = generate_c_from_binary(SIMPLE_ADD_BINARY);

    assert!(c_code.contains("return result;"));
    assert!(c_code.contains("int add(int a, int b)"));
    assert!(!c_code.contains("param_0"));
}

#[test]
fn test_conditional_return() {
    // Input: function with if-else and multiple returns
    let c_code = generate_c_from_binary(CONDITIONAL_BINARY);

    assert!(c_code.contains("if (x > 0)"));
    assert!(c_code.contains("return 1;"));
    assert!(c_code.contains("return 0;"));
}

#[test]
fn test_loop_variables() {
    // Input: for loop
    let c_code = generate_c_from_binary(FOR_LOOP_BINARY);

    assert!(c_code.contains("for (i = 0; i < 10; i++)"));
    assert!(!c_code.contains("i_0"));
}
```

## Phase 2: Pattern Recognition Enhancement (Week 2)

### Improve Medium IR Pattern Detection

1. Include terminators in pattern analysis
2. Create compound patterns for common idioms
3. Add confidence scoring based on pattern completeness

### Pattern Database Design

```yaml
# patterns/loops.yaml
patterns:
  - name: "for_loop_increment"
    match:
      init: "assign(var, const)"
      condition: "compare(var, limit)"
      update: "add(var, 1)"
      terminator: "cond_branch"
    confidence: 90

  - name: "while_loop"
    match:
      condition: "compare(var, const)"
      body: "any"
      terminator: "cond_branch(back_edge)"
    confidence: 85
```

## Phase 3: Advanced Improvements (Week 3)

### Type Recovery

- Analyze usage patterns to infer types
- Track pointer arithmetic for array detection
- Identify struct field accesses

### Expression Optimization

- Remove redundant parentheses
- Simplify boolean expressions
- Fold constants

### Control Flow Optimization

- Convert if-goto patterns to structured control flow
- Detect and simplify loop patterns
- Remove unreachable code

## Success Metrics

1. **Correctness**
    - [ ] All test cases produce compilable C code
    - [ ] No missing return statements
    - [ ] Proper control flow structure

2. **Readability**
    - [ ] Meaningful variable names (no `param_0`, `result_0`)
    - [ ] Proper indentation and formatting
    - [ ] Minimal redundant code

3. **Performance**
    - [ ] < 100ms for typical functions
    - [ ] Memory usage proportional to function size
    - [ ] Deterministic output

## Testing Strategy

### Unit Tests

- Test each component in isolation
- Cover all terminator types
- Test name generation edge cases

### Integration Tests

- Full pipeline from binary to C
- Various control flow patterns
- Real-world binaries

### Golden Tests

```bash
# Create golden output files
for test in tests/binaries/*.exe; do
    fireman decompile $test > tests/golden/$(basename $test).c
done

# Verify against golden files
for test in tests/binaries/*.exe; do
    output=$(fireman decompile $test)
    golden=$(cat tests/golden/$(basename $test).c)
    assert_equal "$output" "$golden"
done
```

## Implementation Order

1. **Week 1**: Fix critical bugs (terminators, names)
2. **Week 2**: Enhance patterns and recognition
3. **Week 3**: Optimize output quality
4. **Week 4**: Performance and polish

This systematic approach ensures we fix the most critical issues first while building toward a robust, production-ready
decompiler.
