use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// bit N = NOT(PSTATE.C) AND NOT(PSTATE.Z);
/// bit Z = PSTATE.Z AND PSTATE.C;
/// bit C = PSTATE.C OR PSTATE.Z;
/// bit V = NOT(PSTATE.C) AND PSTATE.Z;
/// 
/// PSTATE.N = N;
/// PSTATE.Z = Z;
/// PSTATE.C = C;
/// PSTATE.V = V;
/// ```
#[box_to_static_reference]
pub(super) fn xaflag() -> &'static [IrStatement] {
    let old_c = pstate_c.clone();
    let old_z = pstate_z.clone();
    let set_n = assign(b::and(u::not(old_c.clone()), u::not(old_z.clone())), pstate_n.clone(), size_relative(pstate_n.clone()));
    let set_z = assign(b::and(old_z.clone(), old_c.clone()), pstate_z.clone(), size_relative(pstate_z.clone()));
    let set_c = assign(b::or(old_c.clone(), old_z.clone()), pstate_c.clone(), size_relative(pstate_c.clone()));
    let set_v = assign(b::and(u::not(old_c.clone()), old_z.clone()), pstate_v.clone(), size_relative(pstate_v.clone()));
    [set_n, set_z, set_c, set_v].into()
}

/// # Pseudocode
/// ```text
/// AArch64.CheckFPAdvSIMDEnabled();
/// 
/// bits(128) Vm = V[m];
/// bits(128) Vn = V[n];
/// bits(128) tmp;
/// tmp = Vn EOR Vm;
/// V[d] = ROR(tmp<127:64>, UInt(imm6)):ROR(tmp<63:0>, UInt(imm6));
/// ```
#[box_to_static_reference]
pub(super) fn xar() -> &'static [IrStatement] {
    [exception("xar")].into()
}

/// # Pseudocode
/// ```text
/// if HavePACExt() then
///     X[d] = Strip(X[d], data);
/// ```
#[box_to_static_reference]
pub(super) fn xpacd() -> &'static [IrStatement] {
    [exception("xpacd")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(2*datasize) operand = V[n];
/// bits(datasize) result;
/// bits(2*esize) element;
/// 
/// for e = 0 to elements-1
///     element = Elem[operand, e, 2*esize];
///     Elem[result, e, esize] = element<esize-1:0>;
/// Vpart[d, part] = result;
/// ```
#[box_to_static_reference]
pub(super) fn xtn() -> &'static [IrStatement] {
    [exception("xtn")].into()
}
