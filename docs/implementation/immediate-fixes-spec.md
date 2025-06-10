# Immediate C Generation Fixes - Technical Specification

## Overview

This document provides exact code changes needed to fix the two critical issues:

1. Missing return statements in generated C code
2. Poor variable naming (param_0, result_0, etc.)

## Fix 1: Terminator Handling

### Problem

The High IR's `convert_low_ir_instructions()` only processes the instruction array from `Pattern::LowIR`, ignoring the
terminator that contains return/branch information.

### Solution

#### 1.1 Update Pattern Definition

```rust
// File: fireball/src/ir/medium_ir/mod.rs
// Line: ~50 (in Pattern enum)

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    // ... other variants

    /// Low-level IR pattern (fallback when no high-level pattern matches)
    LowIR {
        instructions: Vec<low_ir::Instruction>,
        terminator: Option<low_ir::Terminator>,  // NEW FIELD
        source_block: low_ir::BlockId,           // NEW FIELD
        confidence: Confidence,
    },
}
```

#### 1.2 Update Pattern Creation in Analyzer

```rust
// File: fireball/src/ir/medium_ir/analyzer.rs
// Line: ~200 (in analyze_block or similar)

fn create_fallback_pattern(&self, block: &low_ir::BasicBlock) -> Pattern {
    Pattern::LowIR {
        instructions: block.instructions.clone(),
        terminator: Some(block.terminator.clone()),  // NEW
        source_block: block.id.clone(),              // NEW
        confidence: Confidence(50), // Lower confidence for fallback
    }
}
```

#### 1.3 Update High IR Conversion

```rust
// File: fireball/src/ir/high_ir/mod.rs
// Line: ~730 (in convert_pattern_to_statements)

Pattern::LowIR { instructions, terminator, .. } => {
    let mut stmts = self.convert_low_ir_instructions(instructions);

    // NEW: Handle terminator
    if let Some(term) = terminator {
        if let Some(term_stmt) = self.convert_terminator(term) {
            stmts.push(term_stmt);
        }
    }

    stmts
}
```

#### 1.4 Add Terminator Converter

```rust
// File: fireball/src/ir/high_ir/mod.rs
// Add new method after convert_low_ir_instructions

/// Convert Low IR terminator to High IR statement
fn convert_terminator(&mut self, terminator: &low_ir::Terminator) -> Option<Statement> {
    match terminator {
        low_ir::Terminator::Return(None) => {
            Some(Statement::Return(None))
        }
        low_ir::Terminator::Return(Some((value, _ty))) => {
            let expr = self.convert_low_ir_value(value);
            Some(Statement::Return(Some(expr)))
        }
        low_ir::Terminator::Branch(target) => {
            Some(Statement::Goto(format!("L_{:x}", target.0)))
        }
        low_ir::Terminator::CondBranch { cond, true_dest, false_dest } => {
            // For now, generate if-goto pattern
            // TODO: Optimize to structured control flow
            let cond_expr = self.convert_low_ir_value(cond);
            Some(Statement::If {
                condition: cond_expr,
                then_branch: Box::new(Statement::Goto(format!("L_{:x}", true_dest.0))),
                else_branch: Some(Box::new(Statement::Goto(format!("L_{:x}", false_dest.0)))),
            })
        }
        _ => None, // Handle other terminators later
    }
}
```

## Fix 2: Variable Naming

### Problem

Variable names are generated as `purpose_index` (e.g., `result_0`), and parameters use generic names like `param_0`.

### Solution

#### 2.1 Improve Variable Name Generation

```rust
// File: fireball/src/ir/high_ir/mod.rs
// Line: ~909 (in convert_local_id_to_name)

fn convert_local_id_to_name(&mut self, local_id: &low_ir::LocalId) -> String {
    // Strategy 1: Use purpose directly if it's meaningful
    if !local_id.purpose.is_empty() &&
       local_id.purpose != "temp" &&
       local_id.purpose != "unknown" {

        // For single-use variables, don't add index
        if local_id.version == 0 && !self.has_name_conflict(&local_id.purpose) {
            return local_id.purpose.to_string();
        }

        // For SSA versions, use underscore notation
        if local_id.version > 0 {
            return format!("{}_{}", local_id.purpose, local_id.version);
        }
    }

    // Strategy 2: Generate semantic names based on usage
    match self.get_variable_usage(local_id) {
        Usage::LoopCounter => return "i".to_string(),
        Usage::Condition => return "cond".to_string(),
        Usage::ArrayIndex => return "idx".to_string(),
        _ => {}
    }

    // Fallback: Generate unique name
    self.name_gen.generate_temp_name()
}

/// Check if a name would conflict with existing variables
fn has_name_conflict(&self, name: &str) -> bool {
    self.used_names.contains(name)
}
```

#### 2.2 Fix Parameter Generation

```rust
// File: fireball/src/ir/high_ir/mod.rs
// Line: ~493 (in generate_parameters)

fn generate_parameters(&mut self, sig: &medium_ir::FunctionSignature) -> Vec<Parameter> {
    sig.parameters
        .iter()
        .enumerate()
        .map(|(i, (hint, ty))| {
            let name = self.generate_parameter_name(i, hint, ty);
            self.used_names.insert(name.clone()); // Track used names
            Parameter {
                name,
                ty: self.type_inference.convert_type(ty),
            }
        })
        .collect()
}

/// Generate meaningful parameter names
fn generate_parameter_name(
    &self,
    index: usize,
    hint: &str,
    ty: &medium_ir::TypeRef
) -> String {
    // Use hint if it's not generic
    if !hint.is_empty() && !hint.starts_with("param") {
        return hint.to_string();
    }

    // Common patterns based on position and type
    match (index, ty) {
        // main() parameters
        (0, TypeRef::Primitive(PrimitiveType::I32)) if self.is_main_function() => {
            "argc".to_string()
        }
        (1, TypeRef::Pointer(inner)) if self.is_main_function() => {
            "argv".to_string()
        }
        // String parameters
        (_, TypeRef::Pointer(inner)) => {
            match &**inner {
                TypeRef::Primitive(PrimitiveType::I8) => format!("str{}", index),
                TypeRef::Primitive(PrimitiveType::U8) => format!("data{}", index),
                _ => format!("ptr{}", index)
            }
        }
        // Numeric parameters
        (_, TypeRef::Primitive(PrimitiveType::I32)) => {
            match index {
                0 => "x".to_string(),
                1 => "y".to_string(),
                2 => "z".to_string(),
                _ => format!("n{}", index)
            }
        }
        // Fallback
        _ => format!("arg{}", index)
    }
}
```

#### 2.3 Add Name Tracking

```rust
// File: fireball/src/ir/high_ir/mod.rs
// Line: ~414 (in HighIRGenerator struct)

pub struct HighIRGenerator {
    /// Variable name generator
    name_gen: NameGenerator,

    /// Type inference engine
    type_inference: TypeInference,

    /// Pattern simplifier
    simplifier: PatternSimplifier,

    /// Track used variable names to avoid conflicts
    used_names: HashSet<String>,  // NEW FIELD

    /// Current function context
    current_function: Option<String>,  // NEW FIELD
}
```

## Fix 3: C Code Generator Updates

### Update the simple C code generator to handle these improvements:

```rust
// File: fireball/src/ir/high_ir/c_codegen.rs
// Line: ~296 (in generate_expression)

hir::Expression::Variable(name) => {
    // Clean up variable names - remove unnecessary suffixes
    let clean_name = if name.ends_with("_0") && !self.has_multiple_versions(name) {
        &name[..name.len()-2]  // Remove "_0" suffix
    } else {
        name
    };
    self.write(clean_name);
}
```

## Testing

### Test Case 1: Simple Function

```rust
// Input Low IR
fn test_simple_add() {
    // BasicBlock with:
    // - Instructions: result = a + b
    // - Terminator: Return(result)

    // Expected C output:
    // int add(int a, int b) {
    //     int result = a + b;
    //     return result;
    // }
}
```

### Test Case 2: Conditional

```rust
// Input Low IR
fn test_conditional() {
    // Multiple blocks with conditional branches

    // Expected C output:
    // int max(int x, int y) {
    //     if (x > y) {
    //         return x;
    //     } else {
    //         return y;
    //     }
    // }
}
```

## Implementation Checklist

- [ ] Update Pattern enum to include terminator
- [ ] Modify analyzer to populate terminator field
- [ ] Add convert_terminator method
- [ ] Update Pattern::LowIR handling
- [ ] Improve convert_local_id_to_name
- [ ] Add generate_parameter_name
- [ ] Add name conflict tracking
- [ ] Update C code generator
- [ ] Add unit tests
- [ ] Add integration tests
- [ ] Verify all existing tests pass

## Rollout Plan

1. **Phase 1** (Day 1): Implement terminator handling
    - Most critical fix
    - Makes generated code compilable

2. **Phase 2** (Day 2): Implement variable naming
    - Improves readability
    - Makes debugging easier

3. **Phase 3** (Day 3): Testing and refinement
    - Ensure no regressions
    - Add comprehensive test coverage

This approach minimizes risk by fixing one issue at a time while maintaining backward compatibility.
