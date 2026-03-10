use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// ST(0) := (2ST⁽⁰⁾ - 1);
/// ```
#[box_to_static_reference]
pub(super) fn f2xm1() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// ST(0) := |ST(0)|;
/// ```
#[box_to_static_reference]
pub(super) fn fabs() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF Instruction = FIADD
///     THEN
///         DEST := DEST + ConvertToDoubleExtendedPrecisionFP(SRC);
///     ELSE (* Source operand is floating-point value *)
///         DEST := DEST + SRC;
/// FI;
/// IF Instruction = FADDP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fadd() -> &'static [IrStatement] {
    let op = b::add(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF Instruction = FIADD
///     THEN
///         DEST := DEST + ConvertToDoubleExtendedPrecisionFP(SRC);
///     ELSE (* Source operand is floating-point value *)
///         DEST := DEST + SRC;
/// FI;
/// IF Instruction = FADDP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn faddp() -> &'static [IrStatement] {
    let op = b::add(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// TOP := TOP - 1;
/// ST(0) := ConvertToDoubleExtendedPrecisionFP(SRC);
/// ```
#[box_to_static_reference]
pub(super) fn fbld() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST := BCD(ST(0));
/// PopRegisterStack;
/// ```
#[box_to_static_reference]
pub(super) fn fbstp() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// SignBit(ST(0)) := NOT (SignBit(ST(0)));
/// ```
#[box_to_static_reference]
pub(super) fn fchs() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF condition TRUE
///     THEN ST(0) := ST(i);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fclex() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// CASE (relation of operands) OF
///     ST > SRC:
///                     C3, C2, C0 := 000;
///     ST < SRC:
///                     C3, C2, C0 := 001;
///     ST = SRC:
///                     C3, C2, C0 := 100;
/// ESAC;
/// IF ST(0) or SRC = NaN or unsupported format
///     THEN
///         #IA
///         IF FPUControlWord.IM = 1
///             THEN
///                 C3, C2, C0 := 111;
///         FI;
/// FI;
/// IF Instruction = FCOMP
///     THEN
///         PopRegisterStack;
/// FI;
/// IF Instruction = FCOMPP
///     THEN
///         PopRegisterStack;
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fcom() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (relation of operands) OF
///     ST(0) > ST(i):
///                     ZF, PF, CF := 000;
///     ST(0) < ST(i):
///                     ZF, PF, CF := 001;
///     ST(0) = ST(i):
///                     ZF, PF, CF := 100;
/// ESAC;
/// IF Instruction is FCOMI or FCOMIP
///     THEN
///         IF ST(0) or ST(i) = NaN or unsupported format
///             THEN
///                 #IA
///                 IF FPUControlWord.IM = 1
///                     THEN
///                         ZF, PF, CF := 111;
///                 FI;
///         FI;
/// FI;
/// IF Instruction is FUCOMI or FUCOMIP
///     THEN
///         IF ST(0) or ST(i) = QNaN, but not SNaN or unsupported format
///             THEN
///                 ZF, PF, CF := 111;
///             ELSE (* ST(0) or ST(i) is SNaN or unsupported format *)
///                 #IA;
///                 IF FPUControlWord.IM = 1
///                     THEN
///                         ZF, PF, CF := 111;
///                 FI;
///         FI;
/// FI;
/// IF Instruction is FCOMIP or FUCOMIP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fcomi() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (relation of operands) OF
///     ST(0) > ST(i):
///                     ZF, PF, CF := 000;
///     ST(0) < ST(i):
///                     ZF, PF, CF := 001;
///     ST(0) = ST(i):
///                     ZF, PF, CF := 100;
/// ESAC;
/// IF Instruction is FCOMI or FCOMIP
///     THEN
///         IF ST(0) or ST(i) = NaN or unsupported format
///             THEN
///                 #IA
///                 IF FPUControlWord.IM = 1
///                     THEN
///                         ZF, PF, CF := 111;
///                 FI;
///         FI;
/// FI;
/// IF Instruction is FUCOMI or FUCOMIP
///     THEN
///         IF ST(0) or ST(i) = QNaN, but not SNaN or unsupported format
///             THEN
///                 ZF, PF, CF := 111;
///             ELSE (* ST(0) or ST(i) is SNaN or unsupported format *)
///                 #IA;
///                 IF FPUControlWord.IM = 1
///                     THEN
///                         ZF, PF, CF := 111;
///                 FI;
///         FI;
/// FI;
/// IF Instruction is FCOMIP or FUCOMIP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fcomip() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (relation of operands) OF
///     ST > SRC:
///                     C3, C2, C0 := 000;
///     ST < SRC:
///                     C3, C2, C0 := 001;
///     ST = SRC:
///                     C3, C2, C0 := 100;
/// ESAC;
/// IF ST(0) or SRC = NaN or unsupported format
///     THEN
///         #IA
///         IF FPUControlWord.IM = 1
///             THEN
///                 C3, C2, C0 := 111;
///         FI;
/// FI;
/// IF Instruction = FCOMP
///     THEN
///         PopRegisterStack;
/// FI;
/// IF Instruction = FCOMPP
///     THEN
///         PopRegisterStack;
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fcomp() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (relation of operands) OF
///     ST > SRC:
///                     C3, C2, C0 := 000;
///     ST < SRC:
///                     C3, C2, C0 := 001;
///     ST = SRC:
///                     C3, C2, C0 := 100;
/// ESAC;
/// IF ST(0) or SRC = NaN or unsupported format
///     THEN
///         #IA
///         IF FPUControlWord.IM = 1
///             THEN
///                 C3, C2, C0 := 111;
///         FI;
/// FI;
/// IF Instruction = FCOMP
///     THEN
///         PopRegisterStack;
/// FI;
/// IF Instruction = FCOMPP
///     THEN
///         PopRegisterStack;
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fcompp() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// IF |ST(0)| < 2
/// THEN
///     C2 := 0;
///     ST(0) := FCOS(ST(0)); // approximation of cosine
/// ELSE (* Source operand is out-of-range *)
///     C2 := 1;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fcos() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF TOP = 0
///     THEN TOP := 7;
///     ELSE TOP := TOP - 1;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fdecstp() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// IF SRC = 0
///     THEN
///         #Z;
///     ELSE
///         IF Instruction is FIDIV
///             THEN
///                 DEST := DEST / ConvertToDoubleExtendedPrecisionFP(SRC);
///             ELSE (* Source operand is floating-point value *)
///                 DEST := DEST / SRC;
///         FI;
/// FI;
/// IF Instruction = FDIVP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fdiv() -> &'static [IrStatement] {
    let op = b::signed_div(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF SRC = 0
///     THEN
///         #Z;
///     ELSE
///         IF Instruction is FIDIV
///             THEN
///                 DEST := DEST / ConvertToDoubleExtendedPrecisionFP(SRC);
///             ELSE (* Source operand is floating-point value *)
///                 DEST := DEST / SRC;
///         FI;
/// FI;
/// IF Instruction = FDIVP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fdivp() -> &'static [IrStatement] {
    let op = b::signed_div(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF DEST = 0
///     THEN
///         #Z;
///     ELSE
///         IF Instruction = FIDIVR
///             THEN
///                 DEST := ConvertToDoubleExtendedPrecisionFP(SRC) / DEST;
///             ELSE (* Source operand is floating-point value *)
///                 DEST := SRC / DEST;
///         FI;
/// FI;
/// IF Instruction = FDIVRP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fdivr() -> &'static [IrStatement] {
    let op = b::signed_div(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF DEST = 0
///     THEN
///         #Z;
///     ELSE
///         IF Instruction = FIDIVR
///             THEN
///                 DEST := ConvertToDoubleExtendedPrecisionFP(SRC) / DEST;
///             ELSE (* Source operand is floating-point value *)
///                 DEST := SRC / DEST;
///         FI;
/// FI;
/// IF Instruction = FDIVRP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fdivrp() -> &'static [IrStatement] {
    let op = b::signed_div(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// TAG(i) := 11B;
/// ```
#[box_to_static_reference]
pub(super) fn ffree() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// IF Instruction = FIADD
///     THEN
///         DEST := DEST + ConvertToDoubleExtendedPrecisionFP(SRC);
///     ELSE (* Source operand is floating-point value *)
///         DEST := DEST + SRC;
/// FI;
/// IF Instruction = FADDP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fiadd() -> &'static [IrStatement] {
    let op = b::add(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CASE (relation of operands) OF
///     ST(0) > SRC:
///             C3, C2, C0 := 000;
///     ST(0) < SRC:
///             C3, C2, C0 := 001;
///     ST(0) = SRC:
///             C3, C2, C0 := 100;
///     Unordered:
///             C3, C2, C0 := 111;
/// ESAC;
/// IF Instruction = FICOMP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn ficom() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (relation of operands) OF
///     ST(0) > SRC:
///             C3, C2, C0 := 000;
///     ST(0) < SRC:
///             C3, C2, C0 := 001;
///     ST(0) = SRC:
///             C3, C2, C0 := 100;
///     Unordered:
///             C3, C2, C0 := 111;
/// ESAC;
/// IF Instruction = FICOMP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn ficomp() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// IF SRC = 0
///     THEN
///         #Z;
///     ELSE
///         IF Instruction is FIDIV
///             THEN
///                 DEST := DEST / ConvertToDoubleExtendedPrecisionFP(SRC);
///             ELSE (* Source operand is floating-point value *)
///                 DEST := DEST / SRC;
///         FI;
/// FI;
/// IF Instruction = FDIVP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fidiv() -> &'static [IrStatement] {
    let op = b::signed_div(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF DEST = 0
///     THEN
///         #Z;
///     ELSE
///         IF Instruction = FIDIVR
///             THEN
///                 DEST := ConvertToDoubleExtendedPrecisionFP(SRC) / DEST;
///             ELSE (* Source operand is floating-point value *)
///                 DEST := SRC / DEST;
///         FI;
/// FI;
/// IF Instruction = FDIVRP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fidivr() -> &'static [IrStatement] {
    let op = b::signed_div(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// TOP := TOP - 1;
/// ST(0) := ConvertToDoubleExtendedPrecisionFP(SRC);
/// ```
#[box_to_static_reference]
pub(super) fn fild() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF Instruction = FIMUL
///     THEN
///         DEST := DEST * ConvertToDoubleExtendedPrecisionFP(SRC);
///     ELSE (* Source operand is floating-point value *)
///         DEST := DEST * SRC;
/// FI;
/// IF Instruction = FMULP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fimul() -> &'static [IrStatement] {
    let op = b::mul(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF TOP = 7
///     THEN TOP := 0;
///     ELSE TOP := TOP + 1;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fincstp() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// FPUControlWord := 037FH;
/// FPUStatusWord := 0;
/// FPUTagWord := FFFFH;
/// FPUDataPointer := 0;
/// FPUInstructionPointer := 0;
/// FPULastInstructionOpcode := 0;
/// ```
#[box_to_static_reference]
pub(super) fn finit() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// DEST := Integer(ST(0));
/// IF Instruction = FISTP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fist() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST := Integer(ST(0));
/// IF Instruction = FISTP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fistp() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST := ST;
/// pop ST;
/// ```
#[box_to_static_reference]
pub(super) fn fisttp() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF Instruction = FISUB
///     THEN
///         DEST := DEST - ConvertToDoubleExtendedPrecisionFP(SRC);
///     ELSE (* Source operand is floating-point value *)
///         DEST := DEST - SRC;
/// FI;
/// IF Instruction = FSUBP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fisub() -> &'static [IrStatement] {
    let op = b::sub(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF Instruction = FISUBR
///     THEN
///         DEST := ConvertToDoubleExtendedPrecisionFP(SRC) - DEST;
///     ELSE (* Source operand is floating-point value *)
///         DEST := SRC - DEST; FI;
/// IF Instruction = FSUBRP
///     THEN
///         PopRegisterStack; FI;
/// ```
#[box_to_static_reference]
pub(super) fn fisubr() -> &'static [IrStatement] {
    let op = b::sub(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF SRC is ST(i)
///     THEN
///         temp := ST(i);
/// FI;
/// TOP := TOP - 1;
/// IF SRC is memory-operand
///     THEN
///         ST(0) := ConvertToDoubleExtendedPrecisionFP(SRC);
///     ELSE (* SRC is ST(i) *)
///         ST(0) := temp;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fld() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// TOP := TOP - 1;
/// ST(0) := CONSTANT;
/// ```
#[box_to_static_reference]
pub(super) fn fld1() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// FPUControlWord := SRC;
/// ```
#[box_to_static_reference]
pub(super) fn fldcw() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// FPUControlWord := SRC[FPUControlWord];
/// FPUStatusWord := SRC[FPUStatusWord];
/// FPUTagWord := SRC[FPUTagWord];
/// FPUDataPointer := SRC[FPUDataPointer];
/// FPUInstructionPointer := SRC[FPUInstructionPointer];
/// FPULastInstructionOpcode := SRC[FPULastInstructionOpcode];
/// ```
#[box_to_static_reference]
pub(super) fn fldenv() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// TOP := TOP - 1;
/// ST(0) := CONSTANT;
/// ```
#[box_to_static_reference]
pub(super) fn fldl2e() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// TOP := TOP - 1;
/// ST(0) := CONSTANT;
/// ```
#[box_to_static_reference]
pub(super) fn fldl2t() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// TOP := TOP - 1;
/// ST(0) := CONSTANT;
/// ```
#[box_to_static_reference]
pub(super) fn fldlg2() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// TOP := TOP - 1;
/// ST(0) := CONSTANT;
/// ```
#[box_to_static_reference]
pub(super) fn fldln2() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// TOP := TOP - 1;
/// ST(0) := CONSTANT;
/// ```
#[box_to_static_reference]
pub(super) fn fldpi() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// TOP := TOP - 1;
/// ST(0) := CONSTANT;
/// ```
#[box_to_static_reference]
pub(super) fn fldz() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF Instruction = FIMUL
///     THEN
///         DEST := DEST * ConvertToDoubleExtendedPrecisionFP(SRC);
///     ELSE (* Source operand is floating-point value *)
///         DEST := DEST * SRC;
/// FI;
/// IF Instruction = FMULP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fmul() -> &'static [IrStatement] {
    let op = b::mul(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF Instruction = FIMUL
///     THEN
///         DEST := DEST * ConvertToDoubleExtendedPrecisionFP(SRC);
///     ELSE (* Source operand is floating-point value *)
///         DEST := DEST * SRC;
/// FI;
/// IF Instruction = FMULP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fmulp() -> &'static [IrStatement] {
    let op = b::mul(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF condition TRUE
///     THEN ST(0) := ST(i);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fnclex() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// FPUControlWord := 037FH;
/// FPUStatusWord := 0;
/// FPUTagWord := FFFFH;
/// FPUDataPointer := 0;
/// FPUInstructionPointer := 0;
/// FPULastInstructionOpcode := 0;
/// ```
#[box_to_static_reference]
pub(super) fn fninit() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// (* Save FPU State and Registers *)
/// DEST[FPUControlWord] := FPUControlWord;
/// DEST[FPUStatusWord] := FPUStatusWord;
/// DEST[FPUTagWord] := FPUTagWord;
/// DEST[FPUDataPointer] := FPUDataPointer;
/// DEST[FPUInstructionPointer] := FPUInstructionPointer;
/// DEST[FPULastInstructionOpcode] := FPULastInstructionOpcode;
/// DEST[ST(0)] := ST(0);
/// DEST[ST(1)] := ST(1);
/// DEST[ST(2)] := ST(2);
/// DEST[ST(3)] := ST(3);
/// DEST[ST(4)]:= ST(4);
/// DEST[ST(5)] := ST(5);
/// DEST[ST(6)] := ST(6);
/// DEST[ST(7)] := ST(7);
/// (* Initialize FPU *)
/// FPUControlWord := 037FH;
/// FPUStatusWord := 0;
/// FPUTagWord := FFFFH;
/// FPUDataPointer := 0;
/// FPUInstructionPointer := 0;
/// FPULastInstructionOpcode := 0;
/// ```
#[box_to_static_reference]
pub(super) fn fnsave() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// DEST := FPUControlWord;
/// ```
#[box_to_static_reference]
pub(super) fn fnstcw() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// DEST[FPUControlWord] := FPUControlWord;
/// DEST[FPUStatusWord] := FPUStatusWord;
/// DEST[FPUTagWord] := FPUTagWord;
/// DEST[FPUDataPointer] := FPUDataPointer;
/// DEST[FPUInstructionPointer] := FPUInstructionPointer;
/// DEST[FPULastInstructionOpcode] := FPULastInstructionOpcode;
/// ```
#[box_to_static_reference]
pub(super) fn fnstenv() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// DEST := FPUStatusWord;
/// ```
#[box_to_static_reference]
pub(super) fn fnstsw() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// ST(1) := arctan(ST(1) / ST(0));
/// PopRegisterStack;
/// ```
#[box_to_static_reference]
pub(super) fn fpatan() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// D := exponent(ST(0)) - exponent(ST(1));
/// IF D < 64
///     THEN
///         Q := Integer(TruncateTowardZero(ST(0) / ST(1)));
///         ST(0) := ST(0) - (ST(1) * Q);
///         C2 := 0;
///         C0, C3, C1 := LeastSignificantBits(Q); (* Q2, Q1, Q0 *)
///     ELSE
///         C2 := 1;
///         N := An implementation-dependent number between 32 and 63;
///         QQ := Integer(TruncateTowardZero((ST(0)  / ST(1)) / 2⁽D ⁻ N⁾));
///         ST(0) := ST(0) - (ST(1) * QQ * 2⁽D ⁻ N⁾);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fprem() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// D := exponent(ST(0)) - exponent(ST(1));
/// IF D < 64
///     THEN
///         Q := Integer(RoundTowardNearestInteger(ST(0) / ST(1)));
///         ST(0) := ST(0) - (ST(1) * Q);
///         C2 := 0;
///         C0, C3, C1 := LeastSignificantBits(Q); (* Q2, Q1, Q0 *)
///     ELSE
///         C2 := 1;
///         N := An implementation-dependent number between 32 and 63;
///         QQ := Integer(TruncateTowardZero((ST(0)  / ST(1)) / 2⁽D ⁻ N⁾));
///         ST(0) := ST(0) - (ST(1) * QQ * 2⁽D ⁻ N⁾);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fprem1() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF ST(0) < 2⁶³
///     THEN
///         C2 := 0;
///         ST(0) := fptan(ST(0)); // approximation of tan
///         TOP := TOP - 1;
///         ST(0) := 1.0;
///     ELSE (* Source operand is out-of-range *)
///         C2 := 1;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fptan() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// ST(0) := RoundToIntegralValue(ST(0));
/// ```
#[box_to_static_reference]
pub(super) fn frndint() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// FPUControlWord := SRC[FPUControlWord];
/// FPUStatusWord := SRC[FPUStatusWord];
/// FPUTagWord := SRC[FPUTagWord];
/// FPUDataPointer := SRC[FPUDataPointer];
/// FPUInstructionPointer := SRC[FPUInstructionPointer];
/// FPULastInstructionOpcode := SRC[FPULastInstructionOpcode];
/// ST(0) := SRC[ST(0)];
/// ST(1) := SRC[ST(1)];
/// ST(2) := SRC[ST(2)];
/// ST(3) := SRC[ST(3)];
/// ST(4) := SRC[ST(4)];
/// ST(5) := SRC[ST(5)];
/// ST(6) := SRC[ST(6)];
/// ST(7) := SRC[ST(7)];
/// ```
#[box_to_static_reference]
pub(super) fn frstor() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// (* Save FPU State and Registers *)
/// DEST[FPUControlWord] := FPUControlWord;
/// DEST[FPUStatusWord] := FPUStatusWord;
/// DEST[FPUTagWord] := FPUTagWord;
/// DEST[FPUDataPointer] := FPUDataPointer;
/// DEST[FPUInstructionPointer] := FPUInstructionPointer;
/// DEST[FPULastInstructionOpcode] := FPULastInstructionOpcode;
/// DEST[ST(0)] := ST(0);
/// DEST[ST(1)] := ST(1);
/// DEST[ST(2)] := ST(2);
/// DEST[ST(3)] := ST(3);
/// DEST[ST(4)]:= ST(4);
/// DEST[ST(5)] := ST(5);
/// DEST[ST(6)] := ST(6);
/// DEST[ST(7)] := ST(7);
/// (* Initialize FPU *)
/// FPUControlWord := 037FH;
/// FPUStatusWord := 0;
/// FPUTagWord := FFFFH;
/// FPUDataPointer := 0;
/// FPUInstructionPointer := 0;
/// FPULastInstructionOpcode := 0;
/// ```
#[box_to_static_reference]
pub(super) fn fsave() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// ST(0) := ST(0) * 2RoundTowardZero(ST(1));
/// ```
#[box_to_static_reference]
pub(super) fn fscale() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF -2⁶³ < ST(0) < 2⁶³
///     THEN
///         C2 := 0;
///         ST(0) := fsin(ST(0)); // approximation of the mathematical sin function
///     ELSE (* Source operand out of range *)
///         C2 := 1;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fsin() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF ST(0) < 2⁶³
///     THEN
///         C2 := 0;
///         TEMP := fcos(ST(0)); // approximation of cosine
///         ST(0) := fsin(ST(0)); // approximation of sine
///         TOP := TOP - 1;
///         ST(0) := TEMP;
///     ELSE (* Source operand out of range *)
///         C2 := 1;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fsincos() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// ST(0) := SquareRoot(ST(0));
/// ```
#[box_to_static_reference]
pub(super) fn fsqrt() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST := ST(0);
/// IF Instruction = FSTP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fst() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST := FPUControlWord;
/// ```
#[box_to_static_reference]
pub(super) fn fstcw() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// DEST[FPUControlWord] := FPUControlWord;
/// DEST[FPUStatusWord] := FPUStatusWord;
/// DEST[FPUTagWord] := FPUTagWord;
/// DEST[FPUDataPointer] := FPUDataPointer;
/// DEST[FPUInstructionPointer] := FPUInstructionPointer;
/// DEST[FPULastInstructionOpcode] := FPULastInstructionOpcode;
/// ```
#[box_to_static_reference]
pub(super) fn fstenv() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// DEST := ST(0);
/// IF Instruction = FSTP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fstp() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST := FPUStatusWord;
/// ```
#[box_to_static_reference]
pub(super) fn fstsw() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// IF Instruction = FISUB
///     THEN
///         DEST := DEST - ConvertToDoubleExtendedPrecisionFP(SRC);
///     ELSE (* Source operand is floating-point value *)
///         DEST := DEST - SRC;
/// FI;
/// IF Instruction = FSUBP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fsub() -> &'static [IrStatement] {
    let op = b::sub(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF Instruction = FISUB
///     THEN
///         DEST := DEST - ConvertToDoubleExtendedPrecisionFP(SRC);
///     ELSE (* Source operand is floating-point value *)
///         DEST := DEST - SRC;
/// FI;
/// IF Instruction = FSUBP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fsubp() -> &'static [IrStatement] {
    let op = b::sub(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF Instruction = FISUBR
///     THEN
///         DEST := ConvertToDoubleExtendedPrecisionFP(SRC) - DEST;
///     ELSE (* Source operand is floating-point value *)
///         DEST := SRC - DEST; FI;
/// IF Instruction = FSUBRP
///     THEN
///         PopRegisterStack; FI;
/// ```
#[box_to_static_reference]
pub(super) fn fsubr() -> &'static [IrStatement] {
    let op = b::sub(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF Instruction = FISUBR
///     THEN
///         DEST := ConvertToDoubleExtendedPrecisionFP(SRC) - DEST;
///     ELSE (* Source operand is floating-point value *)
///         DEST := SRC - DEST; FI;
/// IF Instruction = FSUBRP
///     THEN
///         PopRegisterStack; FI;
/// ```
#[box_to_static_reference]
pub(super) fn fsubrp() -> &'static [IrStatement] {
    let op = b::sub(o1(), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CASE (relation of operands) OF
///     Not comparable:C3, C2, C0 := 111;
///     ST(0) > 0.0:
///         C3, C2, C0 := 000;
///     ST(0) < 0.0:
///         C3, C2, C0 := 001;
///     ST(0) = 0.0:
///         C3, C2, C0 := 100;
/// ESAC;
/// ```
#[box_to_static_reference]
pub(super) fn ftst() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (relation of operands) OF
///     ST > SRC:
///                     C3, C2, C0 := 000;
///     ST < SRC:
///                     C3, C2, C0 := 001;
///     ST = SRC:
///                     C3, C2, C0 := 100;
/// ESAC;
/// IF ST(0) or SRC = QNaN, but not SNaN or unsupported format
///     THEN
///         C3, C2, C0 := 111;
///     ELSE (* ST(0) or SRC is SNaN or unsupported format *)
///         #IA;
///         IF FPUControlWord.IM = 1
///             THEN
///                 C3, C2, C0 := 111;
///         FI;
/// FI;
/// IF Instruction = FUCOMP
///     THEN
///         PopRegisterStack;
/// FI;
/// IF Instruction = FUCOMPP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fucom() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (relation of operands) OF
///     ST(0) > ST(i):
///                     ZF, PF, CF := 000;
///     ST(0) < ST(i):
///                     ZF, PF, CF := 001;
///     ST(0) = ST(i):
///                     ZF, PF, CF := 100;
/// ESAC;
/// IF Instruction is FCOMI or FCOMIP
///     THEN
///         IF ST(0) or ST(i) = NaN or unsupported format
///             THEN
///                 #IA
///                 IF FPUControlWord.IM = 1
///                     THEN
///                         ZF, PF, CF := 111;
///                 FI;
///         FI;
/// FI;
/// IF Instruction is FUCOMI or FUCOMIP
///     THEN
///         IF ST(0) or ST(i) = QNaN, but not SNaN or unsupported format
///             THEN
///                 ZF, PF, CF := 111;
///             ELSE (* ST(0) or ST(i) is SNaN or unsupported format *)
///                 #IA;
///                 IF FPUControlWord.IM = 1
///                     THEN
///                         ZF, PF, CF := 111;
///                 FI;
///         FI;
/// FI;
/// IF Instruction is FCOMIP or FUCOMIP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fucomi() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (relation of operands) OF
///     ST(0) > ST(i):
///                     ZF, PF, CF := 000;
///     ST(0) < ST(i):
///                     ZF, PF, CF := 001;
///     ST(0) = ST(i):
///                     ZF, PF, CF := 100;
/// ESAC;
/// IF Instruction is FCOMI or FCOMIP
///     THEN
///         IF ST(0) or ST(i) = NaN or unsupported format
///             THEN
///                 #IA
///                 IF FPUControlWord.IM = 1
///                     THEN
///                         ZF, PF, CF := 111;
///                 FI;
///         FI;
/// FI;
/// IF Instruction is FUCOMI or FUCOMIP
///     THEN
///         IF ST(0) or ST(i) = QNaN, but not SNaN or unsupported format
///             THEN
///                 ZF, PF, CF := 111;
///             ELSE (* ST(0) or ST(i) is SNaN or unsupported format *)
///                 #IA;
///                 IF FPUControlWord.IM = 1
///                     THEN
///                         ZF, PF, CF := 111;
///                 FI;
///         FI;
/// FI;
/// IF Instruction is FCOMIP or FUCOMIP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fucomip() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (relation of operands) OF
///     ST > SRC:
///                     C3, C2, C0 := 000;
///     ST < SRC:
///                     C3, C2, C0 := 001;
///     ST = SRC:
///                     C3, C2, C0 := 100;
/// ESAC;
/// IF ST(0) or SRC = QNaN, but not SNaN or unsupported format
///     THEN
///         C3, C2, C0 := 111;
///     ELSE (* ST(0) or SRC is SNaN or unsupported format *)
///         #IA;
///         IF FPUControlWord.IM = 1
///             THEN
///                 C3, C2, C0 := 111;
///         FI;
/// FI;
/// IF Instruction = FUCOMP
///     THEN
///         PopRegisterStack;
/// FI;
/// IF Instruction = FUCOMPP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fucomp() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (relation of operands) OF
///     ST > SRC:
///                     C3, C2, C0 := 000;
///     ST < SRC:
///                     C3, C2, C0 := 001;
///     ST = SRC:
///                     C3, C2, C0 := 100;
/// ESAC;
/// IF ST(0) or SRC = QNaN, but not SNaN or unsupported format
///     THEN
///         C3, C2, C0 := 111;
///     ELSE (* ST(0) or SRC is SNaN or unsupported format *)
///         #IA;
///         IF FPUControlWord.IM = 1
///             THEN
///                 C3, C2, C0 := 111;
///         FI;
/// FI;
/// IF Instruction = FUCOMP
///     THEN
///         PopRegisterStack;
/// FI;
/// IF Instruction = FUCOMPP
///     THEN
///         PopRegisterStack;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fucompp() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CheckForPendingUnmaskedFloatingPointExceptions;
/// ```
#[box_to_static_reference]
pub(super) fn fwait() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// C1 := sign bit of ST; (* 0 for positive, 1 for negative *)
/// CASE (class of value or number in ST(0)) OF
///     Unsupported:C3, C2, C0 := 000;
///     NaN:
///         C3, C2, C0 := 001;
///     Normal:
///         C3, C2, C0 := 010;
///     Infinity:
///         C3, C2, C0 := 011;
///     Zero:
///         C3, C2, C0 := 100;
///     Empty:
///         C3, C2, C0 := 101;
///     Denormal:C3, C2, C0 := 110;
/// ESAC;
/// ```
#[box_to_static_reference]
pub(super) fn fxam() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF (Number-of-operands) is 1
///     THEN
///         temp := ST(0);
///         ST(0) := SRC;
///         SRC := temp;
///     ELSE
///         temp := ST(0);
///         ST(0) := ST(1);
///         ST(1) := temp;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fxch() -> &'static [IrStatement] {
    [exception("fxch")].into()
}

/// # Pseudocode
/// ```text
/// IF 64-Bit Mode
/// THEN
/// (x87 FPU, MMX, XMM15-XMM0, MXCSR)     Load(SRC);
/// ELSE
///     (x87 FPU, MMX, XMM7-XMM0, MXCSR) := Load(SRC);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fxrstor() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// IF 64-Bit Mode
///     THEN
///         IF REX.W = 1
///             THEN
///                 DEST := Save64BitPromotedFxsave(x87 FPU, MMX, XMM15-XMM0,
///                 MXCSR);
///             ELSE
///                 DEST := Save64BitDefaultFxsave(x87 FPU, MMX, XMM15-XMM0, MXCSR);
///         FI;
///     ELSE
///         DEST := SaveLegacyFxsave(x87 FPU, MMX, XMM7-XMM0, MXCSR);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn fxsave() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// TEMP := Significand(ST(0));
/// ST(0) := Exponent(ST(0));
/// TOP := TOP - 1;
/// ST(0) := TEMP;
/// ```
#[box_to_static_reference]
pub(super) fn fxtract() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// ST(1) := ST(1) * logST(0);
/// PopRegisterStack;
/// ```
#[box_to_static_reference]
pub(super) fn fyl2x() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// ST(1) := ST(1) * log(ST(0) + 1.0);
/// PopRegisterStack;
/// ```
#[box_to_static_reference]
pub(super) fn fyl2xp1() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}
