//! Module containing implementations for multiple architectures

pub mod architecture;
pub mod arm32;
pub mod arm64;
pub mod x86;
pub mod x86_64;

// Re-export key types
pub use architecture::{
    ArchType, ArchitectureContext, ArchitectureDetector, ArchitectureInfo, Endianness,
};
