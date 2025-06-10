use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn jmp() -> &'static [IrStatement] {
    let jmp = jump(o1());
    [jmp].into()
}

#[box_to_static_reference]
pub(super) fn je() -> &'static [IrStatement] {
    // Jump if equal (ZF = 1)
    let jmp = jump(o1());
    let cond_jmp = condition(zf.clone(), vec![jmp], vec![]);
    [cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn jne() -> &'static [IrStatement] {
    // Jump if not equal (ZF = 0)
    let jmp = jump(o1());
    let cond_jmp = condition(u::not(zf.clone()), vec![jmp], vec![]);
    [cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn jl() -> &'static [IrStatement] {
    // Jump if less (SF != OF)
    let jmp = jump(o1());
    let cond = b::xor(sf.clone(), of.clone());
    let cond_jmp = condition(cond, vec![jmp], vec![]);
    [cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn jle() -> &'static [IrStatement] {
    // Jump if less or equal (ZF = 1 OR SF != OF)
    let jmp = jump(o1());
    let less_cond = b::xor(sf.clone(), of.clone());
    let cond = b::or(zf.clone(), less_cond);
    let cond_jmp = condition(cond, vec![jmp], vec![]);
    [cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn jg() -> &'static [IrStatement] {
    // Jump if greater (ZF = 0 AND SF = OF)
    let jmp = jump(o1());
    let equal_flags = u::not(b::xor(sf.clone(), of.clone()));
    let cond = b::and(u::not(zf.clone()), equal_flags);
    let cond_jmp = condition(cond, vec![jmp], vec![]);
    [cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn jge() -> &'static [IrStatement] {
    // Jump if greater or equal (SF = OF)
    let jmp = jump(o1());
    let cond = u::not(b::xor(sf.clone(), of.clone()));
    let cond_jmp = condition(cond, vec![jmp], vec![]);
    [cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn ja() -> &'static [IrStatement] {
    // Jump if above (CF = 0 AND ZF = 0)
    let jmp = jump(o1());
    let cond = b::and(u::not(cf.clone()), u::not(zf.clone()));
    let cond_jmp = condition(cond, vec![jmp], vec![]);
    [cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn jae() -> &'static [IrStatement] {
    // Jump if above or equal (CF = 0)
    let jmp = jump(o1());
    let cond_jmp = condition(u::not(cf.clone()), vec![jmp], vec![]);
    [cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn jb() -> &'static [IrStatement] {
    // Jump if below (CF = 1)
    let jmp = jump(o1());
    let cond_jmp = condition(cf.clone(), vec![jmp], vec![]);
    [cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn jbe() -> &'static [IrStatement] {
    // Jump if below or equal (CF = 1 OR ZF = 1)
    let jmp = jump(o1());
    let cond = b::or(cf.clone(), zf.clone());
    let cond_jmp = condition(cond, vec![jmp], vec![]);
    [cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn js() -> &'static [IrStatement] {
    // Jump if sign (SF = 1)
    let jmp = jump(o1());
    let cond_jmp = condition(sf.clone(), vec![jmp], vec![]);
    [cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn jns() -> &'static [IrStatement] {
    // Jump if not sign (SF = 0)
    let jmp = jump(o1());
    let cond_jmp = condition(u::not(sf.clone()), vec![jmp], vec![]);
    [cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn jo() -> &'static [IrStatement] {
    // Jump if overflow (OF = 1)
    let jmp = jump(o1());
    let cond_jmp = condition(of.clone(), vec![jmp], vec![]);
    [cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn jno() -> &'static [IrStatement] {
    // Jump if not overflow (OF = 0)
    let jmp = jump(o1());
    let cond_jmp = condition(u::not(of.clone()), vec![jmp], vec![]);
    [cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn jp() -> &'static [IrStatement] {
    // Jump if parity (PF = 1)
    let jmp = jump(o1());
    let cond_jmp = condition(pf.clone(), vec![jmp], vec![]);
    [cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn jnp() -> &'static [IrStatement] {
    // Jump if not parity (PF = 0)
    let jmp = jump(o1());
    let cond_jmp = condition(u::not(pf.clone()), vec![jmp], vec![]);
    [cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn jcxz() -> &'static [IrStatement] {
    // Jump if CX is zero
    let jmp = jump(o1());
    // Check if CX is zero by comparing with zero flag after a test
    let test_result = b::sub(cx.clone(), c(0));
    let calc_flags = calc_flags_automatically(test_result, size_relative(cx.clone()), &[&zf]);
    let cond_jmp = condition(zf.clone(), vec![jmp], vec![]);
    [calc_flags, cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn jecxz() -> &'static [IrStatement] {
    // Jump if ECX is zero
    let jmp = jump(o1());
    // Check if ECX is zero by comparing with zero flag after a test
    let test_result = b::sub(ecx.clone(), c(0));
    let calc_flags = calc_flags_automatically(test_result, size_relative(ecx.clone()), &[&zf]);
    let cond_jmp = condition(zf.clone(), vec![jmp], vec![]);
    [calc_flags, cond_jmp].into()
}

#[box_to_static_reference]
pub(super) fn jrcxz() -> &'static [IrStatement] {
    // Jump if RCX is zero
    let jmp = jump(o1());
    // Check if RCX is zero by comparing with zero flag after a test
    let test_result = b::sub(rcx.clone(), c(0));
    let calc_flags = calc_flags_automatically(test_result, size_relative(rcx.clone()), &[&zf]);
    let cond_jmp = condition(zf.clone(), vec![jmp], vec![]);
    [calc_flags, cond_jmp].into()
}
