use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

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
    [set_sp, save_ret, call].into()
}

#[box_to_static_reference]
pub(super) fn cmp() -> &'static [IrStatement] {
    let sub = b::sub(o1(), u::sign_extend(o2()));
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

#[box_to_static_reference]
pub(super) fn cmps() -> &'static [IrStatement] {
    let source = d(rsi.clone());
    let destination = d(rdi.clone());
    let sub = b::sub(source, u::sign_extend(destination));
    let calc_flags =
        calc_flags_automatically(sub, size_architecture(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}
#[box_to_static_reference]
pub(super) fn cmpsb() -> &'static [IrStatement] {
    let source = d(rsi.clone());
    let destination = d(rdi.clone());
    let sub = b::sub(source, u::sign_extend(destination));
    let calc_flags =
        calc_flags_automatically(sub, size_result_byte(c(1)), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}
#[box_to_static_reference]
pub(super) fn cmpsw() -> &'static [IrStatement] {
    let source = d(rsi.clone());
    let destination = d(rdi.clone());
    let sub = b::sub(source, u::sign_extend(destination));
    let calc_flags =
        calc_flags_automatically(sub, size_result_byte(c(2)), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}
#[box_to_static_reference]
pub(super) fn cmpsd() -> &'static [IrStatement] {
    let source = d(rsi.clone());
    let destination = d(rdi.clone());
    let sub = b::sub(source, u::sign_extend(destination));
    let calc_flags =
        calc_flags_automatically(sub, size_result_byte(c(4)), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}
#[box_to_static_reference]
pub(super) fn cmpsq() -> &'static [IrStatement] {
    let source = d(rsi.clone());
    let destination = d(rdi.clone());
    let sub = b::sub(source, u::sign_extend(destination));
    let calc_flags =
        calc_flags_automatically(sub, size_result_byte(c(8)), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}
