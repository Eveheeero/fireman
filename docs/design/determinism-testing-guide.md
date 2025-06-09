# Determinism Testing Guide for Fireman

## The Golden Rule

**Same Assembly Bytes at Same Addresses = Same IR Output (ALWAYS!)**

This is not a nice-to-have feature - it's a fundamental requirement. Any violation is a critical bug.

## Why Determinism Matters

1. **Reproducible Bugs**: Users can share binaries knowing you'll see the same IR
2. **Regression Testing**: Changes that alter IR for existing binaries are immediately caught
3. **Caching**: Can cache IR based on content hash
4. **Collaboration**: Team members see identical results
5. **Trust**: Users rely on consistent behavior

## Testing Strategy

### Level 1: Unit Tests for Determinism

```rust
#[cfg(test)]
mod determinism_unit_tests {
    use super::*;
    
    /// Test single instruction determinism
    #[test]
    fn test_single_instruction_determinism() {
        // Test each instruction type multiple times
        let test_cases = vec![
            // MOV variants
            X86Inst::mov_reg_reg(0x401000, Reg::RAX, Reg::RBX),
            X86Inst::mov_reg_imm(0x401003, Reg::RAX, 0x42),
            X86Inst::mov_reg_mem(0x401006, Reg::RAX, Mem::base_disp(Reg::RBX, 8)),
            X86Inst::mov_mem_reg(0x401009, Mem::base_disp(Reg::RBX, 8), Reg::RAX),
            
            // Arithmetic
            X86Inst::add_reg_reg(0x40100C, Reg::RAX, Reg::RBX),
            X86Inst::sub_reg_imm(0x40100F, Reg::RAX, 1),
            
            // Control flow
            X86Inst::jmp(0x401012, 0x401020),
            X86Inst::jz(0x401015, 0x401030),
            X86Inst::call(0x401018, 0x402000),
        ];
        
        for inst in test_cases {
            // Lift same instruction 100 times
            let mut results = Vec::new();
            
            for i in 0..100 {
                // Fresh lifter each time
                let mut lifter = X86Lifter::new();
                
                // Pollute state differently
                for j in 0..i {
                    lifter.temp_alloc.new_temp(Address(j as u64), "pollution");
                }
                
                // Lift instruction
                let ir = lifter.lift_instruction(&inst);
                
                // Serialize for exact comparison
                let bytes = bincode::serialize(&ir).unwrap();
                results.push(bytes);
            }
            
            // All results must be identical
            let first = &results[0];
            for (i, result) in results.iter().enumerate() {
                assert_eq!(first, result, 
                    "Instruction {:?} produced different IR on iteration {}", inst, i);
            }
        }
    }
}
```

### Level 2: Function-Level Determinism

```rust
#[test]
fn test_function_determinism() {
    struct TestFunction {
        name: &'static str,
        bytes: Vec<u8>,
        entry: Address,
    }
    
    let test_functions = vec![
        TestFunction {
            name: "simple_add",
            bytes: vec![
                0x55,              // push rbp
                0x48, 0x89, 0xe5,  // mov rbp, rsp
                0x89, 0xf8,        // mov eax, edi
                0x01, 0xf0,        // add eax, esi
                0x5d,              // pop rbp
                0xc3,              // ret
            ],
            entry: Address(0x401000),
        },
        TestFunction {
            name: "loop_function",
            bytes: vec![
                0x31, 0xc0,        // xor eax, eax
                0x31, 0xc9,        // xor ecx, ecx
                0x39, 0xf9,        // cmp ecx, edi
                0x7d, 0x06,        // jge +6
                0x01, 0xc8,        // add eax, ecx
                0xff, 0xc1,        // inc ecx
                0xeb, 0xf6,        // jmp -10
                0xc3,              // ret
            ],
            entry: Address(0x402000),
        },
    ];
    
    for func in test_functions {
        let mut hashes = Vec::new();
        
        // Generate IR 100 times under different conditions
        for i in 0..100 {
            // Vary memory state
            let _garbage: Vec<Vec<u8>> = (0..i)
                .map(|x| vec![x as u8; (x * 7) % 100])
                .collect();
            
            // Create binary
            let binary = Binary {
                bytes: func.bytes.clone(),
                base: func.entry,
            };
            
            // Full decompilation
            let decompiler = Decompiler::new();
            let ir = decompiler.decompile_function(&binary, func.entry);
            
            // Hash the result
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(&bincode::serialize(&ir).unwrap());
            let hash = hasher.finalize();
            
            hashes.push(hash);
        }
        
        // All hashes must be identical
        let first = &hashes[0];
        for (i, hash) in hashes.iter().enumerate() {
            assert_eq!(first, hash,
                "Function '{}' produced different IR on run {}", func.name, i);
        }
    }
}
```

### Level 3: Whole-Program Determinism

```rust
#[test]
fn test_whole_program_determinism() {
    let test_binaries = vec![
        ("hello_world", include_bytes!("../testdata/hello_world.exe")),
        ("calculator", include_bytes!("../testdata/calc.exe")),
        ("malware_sample", include_bytes!("../testdata/wannacry_safe.exe")),
    ];
    
    for (name, binary_data) in test_binaries {
        println!("Testing determinism for: {}", name);
        
        // Run 10 times serially
        let serial_results: Vec<_> = (0..10)
            .map(|_| decompile_full(binary_data))
            .collect();
        
        // Run 10 times in parallel
        let parallel_results: Vec<_> = (0..10)
            .into_par_iter()
            .map(|_| decompile_full(binary_data))
            .collect();
        
        // All must be identical
        let first = &serial_results[0];
        
        for (i, result) in serial_results.iter().enumerate() {
            assert_eq!(first, result, "{}: Serial run {} differs", name, i);
        }
        
        for (i, result) in parallel_results.iter().enumerate() {
            assert_eq!(first, result, "{}: Parallel run {} differs", name, i);
        }
    }
}

fn decompile_full(binary: &[u8]) -> Vec<u8> {
    let decompiler = Decompiler::new();
    let ir = decompiler.decompile_binary(binary);
    bincode::serialize(&ir).unwrap()
}
```

### Level 4: Stress Testing Determinism

```rust
#[test]
fn test_determinism_under_stress() {
    let binary = include_bytes!("../testdata/large_binary.exe");
    
    // Test under different conditions
    let conditions = vec![
        "normal",
        "low_memory",
        "high_cpu_load",
        "many_threads",
        "after_gc",
    ];
    
    let mut results = HashMap::new();
    
    for condition in conditions {
        let ir_bytes = match condition {
            "normal" => {
                decompile_full(binary)
            }
            
            "low_memory" => {
                // Allocate most memory
                let _bloat: Vec<_> = (0..1000)
                    .map(|_| vec![0u8; 1_000_000])
                    .collect();
                decompile_full(binary)
            }
            
            "high_cpu_load" => {
                // Spawn CPU-intensive threads
                let handles: Vec<_> = (0..num_cpus::get())
                    .map(|_| {
                        thread::spawn(|| {
                            while !STOP.load(Ordering::Relaxed) {
                                std::hint::spin_loop();
                            }
                        })
                    })
                    .collect();
                
                let result = decompile_full(binary);
                STOP.store(true, Ordering::Relaxed);
                for h in handles { h.join().unwrap(); }
                result
            }
            
            "many_threads" => {
                // Many concurrent decompilatons
                let handles: Vec<_> = (0..100)
                    .map(|_| {
                        thread::spawn(move || {
                            decompile_full(include_bytes!("../testdata/small.exe"))
                        })
                    })
                    .collect();
                
                let result = decompile_full(binary);
                for h in handles { h.join().unwrap(); }
                result
            }
            
            "after_gc" => {
                // Force allocator churn
                for _ in 0..1000 {
                    let _temp: Vec<_> = (0..10000)
                        .map(|i| Box::new(i))
                        .collect();
                }
                decompile_full(binary)
            }
            
            _ => unreachable!(),
        };
        
        results.insert(condition, ir_bytes);
    }
    
    // All conditions must produce identical results
    let first = &results["normal"];
    for (condition, result) in &results {
        assert_eq!(first.len(), result.len(), 
            "Different size under condition: {}", condition);
        assert_eq!(first, result,
            "Different IR under condition: {}", condition);
    }
}
```

### Level 5: Cross-Platform Determinism

```rust
#[test]
#[cfg(not(target_env = "msvc"))]  // Skip on Windows CI
fn test_cross_platform_determinism() {
    use std::process::Command;
    
    let binary = include_bytes!("../testdata/cross_platform.exe");
    
    // Save binary to temp file
    let temp_path = "/tmp/test_binary.exe";
    std::fs::write(temp_path, binary).unwrap();
    
    // Run on different platforms via Docker
    let platforms = vec![
        ("linux/amd64", "rust:latest"),
        ("linux/arm64", "rust:latest"),  
        ("linux/386", "i386/rust:latest"),
    ];
    
    let mut results = HashMap::new();
    
    for (platform, image) in platforms {
        println!("Testing on platform: {}", platform);
        
        let output = Command::new("docker")
            .args(&[
                "run", "--rm",
                "--platform", platform,
                "-v", &format!("{}:{}", temp_path, temp_path),
                "-v", &format!("{}:/app", env!("CARGO_MANIFEST_DIR")),
                image,
                "cargo", "test", "--", "decompile_binary_test",
            ])
            .output()
            .expect("Failed to run docker");
            
        assert!(output.status.success(), 
            "Failed on {}: {}", platform, String::from_utf8_lossy(&output.stderr));
            
        // Extract IR hash from output
        let output_str = String::from_utf8_lossy(&output.stdout);
        let hash = extract_ir_hash(&output_str);
        results.insert(platform, hash);
    }
    
    // All platforms must produce same IR
    let first = results.values().next().unwrap();
    for (platform, hash) in &results {
        assert_eq!(first, hash, "Platform {} produced different IR", platform);
    }
}
```

## Common Determinism Bugs to Test For

### 1. HashMap Iteration

```rust
#[test]
fn test_no_hashmap_in_ir_path() {
    // This test uses proc macros to check that HashMap is not used
    check_no_hashmap_usage!("src/ir/");
}
```

### 2. Time-Based Behavior

```rust
#[test]
fn test_no_time_dependency() {
    let binary = include_bytes!("../testdata/simple.exe");
    
    // Decompile at different times
    let result1 = decompile_full(binary);
    
    // Sleep to ensure time changes
    thread::sleep(Duration::from_secs(2));
    
    let result2 = decompile_full(binary);
    
    assert_eq!(result1, result2, "IR depends on system time!");
}
```

### 3. Thread-Local State

```rust
#[test]
fn test_no_thread_local_state() {
    let binary = include_bytes!("../testdata/simple.exe");
    
    // Run in different threads
    let handle1 = thread::spawn(|| decompile_full(binary));
    let handle2 = thread::spawn(|| decompile_full(binary));
    
    let result1 = handle1.join().unwrap();
    let result2 = handle2.join().unwrap();
    
    assert_eq!(result1, result2, "IR depends on thread-local state!");
}
```

## Continuous Integration Setup

```yaml
# .github/workflows/determinism.yml
name: Determinism Tests

on: [push, pull_request]

jobs:
  determinism:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        
    steps:
    - uses: actions/checkout@v2
    
    - name: Run determinism tests
      run: |
        cargo test --test determinism -- --nocapture
        
    - name: Run determinism benchmarks
      run: |
        cargo bench --bench determinism_bench
        
    - name: Check for non-deterministic code patterns
      run: |
        # Grep for HashMap usage in core paths
        ! grep -r "HashMap" src/ir/ src/lifting/ src/core/
        
        # Check for time/random usage
        ! grep -r "SystemTime\|Instant::now\|rand::" src/
        
        # Check for thread_local usage
        ! grep -r "thread_local!" src/
```

## Performance Impact of Determinism

```rust
#[bench]
fn bench_deterministic_vs_fast(b: &mut Bencher) {
    let binary = include_bytes!("../testdata/bench.exe");
    
    // Benchmark deterministic version
    let det_time = b.iter(|| {
        let decompiler = Decompiler::new_deterministic();
        decompiler.decompile_binary(binary)
    });
    
    // Benchmark "fast" version (if it existed)
    let fast_time = b.iter(|| {
        let decompiler = Decompiler::new_fast();  // Uses HashMap, etc
        decompiler.decompile_binary(binary)
    });
    
    // Determinism should not cost more than 10%
    assert!(det_time < fast_time * 1.1, 
        "Determinism costs too much: {}% overhead", 
        (det_time - fast_time) * 100.0 / fast_time);
}
```

## Debugging Non-Determinism

When a determinism test fails:

1. **Binary diff the IR outputs**

```bash
# Save two runs
cargo run -- decompile binary.exe > run1.ir
cargo run -- decompile binary.exe > run2.ir

# Find first difference
diff run1.ir run2.ir | head -20
```

2. **Add logging to find divergence**

```rust
// Temporary debugging code
println!("DETERMINISM_DEBUG: {} at {:016x}", temp_name, addr);
```

3. **Check for common issues**

- HashMap/HashSet usage
- Global state/counters
- Thread-local storage
- Time-based decisions
- Random number usage
- Pointer address printing

4. **Minimize the test case**

```rust
// Find smallest input that shows non-determinism
for size in [1, 10, 100, 1000] {
    let small_binary = &full_binary[..size.min(full_binary.len())];
    if !is_deterministic(small_binary) {
        println!("Non-determinism with {} bytes", size);
        break;
    }
}
```

## Remember

**Determinism is not optional!** It's a core feature that users depend on. Any PR that breaks determinism should be
immediately reverted until fixed.