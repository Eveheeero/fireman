use super::{super::static_register::*, shortcuts::*};
use crate::utils::Aos;
use std::ops::Deref;

#[inline]
fn jcc(condition_data: Aos<IrData>) -> IrStatement {
    let fallthrough = b::add(rip.clone(), instruction_byte_size());
    condition(condition_data, [jump(o1())], [jump(fallthrough)])
}

#[inline]
fn sf_eq_of() -> Aos<IrData> {
    b::equal(sf.clone(), of.clone(), size_relative(sf.clone()))
}

#[inline]
fn sf_ne_of() -> Aos<IrData> {
    u::not(sf_eq_of())
}

#[box_to_static_reference]
pub(super) fn jmp() -> &'static [IrStatement] {
    let jmp = jump(o1());
    [jmp].into()
}

#[box_to_static_reference]
pub(super) fn ja() -> &'static [IrStatement] {
    let cond = b::and(u::not(cf.clone()), u::not(zf.clone()));
    [jcc(cond)].into()
}

#[box_to_static_reference]
pub(super) fn jae() -> &'static [IrStatement] {
    let cond = u::not(cf.clone());
    [jcc(cond)].into()
}

#[box_to_static_reference]
pub(super) fn jb() -> &'static [IrStatement] {
    [jcc(cf.clone())].into()
}

#[box_to_static_reference]
pub(super) fn jbe() -> &'static [IrStatement] {
    let cond = b::or(cf.clone(), zf.clone());
    [jcc(cond)].into()
}

#[box_to_static_reference]
pub(super) fn jz() -> &'static [IrStatement] {
    [jcc(zf.clone())].into()
}

#[inline]
pub(super) fn je() -> &'static [IrStatement] {
    jz()
}

#[box_to_static_reference]
pub(super) fn jg() -> &'static [IrStatement] {
    let cond = b::and(u::not(zf.clone()), sf_eq_of());
    [jcc(cond)].into()
}

#[box_to_static_reference]
pub(super) fn jge() -> &'static [IrStatement] {
    [jcc(sf_eq_of())].into()
}

#[box_to_static_reference]
pub(super) fn jl() -> &'static [IrStatement] {
    [jcc(sf_ne_of())].into()
}

#[box_to_static_reference]
pub(super) fn jle() -> &'static [IrStatement] {
    let cond = b::or(zf.clone(), sf_ne_of());
    [jcc(cond)].into()
}

#[box_to_static_reference]
pub(super) fn jnz() -> &'static [IrStatement] {
    [jcc(u::not(zf.clone()))].into()
}

#[inline]
pub(super) fn jne() -> &'static [IrStatement] {
    jnz()
}

#[box_to_static_reference]
pub(super) fn jno() -> &'static [IrStatement] {
    [jcc(u::not(of.clone()))].into()
}

#[box_to_static_reference]
pub(super) fn jnp() -> &'static [IrStatement] {
    [jcc(u::not(pf.clone()))].into()
}

#[box_to_static_reference]
pub(super) fn jns() -> &'static [IrStatement] {
    [jcc(u::not(sf.clone()))].into()
}

#[box_to_static_reference]
pub(super) fn jo() -> &'static [IrStatement] {
    [jcc(of.clone())].into()
}

#[box_to_static_reference]
pub(super) fn jp() -> &'static [IrStatement] {
    [jcc(pf.clone())].into()
}

#[box_to_static_reference]
pub(super) fn js() -> &'static [IrStatement] {
    [jcc(sf.clone())].into()
}
