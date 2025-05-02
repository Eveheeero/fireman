use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn call() -> &'static [IrStatement] {
    let set_sp = assign(
        b::sub(r(&rsp), architecture_byte_size()),
        r(&rsp),
        size_architecture(),
    );
    let ret_address = b::add(r(&rip), instruction_byte_size());
    let save_ret = assign(ret_address, d(r(&rsp)), size_architecture());
    let call = super::shortcuts::call(o1());
    [set_sp, save_ret, call].into()
}

#[box_to_static_reference]
pub(super) fn cmp() -> &'static [IrStatement] {
    let sub = b::sub(o1(), u::sign_extend(o2()));
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}
