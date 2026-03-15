use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// bits(64) address = if n == 31 then SP[] else X[n];
/// bits(64) mask = X[m];
/// bits(4) tag = AArch64.AllocationTagFromAddress(address);
/// 
/// mask<UInt(tag)> = '1';
/// X[d] = mask;
/// ```
#[box_to_static_reference]
pub(super) fn gmi() -> &'static [IrStatement] {
    [exception("gmi")].into()
}
