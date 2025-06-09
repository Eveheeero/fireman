use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

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
pub(super) fn sub() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let assignment = assign(sub.clone(), o1(), o1_size());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags, assignment].into()
}

#[box_to_static_reference]
pub(super) fn subps() -> &'static [IrStatement] {
    // SUBPS subtracts four single-precision floating-point values from source to destination
    let size = o1_size();
    let sub = b::sub(o1(), o2());
    let assignment = assign(sub, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn subpd() -> &'static [IrStatement] {
    // SUBPD subtracts two double-precision floating-point values from source to destination
    let size = o1_size();
    let sub = b::sub(o1(), o2());
    let assignment = assign(sub, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn subss() -> &'static [IrStatement] {
    // SUBSS subtracts the low single-precision floating-point values from source to destination
    let size = o1_size();
    let sub = b::sub(o1(), o2());
    let assignment = assign(sub, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn subsd() -> &'static [IrStatement] {
    // SUBSD subtracts the low double-precision floating-point values from source to destination
    let size = o1_size();
    let sub = b::sub(o1(), o2());
    let assignment = assign(sub, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn sqrtps() -> &'static [IrStatement] {
    // SQRTPS computes square root of four single-precision floating-point values
    // TODO: Implement square root unary operator in IR
    // For now, mark as unknown operation
    let size = o1_size();
    let sqrt = unknown_data();
    let assignment = assign(sqrt, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn sqrtpd() -> &'static [IrStatement] {
    // SQRTPD computes square root of two double-precision floating-point values
    // TODO: Implement square root unary operator in IR
    let size = o1_size();
    let sqrt = unknown_data();
    let assignment = assign(sqrt, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn sqrtss() -> &'static [IrStatement] {
    // SQRTSS computes square root of the low single-precision floating-point value
    // TODO: Implement square root unary operator in IR
    let size = o1_size();
    let sqrt = unknown_data();
    let assignment = assign(sqrt, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn sqrtsd() -> &'static [IrStatement] {
    // SQRTSD computes square root of the low double-precision floating-point value
    // TODO: Implement square root unary operator in IR
    let size = o1_size();
    let sqrt = unknown_data();
    let assignment = assign(sqrt, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn sahf() -> &'static [IrStatement] {
    // SAHF loads SF, ZF, AF, PF, and CF from AH into EFLAGS register
    // AH bits: 7=SF, 6=ZF, 4=AF, 2=PF, 0=CF
    let ah_val = ah.clone();

    // Extract individual bits from AH and assign to flags
    let flag_size = size_result_bit(c(1));
    let set_cf = assign(b::and(ah_val.clone(), c(0x01)), cf.clone(), &flag_size);
    let set_pf = assign(
        b::shr(b::and(ah_val.clone(), c(0x04)), c(2)),
        pf.clone(),
        &flag_size,
    );
    let set_af = assign(
        b::shr(b::and(ah_val.clone(), c(0x10)), c(4)),
        af.clone(),
        &flag_size,
    );
    let set_zf = assign(
        b::shr(b::and(ah_val.clone(), c(0x40)), c(6)),
        zf.clone(),
        &flag_size,
    );
    let set_sf = assign(
        b::shr(b::and(ah_val, c(0x80)), c(7)),
        sf.clone(),
        &flag_size,
    );

    [set_cf, set_pf, set_af, set_zf, set_sf].into()
}

#[box_to_static_reference]
pub(super) fn stc() -> &'static [IrStatement] {
    // STC sets the carry flag
    let flag_size = size_result_bit(c(1));
    let set_cf = assign(c(1), cf.clone(), &flag_size);
    [set_cf].into()
}

#[box_to_static_reference]
pub(super) fn std() -> &'static [IrStatement] {
    // STD sets the direction flag
    let flag_size = size_result_bit(c(1));
    let set_df = assign(c(1), df.clone(), &flag_size);
    [set_df].into()
}

#[box_to_static_reference]
pub(super) fn sar() -> &'static [IrStatement] {
    // SAR - Shift Arithmetic Right (preserves sign bit)
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
    // SBB - Subtract with borrow (dest = dest - src - CF)
    let cf_val = sized(cf.clone(), o1_size());
    let sub_with_cf = b::sub(b::sub(o1(), o2()), cf_val);
    let assignment = assign(sub_with_cf.clone(), o1(), o1_size());
    let calc_flags =
        calc_flags_automatically(sub_with_cf, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags, assignment].into()
}

#[box_to_static_reference]
pub(super) fn stosb() -> &'static [IrStatement] {
    // STOSB stores AL at address [RDI] and updates RDI
    let size = size_result_byte(c(1));
    let dst = d(rdi.clone());
    let store = assign(al.clone(), dst, &size);
    let inc_rdi = assign(
        b::add(rdi.clone(), c(1)),
        rdi.clone(),
        &size_relative(rdi.clone()),
    );

    [store, inc_rdi].into()
}

#[box_to_static_reference]
pub(super) fn stosw() -> &'static [IrStatement] {
    // STOSW stores AX at address [RDI] and updates RDI
    let size = size_result_byte(c(2));
    let dst = d(rdi.clone());
    let store = assign(ax.clone(), dst, &size);
    let inc_rdi = assign(
        b::add(rdi.clone(), c(2)),
        rdi.clone(),
        &size_relative(rdi.clone()),
    );

    [store, inc_rdi].into()
}

#[box_to_static_reference]
pub(super) fn stosd() -> &'static [IrStatement] {
    // STOSD stores EAX at address [RDI] and updates RDI
    let size = size_result_byte(c(4));
    let dst = d(rdi.clone());
    let store = assign(eax.clone(), dst, &size);
    let inc_rdi = assign(
        b::add(rdi.clone(), c(4)),
        rdi.clone(),
        &size_relative(rdi.clone()),
    );

    [store, inc_rdi].into()
}

#[box_to_static_reference]
pub(super) fn stosq() -> &'static [IrStatement] {
    // STOSQ stores RAX at address [RDI] and updates RDI
    let size = size_result_byte(c(8));
    let dst = d(rdi.clone());
    let store = assign(rax.clone(), dst, &size);
    let inc_rdi = assign(
        b::add(rdi.clone(), c(8)),
        rdi.clone(),
        &size_relative(rdi.clone()),
    );

    [store, inc_rdi].into()
}
