//! Module defining branch relationships discovered during analysis

use crate::core::Address;

/// Represents a connection between this code block and another (e.g., jmp, call)
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Relation {
    /// ID of the source block for this connection
    from: usize,
    /// Address of the destination block
    to: Option<Address>,
    /// Type of the destination address
    destination_type: DestinationType,
    /// Type of this relation
    relation_type: RelationType,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum DestinationType {
    /// Static address
    Static,
    /// Dynamic address
    Dynamic,
}

/// Types of connections between code blocks
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum RelationType {
    /// Indicates a call connection
    Call,
    /// Indicates a halt connection (by return)
    Halt,
    /// Indicates a jump connection
    Jump,
    Jcc,
    /// Indicates the block continues into multiple blocks
    Continued,
    /// Indicates a return connection
    Return,
}

impl Relation {
    /// Creates a new relation.
    ///
    /// ### Arguments
    /// - `from: usize` - ID of the source block
    /// - `to: Option<Address>` - Address of the destination block
    /// - `destination_type: DestinationType` - Type of the destination address
    /// - `relation_type: RelationType` - Type of the connection
    ///
    /// ### Returns
    /// - `Self` - newly created relation
    pub fn new(
        from: usize,
        to: Option<Address>,
        destination_type: DestinationType,
        relation_type: RelationType,
    ) -> Self {
        Self {
            from,
            to,
            destination_type,
            relation_type,
        }
    }

    /// Returns the source block ID.
    ///
    /// ### Returns
    /// - `usize` - source block ID
    pub fn from(&self) -> usize {
        self.from
    }

    /// Returns the destination address.
    ///
    /// ### Returns
    /// - `Option<Address>` - destination address
    pub fn to(&self) -> Option<Address> {
        self.to.clone()
    }

    pub fn destination_type(&self) -> &DestinationType {
        &self.destination_type
    }
    pub fn relation_type(&self) -> &RelationType {
        &self.relation_type
    }
}
