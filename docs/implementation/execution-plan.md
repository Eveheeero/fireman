# C Code Generation Fix - Execution Plan

## Week 1: Critical Fixes

### Monday-Tuesday: Terminator Handling

#### Morning: Add Terminator to Pattern

```bash
# Files to modify:
fireball/src/ir/medium_ir/mod.rs       # Add terminator field to Pattern::LowIR
fireball/src/ir/medium_ir/analyzer.rs  # Populate terminator when creating patterns
```

**Code Changes:**

1. In `medium_ir/mod.rs` line ~50:

```rust
LowIR {
    instructions: Vec<low_ir::Instruction>,
    terminator: Option<low_ir::Terminator>,  // ADD
    source_block: low_ir::BlockId,            // ADD
    confidence: Confidence,
}
```

2. In `medium_ir/analyzer.rs` find where `Pattern::LowIR` is created:

```rust
// Look for pattern creation, add:
terminator: Some(block.terminator.clone()),
source_block: block.id.clone(),
```

#### Afternoon: Handle Terminator in High IR

```bash
# Files to modify:
fireball/src/ir/high_ir/mod.rs  # Add terminator conversion
```

**Add new method after line ~880:**

```rust
fn convert_terminator(&mut self, terminator: &low_ir::Terminator) -> Option<Statement> {
    match terminator {
        low_ir::Terminator::Return(None) => Some(Statement::Return(None)),
        low_ir::Terminator::Return(Some((value, _))) => {
            let expr = self.convert_low_ir_value(value);
            Some(Statement::Return(Some(expr)))
        }
        _ => None
    }
}
```

**Update `convert_pattern_to_statements` at line ~731:**

```rust
Pattern::LowIR { instructions, terminator, .. } => {
    let mut stmts = self.convert_low_ir_instructions(instructions);
    if let Some(term) = terminator {
        if let Some(stmt) = self.convert_terminator(term) {
            stmts.push(stmt);
        }
    }
    stmts
}
```

#### Test & Verify

```bash
# Run existing tests
cargo test test_simple_function_codegen --test c_codegen_simple -- --nocapture

# Expected: Should now see "return result;" in output
```

### Wednesday-Thursday: Variable Naming

#### Morning: Fix Local Variable Names

```bash
# Files to modify:
fireball/src/ir/high_ir/mod.rs  # Improve convert_local_id_to_name
```

**Replace method at line ~909:**

```rust
fn convert_local_id_to_name(&mut self, local_id: &low_ir::LocalId) -> String {
    // Clean, simple names when possible
    if !local_id.purpose.is_empty() && local_id.purpose != "temp" {
        if local_id.version == 0 {
            return local_id.purpose.to_string();
        } else {
            return format!("{}_{}", local_id.purpose, local_id.version);
        }
    }

    // Fallback
    format!("var_{}", self.temp_counter)
}
```

#### Afternoon: Fix Parameter Names

```bash
# Files to modify:
fireball/src/ir/high_ir/mod.rs  # Fix generate_parameters
```

**Update method at line ~493:**

```rust
fn generate_parameters(&mut self, sig: &medium_ir::FunctionSignature) -> Vec<Parameter> {
    sig.parameters
        .iter()
        .enumerate()
        .map(|(i, (hint, ty))| {
            // Better parameter names
            let name = match (i, hint.as_str()) {
                (_, h) if !h.is_empty() && h != "param" => h.to_string(),
                (0, _) => "a".to_string(),
                (1, _) => "b".to_string(),
                (2, _) => "c".to_string(),
                _ => format!("arg{}", i)
            };
            Parameter {
                name,
                ty: self.type_inference.convert_type(ty),
            }
        })
        .collect()
}
```

### Friday: Integration & Testing

#### Create Comprehensive Tests

```rust
// New file: fireball/tests/c_generation_comprehensive.rs

#[test]
fn test_return_statement_generation() {
    let module = create_add_function();
    let high_module = generate_high_ir(module);
    let c_code = generate_c_code(&high_module);

    // Must have return
    assert!(c_code.contains("return result;"));
    // No generic names
    assert!(!c_code.contains("param_0"));
    assert!(!c_code.contains("result_0"));
}

#[test]
fn test_parameter_names() {
    let c_code = decompile_simple_function();
    assert!(c_code.contains("(int a, int b)"));
}

#[test]
fn test_compilable_output() {
    let c_code = decompile_test_binary();
    // Write to temp file and compile
    std::fs::write("/tmp/test.c", c_code).unwrap();
    let output = std::process::Command::new("gcc")
        .args(&["-c", "/tmp/test.c", "-o", "/tmp/test.o"])
        .output()
        .unwrap();
    assert!(output.status.success(), "C code must compile");
}
```

## Week 2: Pattern Enhancement

### Monday: Array Access Pattern

```rust
// In medium_ir/patterns.rs
fn detect_array_access(block: &BasicBlock) -> Option<Pattern> {
    // Look for: base + (index * element_size)
    // Generate: ArrayAccess pattern
}
```

### Tuesday-Wednesday: Pattern Database

```yaml
# patterns/common.yaml
patterns:
  - name: array_access
    match: "add(ptr, mul(index, size))"
    confidence: 85

  - name: strlen_pattern
    match: "loop(inc(ptr), cmp(*ptr, 0))"
    confidence: 90
```

### Thursday-Friday: Testing & Optimization

## Week 3: Quality & Performance

### Focus Areas:

1. Expression simplification
2. Dead code elimination
3. Control flow optimization
4. Performance benchmarking

## Success Criteria Checklist

### Week 1 Completion:

- [ ] All tests generate `return` statements
- [ ] No more `param_0` or `result_0` names
- [ ] Generated C compiles with gcc/clang
- [ ] All existing tests still pass

### Week 2 Completion:

- [ ] Array access patterns detected
- [ ] Pattern database integrated
- [ ] 90%+ pattern recognition accuracy

### Week 3 Completion:

- [ ] Clean, readable C output
- [ ] < 100ms generation time
- [ ] Deterministic output verified

## Daily Standup Questions

1. What was completed yesterday?
2. What will be done today?
3. Are there any blockers?
4. Do the tests pass?

## Emergency Fallback

If critical issues arise:

1. Revert to last working commit
2. Implement minimal fix only
3. Add regression test
4. Document limitation

This plan ensures steady progress with daily deliverables and clear success metrics.
