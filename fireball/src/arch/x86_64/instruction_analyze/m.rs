use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn mov() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

#[box_to_static_reference]
pub(super) fn movaps() -> &'static [IrStatement] {
    // MOVAPS moves 128 bits (4 single-precision floating-point values) from source to destination
    // Both operands must be 16-byte aligned
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

#[box_to_static_reference]
pub(super) fn movapd() -> &'static [IrStatement] {
    // MOVAPD moves 128 bits (2 double-precision floating-point values) from source to destination
    // Both operands must be 16-byte aligned
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

#[box_to_static_reference]
pub(super) fn movups() -> &'static [IrStatement] {
    // MOVUPS moves 128 bits (4 single-precision floating-point values) from source to destination
    // Unaligned memory access allowed
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

#[box_to_static_reference]
pub(super) fn movupd() -> &'static [IrStatement] {
    // MOVUPD moves 128 bits (2 double-precision floating-point values) from source to destination
    // Unaligned memory access allowed
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

#[box_to_static_reference]
pub(super) fn movss() -> &'static [IrStatement] {
    // MOVSS moves a single-precision floating-point value
    let assignment = assign(o2(), o1(), o1_size());
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn mul() -> &'static [IrStatement] {
    let assertion = assertion(u::not(is_o2_exists()));

    let operand_bit_size = bit_size_of_o1();

    let value = b::mul(sized(al.clone(), size_relative(al.clone())), o1());
    let mul_8 = [
        calc_flags_automatically(value.clone(), o1_size(), &[&of, &cf]),
        assign(value, ax.clone(), size_relative(ax.clone())),
    ];

    let value = b::mul(sized(rax.clone(), o1_size()), o1());
    let mul_etc = [
        calc_flags_automatically(value.clone(), o1_size(), &[&of, &cf]),
        assign(value.clone(), rax.clone(), o1_size()),
        assign(
            b::shr(u::zero_extend(value), operand_bit_size.clone()),
            rdx.clone(),
            o1_size(),
        ),
    ];

    let mul = condition(
        b::equal(operand_bit_size, c(8), size_unlimited()),
        mul_8,
        mul_etc,
    );
    extend_undefined_flags(&[assertion, mul], &[&sf, &zf, &af, &pf])
}

#[box_to_static_reference]
pub(super) fn mulps() -> &'static [IrStatement] {
    // MULPS multiplies four single-precision floating-point values from source to destination
    let size = o1_size();
    let mul = b::mul(o1(), o2());
    let assignment = assign(mul, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn mulpd() -> &'static [IrStatement] {
    // MULPD multiplies two double-precision floating-point values from source to destination
    let size = o1_size();
    let mul = b::mul(o1(), o2());
    let assignment = assign(mul, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn mulss() -> &'static [IrStatement] {
    // MULSS multiplies the low single-precision floating-point values from source to destination
    let size = o1_size();
    let mul = b::mul(o1(), o2());
    let assignment = assign(mul, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn mulsd() -> &'static [IrStatement] {
    // MULSD multiplies the low double-precision floating-point values from source to destination
    let size = o1_size();
    let mul = b::mul(o1(), o2());
    let assignment = assign(mul, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn minps() -> &'static [IrStatement] {
    // MINPS returns the minimum of packed single-precision floating-point values
    // TODO: Implement proper min/max operations in IR
    // For now, use unknown data as placeholder
    let size = o1_size();
    let min = unknown_data();
    let assignment = assign(min, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn minpd() -> &'static [IrStatement] {
    // MINPD returns the minimum of packed double-precision floating-point values
    // TODO: Implement proper min/max operations in IR
    let size = o1_size();
    let min = unknown_data();
    let assignment = assign(min, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn minss() -> &'static [IrStatement] {
    // MINSS returns the minimum of scalar single-precision floating-point values
    // TODO: Implement proper min/max operations in IR
    let size = o1_size();
    let min = unknown_data();
    let assignment = assign(min, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn minsd() -> &'static [IrStatement] {
    // MINSD returns the minimum of scalar double-precision floating-point values
    // TODO: Implement proper min/max operations in IR
    let size = o1_size();
    let min = unknown_data();
    let assignment = assign(min, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn maxps() -> &'static [IrStatement] {
    // MAXPS returns the maximum of packed single-precision floating-point values
    // TODO: Implement proper min/max operations in IR
    let size = o1_size();
    let max = unknown_data();
    let assignment = assign(max, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn maxpd() -> &'static [IrStatement] {
    // MAXPD returns the maximum of packed double-precision floating-point values
    // TODO: Implement proper min/max operations in IR
    let size = o1_size();
    let max = unknown_data();
    let assignment = assign(max, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn maxss() -> &'static [IrStatement] {
    // MAXSS returns the maximum of scalar single-precision floating-point values
    // TODO: Implement proper min/max operations in IR
    let size = o1_size();
    let max = unknown_data();
    let assignment = assign(max, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn maxsd() -> &'static [IrStatement] {
    // MAXSD returns the maximum of scalar double-precision floating-point values
    // TODO: Implement proper min/max operations in IR
    let size = o1_size();
    let max = unknown_data();
    let assignment = assign(max, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn movsx() -> &'static [IrStatement] {
    let size = o1_size();
    // Sign extend from source size to destination size
    let extended = u::sign_extend(o2());
    let assignment = assign(extended, o1(), &size);
    [assignment].into()
}

#[box_to_static_reference]
pub(super) fn movsxd() -> &'static [IrStatement] {
    // MOVSXD is specific for 32->64 bit sign extension
    let size = o1_size();
    let extended = u::sign_extend(o2());
    let assignment = assign(extended, o1(), &size);
    [assignment].into()
}

#[box_to_static_reference]
pub(super) fn movzx() -> &'static [IrStatement] {
    let size = o1_size();
    // Zero extend from source size to destination size
    let extended = u::zero_extend(o2());
    let assignment = assign(extended, o1(), &size);
    [assignment].into()
}

#[box_to_static_reference]
pub(super) fn movsb() -> &'static [IrStatement] {
    // MOVSB moves byte from [RSI] to [RDI] and updates pointers
    let size = size_result_byte(c(1));
    let src_val = d(rsi.clone());
    let dst = d(rdi.clone());

    let move_stmt = assign(src_val, dst, &size);
    let inc_rsi = assign(
        b::add(rsi.clone(), c(1)),
        rsi.clone(),
        &size_relative(rsi.clone()),
    );
    let inc_rdi = assign(
        b::add(rdi.clone(), c(1)),
        rdi.clone(),
        &size_relative(rdi.clone()),
    );

    [move_stmt, inc_rsi, inc_rdi].into()
}

#[box_to_static_reference]
pub(super) fn movsw() -> &'static [IrStatement] {
    // MOVSW moves word from [RSI] to [RDI] and updates pointers
    let size = size_result_byte(c(2));
    let src_val = d(rsi.clone());
    let dst = d(rdi.clone());

    let move_stmt = assign(src_val, dst, &size);
    let inc_rsi = assign(
        b::add(rsi.clone(), c(2)),
        rsi.clone(),
        &size_relative(rsi.clone()),
    );
    let inc_rdi = assign(
        b::add(rdi.clone(), c(2)),
        rdi.clone(),
        &size_relative(rdi.clone()),
    );

    [move_stmt, inc_rsi, inc_rdi].into()
}

#[box_to_static_reference]
pub(super) fn movsd() -> &'static [IrStatement] {
    // MOVSD (string) moves dword from [RSI] to [RDI] and updates pointers
    let size = size_result_byte(c(4));
    let src_val = d(rsi.clone());
    let dst = d(rdi.clone());

    let move_stmt = assign(src_val, dst, &size);
    let inc_rsi = assign(
        b::add(rsi.clone(), c(4)),
        rsi.clone(),
        &size_relative(rsi.clone()),
    );
    let inc_rdi = assign(
        b::add(rdi.clone(), c(4)),
        rdi.clone(),
        &size_relative(rdi.clone()),
    );

    [move_stmt, inc_rsi, inc_rdi].into()
}

#[box_to_static_reference]
pub(super) fn movsq() -> &'static [IrStatement] {
    // MOVSQ moves qword from [RSI] to [RDI] and updates pointers
    let size = size_result_byte(c(8));
    let src_val = d(rsi.clone());
    let dst = d(rdi.clone());

    let move_stmt = assign(src_val, dst, &size);
    let inc_rsi = assign(
        b::add(rsi.clone(), c(8)),
        rsi.clone(),
        &size_relative(rsi.clone()),
    );
    let inc_rdi = assign(
        b::add(rdi.clone(), c(8)),
        rdi.clone(),
        &size_relative(rdi.clone()),
    );

    [move_stmt, inc_rsi, inc_rdi].into()
}
