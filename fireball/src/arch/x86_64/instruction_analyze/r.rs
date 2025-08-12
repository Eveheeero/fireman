use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn ret() -> &'static [IrStatement] {
    let jmp = jump(d(rsp.clone()));
    let set_sp = assign(
        b::add(rsp.clone(), architecture_byte_size()),
        rsp.clone(),
        size_architecture(),
    );
    let operand_condition = condition(
        is_o1_exists(),
        [assign(
            b::add(rsp.clone(), u::zero_extend(o1())),
            rsp.clone(),
            size_architecture(),
        )],
        [],
    );
    let halt = halt();
    [set_sp, operand_condition, jmp, halt].into()
}
