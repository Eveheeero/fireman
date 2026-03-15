use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// IF DEST = 0
///     THEN CF := 0;
///     ELSE CF := 1;
/// FI;
/// DEST := [- (DEST)]
/// ```
#[box_to_static_reference]
pub(super) fn neg() -> &'static [IrStatement] {
    let neg = u::neg(o1());
    let calc_flags = calc_flags_automatically(neg.clone(), o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    let assignment = assign(neg, o1(), o1_size());
    [calc_flags, assignment].into()
}

/// # Pseudocode
/// ```text
/// The one-byte NOP instruction is an alias mnemonic for the XCHG (E)AX, (E)AX instruction.
/// The multi-byte NOP instruction performs no operation on supported processors and generates undefined opcode
/// exception on processors that do not support the multi-byte NOP instruction.
/// The memory operand form of the instruction allows software to create a byte sequence of "no operation" as one
/// instruction. For situations where multiple-byte NOPs are needed, the recommended operations (32-bit mode and
/// 64-bit mode) are:
/// ```
#[box_to_static_reference]
pub(super) fn nop() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// DEST := NOT DEST;
/// ```
#[box_to_static_reference]
pub(super) fn not() -> &'static [IrStatement] {
    let not_val = u::not(o1());
    let assignment = assign(not_val, o1(), o1_size());
    [assignment].into()
}
