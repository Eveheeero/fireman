//! Pattern file parser for the Fireball pattern format
//!
//! This module implements parsing for .pat, .idiom, and .arch pattern files

use super::pattern_database::Platform;
use super::*;
use std::fs;
use std::path::Path;

/// Pattern file parser
pub struct PatternParser {
    /// Current file being parsed
    current_file: Option<String>,
    /// Current line number
    current_line: usize,
}

impl PatternParser {
    /// Create a new pattern parser
    pub fn new() -> Self {
        Self {
            current_file: None,
            current_line: 0,
        }
    }

    /// Parse a pattern file
    pub fn parse_file(&mut self, path: &Path) -> Result<ParsedPatterns, PatternParseError> {
        self.current_file = Some(path.display().to_string());
        self.current_line = 0;

        let content =
            fs::read_to_string(path).map_err(|e| PatternParseError::IoError(e.to_string()))?;

        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| PatternParseError::InvalidExtension)?;

        match extension {
            "pat" => self.parse_library_patterns(&content),
            "idiom" => self.parse_idiom_patterns(&content),
            "arch" => self.parse_arch_patterns(&content),
            _ => Err(PatternParseError::InvalidExtension),
        }
    }

    /// Parse library patterns (.pat files)
    pub fn parse_library_patterns(
        &mut self,
        content: &str,
    ) -> Result<ParsedPatterns, PatternParseError> {
        let mut patterns = Vec::new();
        let mut lines = content.lines().enumerate().peekable();
        let mut metadata = PatternMetadata::default();

        while let Some((line_num, line)) = lines.next() {
            self.current_line = line_num + 1;
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                // Parse metadata comments
                if line.starts_with("# Version:") {
                    metadata.version = line.strip_prefix("# Version:").unwrap().trim().to_string();
                } else if line.starts_with("# Platform:") {
                    metadata.platform =
                        self.parse_platform(line.strip_prefix("# Platform:").unwrap().trim());
                } else if line.starts_with("# Architecture:") {
                    metadata.architecture = line
                        .strip_prefix("# Architecture:")
                        .unwrap()
                        .trim()
                        .to_string();
                }
                continue;
            }

            if line.starts_with("FUNCTION:") {
                let pattern = self.parse_function_pattern(line, &mut lines)?;
                patterns.push(ParsedPattern::Library(pattern));
            } else if line.starts_with("TEST_CASE:") {
                // Skip test cases for now
                self.skip_until_end_marker(&mut lines, "END_TEST")?;
            }
        }

        Ok(ParsedPatterns { metadata, patterns })
    }

    /// Parse a single function pattern
    fn parse_function_pattern(
        &mut self,
        header: &str,
        lines: &mut std::iter::Peekable<std::iter::Enumerate<std::str::Lines>>,
    ) -> Result<LibraryPattern, PatternParseError> {
        // Parse header: FUNCTION: library::function_name
        let function_spec = header
            .strip_prefix("FUNCTION:")
            .ok_or(PatternParseError::InvalidSyntax(
                "Expected FUNCTION:".to_string(),
            ))?
            .trim();

        let parts: Vec<&str> = function_spec.split("::").collect();
        if parts.len() != 2 {
            return Err(PatternParseError::InvalidSyntax(
                "Function name must be in format library::function".to_string(),
            ));
        }

        let library = parts[0].to_string();
        let name = parts[1].to_string();

        let mut return_type = TypeRef::Unknown;
        let mut parameters = Vec::new();
        let mut convention = low_ir::CallConv::X86_64SysV;
        let mut behavior = PatternBehavior::Pure;
        let mut attributes = Vec::new();

        while let Some((line_num, line)) = lines.peek() {
            let full_line = *line;
            let line = line.trim();

            // Check if we've reached the end of this function
            // A line that starts without indentation and isn't empty signals a new section
            if line.is_empty()
                || (!full_line.starts_with(' ')
                    && !full_line.starts_with('\t')
                    && !line.starts_with("RETURNS:")
                    && !line.starts_with("PARAM:")
                    && !line.starts_with("CONVENTION:")
                    && !line.starts_with("BEHAVIOR:")
                    && !line.starts_with("ATTRIBUTES:"))
            {
                break;
            }

            self.current_line = line_num + 1;
            lines.next(); // Consume the line

            if line.starts_with("RETURNS:") {
                return_type = self.parse_type(line.strip_prefix("RETURNS:").unwrap().trim())?;
            } else if line.starts_with("PARAM:") {
                let param_str = line.strip_prefix("PARAM:").unwrap().trim();
                let parts: Vec<&str> = param_str.splitn(2, ' ').collect();
                if parts.len() != 2 {
                    return Err(PatternParseError::InvalidSyntax(
                        "PARAM must be in format: name type".to_string(),
                    ));
                }
                let param_type = self.parse_type(parts[1])?;
                parameters.push((parts[0].to_string(), param_type));
            } else if line.starts_with("CONVENTION:") {
                convention = self
                    .parse_calling_convention(line.strip_prefix("CONVENTION:").unwrap().trim())?;
            } else if line.starts_with("BEHAVIOR:") {
                behavior = self.parse_behavior(line.strip_prefix("BEHAVIOR:").unwrap().trim())?;
            } else if line.starts_with("ATTRIBUTES:") {
                let attr_str = line.strip_prefix("ATTRIBUTES:").unwrap().trim();
                attributes = attr_str.split(',').map(|s| s.trim().to_string()).collect();
            }
        }

        Ok(LibraryPattern {
            name,
            library,
            signature: FunctionSignature {
                return_type,
                parameters,
                convention,
                variadic: attributes.contains(&"variadic".to_string()),
            },
            behavior,
        })
    }

    /// Parse type from string
    pub fn parse_type(&self, type_str: &str) -> Result<TypeRef, PatternParseError> {
        let type_str = type_str.trim();

        // Handle const qualifier
        let (_is_const, base_type) = if type_str.starts_with("const ") {
            (true, type_str.strip_prefix("const ").unwrap())
        } else {
            (false, type_str)
        };

        // Handle pointer types
        if base_type.ends_with('*') {
            let inner_type = self.parse_type(&base_type[..base_type.len() - 1])?;
            return Ok(TypeRef::Pointer(Box::new(inner_type)));
        }

        // Handle array types
        if let Some(bracket_pos) = base_type.find('[') {
            let element_type = self.parse_type(&base_type[..bracket_pos])?;
            let size_str = base_type[bracket_pos + 1..base_type.len() - 1].trim();
            let size = size_str.parse::<usize>().ok();
            return Ok(TypeRef::Array {
                element: Box::new(element_type),
                size,
            });
        }

        // Parse primitive types
        let primitive = match base_type {
            "void" => PrimitiveType::Void,
            "bool" => PrimitiveType::Bool,
            "char" | "i8" => PrimitiveType::I8,
            "short" | "i16" => PrimitiveType::I16,
            "int" | "i32" => PrimitiveType::I32,
            "long" | "i64" => PrimitiveType::I64,
            "uchar" | "u8" => PrimitiveType::U8,
            "ushort" | "u16" => PrimitiveType::U16,
            "uint" | "u32" => PrimitiveType::U32,
            "ulong" | "u64" => PrimitiveType::U64,
            "float" | "f32" => PrimitiveType::F32,
            "double" | "f64" => PrimitiveType::F64,
            _ => return Ok(TypeRef::Unknown),
        };

        Ok(TypeRef::Primitive(primitive))
    }

    /// Parse calling convention
    fn parse_calling_convention(
        &self,
        conv_str: &str,
    ) -> Result<low_ir::CallConv, PatternParseError> {
        match conv_str {
            "cdecl" => Ok(low_ir::CallConv::C),
            "stdcall" => Ok(low_ir::CallConv::X86Stdcall),
            "fastcall" => Ok(low_ir::CallConv::X86Fastcall),
            "sysv" => Ok(low_ir::CallConv::X86_64SysV),
            "ms64" => Ok(low_ir::CallConv::X86Stdcall), // Using stdcall as fallback
            _ => Err(PatternParseError::UnknownConvention(conv_str.to_string())),
        }
    }

    /// Parse behavior
    fn parse_behavior(&self, behavior_str: &str) -> Result<PatternBehavior, PatternParseError> {
        match behavior_str {
            "pure" => Ok(PatternBehavior::Pure),
            "modifies_memory" => Ok(PatternBehavior::ModifiesMemory {
                regions: vec![MemoryRegion::Heap],
            }),
            "io_read" => Ok(PatternBehavior::IO {
                operation: IOOperation::FileRead,
            }),
            "io_write" => Ok(PatternBehavior::IO {
                operation: IOOperation::FileWrite,
            }),
            "system_call" => Ok(PatternBehavior::SystemCall { number: None }),
            "throws" => Ok(PatternBehavior::Pure), // TODO: Add exception support
            _ => Err(PatternParseError::UnknownBehavior(behavior_str.to_string())),
        }
    }

    /// Parse platform
    fn parse_platform(&self, platform_str: &str) -> Platform {
        match platform_str.to_lowercase().as_str() {
            "windows" => Platform::Windows,
            "linux" => Platform::Linux,
            "macos" => Platform::MacOS,
            _ => Platform::Generic,
        }
    }

    /// Parse idiom patterns (.idiom files)
    pub fn parse_idiom_patterns(
        &mut self,
        content: &str,
    ) -> Result<ParsedPatterns, PatternParseError> {
        let mut patterns = Vec::new();
        let mut lines = content.lines().enumerate().peekable();
        let mut metadata = PatternMetadata::default();

        while let Some((line_num, line)) = lines.next() {
            self.current_line = line_num + 1;
            let line = line.trim();

            // Skip empty lines and parse metadata comments
            if line.is_empty() || line.starts_with('#') {
                if line.starts_with("# Version:") {
                    metadata.version = line.strip_prefix("# Version:").unwrap().trim().to_string();
                }
                continue;
            }

            if line.starts_with("IDIOM:") {
                let idiom = self.parse_single_idiom(line, &mut lines)?;
                patterns.push(ParsedPattern::Idiom(idiom));
            } else if line.starts_with("TEST_CASE:") {
                self.skip_until_end_marker(&mut lines, "END_TEST")?;
            }
        }

        Ok(ParsedPatterns { metadata, patterns })
    }

    /// Parse a single idiom pattern
    fn parse_single_idiom(
        &mut self,
        header: &str,
        lines: &mut std::iter::Peekable<std::iter::Enumerate<std::str::Lines>>,
    ) -> Result<IdiomPattern, PatternParseError> {
        let name = header
            .strip_prefix("IDIOM:")
            .ok_or(PatternParseError::InvalidSyntax(
                "Expected IDIOM:".to_string(),
            ))?
            .trim()
            .to_string();

        let mut description = String::new();
        let mut confidence_boost = 0;
        let mut pattern_lines = Vec::new();

        while let Some((line_num, line)) = lines.peek() {
            self.current_line = line_num + 1;
            let line = line.trim();

            if line == "END_PATTERN" {
                lines.next(); // Consume END_PATTERN
                break;
            }

            lines.next(); // Consume the line

            if line.starts_with("DESCRIPTION:") {
                description = line
                    .strip_prefix("DESCRIPTION:")
                    .unwrap()
                    .trim()
                    .to_string();
            } else if line.starts_with("CONFIDENCE_BOOST:") {
                let boost_str = line.strip_prefix("CONFIDENCE_BOOST:").unwrap().trim();
                confidence_boost = boost_str
                    .parse::<i8>()
                    .map_err(|_| PatternParseError::InvalidNumber(boost_str.to_string()))?;
            } else if line.starts_with("PATTERN:") {
                // Collect pattern lines until END_PATTERN
                while let Some((_, pattern_line)) = lines.peek() {
                    if pattern_line.trim() == "END_PATTERN" {
                        break;
                    }
                    pattern_lines.push(lines.next().unwrap().1.to_string());
                }
            }
        }

        // For now, create a placeholder matcher
        // TODO: Implement proper idiom pattern parsing
        let matcher = PatternMatcher::ControlFlow(ControlFlowMatcher {});

        Ok(IdiomPattern {
            name,
            description,
            matcher,
            confidence_boost,
        })
    }

    /// Parse architecture patterns (.arch files)
    pub fn parse_arch_patterns(
        &mut self,
        content: &str,
    ) -> Result<ParsedPatterns, PatternParseError> {
        let mut patterns = Vec::new();
        let mut lines = content.lines().enumerate().peekable();
        let mut metadata = PatternMetadata::default();

        while let Some((line_num, line)) = lines.next() {
            self.current_line = line_num + 1;
            let line = line.trim();

            // Skip empty lines and parse metadata comments
            if line.is_empty() || line.starts_with('#') {
                if line.starts_with("# Architecture:") {
                    metadata.architecture = line
                        .strip_prefix("# Architecture:")
                        .unwrap()
                        .trim()
                        .to_string();
                }
                continue;
            }

            if line.starts_with("ARCH_PATTERN:") {
                let arch_pattern = self.parse_single_arch_pattern(line, &mut lines)?;
                patterns.push(ParsedPattern::Architecture(arch_pattern));
            } else if line.starts_with("TEST_CASE:") {
                self.skip_until_end_marker(&mut lines, "END_TEST")?;
            }
        }

        Ok(ParsedPatterns { metadata, patterns })
    }

    /// Parse a single architecture pattern
    fn parse_single_arch_pattern(
        &mut self,
        header: &str,
        lines: &mut std::iter::Peekable<std::iter::Enumerate<std::str::Lines>>,
    ) -> Result<ArchPattern, PatternParseError> {
        let name = header
            .strip_prefix("ARCH_PATTERN:")
            .ok_or(PatternParseError::InvalidSyntax(
                "Expected ARCH_PATTERN:".to_string(),
            ))?
            .trim()
            .to_string();

        let mut architecture = String::new();
        let mut _description = String::new();
        let mut sequence = Vec::new();
        let mut _confidence_boost = 0;

        while let Some((line_num, line)) = lines.peek() {
            self.current_line = line_num + 1;
            let line = line.trim();

            if line == "END_SEQUENCE" {
                lines.next(); // Consume END_SEQUENCE
                break;
            }

            lines.next(); // Consume the line

            if line.starts_with("ARCHITECTURE:") {
                architecture = line
                    .strip_prefix("ARCHITECTURE:")
                    .unwrap()
                    .trim()
                    .to_string();
            } else if line.starts_with("DESCRIPTION:") {
                _description = line
                    .strip_prefix("DESCRIPTION:")
                    .unwrap()
                    .trim()
                    .to_string();
            } else if line.starts_with("CONFIDENCE_BOOST:") {
                let boost_str = line.strip_prefix("CONFIDENCE_BOOST:").unwrap().trim();
                _confidence_boost = boost_str
                    .parse::<i8>()
                    .map_err(|_| PatternParseError::InvalidNumber(boost_str.to_string()))?;
            } else if line.starts_with("SEQUENCE:") {
                // Parse instruction sequence
                while let Some((_, seq_line)) = lines.peek() {
                    if seq_line.trim() == "END_SEQUENCE" {
                        break;
                    }
                    let instruction = lines.next().unwrap().1.trim();
                    if !instruction.is_empty() {
                        sequence.push(self.parse_instruction_pattern(instruction)?);
                    }
                }
            }
        }

        Ok(ArchPattern {
            name,
            arch: architecture,
            matcher: PatternMatcher::InstructionSequence(sequence),
        })
    }

    /// Parse an instruction pattern
    fn parse_instruction_pattern(
        &self,
        pattern: &str,
    ) -> Result<InstructionMatcher, PatternParseError> {
        let pattern = pattern.trim();

        // Handle optional prefix
        let (_is_optional, pattern) = if pattern.starts_with('?') {
            (true, &pattern[1..])
        } else {
            (false, pattern)
        };

        // Parse specific instruction patterns
        if pattern.starts_with("push ") {
            let reg = pattern.strip_prefix("push ").unwrap();
            Ok(InstructionMatcher::Push(Box::leak(
                reg.to_string().into_boxed_str(),
            )))
        } else if pattern.starts_with("pop ") {
            let reg = pattern.strip_prefix("pop ").unwrap();
            Ok(InstructionMatcher::Pop(Box::leak(
                reg.to_string().into_boxed_str(),
            )))
        } else if pattern.starts_with("mov ") {
            let parts: Vec<&str> = pattern.strip_prefix("mov ").unwrap().split(',').collect();
            if parts.len() == 2 {
                Ok(InstructionMatcher::MovReg(
                    Box::leak(parts[0].trim().to_string().into_boxed_str()),
                    Box::leak(parts[1].trim().to_string().into_boxed_str()),
                ))
            } else {
                Ok(InstructionMatcher::Any)
            }
        } else if pattern == "ret" {
            Ok(InstructionMatcher::Ret)
        } else if pattern == "leave" {
            Ok(InstructionMatcher::Leave)
        } else if pattern.contains('<') && pattern.contains('>') {
            // Wildcard pattern
            Ok(InstructionMatcher::Any)
        } else {
            // Generic instruction
            Ok(InstructionMatcher::Any)
        }
    }

    /// Skip lines until an end marker is found
    fn skip_until_end_marker(
        &mut self,
        lines: &mut std::iter::Peekable<std::iter::Enumerate<std::str::Lines>>,
        marker: &str,
    ) -> Result<(), PatternParseError> {
        while let Some((line_num, line)) = lines.next() {
            self.current_line = line_num + 1;
            if line.trim() == marker {
                return Ok(());
            }
        }
        Err(PatternParseError::UnexpectedEof(format!(
            "Expected {}",
            marker
        )))
    }
}

/// Parsed patterns from a file
#[derive(Debug)]
pub struct ParsedPatterns {
    /// File metadata
    pub metadata: PatternMetadata,
    /// Parsed patterns
    pub patterns: Vec<ParsedPattern>,
}

/// Pattern file metadata
#[derive(Debug)]
pub struct PatternMetadata {
    /// Version of the pattern file
    pub version: String,
    /// Target platform
    pub platform: Platform,
    /// Target architecture
    pub architecture: String,
}

impl Default for PatternMetadata {
    fn default() -> Self {
        Self {
            version: String::new(),
            platform: Platform::Generic,
            architecture: String::new(),
        }
    }
}

/// A parsed pattern
#[derive(Debug)]
pub enum ParsedPattern {
    /// Library function pattern
    Library(LibraryPattern),
    /// Code idiom pattern
    Idiom(IdiomPattern),
    /// Architecture-specific pattern
    Architecture(ArchPattern),
}

/// Pattern parsing errors
#[derive(Debug)]
pub enum PatternParseError {
    /// I/O error
    IoError(String),
    /// Invalid file extension
    InvalidExtension,
    /// Invalid syntax
    InvalidSyntax(String),
    /// Unknown calling convention
    UnknownConvention(String),
    /// Unknown behavior
    UnknownBehavior(String),
    /// Invalid number
    InvalidNumber(String),
    /// Unexpected end of file
    UnexpectedEof(String),
}

impl std::fmt::Display for PatternParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternParseError::IoError(e) => write!(f, "I/O error: {}", e),
            PatternParseError::InvalidExtension => write!(f, "Invalid file extension"),
            PatternParseError::InvalidSyntax(s) => write!(f, "Invalid syntax: {}", s),
            PatternParseError::UnknownConvention(c) => {
                write!(f, "Unknown calling convention: {}", c)
            }
            PatternParseError::UnknownBehavior(b) => write!(f, "Unknown behavior: {}", b),
            PatternParseError::InvalidNumber(n) => write!(f, "Invalid number: {}", n),
            PatternParseError::UnexpectedEof(s) => write!(f, "Unexpected end of file: {}", s),
        }
    }
}

impl std::error::Error for PatternParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_library_pattern() {
        let content = r#"
# Version: 1.0
# Platform: generic

FUNCTION: libc::strlen
  RETURNS: u64
  PARAM: str char*
  CONVENTION: cdecl
  BEHAVIOR: pure
"#;

        let mut parser = PatternParser::new();
        let result = parser.parse_library_patterns(content).unwrap();

        assert_eq!(result.patterns.len(), 1);
        if let ParsedPattern::Library(lib) = &result.patterns[0] {
            assert_eq!(lib.name, "strlen");
            assert_eq!(lib.library, "libc");
        } else {
            panic!("Expected library pattern");
        }
    }

    #[test]
    fn test_parse_type() {
        let parser = PatternParser::new();

        // Test primitive types
        assert!(matches!(
            parser.parse_type("int").unwrap(),
            TypeRef::Primitive(PrimitiveType::I32)
        ));

        // Test pointer types
        assert!(matches!(
            parser.parse_type("char*").unwrap(),
            TypeRef::Pointer(_)
        ));

        // Test array types
        assert!(matches!(
            parser.parse_type("int[10]").unwrap(),
            TypeRef::Array { .. }
        ));
    }
}
