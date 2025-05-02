use super::*;

/// extend result to undefined flags
/// ```rust,ignore
/// extend_undefined_flags([add, assignment], &[&of, &sf, &zf, &af, &cf, &pf])
/// ```
#[inline]
#[must_use]
pub(in crate::arch) fn extend_undefined_flags(
    ir: &[IrStatement],
    affected_registers: &[&Aos<IrData>],
) -> Box<[IrStatement]> {
    let mut result: Vec<_> = ir.into();
    for register in affected_registers {
        let register = (*register).clone();
        let assignment = assign(undefined_data(), register.clone(), size_relative(register));
        result.push(assignment);
    }
    result.into_boxed_slice()
}
