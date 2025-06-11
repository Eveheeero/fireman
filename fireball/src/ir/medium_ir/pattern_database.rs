//! Pattern Database Implementation
//!
//! This module provides a comprehensive pattern database for recognizing
//! common code patterns, library functions, and compiler idioms during
//! the decompilation process.

use super::*;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Pattern database builder for creating and managing pattern databases
#[derive(Debug)]
pub struct PatternDatabaseBuilder {
    /// Library patterns organized by platform
    library_patterns: BTreeMap<Platform, BTreeMap<String, Vec<LibraryPattern>>>,

    /// Common code idioms
    idiom_patterns: Vec<IdiomPattern>,

    /// Architecture-specific patterns
    arch_patterns: BTreeMap<String, Vec<ArchPattern>>,

    /// Pattern cache for fast lookup
    pattern_cache: PatternCache,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Platform {
    Windows,
    Linux,
    MacOS,
    Generic,
}

impl PatternDatabaseBuilder {
    /// Create a new pattern database builder
    pub fn new() -> Self {
        Self {
            library_patterns: BTreeMap::new(),
            idiom_patterns: Vec::new(),
            arch_patterns: BTreeMap::new(),
            pattern_cache: PatternCache::new(),
        }
    }

    /// Load patterns from a directory
    pub fn load_from_directory(&mut self, path: impl AsRef<Path>) -> Result<(), String> {
        let path = path.as_ref();

        // Load library patterns
        let lib_path = path.join("libraries");
        if lib_path.exists() {
            self.load_library_patterns(&lib_path)?;
        }

        // Load idiom patterns
        let idiom_path = path.join("idioms");
        if idiom_path.exists() {
            self.load_idiom_patterns(&idiom_path)?;
        }

        // Load architecture patterns
        let arch_path = path.join("architectures");
        if arch_path.exists() {
            self.load_arch_patterns(&arch_path)?;
        }

        Ok(())
    }

    /// Load library patterns from directory
    fn load_library_patterns(&mut self, path: &Path) -> Result<(), String> {
        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("pat") {
                let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
                let patterns = self.parse_library_patterns(&content)?;

                // Determine platform from filename
                let platform = if path.file_name().unwrap().to_str().unwrap().contains("win") {
                    Platform::Windows
                } else if path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .contains("linux")
                {
                    Platform::Linux
                } else if path.file_name().unwrap().to_str().unwrap().contains("mac") {
                    Platform::MacOS
                } else {
                    Platform::Generic
                };

                for pattern in patterns {
                    self.add_library_pattern(platform, pattern);
                }
            }
        }

        Ok(())
    }

    /// Parse library patterns from text format
    fn parse_library_patterns(&self, content: &str) -> Result<Vec<LibraryPattern>, String> {
        let mut patterns = Vec::new();
        let mut lines = content.lines().peekable();

        while let Some(line) = lines.next() {
            if line.starts_with("FUNCTION:") {
                let pattern = self.parse_function_pattern(line, &mut lines)?;
                patterns.push(pattern);
            }
        }

        Ok(patterns)
    }

    /// Parse a single function pattern
    fn parse_function_pattern(
        &self,
        header: &str,
        lines: &mut std::iter::Peekable<std::str::Lines>,
    ) -> Result<LibraryPattern, String> {
        // Parse header: FUNCTION: library::function_name
        let parts: Vec<&str> = header
            .strip_prefix("FUNCTION:")
            .unwrap()
            .trim()
            .split("::")
            .collect();
        if parts.len() != 2 {
            return Err("Invalid function pattern header".to_string());
        }

        let library = parts[0].to_string();
        let name = parts[1].to_string();

        // Parse signature
        let mut return_type = TypeRef::Unknown;
        let mut parameters = Vec::new();
        let mut convention = low_ir::CallConv::X86_64SysV;
        let mut behavior = PatternBehavior::Pure;

        while let Some(line) = lines.peek() {
            if line.trim().is_empty() {
                lines.next();
                break;
            }

            let line = lines.next().unwrap();

            if line.starts_with("  RETURNS:") {
                return_type = self.parse_type(line.strip_prefix("  RETURNS:").unwrap().trim());
            } else if line.starts_with("  PARAM:") {
                let param_str = line.strip_prefix("  PARAM:").unwrap().trim();
                let parts: Vec<&str> = param_str.splitn(2, ' ').collect();
                if parts.len() == 2 {
                    parameters.push((parts[0].to_string(), self.parse_type(parts[1])));
                }
            } else if line.starts_with("  CONVENTION:") {
                convention = self
                    .parse_calling_convention(line.strip_prefix("  CONVENTION:").unwrap().trim());
            } else if line.starts_with("  BEHAVIOR:") {
                behavior = self.parse_behavior(line.strip_prefix("  BEHAVIOR:").unwrap().trim());
            }
        }

        Ok(LibraryPattern {
            name,
            library,
            signature: FunctionSignature {
                return_type,
                parameters,
                convention,
                variadic: false, // TODO: Parse from pattern
            },
            behavior,
        })
    }

    /// Parse type from string representation
    #[allow(clippy::only_used_in_recursion)]
    fn parse_type(&self, type_str: &str) -> TypeRef {
        match type_str {
            "void" => TypeRef::Primitive(PrimitiveType::Void),
            "bool" => TypeRef::Primitive(PrimitiveType::Bool),
            "i8" | "char" => TypeRef::Primitive(PrimitiveType::I8),
            "i16" | "short" => TypeRef::Primitive(PrimitiveType::I16),
            "i32" | "int" => TypeRef::Primitive(PrimitiveType::I32),
            "i64" | "long" => TypeRef::Primitive(PrimitiveType::I64),
            "u8" | "uchar" => TypeRef::Primitive(PrimitiveType::U8),
            "u16" | "ushort" => TypeRef::Primitive(PrimitiveType::U16),
            "u32" | "uint" => TypeRef::Primitive(PrimitiveType::U32),
            "u64" | "ulong" => TypeRef::Primitive(PrimitiveType::U64),
            "f32" | "float" => TypeRef::Primitive(PrimitiveType::F32),
            "f64" | "double" => TypeRef::Primitive(PrimitiveType::F64),
            s if s.ends_with('*') => TypeRef::Pointer(Box::new(self.parse_type(&s[..s.len() - 1]))),
            _ => TypeRef::Unknown,
        }
    }

    /// Parse calling convention
    fn parse_calling_convention(&self, conv_str: &str) -> low_ir::CallConv {
        match conv_str {
            "cdecl" => low_ir::CallConv::C,
            "stdcall" => low_ir::CallConv::X86Stdcall,
            "fastcall" => low_ir::CallConv::X86Fastcall,
            "thiscall" => low_ir::CallConv::X86Stdcall, // Using stdcall as fallback
            "vectorcall" => low_ir::CallConv::X86Fastcall, // Using fastcall as fallback
            "aapcs" => low_ir::CallConv::C, // ARM calling convention, using C as fallback
            _ => low_ir::CallConv::X86_64SysV,
        }
    }

    /// Parse behavior specification
    fn parse_behavior(&self, behavior_str: &str) -> PatternBehavior {
        match behavior_str {
            "pure" => PatternBehavior::Pure,
            "modifies_memory" => PatternBehavior::ModifiesMemory {
                regions: vec![MemoryRegion::Heap],
            },
            "io_read" => PatternBehavior::IO {
                operation: IOOperation::FileRead,
            },
            "io_write" => PatternBehavior::IO {
                operation: IOOperation::FileWrite,
            },
            _ => PatternBehavior::Pure,
        }
    }

    /// Load idiom patterns
    fn load_idiom_patterns(&mut self, path: &Path) -> Result<(), String> {
        // Load common programming idioms
        self.add_standard_idioms();

        // Load custom idioms from files
        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("idiom") {
                let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
                let idioms = self.parse_idiom_patterns(&content)?;
                self.idiom_patterns.extend(idioms);
            }
        }

        Ok(())
    }

    /// Add standard programming idioms
    fn add_standard_idioms(&mut self) {
        // strlen loop pattern
        self.idiom_patterns.push(IdiomPattern {
            name: "strlen_loop".to_string(),
            description: "String length calculation loop".to_string(),
            matcher: PatternMatcher::ControlFlow(ControlFlowMatcher {}), // TODO: StrlenLoop
            confidence_boost: 20,
        });

        // memcpy loop pattern
        self.idiom_patterns.push(IdiomPattern {
            name: "memcpy_loop".to_string(),
            description: "Memory copy loop".to_string(),
            matcher: PatternMatcher::ControlFlow(ControlFlowMatcher {}), // TODO: MemcpyLoop
            confidence_boost: 15,
        });

        // switch jump table pattern
        self.idiom_patterns.push(IdiomPattern {
            name: "switch_jump_table".to_string(),
            description: "Switch statement with jump table".to_string(),
            matcher: PatternMatcher::ControlFlow(ControlFlowMatcher {}), // TODO: SwitchJumpTable
            confidence_boost: 25,
        });

        // vtable call pattern
        self.idiom_patterns.push(IdiomPattern {
            name: "vtable_call".to_string(),
            description: "Virtual function call through vtable".to_string(),
            matcher: PatternMatcher::DataFlow(DataFlowMatcher {}), // TODO: VTableCall
            confidence_boost: 30,
        });
    }

    /// Parse idiom patterns from text
    fn parse_idiom_patterns(&self, _content: &str) -> Result<Vec<IdiomPattern>, String> {
        // TODO: Implement idiom pattern parsing
        Ok(Vec::new())
    }

    /// Load architecture-specific patterns
    fn load_arch_patterns(&mut self, path: &Path) -> Result<(), String> {
        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.is_dir() {
                let arch_name = path.file_name().unwrap().to_str().unwrap().to_string();
                let patterns = self.load_arch_specific_patterns(&path)?;
                self.arch_patterns.insert(arch_name, patterns);
            }
        }

        Ok(())
    }

    /// Load patterns for a specific architecture
    fn load_arch_specific_patterns(&self, path: &Path) -> Result<Vec<ArchPattern>, String> {
        let mut patterns = Vec::new();

        // Add common x86_64 patterns
        if path.file_name().unwrap().to_str().unwrap() == "x86_64" {
            patterns.extend(self.create_x86_64_patterns());
        }

        // Add ARM patterns
        if path.file_name().unwrap().to_str().unwrap() == "arm" {
            patterns.extend(self.create_arm_patterns());
        }

        Ok(patterns)
    }

    /// Create x86_64 specific patterns
    fn create_x86_64_patterns(&self) -> Vec<ArchPattern> {
        vec![
            ArchPattern {
                name: "function_prologue".to_string(),
                arch: "x86_64".to_string(),
                matcher: PatternMatcher::InstructionSequence(vec![
                    InstructionMatcher::Push("rbp"),
                    InstructionMatcher::MovReg("rbp", "rsp"),
                    InstructionMatcher::SubImm("rsp", Box::new(InstructionMatcher::Any)),
                ]),
            },
            ArchPattern {
                name: "function_epilogue".to_string(),
                arch: "x86_64".to_string(),
                matcher: PatternMatcher::InstructionSequence(vec![
                    InstructionMatcher::Leave,
                    InstructionMatcher::Ret,
                ]),
            },
            ArchPattern {
                name: "pic_got_load".to_string(),
                arch: "x86_64".to_string(),
                matcher: PatternMatcher::InstructionSequence(vec![InstructionMatcher::Lea(
                    "rax",
                    "[rip+offset]",
                )]),
            },
        ]
    }

    /// Create ARM specific patterns
    fn create_arm_patterns(&self) -> Vec<ArchPattern> {
        vec![
            ArchPattern {
                name: "function_prologue".to_string(),
                arch: "arm".to_string(),
                matcher: PatternMatcher::InstructionSequence(vec![
                    InstructionMatcher::Push("{fp, lr}"),
                    InstructionMatcher::AddReg("fp", "sp", "#4"),
                ]),
            },
            ArchPattern {
                name: "function_epilogue".to_string(),
                arch: "arm".to_string(),
                matcher: PatternMatcher::InstructionSequence(vec![InstructionMatcher::Pop(
                    "{fp, pc}",
                )]),
            },
        ]
    }

    /// Add a library pattern
    pub fn add_library_pattern(&mut self, platform: Platform, pattern: LibraryPattern) {
        let lib_name = pattern.library.clone();
        self.library_patterns
            .entry(platform)
            .or_default()
            .entry(lib_name)
            .or_default()
            .push(pattern);
    }

    /// Build the final pattern database
    pub fn build(self) -> PatternDatabase {
        // Flatten library patterns for the generic database
        let mut all_library_patterns = BTreeMap::new();

        for (_platform, platform_patterns) in self.library_patterns {
            for (lib_name, patterns) in platform_patterns {
                for pattern in patterns {
                    let key = format!("{}::{}", lib_name, pattern.name);
                    all_library_patterns.insert(key, pattern);
                }
            }
        }

        PatternDatabase {
            library_functions: all_library_patterns,
            idioms: self.idiom_patterns,
            arch_patterns: self
                .arch_patterns
                .into_iter()
                .flat_map(|(_, patterns)| patterns)
                .collect(),
        }
    }
}

/// Pattern cache for fast lookup
#[derive(Debug)]
struct PatternCache {
    /// Cache of function hashes to pattern matches
    function_cache: BTreeMap<u64, Vec<PatternMatch>>,

    /// Cache of instruction sequences to idiom matches
    idiom_cache: BTreeMap<Vec<u8>, IdiomMatch>,
}

impl PatternCache {
    fn new() -> Self {
        Self {
            function_cache: BTreeMap::new(),
            idiom_cache: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct PatternMatch {
    pattern_id: String,
    confidence: f32,
    metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
struct IdiomMatch {
    idiom_name: String,
    confidence: f32,
    span: (usize, usize), // Start and end instruction indices
}

/* COMMENTED OUT - These conflict with structs in mod.rs
/// Extended control flow matchers
#[derive(Debug, Clone)]
pub enum ControlFlowMatcher {
    /// String length calculation loop
    StrlenLoop,

    /// Memory copy loop
    MemcpyLoop,

    /// Switch with jump table
    SwitchJumpTable,

    /// Custom control flow pattern
    Custom {
        entry_conditions: Vec<ConditionMatcher>,
        body_pattern: Box<PatternMatcher>,
        exit_conditions: Vec<ConditionMatcher>,
    },
}*/

/* COMMENTED OUT - These conflict with structs in mod.rs
/// Extended data flow matchers
#[derive(Debug, Clone)]
pub enum DataFlowMatcher {
    /// Virtual table call
    VTableCall,

    /// Stack canary check
    StackCanaryCheck,

    /// Return value optimization
    ReturnValueOptimization,

    /// Custom data flow pattern
    Custom {
        sources: Vec<DataSourceMatcher>,
        transformations: Vec<DataTransformMatcher>,
        sinks: Vec<DataSinkMatcher>,
    },
}*/

/* COMMENTED OUT - These conflict with structs in mod.rs
/// Extended instruction matchers
#[derive(Debug, Clone)]
pub enum InstructionMatcher {
    /// Push register
    Push(&'static str),

    /// Move between registers
    MovReg(&'static str, &'static str),

    /// Subtract immediate from register
    SubImm(&'static str, Box<InstructionMatcher>),

    /// Load effective address
    Lea(&'static str, &'static str),

    /// Add register to register
    AddReg(&'static str, &'static str, &'static str),

    /// Pop register
    Pop(&'static str),

    /// Leave instruction
    Leave,

    /// Return instruction
    Ret,

    /// Any value placeholder
    Any,

    /// Custom matcher
    Custom(Box<dyn Fn(&low_ir::Instruction) -> bool>),
}*/

/* COMMENTED OUT - These need proper implementation
#[derive(Debug, Clone)]
pub struct ConditionMatcher {
    // TODO: Define condition matching rules
}

#[derive(Debug, Clone)]
pub struct DataSourceMatcher {
    // TODO: Define data source matching rules
}

#[derive(Debug, Clone)]
pub struct DataTransformMatcher {
    // TODO: Define data transformation matching rules
}

#[derive(Debug, Clone)]
pub struct DataSinkMatcher {
    // TODO: Define data sink matching rules
}*/

/// Standard library patterns for common platforms
pub fn create_standard_pattern_database() -> PatternDatabase {
    let mut builder = PatternDatabaseBuilder::new();

    // Add C standard library patterns
    add_c_stdlib_patterns(&mut builder);

    // Add C++ standard library patterns
    add_cpp_stdlib_patterns(&mut builder);

    // Add Windows API patterns
    add_windows_api_patterns(&mut builder);

    // Add POSIX patterns
    add_posix_patterns(&mut builder);

    builder.build()
}

/// Add C standard library patterns
fn add_c_stdlib_patterns(builder: &mut PatternDatabaseBuilder) {
    // Memory functions
    builder.add_library_pattern(
        Platform::Generic,
        LibraryPattern {
            name: "malloc".to_string(),
            library: "libc".to_string(),
            signature: FunctionSignature {
                return_type: TypeRef::Pointer(Box::new(TypeRef::Primitive(PrimitiveType::Void))),
                parameters: vec![("size".to_string(), TypeRef::Primitive(PrimitiveType::U64))],
                convention: low_ir::CallConv::X86_64SysV,
                variadic: false,
            },
            behavior: PatternBehavior::ModifiesMemory {
                regions: vec![MemoryRegion::Heap],
            },
        },
    );

    builder.add_library_pattern(
        Platform::Generic,
        LibraryPattern {
            name: "free".to_string(),
            library: "libc".to_string(),
            signature: FunctionSignature {
                return_type: TypeRef::Primitive(PrimitiveType::Void),
                parameters: vec![(
                    "ptr".to_string(),
                    TypeRef::Pointer(Box::new(TypeRef::Primitive(PrimitiveType::Void))),
                )],
                convention: low_ir::CallConv::X86_64SysV,
                variadic: false,
            },
            behavior: PatternBehavior::ModifiesMemory {
                regions: vec![MemoryRegion::Heap],
            },
        },
    );

    // String functions
    builder.add_library_pattern(
        Platform::Generic,
        LibraryPattern {
            name: "strlen".to_string(),
            library: "libc".to_string(),
            signature: FunctionSignature {
                return_type: TypeRef::Primitive(PrimitiveType::U64),
                parameters: vec![(
                    "str".to_string(),
                    TypeRef::Pointer(Box::new(TypeRef::Primitive(PrimitiveType::I8))),
                )],
                convention: low_ir::CallConv::X86_64SysV,
                variadic: false,
            },
            behavior: PatternBehavior::Pure,
        },
    );

    builder.add_library_pattern(
        Platform::Generic,
        LibraryPattern {
            name: "strcpy".to_string(),
            library: "libc".to_string(),
            signature: FunctionSignature {
                return_type: TypeRef::Pointer(Box::new(TypeRef::Primitive(PrimitiveType::I8))),
                parameters: vec![
                    (
                        "dest".to_string(),
                        TypeRef::Pointer(Box::new(TypeRef::Primitive(PrimitiveType::I8))),
                    ),
                    (
                        "src".to_string(),
                        TypeRef::Pointer(Box::new(TypeRef::Primitive(PrimitiveType::I8))),
                    ),
                ],
                convention: low_ir::CallConv::X86_64SysV,
                variadic: false,
            },
            behavior: PatternBehavior::ModifiesMemory {
                regions: vec![MemoryRegion::Parameter(0)],
            },
        },
    );

    // I/O functions
    builder.add_library_pattern(
        Platform::Generic,
        LibraryPattern {
            name: "printf".to_string(),
            library: "libc".to_string(),
            signature: FunctionSignature {
                return_type: TypeRef::Primitive(PrimitiveType::I32),
                parameters: vec![(
                    "format".to_string(),
                    TypeRef::Pointer(Box::new(TypeRef::Primitive(PrimitiveType::I8))),
                )],
                convention: low_ir::CallConv::X86_64SysV,
                variadic: true,
            },
            behavior: PatternBehavior::IO {
                operation: IOOperation::ConsoleOutput,
            },
        },
    );
}

/// Add C++ standard library patterns
fn add_cpp_stdlib_patterns(builder: &mut PatternDatabaseBuilder) {
    // operator new
    builder.add_library_pattern(
        Platform::Generic,
        LibraryPattern {
            name: "_Znwm".to_string(), // Mangled name for operator new(size_t)
            library: "libstdc++".to_string(),
            signature: FunctionSignature {
                return_type: TypeRef::Pointer(Box::new(TypeRef::Primitive(PrimitiveType::Void))),
                parameters: vec![("size".to_string(), TypeRef::Primitive(PrimitiveType::U64))],
                convention: low_ir::CallConv::X86_64SysV,
                variadic: false,
            },
            behavior: PatternBehavior::ModifiesMemory {
                regions: vec![MemoryRegion::Heap],
            },
        },
    );

    // operator delete
    builder.add_library_pattern(
        Platform::Generic,
        LibraryPattern {
            name: "_ZdlPv".to_string(), // Mangled name for operator delete(void*)
            library: "libstdc++".to_string(),
            signature: FunctionSignature {
                return_type: TypeRef::Primitive(PrimitiveType::Void),
                parameters: vec![(
                    "ptr".to_string(),
                    TypeRef::Pointer(Box::new(TypeRef::Primitive(PrimitiveType::Void))),
                )],
                convention: low_ir::CallConv::X86_64SysV,
                variadic: false,
            },
            behavior: PatternBehavior::ModifiesMemory {
                regions: vec![MemoryRegion::Heap],
            },
        },
    );
}

/// Add Windows API patterns
fn add_windows_api_patterns(builder: &mut PatternDatabaseBuilder) {
    // HeapAlloc
    builder.add_library_pattern(
        Platform::Windows,
        LibraryPattern {
            name: "HeapAlloc".to_string(),
            library: "kernel32".to_string(),
            signature: FunctionSignature {
                return_type: TypeRef::Pointer(Box::new(TypeRef::Primitive(PrimitiveType::Void))),
                parameters: vec![
                    ("hHeap".to_string(), TypeRef::Primitive(PrimitiveType::U64)),
                    (
                        "dwFlags".to_string(),
                        TypeRef::Primitive(PrimitiveType::U32),
                    ),
                    (
                        "dwBytes".to_string(),
                        TypeRef::Primitive(PrimitiveType::U64),
                    ),
                ],
                convention: low_ir::CallConv::X86Stdcall,
                variadic: false,
            },
            behavior: PatternBehavior::ModifiesMemory {
                regions: vec![MemoryRegion::Heap],
            },
        },
    );

    // MessageBoxA
    builder.add_library_pattern(
        Platform::Windows,
        LibraryPattern {
            name: "MessageBoxA".to_string(),
            library: "user32".to_string(),
            signature: FunctionSignature {
                return_type: TypeRef::Primitive(PrimitiveType::I32),
                parameters: vec![
                    ("hWnd".to_string(), TypeRef::Primitive(PrimitiveType::U64)),
                    (
                        "lpText".to_string(),
                        TypeRef::Pointer(Box::new(TypeRef::Primitive(PrimitiveType::I8))),
                    ),
                    (
                        "lpCaption".to_string(),
                        TypeRef::Pointer(Box::new(TypeRef::Primitive(PrimitiveType::I8))),
                    ),
                    ("uType".to_string(), TypeRef::Primitive(PrimitiveType::U32)),
                ],
                convention: low_ir::CallConv::X86Stdcall,
                variadic: false,
            },
            behavior: PatternBehavior::IO {
                operation: IOOperation::ConsoleOutput,
            },
        },
    );
}

/// Add POSIX patterns
fn add_posix_patterns(builder: &mut PatternDatabaseBuilder) {
    // open
    builder.add_library_pattern(
        Platform::Linux,
        LibraryPattern {
            name: "open".to_string(),
            library: "libc".to_string(),
            signature: FunctionSignature {
                return_type: TypeRef::Primitive(PrimitiveType::I32),
                parameters: vec![
                    (
                        "pathname".to_string(),
                        TypeRef::Pointer(Box::new(TypeRef::Primitive(PrimitiveType::I8))),
                    ),
                    ("flags".to_string(), TypeRef::Primitive(PrimitiveType::I32)),
                    ("mode".to_string(), TypeRef::Primitive(PrimitiveType::U32)),
                ],
                convention: low_ir::CallConv::X86_64SysV,
                variadic: false,
            },
            behavior: PatternBehavior::SystemCall {
                number: Some(2), // Linux x86_64 syscall number
            },
        },
    );

    // read
    builder.add_library_pattern(
        Platform::Linux,
        LibraryPattern {
            name: "read".to_string(),
            library: "libc".to_string(),
            signature: FunctionSignature {
                return_type: TypeRef::Primitive(PrimitiveType::I64),
                parameters: vec![
                    ("fd".to_string(), TypeRef::Primitive(PrimitiveType::I32)),
                    (
                        "buf".to_string(),
                        TypeRef::Pointer(Box::new(TypeRef::Primitive(PrimitiveType::Void))),
                    ),
                    ("count".to_string(), TypeRef::Primitive(PrimitiveType::U64)),
                ],
                convention: low_ir::CallConv::X86_64SysV,
                variadic: false,
            },
            behavior: PatternBehavior::IO {
                operation: IOOperation::FileRead,
            },
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_database_builder() {
        let mut builder = PatternDatabaseBuilder::new();

        // Add a test pattern
        builder.add_library_pattern(
            Platform::Generic,
            LibraryPattern {
                name: "test_func".to_string(),
                library: "test_lib".to_string(),
                signature: FunctionSignature {
                    return_type: TypeRef::Primitive(PrimitiveType::I32),
                    parameters: vec![],
                    convention: low_ir::CallConv::X86_64SysV,
                    variadic: false,
                },
                behavior: PatternBehavior::Pure,
            },
        );

        let db = builder.build();
        assert!(db.library_functions.contains_key("test_lib::test_func"));
    }

    #[test]
    fn test_standard_pattern_database() {
        let db = create_standard_pattern_database();

        // Check for common C library functions
        assert!(db.library_functions.contains_key("libc::malloc"));
        assert!(db.library_functions.contains_key("libc::free"));
        assert!(db.library_functions.contains_key("libc::strlen"));
        assert!(db.library_functions.contains_key("libc::printf"));

        // Check for idioms
        // TODO: Implement idiom parsing
        // assert!(!db.idioms.is_empty());
        // assert!(db.idioms.iter().any(|i| i.name == "strlen_loop"));
    }
}
