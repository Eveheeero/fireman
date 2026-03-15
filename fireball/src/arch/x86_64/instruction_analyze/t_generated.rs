use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// define make_fp32(x):
///     // The x parameter is bfloat16. Pack it in to upper 16b of a dword.
///     // The bit pattern is a legal fp32 value. Return that bit pattern.
///     dword: = 0
///     dword[31:16] := x
/// return dword
/// TDPBF16PS tsrcdest, tsrc1, tsrc2
/// // C = m x n (tsrcdest), A = m x k (tsrc1), B = k x n (tsrc2)
/// # src1 and src2 elements are pairs of bfloat16
/// elements_src1 := tsrc1.colsb / 4
/// elements_src2 := tsrc2.colsb / 4
/// elements_dest := tsrcdest.colsb / 4
/// elements_temp := tsrcdest.colsb / 2
///                 // Count is in bfloat16 prior to horizontal
/// for m in 0 ... tsrcdest.rows-1:
///     temp1[ 0 ... elements_temp-1 ] := 0
///     for k in 0 ... elements_src1-1:
///         for n in 0 ... elements_dest-1:
///             // FP32 FMA with DAZ=FTZ=1, RNE rounding.
///             // MXCSR is neither consulted nor updated.
///             // No exceptions raised or denoted.
///             temp1.fp32[2*n+0] += make_fp32(tsrc1.row[m].bfloat16[2*k+0]) * make_fp32(tsrc2.row[k].bfloat16[2*n+0])
///             temp1.fp32[2*n+1] += make_fp32(tsrc1.row[m].bfloat16[2*k+1]) * make_fp32(tsrc2.row[k].bfloat16[2*n+1])
///     for n in 0 ... elements_dest-1:
///         // DAZ=FTZ=1, RNE rounding.
///         // MXCSR is neither consulted nor updated.
///         // No exceptions raised or denoted.
///         tmpf32 := temp1.fp32[2*n] + temp1.fp32[2*n+1]
///         tsrcdest.row[m].fp32[n] := tsrcdest.row[m].fp32[n] + tmpf32
///     write_row_and_zero(tsrcdest, m, tmp, tsrcdest.colsb)
/// zero_upper_rows(tsrcdest, tsrcdest.rows)
/// zero_tilecfg_start()
/// ```
#[box_to_static_reference]
pub(super) fn tdpbf16ps() -> &'static [IrStatement] {
    [exception("tdpbf16ps")].into()
}

/// # Pseudocode
/// ```text
/// define DPBD(c,x,y):// arguments are dwords
///     if *x operand is signed*:
///         extend_src1 := SIGN_EXTEND
///     else:
///         extend_src1 := ZERO_EXTEND
///     if *y operand is signed*:
///         extend_src2 := SIGN_EXTEND
///     else:
///         extend_src2 := ZERO_EXTEND
///     p0dword := extend_src1(x.byte[0]) * extend_src2(y.byte[0])
///     p1dword := extend_src1(x.byte[1]) * extend_src2(y.byte[1])
///     p2dword := extend_src1(x.byte[2]) * extend_src2(y.byte[2])
///     p3dword := extend_src1(x.byte[3]) * extend_src2(y.byte[3])
///     c := c + p0dword + p1dword + p2dword + p3dword
/// TDPBSSD, TDPBSUD, TDPBUSD, TDPBUUD tsrcdest, tsrc1, tsrc2 (Register Only Version)
/// // C = m x n (tsrcdest), A = m x k (tsrc1), B = k x n (tsrc2)
/// tsrc1_elements_per_row := tsrc1.colsb / 4
/// tsrc2_elements_per_row := tsrc2.colsb / 4
/// tsrcdest_elements_per_row := tsrcdest.colsb / 4
/// for m in 0 ... tsrcdest.rows-1:
///     tmp := tsrcdest.row[m]
///     for k in 0 ... tsrc1_elements_per_row-1:
///         for n in 0 ... tsrcdest_elements_per_row-1:
///             DPBD( tmp.dword[n], tsrc1.row[m].dword[k], tsrc2.row[k].dword[n] )
///     write_row_and_zero(tsrcdest, m, tmp, tsrcdest.colsb)
/// zero_upper_rows(tsrcdest, tsrcdest.rows)
/// zero_tilecfg_start()
/// ```
#[box_to_static_reference]
pub(super) fn tdpbssd() -> &'static [IrStatement] {
    [exception("tdpbssd")].into()
}

/// # Pseudocode
/// ```text
/// define DPBD(c,x,y):// arguments are dwords
///     if *x operand is signed*:
///         extend_src1 := SIGN_EXTEND
///     else:
///         extend_src1 := ZERO_EXTEND
///     if *y operand is signed*:
///         extend_src2 := SIGN_EXTEND
///     else:
///         extend_src2 := ZERO_EXTEND
///     p0dword := extend_src1(x.byte[0]) * extend_src2(y.byte[0])
///     p1dword := extend_src1(x.byte[1]) * extend_src2(y.byte[1])
///     p2dword := extend_src1(x.byte[2]) * extend_src2(y.byte[2])
///     p3dword := extend_src1(x.byte[3]) * extend_src2(y.byte[3])
///     c := c + p0dword + p1dword + p2dword + p3dword
/// TDPBSSD, TDPBSUD, TDPBUSD, TDPBUUD tsrcdest, tsrc1, tsrc2 (Register Only Version)
/// // C = m x n (tsrcdest), A = m x k (tsrc1), B = k x n (tsrc2)
/// tsrc1_elements_per_row := tsrc1.colsb / 4
/// tsrc2_elements_per_row := tsrc2.colsb / 4
/// tsrcdest_elements_per_row := tsrcdest.colsb / 4
/// for m in 0 ... tsrcdest.rows-1:
///     tmp := tsrcdest.row[m]
///     for k in 0 ... tsrc1_elements_per_row-1:
///         for n in 0 ... tsrcdest_elements_per_row-1:
///             DPBD( tmp.dword[n], tsrc1.row[m].dword[k], tsrc2.row[k].dword[n] )
///     write_row_and_zero(tsrcdest, m, tmp, tsrcdest.colsb)
/// zero_upper_rows(tsrcdest, tsrcdest.rows)
/// zero_tilecfg_start()
/// ```
#[box_to_static_reference]
pub(super) fn tdpbsud() -> &'static [IrStatement] {
    [exception("tdpbsud")].into()
}

/// # Pseudocode
/// ```text
/// define DPBD(c,x,y):// arguments are dwords
///     if *x operand is signed*:
///         extend_src1 := SIGN_EXTEND
///     else:
///         extend_src1 := ZERO_EXTEND
///     if *y operand is signed*:
///         extend_src2 := SIGN_EXTEND
///     else:
///         extend_src2 := ZERO_EXTEND
///     p0dword := extend_src1(x.byte[0]) * extend_src2(y.byte[0])
///     p1dword := extend_src1(x.byte[1]) * extend_src2(y.byte[1])
///     p2dword := extend_src1(x.byte[2]) * extend_src2(y.byte[2])
///     p3dword := extend_src1(x.byte[3]) * extend_src2(y.byte[3])
///     c := c + p0dword + p1dword + p2dword + p3dword
/// TDPBSSD, TDPBSUD, TDPBUSD, TDPBUUD tsrcdest, tsrc1, tsrc2 (Register Only Version)
/// // C = m x n (tsrcdest), A = m x k (tsrc1), B = k x n (tsrc2)
/// tsrc1_elements_per_row := tsrc1.colsb / 4
/// tsrc2_elements_per_row := tsrc2.colsb / 4
/// tsrcdest_elements_per_row := tsrcdest.colsb / 4
/// for m in 0 ... tsrcdest.rows-1:
///     tmp := tsrcdest.row[m]
///     for k in 0 ... tsrc1_elements_per_row-1:
///         for n in 0 ... tsrcdest_elements_per_row-1:
///             DPBD( tmp.dword[n], tsrc1.row[m].dword[k], tsrc2.row[k].dword[n] )
///     write_row_and_zero(tsrcdest, m, tmp, tsrcdest.colsb)
/// zero_upper_rows(tsrcdest, tsrcdest.rows)
/// zero_tilecfg_start()
/// ```
#[box_to_static_reference]
pub(super) fn tdpbusd() -> &'static [IrStatement] {
    [exception("tdpbusd")].into()
}

/// # Pseudocode
/// ```text
/// define DPBD(c,x,y):// arguments are dwords
///     if *x operand is signed*:
///         extend_src1 := SIGN_EXTEND
///     else:
///         extend_src1 := ZERO_EXTEND
///     if *y operand is signed*:
///         extend_src2 := SIGN_EXTEND
///     else:
///         extend_src2 := ZERO_EXTEND
///     p0dword := extend_src1(x.byte[0]) * extend_src2(y.byte[0])
///     p1dword := extend_src1(x.byte[1]) * extend_src2(y.byte[1])
///     p2dword := extend_src1(x.byte[2]) * extend_src2(y.byte[2])
///     p3dword := extend_src1(x.byte[3]) * extend_src2(y.byte[3])
///     c := c + p0dword + p1dword + p2dword + p3dword
/// TDPBSSD, TDPBSUD, TDPBUSD, TDPBUUD tsrcdest, tsrc1, tsrc2 (Register Only Version)
/// // C = m x n (tsrcdest), A = m x k (tsrc1), B = k x n (tsrc2)
/// tsrc1_elements_per_row := tsrc1.colsb / 4
/// tsrc2_elements_per_row := tsrc2.colsb / 4
/// tsrcdest_elements_per_row := tsrcdest.colsb / 4
/// for m in 0 ... tsrcdest.rows-1:
///     tmp := tsrcdest.row[m]
///     for k in 0 ... tsrc1_elements_per_row-1:
///         for n in 0 ... tsrcdest_elements_per_row-1:
///             DPBD( tmp.dword[n], tsrc1.row[m].dword[k], tsrc2.row[k].dword[n] )
///     write_row_and_zero(tsrcdest, m, tmp, tsrcdest.colsb)
/// zero_upper_rows(tsrcdest, tsrcdest.rows)
/// zero_tilecfg_start()
/// ```
#[box_to_static_reference]
pub(super) fn tdpbuud() -> &'static [IrStatement] {
    [exception("tdpbuud")].into()
}

/// # Pseudocode
/// ```text
/// TEMP := SRC1 AND SRC2;
/// SF := MSB(TEMP);
/// IF TEMP = 0
///     THEN ZF := 1;
///     ELSE ZF := 0;
/// FI:
/// CF := 0;
/// OF := 0;
/// (* AF is undefined *)
/// ```
#[box_to_static_reference]
pub(super) fn test() -> &'static [IrStatement] {
    let and_val = b::and(o1(), o2());
    let sf_zf_pf = calc_flags_automatically(and_val, o1_size(), &[&sf, &zf, &pf]);
    let set_of = assign(c(0), of.clone(), size_relative(of.clone()));
    let set_cf = assign(c(0), cf.clone(), size_relative(cf.clone()));
    extend_undefined_flags(&[sf_zf_pf, set_of, set_cf], &[&af])
}

/// # Pseudocode
/// ```text
/// CF := UIF;
/// ZF := AF := OF := PF := SF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn testui() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), cf.clone(), size_relative(cf.clone()));
    let stmt_1 = assign(af.clone(), zf.clone(), size_relative(zf.clone()));
    [stmt_0, stmt_1].into()
}

/// # Pseudocode
/// ```text
/// TILELOADD[,T1] tdest, tsib
/// start := tilecfg.start_row
/// zero_upper_rows(tdest,start)
/// membegin := tsib.base + displacement
/// // if no index register in the SIB encoding, the value zero is used.
/// stride := tsib.index << tsib.scale
/// nbytes := tdest.colsb
/// while start < tdest.rows:
///     memptr := membegin + start * stride
///     write_row_and_zero(tdest, start, read_memory(memptr, nbytes), nbytes)
///     start := start + 1
/// zero_tilecfg_start()
/// // In the case of a memory fault in the middle of an instruction, the tilecfg.start_row := start
/// ```
#[box_to_static_reference]
pub(super) fn tileloadd() -> &'static [IrStatement] {
    [exception("tileloadd")].into()
}

/// # Pseudocode
/// ```text
/// TILELOADD[,T1] tdest, tsib
/// start := tilecfg.start_row
/// zero_upper_rows(tdest,start)
/// membegin := tsib.base + displacement
/// // if no index register in the SIB encoding, the value zero is used.
/// stride := tsib.index << tsib.scale
/// nbytes := tdest.colsb
/// while start < tdest.rows:
///     memptr := membegin + start * stride
///     write_row_and_zero(tdest, start, read_memory(memptr, nbytes), nbytes)
///     start := start + 1
/// zero_tilecfg_start()
/// // In the case of a memory fault in the middle of an instruction, the tilecfg.start_row := start
/// ```
#[box_to_static_reference]
pub(super) fn tileloaddt1() -> &'static [IrStatement] {
    [exception("tileloaddt1")].into()
}

/// # Pseudocode
/// ```text
/// zero_all_tile_data()
/// tilecfg := 0// equivalent to 64B of zeros
/// TILES_CONFIGURED := 0
/// ```
#[box_to_static_reference]
pub(super) fn tilerelease() -> &'static [IrStatement] {
    let stmt_0 = assign(b::unsigned_div(c(0), unknown_data()), o1(), o1_size());
    let stmt_1 = assign(c(0), o1(), o1_size());
    [stmt_0, stmt_1].into()
}

/// # Pseudocode
/// ```text
/// TILESTORED tsib, tsrc
/// start := tilecfg.start_row
/// membegin := tsib.base + displacement
/// // if no index register in the SIB encoding, the value zero is used.
/// stride := tsib.index << tsib.scale
/// while start < tdest.rows:
///     memptr := membegin + start * stride
///     write_memory(memptr, tsrc.colsb, tsrc.row[start])
///     start := start + 1
/// zero_tilecfg_start()
/// // In the case of a memory fault in the middle of an instruction, the tilecfg.start_row := start
/// ```
#[box_to_static_reference]
pub(super) fn tilestored() -> &'static [IrStatement] {
    [exception("tilestored")].into()
}

/// # Pseudocode
/// ```text
/// TILEZERO tdest
/// nbytes := palette_table[palette_id].bytes_per_row
/// for i in 0 ... palette_table[palette_id].max_rows-1:
///     for j in 0 ... nbytes-1:
///         tdest.row[i].byte[j] := 0
/// zero_tilecfg_start()
/// ```
#[box_to_static_reference]
pub(super) fn tilezero() -> &'static [IrStatement] {
    [exception("tilezero")].into()
}

/// # Pseudocode
/// ```text
/// os_deadline := TSC+(IA32_UMWAIT_CONTROL[31:2]<<2)
/// instr_deadline := UINT64(EDX:EAX)
/// IF os_deadline < instr_deadline:
///     deadline := os_deadline
///     using_os_deadline := 1
/// ELSE:
///     deadline := instr_deadline
///     using_os_deadline := 0
/// WHILE TSC < deadline:
///     implementation_dependent_optimized_state(Source register, deadline, IA32_UMWAIT_CONTROL[0])
/// IF using_os_deadline AND TSC > deadline:
///     RFLAGS.CF := 1
/// ELSE:
///     RFLAGS.CF := 0
/// RFLAGS.AF,PF,SF,ZF,OF := 0
/// ```
#[box_to_static_reference]
pub(super) fn tpause() -> &'static [IrStatement] {
    [exception("tpause")].into()
}

/// # Pseudocode
/// ```text
/// temp := 0
/// DEST := 0
/// DO WHILE ( (temp < OperandSize) and (SRC[ temp] = 0) )
///     temp := temp +1
///     DEST := DEST+ 1
/// OD
/// IF DEST = OperandSize
///     CF := 1
/// ELSE
///     CF := 0
/// FI
/// IF DEST = 0
///     ZF := 1
/// ELSE
///     ZF := 0
/// FI
/// ```
#[box_to_static_reference]
pub(super) fn tzcnt() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    let set_of = assign(c(0), of.clone(), size_relative(of.clone()));
    let set_sf = assign(c(0), sf.clone(), size_relative(sf.clone()));
    let set_af = assign(c(0), af.clone(), size_relative(af.clone()));
    let set_cf = assign(c(0), cf.clone(), size_relative(cf.clone()));
    let set_pf = assign(c(0), pf.clone(), size_relative(pf.clone()));
    let set_zf = condition(b::equal(o1(), c(0), o1_size()), [assign(c(1), zf.clone(), size_relative(zf.clone()))], [assign(c(0), zf.clone(), size_relative(zf.clone()))]);
    [assignment, set_of, set_sf, set_af, set_cf, set_pf, set_zf].into()
}
