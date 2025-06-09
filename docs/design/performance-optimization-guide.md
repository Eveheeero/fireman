# Performance Optimization Guide for Fireman Decompiler

## Performance Goals

- **PE Parsing**: <10ms for typical executables
- **CFG Construction**: 10,000+ blocks/second
- **IR Generation**: 5,000+ instructions/second
- **Memory Usage**: <20GB for 1GB binary (with streaming)
- **Parallelism**: Near-linear scaling up to 32 cores

## Zero-Copy Architecture

### Memory-Mapped Binary Access

```rust
pub struct ZeroCopyBinaryView {
    /// Memory-mapped file - no allocation
    mmap: Mmap,
    
    /// Views into mmap - no copies
    sections: BTreeMap<Address, SectionView<'_>>,
}

impl ZeroCopyBinaryView {
    pub fn new(path: &Path) -> Result<Self> {
        // Open file for memory mapping
        let file = File::open(path)?;
        let mmap = unsafe { 
            MmapOptions::new()
                .populate() // Pre-fault pages
                .map(&file)?
        };
        
        // Parse headers without copying
        let sections = Self::parse_sections_zero_copy(&mmap)?;
        
        Ok(Self { mmap, sections })
    }
    
    /// Zero-copy section access
    pub fn section_bytes(&self, addr: Address) -> &[u8] {
        let section = &self.sections[&addr];
        &self.mmap[section.offset..section.offset + section.size]
    }
}

/// Instruction iteration without allocation
pub struct ZeroCopyInstructionIter<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Iterator for ZeroCopyInstructionIter<'a> {
    type Item = InstructionView<'a>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.data.len() {
            return None;
        }
        
        // Decode in-place, no allocation
        let (insn, len) = decode_instruction_view(&self.data[self.offset..])?;
        self.offset += len;
        
        Some(insn)
    }
}
```

### Arena Allocation Strategy

```rust
pub struct ArenaAllocator {
    /// Per-function arenas
    arenas: Vec<TypedArena<IRNode>>,
    
    /// Current arena index
    current: usize,
}

impl ArenaAllocator {
    /// Allocate all IR for a function in one arena
    pub fn allocate_function<'a>(&'a self, nodes: Vec<IRNode>) -> &'a [IRNode] {
        let arena = &self.arenas[self.current];
        
        // Bulk allocate all nodes
        let slice = arena.alloc_slice(&nodes);
        
        // Arena dropped after function processing
        slice
    }
}

/// Example usage in decompiler
pub struct FunctionLifter<'arena> {
    arena: &'arena ArenaAllocator,
    
    pub fn lift(&self, func: &Function) -> &'arena [IRNode] {
        let mut nodes = Vec::with_capacity(func.instruction_count() * 2);
        
        // Build IR nodes
        for insn in func.instructions() {
            nodes.extend(self.lift_instruction(insn));
        }
        
        // Single arena allocation
        self.arena.allocate_function(nodes)
    }
}
```

## Parallel Analysis

### Work-Stealing Function Analysis

```rust
use rayon::prelude::*;
use crossbeam::channel::{bounded, Sender, Receiver};

pub struct ParallelAnalyzer {
    /// Thread pool for CPU-bound work
    pool: ThreadPool,
    
    /// Work-stealing queue
    work_queue: deque::Stealer<FunctionWork>,
}

impl ParallelAnalyzer {
    pub fn analyze_functions(&self, functions: Vec<Function>) -> Vec<AnalysisResult> {
        // Sort by size for better load balancing
        let mut functions = functions;
        functions.sort_by_key(|f| std::cmp::Reverse(f.size()));
        
        // Process in parallel with work-stealing
        functions.par_iter()
            .map(|func| {
                // Each thread processes independently
                let mut analyzer = LocalAnalyzer::new();
                analyzer.analyze(func)
            })
            .collect()
    }
}

/// Lock-free result collection
pub struct LockFreeCollector<T> {
    results: DashMap<Address, T>,
}

impl<T> LockFreeCollector<T> {
    pub fn insert(&self, addr: Address, result: T) {
        // Lock-free insertion
        self.results.insert(addr, result);
    }
    
    pub fn into_sorted_vec(self) -> Vec<(Address, T)> {
        // Extract and sort for determinism
        let mut results: Vec<_> = self.results.into_iter().collect();
        results.sort_by_key(|(addr, _)| *addr);
        results
    }
}
```

### Parallel Basic Block Construction

```rust
pub struct ParallelCFGBuilder {
    /// Concurrent block discovery
    discovered: DashSet<Address>,
    
    /// Lock-free block storage
    blocks: DashMap<Address, BasicBlock>,
}

impl ParallelCFGBuilder {
    pub fn build_parallel(&self, entries: Vec<Address>) -> ControlFlowGraph {
        // Phase 1: Parallel block discovery
        entries.par_iter().for_each(|&entry| {
            self.discover_blocks_from(entry);
        });
        
        // Phase 2: Parallel edge construction
        self.blocks.par_iter().for_each(|entry| {
            let (addr, block) = entry.pair();
            self.analyze_block_edges(*addr, block);
        });
        
        // Phase 3: Deterministic collection
        let blocks: BTreeMap<_, _> = self.blocks
            .into_iter()
            .collect();
            
        ControlFlowGraph { blocks }
    }
    
    fn discover_blocks_from(&self, start: Address) {
        let mut stack = vec![start];
        
        while let Some(addr) = stack.pop() {
            // Check if already discovered (lock-free)
            if !self.discovered.insert(addr) {
                continue;
            }
            
            // Analyze block
            let block = self.analyze_block_at(addr);
            
            // Add successors to work
            for succ in block.successors() {
                stack.push(succ);
            }
            
            // Store block (lock-free)
            self.blocks.insert(addr, block);
        }
    }
}
```

## SIMD Acceleration

### Pattern Matching with SIMD

```rust
use std::simd::*;

pub struct SimdPatternMatcher {
    /// Common byte patterns
    patterns: Vec<Simd<u8, 32>>,
}

impl SimdPatternMatcher {
    /// Find NOP sleds, padding, etc.
    pub fn find_patterns(&self, data: &[u8]) -> Vec<PatternMatch> {
        let mut matches = Vec::new();
        
        // Process 32 bytes at a time
        for (offset, chunk) in data.chunks_exact(32).enumerate() {
            let chunk_simd = Simd::from_slice(chunk);
            
            // Check against all patterns in parallel
            for (idx, pattern) in self.patterns.iter().enumerate() {
                let eq_mask = chunk_simd.simd_eq(*pattern);
                
                if eq_mask.any() {
                    matches.push(PatternMatch {
                        offset: offset * 32,
                        pattern_id: idx,
                        mask: eq_mask.to_bitmask(),
                    });
                }
            }
        }
        
        matches
    }
}

/// SIMD-accelerated constant extraction
pub fn extract_constants_simd(data: &[u8]) -> Vec<u64> {
    let mut constants = Vec::new();
    
    // Process 8 bytes at a time as potential constants
    for chunk in data.chunks_exact(8) {
        let value = u64::from_le_bytes(chunk.try_into().unwrap());
        
        // Heuristic: likely constant if in certain ranges
        if is_likely_constant(value) {
            constants.push(value);
        }
    }
    
    // Deduplicate while maintaining order
    constants.sort_unstable();
    constants.dedup();
    constants
}
```

### SIMD Disassembly Optimization

```rust
pub struct SimdDisassembler {
    /// Instruction prefix detection
    pub fn find_instruction_boundaries(&self, data: &[u8]) -> Vec<usize> {
        let mut boundaries = Vec::new();
        
        // Common x86-64 instruction prefixes
        let prefix_mask = u8x32::from_array([
            0x48, 0x66, 0x67, 0xF0, 0xF2, 0xF3, // REX, size, addr, LOCK, REPNE, REP
            0x2E, 0x36, 0x3E, 0x26, 0x64, 0x65, // Segment overrides
            0x0F, 0xFF, 0x90, 0xC3, 0xE8, 0xE9, // Common opcodes
            // ... more patterns
        ]);
        
        for (offset, window) in data.windows(32).enumerate() {
            let chunk = u8x32::from_slice(window);
            
            // Check for any instruction start patterns
            for i in 0..32 {
                let byte = chunk[i];
                if prefix_mask.as_array().contains(&byte) {
                    boundaries.push(offset + i);
                }
            }
        }
        
        boundaries
    }
}
```

## Memory Optimization

### Streaming Large Binaries

```rust
pub struct StreamingAnalyzer {
    /// Process in chunks to limit memory
    chunk_size: usize,
    
    /// Intermediate results
    results: IntermediateResults,
}

impl StreamingAnalyzer {
    pub fn analyze_large_binary(&mut self, path: &Path) -> Result<FinalResult> {
        let file = File::open(path)?;
        let file_size = file.metadata()?.len();
        
        // Process in chunks
        for chunk_start in (0..file_size).step_by(self.chunk_size) {
            let chunk = self.read_chunk(&file, chunk_start)?;
            
            // Analyze chunk
            let chunk_result = self.analyze_chunk(&chunk);
            
            // Merge results
            self.results.merge(chunk_result);
            
            // Free chunk memory
            drop(chunk);
        }
        
        Ok(self.results.finalize())
    }
}

/// Compressed intermediate storage
pub struct CompressedIR {
    /// LZ4-compressed IR blocks
    compressed_blocks: Vec<CompressedBlock>,
    
    /// Index for quick access
    block_index: BTreeMap<Address, usize>,
}

impl CompressedIR {
    pub fn get_block(&self, addr: Address) -> Option<Vec<IRStatement>> {
        let idx = self.block_index.get(&addr)?;
        let compressed = &self.compressed_blocks[*idx];
        
        // Decompress on demand
        Some(lz4::decompress(&compressed.data).ok()?)
    }
}
```

### Copy-on-Write Optimization

```rust
use std::borrow::Cow;

pub struct CowIR<'a> {
    /// Shared immutable data
    statements: Cow<'a, [IRStatement]>,
    
    /// Only clone when modified
    pub fn optimize(&mut self) -> bool {
        // Check if optimization needed
        if !self.needs_optimization() {
            return false;
        }
        
        // Now we need to modify - clone
        let statements = self.statements.to_mut();
        
        // Apply optimizations
        self.apply_optimizations(statements);
        
        true
    }
}
```

## Cache Optimization

### CPU Cache-Friendly Layouts

```rust
/// Align to cache lines
#[repr(align(64))]
pub struct CacheAlignedBlock {
    /// Hot data together
    address: Address,
    size: u32,
    flags: u32,
    
    /// Frequently accessed together
    predecessors: ArrayVec<Address, 4>,
    successors: ArrayVec<Address, 4>,
    
    /// Cold data at end
    _padding: [u8; 16],
    metadata: Box<BlockMetadata>,
}

/// Structure-of-Arrays for better cache usage
pub struct SoAInstructions {
    /// All addresses contiguous
    addresses: Vec<Address>,
    
    /// All opcodes contiguous
    opcodes: Vec<u8>,
    
    /// All operands contiguous
    operands: Vec<Operands>,
}

impl SoAInstructions {
    /// Cache-friendly iteration
    pub fn iter_addresses(&self) -> &[Address] {
        &self.addresses // All in cache
    }
}
```

### Prefetching Strategies

```rust
use std::intrinsics::prefetch_read_data;

pub struct PrefetchingAnalyzer {
    pub unsafe fn analyze_with_prefetch(&self, blocks: &[BasicBlock]) {
        for i in 0..blocks.len() {
            // Prefetch next block while processing current
            if i + 1 < blocks.len() {
                prefetch_read_data(&blocks[i + 1], 3);
            }
            
            // Process current block
            self.analyze_block(&blocks[i]);
        }
    }
}
```

## Profile-Guided Optimization

### Hot Path Detection

```rust
pub struct ProfileData {
    /// Function call counts
    function_hits: BTreeMap<Address, u64>,
    
    /// Block execution counts
    block_hits: BTreeMap<Address, u64>,
}

impl ProfileData {
    pub fn optimize_hot_paths(&self, ir: &mut IR) {
        // Find hot functions (top 20%)
        let mut functions: Vec<_> = self.function_hits.iter().collect();
        functions.sort_by_key(|(_, &count)| std::cmp::Reverse(count));
        
        let hot_threshold = functions.len() / 5;
        let hot_functions: BTreeSet<_> = functions[..hot_threshold]
            .iter()
            .map(|(&addr, _)| addr)
            .collect();
            
        // Apply aggressive optimizations to hot paths
        for func_addr in hot_functions {
            ir.apply_aggressive_opts(func_addr);
        }
    }
}
```

## Memory Pool Management

### Custom Allocators

```rust
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Pool for small frequent allocations
pub struct SmallObjectPool {
    /// Pre-allocated chunks
    chunks: Vec<Box<[u8; 4096]>>,
    
    /// Free list
    free: Vec<*mut u8>,
}

impl SmallObjectPool {
    pub fn allocate<T>(&mut self) -> &mut T {
        let size = std::mem::size_of::<T>();
        assert!(size <= 64); // Small objects only
        
        if let Some(ptr) = self.free.pop() {
            unsafe { &mut *(ptr as *mut T) }
        } else {
            self.grow();
            self.allocate()
        }
    }
}
```

## Benchmarking Infrastructure

### Micro-benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_instruction_decode(c: &mut Criterion) {
    let data = include_bytes!("../testdata/complex.bin");
    
    c.bench_function("decode_instruction", |b| {
        b.iter(|| {
            for chunk in data.chunks(16) {
                black_box(decode_instruction(chunk));
            }
        })
    });
}

fn bench_parallel_cfg(c: &mut Criterion) {
    let mut group = c.benchmark_group("cfg_construction");
    
    for threads in [1, 2, 4, 8, 16] {
        group.bench_with_input(
            BenchmarkId::from_parameter(threads),
            &threads,
            |b, &threads| {
                let pool = ThreadPool::new(threads);
                b.iter(|| {
                    black_box(build_cfg_parallel(&pool, &test_binary));
                })
            }
        );
    }
}

criterion_group!(benches, bench_instruction_decode, bench_parallel_cfg);
criterion_main!(benches);
```

### Performance Monitoring

```rust
pub struct PerformanceMonitor {
    /// Track operation timings
    timings: BTreeMap<&'static str, Vec<Duration>>,
    
    /// Memory usage tracking
    peak_memory: AtomicU64,
}

impl PerformanceMonitor {
    pub fn time_operation<T>(&mut self, name: &'static str, f: impl FnOnce() -> T) -> T {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        
        self.timings.entry(name)
            .or_default()
            .push(duration);
            
        result
    }
    
    pub fn report(&self) -> PerformanceReport {
        PerformanceReport {
            timings: self.calculate_percentiles(),
            peak_memory: self.peak_memory.load(Ordering::Relaxed),
            throughput: self.calculate_throughput(),
        }
    }
}
```

## Configuration Tuning

```toml
[performance]
# Threading
worker_threads = 0  # 0 = number of CPUs
work_stealing = true

# Memory
max_memory_gb = 16
use_memory_pools = true
arena_size_mb = 64

# Caching  
instruction_cache_size = 100000
pattern_cache_size = 10000

# SIMD
enable_simd = true
simd_threshold = 1024  # Minimum data size for SIMD

# Streaming
chunk_size_mb = 256
compression = "lz4"  # For intermediate results
```

## Performance Checklist

- [ ] Use memory-mapped files for binary access
- [ ] Implement arena allocation for IR nodes
- [ ] Enable parallel function analysis
- [ ] Use SIMD for pattern matching
- [ ] Implement streaming for large binaries
- [ ] Align hot data to cache lines
- [ ] Profile and optimize hot paths
- [ ] Use lock-free data structures
- [ ] Batch operations where possible
- [ ] Minimize allocations in hot loops

Remember: **Measure first, optimize second**. Profile actual workloads to identify bottlenecks.