use super::{super::static_register::*, shortcuts::*};
use crate::utils::Aos;
use std::ops::Deref;

#[inline]
fn cmovcc(condition_data: Aos<IrData>) -> IrStatement {
    condition(condition_data, [assign(o2(), o1(), o1_size())], [])
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
pub(super) fn call() -> &'static [IrStatement] {
    let set_sp = assign(
        b::sub(rsp.clone(), architecture_byte_size()),
        rsp.clone(),
        size_architecture(),
    );
    let ret_address = b::add(rip.clone(), instruction_byte_size());
    let save_ret = assign(ret_address, d(rsp.clone()), size_architecture());
    let call = jump_by_call(o1());
    let type1 = type_specified(o1(), o1_size(), DataType::Address);
    let type2 = type_specified(rsp.clone(), size_architecture(), DataType::Address);
    let type3 = type_specified(rip.clone(), size_architecture(), DataType::Address);
    [set_sp, save_ret, call, type1, type2, type3].into()
}

#[box_to_static_reference]
pub(super) fn cmp() -> &'static [IrStatement] {
    let sub = b::sub(o1(), u::sign_extend(o2()));
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn cmps() -> &'static [IrStatement] {
    let source = d(rsi.clone());
    let destination = d(rdi.clone());
    let sub = b::sub(source.clone(), u::sign_extend(destination.clone()));
    let calc_flags =
        calc_flags_automatically(sub, size_architecture(), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(source, size_architecture(), DataType::Int);
    let type2 = type_specified(destination, size_architecture(), DataType::Int);
    let type3 = type_specified(rsi.clone(), size_architecture(), DataType::Address);
    let type4 = type_specified(rdi.clone(), size_architecture(), DataType::Address);
    [calc_flags, type1, type2, type3, type4].into()
}
#[box_to_static_reference]
pub(super) fn cmpsb() -> &'static [IrStatement] {
    let source = d(rsi.clone());
    let destination = d(rdi.clone());
    let sub = b::sub(source.clone(), u::sign_extend(destination.clone()));
    let calc_flags =
        calc_flags_automatically(sub, size_result_byte(c(1)), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(source, size_architecture(), DataType::Int);
    let type2 = type_specified(destination, size_architecture(), DataType::Int);
    let type3 = type_specified(rsi.clone(), size_architecture(), DataType::Address);
    let type4 = type_specified(rdi.clone(), size_architecture(), DataType::Address);
    [calc_flags, type1, type2, type3, type4].into()
}
#[box_to_static_reference]
pub(super) fn cmpsw() -> &'static [IrStatement] {
    let source = d(rsi.clone());
    let destination = d(rdi.clone());
    let sub = b::sub(source.clone(), u::sign_extend(destination.clone()));
    let calc_flags =
        calc_flags_automatically(sub, size_result_byte(c(2)), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(source, size_architecture(), DataType::Int);
    let type2 = type_specified(destination, size_architecture(), DataType::Int);
    let type3 = type_specified(rsi.clone(), size_architecture(), DataType::Address);
    let type4 = type_specified(rdi.clone(), size_architecture(), DataType::Address);
    [calc_flags, type1, type2, type3, type4].into()
}
#[box_to_static_reference]
pub(super) fn cmpsd() -> &'static [IrStatement] {
    let source = d(rsi.clone());
    let destination = d(rdi.clone());
    let sub = b::sub(source.clone(), u::sign_extend(destination.clone()));
    let calc_flags =
        calc_flags_automatically(sub, size_result_byte(c(4)), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(source, size_architecture(), DataType::Int);
    let type2 = type_specified(destination, size_architecture(), DataType::Int);
    let type3 = type_specified(rsi.clone(), size_architecture(), DataType::Address);
    let type4 = type_specified(rdi.clone(), size_architecture(), DataType::Address);
    [calc_flags, type1, type2, type3, type4].into()
}
#[box_to_static_reference]
pub(super) fn cmpsq() -> &'static [IrStatement] {
    let source = d(rsi.clone());
    let destination = d(rdi.clone());
    let sub = b::sub(source.clone(), u::sign_extend(destination.clone()));
    let calc_flags =
        calc_flags_automatically(sub, size_result_byte(c(8)), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(source, size_architecture(), DataType::Int);
    let type2 = type_specified(destination, size_architecture(), DataType::Int);
    let type3 = type_specified(rsi.clone(), size_architecture(), DataType::Address);
    let type4 = type_specified(rdi.clone(), size_architecture(), DataType::Address);
    [calc_flags, type1, type2, type3, type4].into()
}

#[box_to_static_reference]
pub(super) fn clc() -> &'static [IrStatement] {
    let set_cf = assign(c(0), cf.clone(), size_relative(cf.clone()));
    [set_cf].into()
}
#[box_to_static_reference]
pub(super) fn cld() -> &'static [IrStatement] {
    let set_df = assign(c(0), df.clone(), size_relative(df.clone()));
    [set_df].into()
}

#[box_to_static_reference]
pub(super) fn cmc() -> &'static [IrStatement] {
    let set_cl = assign(u::not(cl.clone()), cl.clone(), size_relative(cl.clone()));
    [set_cl].into()
}

#[box_to_static_reference]
pub(super) fn cbw() -> &'static [IrStatement] {
    let set_ax = assign(
        u::sign_extend(al.clone()),
        ax.clone(),
        size_relative(ax.clone()),
    );
    let type1 = type_specified(al.clone(), size_relative(al.clone()), DataType::Int);
    let type2 = type_specified(ax.clone(), size_relative(ax.clone()), DataType::Int);
    [set_ax, type1, type2].into()
}
#[box_to_static_reference]
pub(super) fn cwde() -> &'static [IrStatement] {
    let set_ax = assign(
        u::sign_extend(ax.clone()),
        eax.clone(),
        size_relative(eax.clone()),
    );
    let type1 = type_specified(ax.clone(), size_relative(ax.clone()), DataType::Int);
    let type2 = type_specified(eax.clone(), size_relative(eax.clone()), DataType::Int);
    [set_ax, type1, type2].into()
}
#[box_to_static_reference]
pub(super) fn cdqe() -> &'static [IrStatement] {
    let set_ax = assign(
        u::sign_extend(eax.clone()),
        rax.clone(),
        size_relative(rax.clone()),
    );
    let type1 = type_specified(eax.clone(), size_relative(eax.clone()), DataType::Int);
    let type2 = type_specified(rax.clone(), size_relative(rax.clone()), DataType::Int);
    [set_ax, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn cmpxchg() -> &'static [IrStatement] {
    let cond = b::equal(rax.clone(), d(o1()), o1_size());
    let true_b = [assign(o2(), d(o1()), o1_size())];
    let false_b = [assign(d(o1()), rax.clone(), o1_size())];
    let cmpxchg = condition(cond.clone(), true_b, false_b);
    let calc_flags = calc_flags_automatically(
        cond,
        size_result_byte(c(1)),
        &[&of, &sf, &zf, &af, &cf, &pf],
    );
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    let type3 = type_specified(rax.clone(), size_relative(rax.clone()), DataType::Int);
    [calc_flags, cmpxchg, type1, type2, type3].into()
}

#[box_to_static_reference]
pub(super) fn cwd() -> &'static [IrStatement] {
    let set_tmp = assign(
        u::sign_extend(ax.clone()),
        tmp32.clone(),
        size_relative(tmp32.clone()),
    );
    let set_dx = assign(
        b::shr(tmp32.clone(), c(16)),
        dx.clone(),
        size_relative(dx.clone()),
    );
    let set_ax = assign(tmp32.clone(), ax.clone(), size_relative(ax.clone()));
    let type1 = type_specified(ax.clone(), size_relative(ax.clone()), DataType::Int);
    let type2 = type_specified(dx.clone(), size_relative(dx.clone()), DataType::Int);
    [set_tmp, set_dx, set_ax, type1, type2].into()
}
#[box_to_static_reference]
pub(super) fn cdq() -> &'static [IrStatement] {
    let set_tmp = assign(
        u::sign_extend(eax.clone()),
        tmp64.clone(),
        size_relative(tmp64.clone()),
    );
    let set_dx = assign(
        b::shr(tmp64.clone(), c(16)),
        edx.clone(),
        size_relative(edx.clone()),
    );
    let set_ax = assign(tmp64.clone(), eax.clone(), size_relative(eax.clone()));
    let type1 = type_specified(eax.clone(), size_relative(eax.clone()), DataType::Int);
    let type2 = type_specified(edx.clone(), size_relative(edx.clone()), DataType::Int);
    [set_tmp, set_dx, set_ax, type1, type2].into()
}
#[box_to_static_reference]
pub(super) fn cqo() -> &'static [IrStatement] {
    let set_tmp = assign(
        u::sign_extend(rax.clone()),
        tmp128.clone(),
        size_relative(tmp128.clone()),
    );
    let set_dx = assign(
        b::shr(tmp128.clone(), c(16)),
        rdx.clone(),
        size_relative(rdx.clone()),
    );
    let set_ax = assign(tmp128.clone(), rax.clone(), size_relative(rax.clone()));
    let type1 = type_specified(rax.clone(), size_relative(rax.clone()), DataType::Int);
    let type2 = type_specified(rdx.clone(), size_relative(rdx.clone()), DataType::Int);
    [set_tmp, set_dx, set_ax, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn cmova() -> &'static [IrStatement] {
    let cond = b::and(u::not(cf.clone()), u::not(zf.clone()));
    [cmovcc(cond)].into()
}

#[box_to_static_reference]
pub(super) fn cmovae() -> &'static [IrStatement] {
    [cmovcc(u::not(cf.clone()))].into()
}

#[box_to_static_reference]
pub(super) fn cmovb() -> &'static [IrStatement] {
    [cmovcc(cf.clone())].into()
}

#[box_to_static_reference]
pub(super) fn cmovbe() -> &'static [IrStatement] {
    let cond = b::or(cf.clone(), zf.clone());
    [cmovcc(cond)].into()
}

#[box_to_static_reference]
pub(super) fn cmove() -> &'static [IrStatement] {
    [cmovcc(zf.clone())].into()
}

#[box_to_static_reference]
pub(super) fn cmovg() -> &'static [IrStatement] {
    let cond = b::and(u::not(zf.clone()), sf_eq_of());
    [cmovcc(cond)].into()
}

#[box_to_static_reference]
pub(super) fn cmovge() -> &'static [IrStatement] {
    [cmovcc(sf_eq_of())].into()
}

#[box_to_static_reference]
pub(super) fn cmovl() -> &'static [IrStatement] {
    [cmovcc(sf_ne_of())].into()
}

#[box_to_static_reference]
pub(super) fn cmovle() -> &'static [IrStatement] {
    let cond = b::or(zf.clone(), sf_ne_of());
    [cmovcc(cond)].into()
}

#[box_to_static_reference]
pub(super) fn cmovne() -> &'static [IrStatement] {
    [cmovcc(u::not(zf.clone()))].into()
}

#[box_to_static_reference]
pub(super) fn cmovno() -> &'static [IrStatement] {
    [cmovcc(u::not(of.clone()))].into()
}

#[box_to_static_reference]
pub(super) fn cmovnp() -> &'static [IrStatement] {
    [cmovcc(u::not(pf.clone()))].into()
}

#[box_to_static_reference]
pub(super) fn cmovns() -> &'static [IrStatement] {
    [cmovcc(u::not(sf.clone()))].into()
}

#[box_to_static_reference]
pub(super) fn cmovo() -> &'static [IrStatement] {
    [cmovcc(of.clone())].into()
}

#[box_to_static_reference]
pub(super) fn cmovp() -> &'static [IrStatement] {
    [cmovcc(pf.clone())].into()
}

#[box_to_static_reference]
pub(super) fn cmovs() -> &'static [IrStatement] {
    [cmovcc(sf.clone())].into()
}

#[inline]
pub(super) fn cmovnb() -> &'static [IrStatement] {
    cmovae()
}

#[inline]
pub(super) fn cmovnbe() -> &'static [IrStatement] {
    cmova()
}

#[inline]
pub(super) fn cmovnl() -> &'static [IrStatement] {
    cmovge()
}

#[inline]
pub(super) fn cmovnle() -> &'static [IrStatement] {
    cmovg()
}

#[inline]
pub(super) fn cmovnz() -> &'static [IrStatement] {
    cmovne()
}

#[inline]
pub(super) fn cmovz() -> &'static [IrStatement] {
    cmove()
}

#[inline]
pub(super) fn cmovnc() -> &'static [IrStatement] {
    cmovae()
}

#[inline]
pub(super) fn cmovc() -> &'static [IrStatement] {
    cmovb()
}
