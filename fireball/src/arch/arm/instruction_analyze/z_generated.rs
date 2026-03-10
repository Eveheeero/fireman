use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// 
/// integer base = part * pairs;
/// 
/// for p = 0 to pairs-1
///     Elem[result, 2*p+0, esize] = Elem[operand1, base+p, esize];
///     Elem[result, 2*p+1, esize] = Elem[operand2, base+p, esize];
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn zip1() -> &'static [IrStatement] {
    [exception("zip1")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// 
/// integer base = part * pairs;
/// 
/// for p = 0 to pairs-1
///     Elem[result, 2*p+0, esize] = Elem[operand1, base+p, esize];
///     Elem[result, 2*p+1, esize] = Elem[operand2, base+p, esize];
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn zip2() -> &'static [IrStatement] {
    [exception("zip2")].into()
}
