#![allow(dead_code)]
//! Module containing core traits and structures

mod address;
mod block;
mod blocks;
mod deterministic;
mod fire;
mod fire_raw;
mod instruction;
mod pre_defined_offset;
mod pre_defined_offsets;
mod relation;
mod relations;
mod section;
mod sections;

pub use address::Address;
pub use block::Block;
pub(crate) use blocks::BlockRelationInformation;
pub use blocks::Blocks;
pub use deterministic::{
    BinaryOp, BlockId, DeterministicNamer, FunctionId, GlobalId, LocalId, TempAllocator,
    canonicalize_commutative,
};
pub use fire::Fire;
pub use fire_raw::FireRaw;
pub use instruction::Instruction;
pub use pre_defined_offset::PreDefinedOffset;
pub use pre_defined_offsets::PreDefinedOffsets;
pub use relation::{DestinationType, Relation, RelationType};
pub use relations::Relations;
pub use section::Section;
pub use sections::Sections;
