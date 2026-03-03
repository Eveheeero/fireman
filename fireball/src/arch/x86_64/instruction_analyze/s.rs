use super::{super::static_register::*, shortcuts::*};
use crate::utils::Aos;
use std::ops::Deref;

#[inline]
fn setcc(condition_data: Aos<IrData>) -> IrStatement {
    condition(
        condition_data,
        [assign(c(1), o1(), o1_size())],
        [assign(c(0), o1(), o1_size())],
    )
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
pub(super) fn shl() -> &'static [IrStatement] {
    let shl_1 = b::shl(o1(), o2());
    let shl_1_flags = calc_flags_automatically(shl_1.clone(), o1_size(), &[&sf, &zf, &pf]);
    let shl_2 = b::shl(o1(), c(1));
    let shl_2_flags = calc_flags_automatically(shl_2.clone(), o1_size(), &[&sf, &zf, &pf]);
    let condition = condition(
        is_o2_exists(),
        [shl_1_flags, assign(shl_1, o1(), o1_size())],
        [shl_2_flags, assign(shl_2, o1(), o1_size())],
    );
    extend_undefined_flags(&[condition], &[&of, &af, &cf])
}

#[box_to_static_reference]
pub(super) fn shr() -> &'static [IrStatement] {
    let shr_1 = b::shr(o1(), o2());
    let shr_1_flags = calc_flags_automatically(shr_1.clone(), o1_size(), &[&sf, &zf, &pf]);
    let shr_2 = b::shr(o1(), c(1));
    let shr_2_flags = calc_flags_automatically(shr_2.clone(), o1_size(), &[&sf, &zf, &pf]);
    let condition = condition(
        is_o2_exists(),
        [shr_1_flags, assign(shr_1, o1(), o1_size())],
        [shr_2_flags, assign(shr_2, o1(), o1_size())],
    );
    extend_undefined_flags(&[condition], &[&of, &af, &cf])
}

#[box_to_static_reference]
pub(super) fn sar() -> &'static [IrStatement] {
    let sar_1 = b::sar(o1(), o2());
    let sar_1_flags = calc_flags_automatically(sar_1.clone(), o1_size(), &[&sf, &zf, &pf]);
    let sar_2 = b::sar(o1(), c(1));
    let sar_2_flags = calc_flags_automatically(sar_2.clone(), o1_size(), &[&sf, &zf, &pf]);
    let condition = condition(
        is_o2_exists(),
        [sar_1_flags, assign(sar_1, o1(), o1_size())],
        [sar_2_flags, assign(sar_2, o1(), o1_size())],
    );
    extend_undefined_flags(&[condition], &[&of, &af, &cf])
}

#[box_to_static_reference]
pub(super) fn sbb() -> &'static [IrStatement] {
    let size = o1_size();
    let sub = b::sub(o1(), o2());
    let sub = b::sub(sub, u::zero_extend(cf.clone()));
    let assignment = assign(sub.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(sub, size, &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    let type3 = type_specified(cf.clone(), o1_size(), DataType::Int);
    [calc_flags, assignment, type1, type2, type3].into()
}

#[box_to_static_reference]
pub(super) fn sub() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let assignment = assign(sub.clone(), o1(), o1_size());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags, assignment].into()
}

#[box_to_static_reference]
pub(super) fn seta() -> &'static [IrStatement] {
    let cond = b::and(u::not(cf.clone()), u::not(zf.clone()));
    [setcc(cond)].into()
}

#[box_to_static_reference]
pub(super) fn setae() -> &'static [IrStatement] {
    [setcc(u::not(cf.clone()))].into()
}

#[box_to_static_reference]
pub(super) fn setb() -> &'static [IrStatement] {
    [setcc(cf.clone())].into()
}

#[box_to_static_reference]
pub(super) fn setbe() -> &'static [IrStatement] {
    let cond = b::or(cf.clone(), zf.clone());
    [setcc(cond)].into()
}

#[box_to_static_reference]
pub(super) fn sete() -> &'static [IrStatement] {
    [setcc(zf.clone())].into()
}

#[box_to_static_reference]
pub(super) fn setg() -> &'static [IrStatement] {
    let cond = b::and(u::not(zf.clone()), sf_eq_of());
    [setcc(cond)].into()
}

#[box_to_static_reference]
pub(super) fn setge() -> &'static [IrStatement] {
    [setcc(sf_eq_of())].into()
}

#[box_to_static_reference]
pub(super) fn setl() -> &'static [IrStatement] {
    [setcc(sf_ne_of())].into()
}

#[box_to_static_reference]
pub(super) fn setle() -> &'static [IrStatement] {
    let cond = b::or(zf.clone(), sf_ne_of());
    [setcc(cond)].into()
}

#[box_to_static_reference]
pub(super) fn setne() -> &'static [IrStatement] {
    [setcc(u::not(zf.clone()))].into()
}

#[box_to_static_reference]
pub(super) fn setno() -> &'static [IrStatement] {
    [setcc(u::not(of.clone()))].into()
}

#[box_to_static_reference]
pub(super) fn setnp() -> &'static [IrStatement] {
    [setcc(u::not(pf.clone()))].into()
}

#[box_to_static_reference]
pub(super) fn setns() -> &'static [IrStatement] {
    [setcc(u::not(sf.clone()))].into()
}

#[box_to_static_reference]
pub(super) fn seto() -> &'static [IrStatement] {
    [setcc(of.clone())].into()
}

#[box_to_static_reference]
pub(super) fn setp() -> &'static [IrStatement] {
    [setcc(pf.clone())].into()
}

#[box_to_static_reference]
pub(super) fn sets() -> &'static [IrStatement] {
    [setcc(sf.clone())].into()
}

#[inline]
pub(super) fn setnb() -> &'static [IrStatement] {
    setae()
}

#[inline]
pub(super) fn setnbe() -> &'static [IrStatement] {
    seta()
}

#[inline]
pub(super) fn setnl() -> &'static [IrStatement] {
    setge()
}

#[inline]
pub(super) fn setnle() -> &'static [IrStatement] {
    setg()
}

#[inline]
pub(super) fn setnz() -> &'static [IrStatement] {
    setne()
}

#[inline]
pub(super) fn setz() -> &'static [IrStatement] {
    sete()
}

#[inline]
pub(super) fn setnc() -> &'static [IrStatement] {
    setae()
}

#[inline]
pub(super) fn setc() -> &'static [IrStatement] {
    setb()
}
