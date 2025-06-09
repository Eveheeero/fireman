# Production Decompiler Architecture

## What Really Works: Lessons from Industry Leaders

This document reveals the architecture patterns used by successful decompilers like IDA Pro, Ghidra, and Binary Ninja.

## Core Architecture Principles

### 1. Plugin-Based Everything

```rust
// Why every successful decompiler is extensible
trait DecompilerPlugin {
    fn name(&self) -> &str;
    fn version(&self) -> Version;
    
    // Hook into analysis pipeline
    fn on_function_discovered(&mut self, func: &Function) -> PluginResult;
    fn on_instruction_lifted(&mut self, addr: Address, ir: &mut IR) -> PluginResult;
    fn on_type_inferred(&mut self, var: &Variable, ty: &Type) -> PluginResult;
    
    // Custom analysis
    fn analyze(&mut self, binary: &Binary) -> Analysis;
}

struct PluginManager {
    plugins: Vec<Box<dyn DecompilerPlugin>>,
    hooks: HashMap<HookPoint, Vec<PluginId>>,
}

impl PluginManager {
    fn load_plugin(&mut self, path: &Path) -> Result<PluginId> {
        // Dynamic loading for user plugins
        unsafe {
            let lib = libloading::Library::new(path)?;
            let create: Symbol<fn() -> Box<dyn DecompilerPlugin>> = 
                lib.get(b"create_plugin")?;
            
            let plugin = create();
            let id = PluginId::new();
            self.plugins.push(plugin);
            
            Ok(id)
        }
    }
}
```

### 2. Database-Backed Storage

**Industry Secret**: All major decompilers use databases, not in-memory structures.

```rust
use rusqlite::{Connection, params};

// How IDA Pro actually stores analysis
struct AnalysisDatabase {
    conn: Connection,
}

impl AnalysisDatabase {
    fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        // Schema optimized for decompiler workloads
        conn.execute_batch("
            -- Functions table with analysis state
            CREATE TABLE IF NOT EXISTS functions (
                address INTEGER PRIMARY KEY,
                name TEXT,
                size INTEGER,
                analysis_state INTEGER,
                decompiled_code TEXT,
                last_modified INTEGER
            );
            
            -- Instructions with caching
            CREATE TABLE IF NOT EXISTS instructions (
                address INTEGER PRIMARY KEY,
                bytes BLOB,
                mnemonic TEXT,
                operands TEXT,
                ir_json TEXT,
                flags INTEGER
            );
            
            -- Cross-references (critical for navigation)
            CREATE TABLE IF NOT EXISTS xrefs (
                from_addr INTEGER,
                to_addr INTEGER,
                xref_type INTEGER,
                PRIMARY KEY (from_addr, to_addr)
            );
            CREATE INDEX idx_xrefs_to ON xrefs(to_addr);
            
            -- Comments and annotations
            CREATE TABLE IF NOT EXISTS comments (
                address INTEGER,
                type INTEGER,
                text TEXT,
                author TEXT,
                timestamp INTEGER
            );
            
            -- Type information
            CREATE TABLE IF NOT EXISTS types (
                id INTEGER PRIMARY KEY,
                name TEXT UNIQUE,
                definition TEXT,
                size INTEGER,
                category INTEGER
            );
        ")?;
        
        Ok(Self { conn })
    }
    
    fn save_function(&mut self, func: &Function) -> Result<()> {
        let tx = self.conn.transaction()?;
        
        tx.execute(
            "INSERT OR REPLACE INTO functions VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                func.address,
                func.name,
                func.size,
                func.analysis_state as i32,
                func.decompiled_code,
                chrono::Utc::now().timestamp()
            ],
        )?;
        
        // Save instructions
        for inst in &func.instructions {
            tx.execute(
                "INSERT OR REPLACE INTO instructions VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    inst.address,
                    inst.bytes,
                    inst.mnemonic,
                    serde_json::to_string(&inst.operands)?,
                    serde_json::to_string(&inst.ir)?,
                    inst.flags
                ],
            )?;
        }
        
        tx.commit()?;
        Ok(())
    }
}
```

### 3. Event-Driven Architecture

```rust
use tokio::sync::broadcast;

// How Binary Ninja handles real-time updates
#[derive(Clone, Debug)]
enum AnalysisEvent {
    FunctionDiscovered { address: Address },
    FunctionAnalyzed { address: Address },
    TypeInferred { address: Address, var: String, ty: Type },
    XrefFound { from: Address, to: Address },
    CommentAdded { address: Address, text: String },
}

struct EventDrivenDecompiler {
    event_tx: broadcast::Sender<AnalysisEvent>,
    handlers: HashMap<EventType, Vec<Box<dyn EventHandler>>>,
}

impl EventDrivenDecompiler {
    fn analyze_function(&self, addr: Address) {
        // Notify listeners about discovery
        let _ = self.event_tx.send(AnalysisEvent::FunctionDiscovered { address: addr });
        
        // Do analysis
        let function = self.lift_function(addr);
        
        // Notify about completion
        let _ = self.event_tx.send(AnalysisEvent::FunctionAnalyzed { address: addr });
        
        // UI updates automatically through event subscription
    }
}

// UI subscribes to events
struct DecompilerUI {
    event_rx: broadcast::Receiver<AnalysisEvent>,
}

impl DecompilerUI {
    async fn run(&mut self) {
        while let Ok(event) = self.event_rx.recv().await {
            match event {
                AnalysisEvent::FunctionAnalyzed { address } => {
                    self.update_function_view(address);
                }
                AnalysisEvent::TypeInferred { address, var, ty } => {
                    self.update_type_display(address, &var, &ty);
                }
                _ => {}
            }
        }
    }
}
```

### 4. Undo/Redo System

**Critical Feature**: Users make mistakes - let them undo.

```rust
// Command pattern for reversible operations
trait Command: Send + Sync {
    fn execute(&mut self, state: &mut DecompilerState) -> Result<()>;
    fn undo(&mut self, state: &mut DecompilerState) -> Result<()>;
    fn description(&self) -> String;
}

struct RenameFunction {
    address: Address,
    old_name: Option<String>,
    new_name: String,
}

impl Command for RenameFunction {
    fn execute(&mut self, state: &mut DecompilerState) -> Result<()> {
        let func = state.get_function_mut(self.address)?;
        self.old_name = Some(func.name.clone());
        func.name = self.new_name.clone();
        Ok(())
    }
    
    fn undo(&mut self, state: &mut DecompilerState) -> Result<()> {
        let func = state.get_function_mut(self.address)?;
        func.name = self.old_name.take().unwrap();
        Ok(())
    }
    
    fn description(&self) -> String {
        format!("Rename function at 0x{:x}", self.address)
    }
}

struct UndoManager {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
    max_undo: usize,
}
```

### 5. Collaborative Features

```rust
// Modern decompilers support team work
struct CollaborativeSession {
    project_id: Uuid,
    websocket: WebSocket,
    local_changes: Vec<Change>,
    remote_changes: broadcast::Receiver<Change>,
}

#[derive(Serialize, Deserialize)]
enum Change {
    FunctionRenamed { address: Address, name: String, user: User },
    CommentAdded { address: Address, text: String, user: User },
    TypeChanged { address: Address, var: String, ty: Type, user: User },
}

impl CollaborativeSession {
    async fn sync_changes(&mut self) {
        // Send local changes
        for change in self.local_changes.drain(..) {
            self.websocket.send(&change).await?;
        }
        
        // Apply remote changes
        while let Ok(change) = self.remote_changes.try_recv() {
            self.apply_remote_change(change)?;
        }
    }
}
```

## Performance Architecture

### 1. Lazy Loading Everything

```rust
// Don't load 100K functions into memory
struct LazyFunctionMap {
    cache: LruCache<Address, Arc<Function>>,
    db: AnalysisDatabase,
}

impl LazyFunctionMap {
    fn get(&mut self, addr: Address) -> Result<Arc<Function>> {
        // Check cache first
        if let Some(func) = self.cache.get(&addr) {
            return Ok(func.clone());
        }
        
        // Load from database
        let func = self.db.load_function(addr)?;
        let func = Arc::new(func);
        
        // Cache for next time
        self.cache.put(addr, func.clone());
        
        Ok(func)
    }
}
```

### 2. Background Analysis Queue

```rust
use crossbeam::channel::{unbounded, Sender, Receiver};

struct AnalysisScheduler {
    work_queue: Sender<AnalysisTask>,
    workers: Vec<JoinHandle<()>>,
}

enum AnalysisTask {
    AnalyzeFunction(Address),
    PropagateTypes(Address),
    FindXrefs(Address),
    DecompileFunction(Address),
}

impl AnalysisScheduler {
    fn new(num_workers: usize) -> Self {
        let (tx, rx) = unbounded();
        let rx = Arc::new(Mutex::new(rx));
        
        let workers = (0..num_workers)
            .map(|id| {
                let rx = rx.clone();
                thread::spawn(move || {
                    worker_loop(id, rx);
                })
            })
            .collect();
            
        Self { work_queue: tx, workers }
    }
    
    fn schedule(&self, task: AnalysisTask) {
        // Non-blocking - UI stays responsive
        let _ = self.work_queue.send(task);
    }
}

fn worker_loop(id: usize, rx: Arc<Mutex<Receiver<AnalysisTask>>>) {
    loop {
        let task = {
            let rx = rx.lock().unwrap();
            rx.recv().ok()
        };
        
        match task {
            Some(AnalysisTask::AnalyzeFunction(addr)) => {
                analyze_function_background(addr);
            }
            // ... handle other tasks
            None => break,
        }
    }
}
```

### 3. Incremental Analysis

```rust
// Only re-analyze what changed
struct IncrementalAnalyzer {
    dependency_graph: DiGraph<Address, DependencyType>,
    dirty_functions: HashSet<Address>,
}

impl IncrementalAnalyzer {
    fn mark_dirty(&mut self, addr: Address) {
        self.dirty_functions.insert(addr);
        
        // Mark dependent functions
        for dependent in self.get_dependents(addr) {
            self.dirty_functions.insert(dependent);
        }
    }
    
    fn analyze_dirty(&mut self) {
        while let Some(addr) = self.dirty_functions.iter().next().copied() {
            self.dirty_functions.remove(&addr);
            
            // Only analyze if dependencies are clean
            if self.dependencies_clean(addr) {
                self.analyze_function(addr);
            } else {
                // Re-queue for later
                self.dirty_functions.insert(addr);
            }
        }
    }
}
```

## User Interface Architecture

### 1. Model-View-ViewModel (MVVM)

```rust
// Separation of concerns for complex UIs
struct FunctionViewModel {
    address: Address,
    name: Property<String>,
    decompiled_code: Property<String>,
    assembly: Property<Vec<Instruction>>,
    xrefs_to: Property<Vec<Xref>>,
    xrefs_from: Property<Vec<Xref>>,
}

impl FunctionViewModel {
    fn new(model: Arc<Function>) -> Self {
        Self {
            address: model.address,
            name: Property::new(model.name.clone()),
            decompiled_code: Property::new(model.decompiled_code.clone()),
            // ... initialize other properties
        }
    }
    
    fn rename(&mut self, new_name: String) {
        // Update model
        self.name.set(new_name.clone());
        
        // Command for undo/redo
        let cmd = RenameFunction {
            address: self.address,
            old_name: Some(self.name.get()),
            new_name,
        };
        
        execute_command(cmd);
    }
}
```

### 2. Synchronized Views

```rust
// Multiple views of same data stay in sync
struct ViewSynchronizer {
    views: HashMap<ViewId, Box<dyn View>>,
    subscriptions: HashMap<Address, Vec<ViewId>>,
}

impl ViewSynchronizer {
    fn on_function_changed(&mut self, addr: Address) {
        if let Some(view_ids) = self.subscriptions.get(&addr) {
            for view_id in view_ids {
                if let Some(view) = self.views.get_mut(view_id) {
                    view.refresh();
                }
            }
        }
    }
}
```

## Scalability Patterns

### 1. Sharding Large Binaries

```rust
// Handle 1GB+ binaries by sharding
struct ShardedAnalysis {
    shards: Vec<AnalysisShard>,
    shard_size: usize,
}

struct AnalysisShard {
    start_addr: Address,
    end_addr: Address,
    functions: HashMap<Address, Function>,
    db_path: PathBuf,
}

impl ShardedAnalysis {
    fn get_shard(&self, addr: Address) -> &AnalysisShard {
        let shard_idx = (addr / self.shard_size) as usize;
        &self.shards[shard_idx]
    }
    
    fn analyze_parallel(&mut self) {
        self.shards.par_iter_mut()
            .for_each(|shard| {
                shard.analyze();
            });
    }
}
```

### 2. LOD (Level of Detail) for Large Functions

```rust
// Don't show 10K lines at once
struct FunctionLOD {
    address: Address,
    summary: FunctionSummary,        // Always available
    basic_blocks: Option<Vec<BasicBlock>>,  // Load on demand
    full_decompilation: Option<String>,     // Load on demand
    detailed_ir: Option<Vec<IR>>,          // Load on demand
}

impl FunctionLOD {
    fn get_visible_code(&self, viewport: &Viewport) -> String {
        if viewport.zoom_level < 0.5 {
            // Show summary only
            self.summary.to_string()
        } else if viewport.zoom_level < 0.8 {
            // Show basic blocks
            self.get_basic_blocks()
                .render_simplified()
        } else {
            // Show full detail
            self.get_full_decompilation()
                .substring(viewport.visible_range())
        }
    }
}
```

## Security Considerations

### 1. Sandboxed Plugins

```rust
use wasmtime::{Engine, Module, Store};

// Run untrusted plugins safely
struct SandboxedPlugin {
    engine: Engine,
    module: Module,
    store: Store<PluginState>,
}

impl SandboxedPlugin {
    fn new(wasm_bytes: &[u8]) -> Result<Self> {
        let engine = Engine::default();
        let module = Module::new(&engine, wasm_bytes)?;
        let mut store = Store::new(&engine, PluginState::default());
        
        // Limit resources
        store.limiter(|state| &mut state.limits);
        
        Ok(Self { engine, module, store })
    }
    
    fn analyze(&mut self, data: &[u8]) -> Result<PluginResult> {
        // Run in sandbox with resource limits
        let instance = Instance::new(&mut self.store, &self.module, &[])?;
        let analyze_fn = instance.get_typed_func::<(i32, i32), i32>(&mut self.store, "analyze")?;
        
        // Copy data to WASM memory
        let memory = instance.get_memory(&mut self.store, "memory").unwrap();
        memory.write(&mut self.store, 0, data)?;
        
        // Call with timeout
        let result = analyze_fn.call_async(&mut self.store, (0, data.len() as i32))
            .timeout(Duration::from_secs(5))
            .await?;
            
        Ok(decode_plugin_result(result))
    }
}
```

### 2. Audit Logging

```rust
// Track all user actions for security/compliance
struct AuditLogger {
    log_file: File,
}

impl AuditLogger {
    fn log_action(&mut self, action: &Action) {
        let entry = AuditEntry {
            timestamp: Utc::now(),
            user: current_user(),
            action: action.clone(),
            ip_address: current_ip(),
            session_id: current_session(),
        };
        
        // Cryptographically sign entries
        let signature = sign_entry(&entry);
        
        writeln!(self.log_file, "{}", 
            serde_json::to_string(&SignedEntry { entry, signature }).unwrap()
        ).unwrap();
    }
}
```

## Deployment Architecture

### 1. Client-Server Mode

```rust
// Enterprise deployment with central server
struct DecompilerServer {
    analysis_engine: AnalysisEngine,
    database: PostgresPool,
    cache: RedisPool,
    message_queue: RabbitMQ,
}

impl DecompilerServer {
    async fn handle_analysis_request(&self, req: AnalysisRequest) -> Result<AnalysisResponse> {
        // Check cache first
        if let Some(cached) = self.cache.get(&req.hash()).await? {
            return Ok(cached);
        }
        
        // Queue for analysis
        let job_id = Uuid::new_v4();
        self.message_queue.publish(AnalysisJob {
            id: job_id,
            request: req,
            priority: self.calculate_priority(&req),
        }).await?;
        
        // Return job ID for polling
        Ok(AnalysisResponse::Queued(job_id))
    }
}
```

### 2. Microservices Architecture

```rust
// Scalable architecture for cloud deployment
struct MicroserviceDecompiler {
    // Each service handles specific tasks
    disassembly_service: ServiceClient,
    lifting_service: ServiceClient,
    analysis_service: ServiceClient,
    decompilation_service: ServiceClient,
    storage_service: ServiceClient,
}

// Individual services can scale independently
async fn run_disassembly_service() {
    let service = DisassemblyService::new();
    
    Server::builder()
        .add_service(service)
        .serve("[::1]:50051".parse().unwrap())
        .await
        .unwrap();
}
```

## Lessons Learned

1. **Start with SQLite**: You'll need a database eventually
2. **Design for plugins**: Users will want to extend
3. **Make it collaborative**: Teams use decompilers
4. **Cache aggressively**: Users revisit same code
5. **Background everything**: UI responsiveness is critical
6. **Version your schemas**: Migrations are painful
7. **Audit everything**: Enterprise users require it

Remember: The best architecture is one that ships and scales with user needs.