use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// IF 64-Bit Mode
///     THEN
///         IF CPUID.80000001H:ECX.LAHF-SAHF[bit 0] = 1;
///             THEN AH := RFLAGS(SF:ZF:0:AF:0:PF:1:CF);
///             ELSE #UD;
///         FI;
///     ELSE
///         AH := EFLAGS(SF:ZF:0:AF:0:PF:1:CF);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn lahf() -> &'static [IrStatement] {
    let val = b::or(b::or(b::or(b::or(b::shl(sf.clone(), c(7)), b::shl(zf.clone(), c(6))), b::shl(af.clone(), c(4))), b::shl(pf.clone(), c(2))), cf.clone());
    let assignment = assign(val, ah.clone(), size_relative(ah.clone()));
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF Offset(SRC) > descriptor table limit
///     THEN
///         ZF := 0;
///     ELSE
///         SegmentDescriptor := descriptor referenced by SRC;
///         IF SegmentDescriptor(Type) ≠ conforming code segment
///         and (CPL > DPL) or (RPL > DPL)
///         or SegmentDescriptor(Type) is not valid for instruction
///             THEN
///                 ZF := 0;
///             ELSE
///                 DEST := access rights from SegmentDescriptor as given in Description section;
///                 ZF := 1;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn lar() -> &'static [IrStatement] {
    [exception("lar")].into()
}

/// # Pseudocode
/// ```text
/// LDDQU (128-bit Legacy SSE Version)
/// DEST[127:0] := SRC[127:0]
/// DEST[MAXVL-1:128] (Unmodified)
/// VLDDQU (VEX.128 Encoded Version)
/// DEST[127:0] := SRC[127:0]
/// DEST[MAXVL-1:128] := 0
/// VLDDQU (VEX.256 Encoded Version)
/// DEST[255:0] := SRC[255:0]
/// ```
#[box_to_static_reference]
pub(super) fn lddqu() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MXCSR := m32;
/// ```
#[box_to_static_reference]
pub(super) fn ldmxcsr() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// 64-BIT_MODE
///     IF SS is loaded
///         THEN
///             IF SegmentSelector = NULL and ( (RPL = 3) or
///                     (RPL ≠ 3 and RPL ≠ CPL) )
///                 THEN #GP(0);
///             ELSE IF descriptor is in non-canonical space
///                 THEN #GP(selector); FI;
///             ELSE IF Segment selector index is not within descriptor table limits
///                     or segment selector RPL ≠ CPL
///                     or access rights indicate nonwritable data segment
///                     or DPL ≠ CPL
///                 THEN #GP(selector); FI;
///             ELSE IF Segment marked not present
///                 THEN #SS(selector); FI;
///             FI;
///             SS := SegmentSelector(SRC);
///             SS := SegmentDescriptor([SRC]);
///     ELSE IF attempt to load DS, or ES
///         THEN #UD;
///     ELSE IF FS, or GS is loaded with non-NULL segment selector
///         THEN IF Segment selector index is not within descriptor table limits
///             or access rights indicate segment neither data nor readable code segment
///             or segment is data or nonconforming-code segment
///             and ( RPL > DPL or CPL > DPL)
///                 THEN #GP(selector); FI;
///             ELSE IF Segment marked not present
///                 THEN #NP(selector); FI;
///             FI;
///             SegmentRegister := SegmentSelector(SRC) ;
///             SegmentRegister := SegmentDescriptor([SRC]);
///         FI;
///     ELSE IF FS, or GS is loaded with a NULL selector:
///         THEN
///             SegmentRegister := NULLSelector;
///             SegmentRegister(DescriptorValidBit) := 0; FI; (* Hidden flag;
///                 not accessible by software *)
///     FI;
///     DEST := Offset(SRC);
/// PREOTECTED MODE OR COMPATIBILITY MODE;
///     IF SS is loaded
///         THEN
///             IF SegementSelector = NULL
///                 THEN #GP(0);
///             ELSE IF Segment selector index is not within descriptor table limits
///                     or segment selector RPL ≠ CPL
///                     or access rights indicate nonwritable data segment
///                     or DPL ≠ CPL
///                 THEN #GP(selector); FI;
///             ELSE IF Segment marked not present
///                 THEN #SS(selector); FI;
///             FI;
///             SS := SegmentSelector(SRC);
///             SS := SegmentDescriptor([SRC]);
///     ELSE IF DS, ES, FS, or GS is loaded with non-NULL segment selector
///         THEN IF Segment selector index is not within descriptor table limits
///             or access rights indicate segment neither data nor readable code segment
///             or segment is data or nonconforming-code segment
///             and (RPL > DPL or CPL > DPL)
///                 THEN #GP(selector); FI;
///                 THEN #NP(selector); FI;
///             FI;
///             SegmentRegister := SegmentSelector(SRC) AND RPL;
///             SegmentRegister := SegmentDescriptor([SRC]);
///         FI;
///     ELSE IF DS, ES, FS, or GS is loaded with a NULL selector:
///         THEN
///             SegmentRegister := NULLSelector;
///             SegmentRegister(DescriptorValidBit) := 0; FI; (* Hidden flag;
///                 not accessible by software *)
///     FI;
///     DEST := Offset(SRC);
/// Real-Address or Virtual-8086 Mode
///     SegmentRegister := SegmentSelector(SRC); FI;
///     DEST := Offset(SRC);
/// ```
#[box_to_static_reference]
pub(super) fn lds() -> &'static [IrStatement] {
    [exception("lds")].into()
}

/// # Pseudocode
/// ```text
/// LDTILECFG mem
/// error :=False
/// buf := read_memory(mem, 64)
/// temp_tilecfg.palette_id := buf.byte[0]
/// if temp_tilecfg.palette_id > max_palette:
///     error := True
/// if not xcr0_supports_palette(temp_tilecfg.palette_id):
///     error := True
/// if temp_tilecfg.palette_id !=0:
///     temp_tilecfg.start_row := buf.byte[1]
///     if buf.byte[2..15] is nonzero:
///         error := True
///     p := 16
///     # configure columns
///     for n in 0 ... palette_table[temp_tilecfg.palette_id].max_names-1:
///         temp_tilecfg.t[n].colsb:= buf.word[p/2]
///         p := p + 2
///         if temp_tilecfg.t[n].colsb > palette_table[temp_tilecfg.palette_id].bytes_per_row:
///             error := True
///     if nonzero(buf[p...47]):
///         error := True
///     # configure rows
///     p := 48
///     for n in 0 ... palette_table[temp_tilecfg.palette_id].max_names-1:
///         temp_tilecfg.t[n].rows:= buf.byte[p]
///         if temp_tilecfg.t[n].rows > palette_table[temp_tilecfg.palette_id].max_rows:
///             error := True
///         p := p + 1
///     if nonzero(buf[p...63]):
///         error := True
///     # validate each tile's row & col configs are reasonable and enable the valid tiles
///     for n in 0 ... palette_table[temp_tilecfg.palette_id].max_names-1:
///         if temp_tilecfg.t[n].rows !=0 and temp_tilecfg.t[n].colsb != 0:
///             temp_tilecfg.t[n].valid := 1
///         elif temp_tilecfg.t[n].rows == 0 and temp_tilecfg.t[n].colsb == 0:
///             temp_tilecfg.t[n].valid := 0
///         else:
///             error := True// one of rows or colsbwas 0 but not both.
/// if error:
///     #GP
/// elif temp_tilecfg.palette_id == 0:
///     TILES_CONFIGURED := 0// init state
///     tilecfg := 0// equivalent to 64B of zeros
///     zero_all_tile_data()
/// else:
///     tilecfg := temp_tilecfg
///     zero_all_tile_data()
///     TILES_CONFIGURED := 1
/// ```
#[box_to_static_reference]
pub(super) fn ldtilecfg() -> &'static [IrStatement] {
    [exception("ldtilecfg")].into()
}

/// # Pseudocode
/// ```text
/// IF OperandSize = 16 and AddressSize = 16
///     THEN
///         DEST := EffectiveAddress(SRC); (* 16-bit address *)
///     ELSE IF OperandSize = 16 and AddressSize = 32
///         THEN
///             temp := EffectiveAddress(SRC); (* 32-bit address *)
///             DEST := temp[0:15]; (* 16-bit address *)
///         FI;
///     ELSE IF OperandSize = 32 and AddressSize = 16
///         THEN
///             temp := EffectiveAddress(SRC); (* 16-bit address *)
///             DEST := ZeroExtend(temp); (* 32-bit address *)
///         FI;
///     ELSE IF OperandSize = 32 and AddressSize = 32
///         THEN
///             DEST := EffectiveAddress(SRC); (* 32-bit address *)
///         FI;
///     ELSE IF OperandSize = 16 and AddressSize = 64
///         THEN
///             temp := EffectiveAddress(SRC); (* 64-bit address *)
///             DEST := temp[0:15]; (* 16-bit address *)
///         FI;
///     ELSE IF OperandSize = 32 and AddressSize = 64
///         THEN
///             temp := EffectiveAddress(SRC); (* 64-bit address *)
///             DEST := temp[0:31]; (* 16-bit address *)
///         FI;
///     ELSE IF OperandSize = 64 and AddressSize = 64
///         THEN
///             DEST := EffectiveAddress(SRC); (* 64-bit address *)
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn lea() -> &'static [IrStatement] {
    let address = u::zero_extend(d(o2()));
    let assignment = assign(address, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF StackAddressSize = 32
///     THEN
///         ESP := EBP;
///     ELSE IF StackAddressSize = 64
///         THEN RSP := RBP; FI;
///     ELSE IF StackAddressSize = 16
///         THEN SP := BP; FI;
/// FI;
/// IF OperandSize = 32
///     THEN EBP := Pop();
///     ELSE IF OperandSize = 64
///         THEN RBP := Pop(); FI;
///     ELSE IF OperandSize = 16
///         THEN BP := Pop(); FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn leave() -> &'static [IrStatement] {
    let restore_sp = assign(rbp.clone(), rsp.clone(), size_architecture());
    let pop_rbp = assign(d(rsp.clone()), rbp.clone(), size_architecture());
    let inc_sp = assign(b::add(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    [restore_sp, pop_rbp, inc_sp].into()
}

/// # Pseudocode
/// ```text
/// 64-BIT_MODE
///     IF SS is loaded
///         THEN
///             IF SegmentSelector = NULL and ( (RPL = 3) or
///                     (RPL ≠ 3 and RPL ≠ CPL) )
///                 THEN #GP(0);
///             ELSE IF descriptor is in non-canonical space
///                 THEN #GP(selector); FI;
///             ELSE IF Segment selector index is not within descriptor table limits
///                     or segment selector RPL ≠ CPL
///                     or access rights indicate nonwritable data segment
///                     or DPL ≠ CPL
///                 THEN #GP(selector); FI;
///             ELSE IF Segment marked not present
///                 THEN #SS(selector); FI;
///             FI;
///             SS := SegmentSelector(SRC);
///             SS := SegmentDescriptor([SRC]);
///     ELSE IF attempt to load DS, or ES
///         THEN #UD;
///     ELSE IF FS, or GS is loaded with non-NULL segment selector
///         THEN IF Segment selector index is not within descriptor table limits
///             or access rights indicate segment neither data nor readable code segment
///             or segment is data or nonconforming-code segment
///             and ( RPL > DPL or CPL > DPL)
///                 THEN #GP(selector); FI;
///             ELSE IF Segment marked not present
///                 THEN #NP(selector); FI;
///             FI;
///             SegmentRegister := SegmentSelector(SRC) ;
///             SegmentRegister := SegmentDescriptor([SRC]);
///         FI;
///     ELSE IF FS, or GS is loaded with a NULL selector:
///         THEN
///             SegmentRegister := NULLSelector;
///             SegmentRegister(DescriptorValidBit) := 0; FI; (* Hidden flag;
///                 not accessible by software *)
///     FI;
///     DEST := Offset(SRC);
/// PREOTECTED MODE OR COMPATIBILITY MODE;
///     IF SS is loaded
///         THEN
///             IF SegementSelector = NULL
///                 THEN #GP(0);
///             ELSE IF Segment selector index is not within descriptor table limits
///                     or segment selector RPL ≠ CPL
///                     or access rights indicate nonwritable data segment
///                     or DPL ≠ CPL
///                 THEN #GP(selector); FI;
///             ELSE IF Segment marked not present
///                 THEN #SS(selector); FI;
///             FI;
///             SS := SegmentSelector(SRC);
///             SS := SegmentDescriptor([SRC]);
///     ELSE IF DS, ES, FS, or GS is loaded with non-NULL segment selector
///         THEN IF Segment selector index is not within descriptor table limits
///             or access rights indicate segment neither data nor readable code segment
///             or segment is data or nonconforming-code segment
///             and (RPL > DPL or CPL > DPL)
///                 THEN #GP(selector); FI;
///                 THEN #NP(selector); FI;
///             FI;
///             SegmentRegister := SegmentSelector(SRC) AND RPL;
///             SegmentRegister := SegmentDescriptor([SRC]);
///         FI;
///     ELSE IF DS, ES, FS, or GS is loaded with a NULL selector:
///         THEN
///             SegmentRegister := NULLSelector;
///             SegmentRegister(DescriptorValidBit) := 0; FI; (* Hidden flag;
///                 not accessible by software *)
///     FI;
///     DEST := Offset(SRC);
/// Real-Address or Virtual-8086 Mode
///     SegmentRegister := SegmentSelector(SRC); FI;
///     DEST := Offset(SRC);
/// ```
#[box_to_static_reference]
pub(super) fn les() -> &'static [IrStatement] {
    [exception("les")].into()
}

/// # Pseudocode
/// ```text
/// Wait_On_Following_Instructions_Until(preceding_instructions_complete);
/// ```
#[box_to_static_reference]
pub(super) fn lfence() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// 64-BIT_MODE
///     IF SS is loaded
///         THEN
///             IF SegmentSelector = NULL and ( (RPL = 3) or
///                     (RPL ≠ 3 and RPL ≠ CPL) )
///                 THEN #GP(0);
///             ELSE IF descriptor is in non-canonical space
///                 THEN #GP(selector); FI;
///             ELSE IF Segment selector index is not within descriptor table limits
///                     or segment selector RPL ≠ CPL
///                     or access rights indicate nonwritable data segment
///                     or DPL ≠ CPL
///                 THEN #GP(selector); FI;
///             ELSE IF Segment marked not present
///                 THEN #SS(selector); FI;
///             FI;
///             SS := SegmentSelector(SRC);
///             SS := SegmentDescriptor([SRC]);
///     ELSE IF attempt to load DS, or ES
///         THEN #UD;
///     ELSE IF FS, or GS is loaded with non-NULL segment selector
///         THEN IF Segment selector index is not within descriptor table limits
///             or access rights indicate segment neither data nor readable code segment
///             or segment is data or nonconforming-code segment
///             and ( RPL > DPL or CPL > DPL)
///                 THEN #GP(selector); FI;
///             ELSE IF Segment marked not present
///                 THEN #NP(selector); FI;
///             FI;
///             SegmentRegister := SegmentSelector(SRC) ;
///             SegmentRegister := SegmentDescriptor([SRC]);
///         FI;
///     ELSE IF FS, or GS is loaded with a NULL selector:
///         THEN
///             SegmentRegister := NULLSelector;
///             SegmentRegister(DescriptorValidBit) := 0; FI; (* Hidden flag;
///                 not accessible by software *)
///     FI;
///     DEST := Offset(SRC);
/// PREOTECTED MODE OR COMPATIBILITY MODE;
///     IF SS is loaded
///         THEN
///             IF SegementSelector = NULL
///                 THEN #GP(0);
///             ELSE IF Segment selector index is not within descriptor table limits
///                     or segment selector RPL ≠ CPL
///                     or access rights indicate nonwritable data segment
///                     or DPL ≠ CPL
///                 THEN #GP(selector); FI;
///             ELSE IF Segment marked not present
///                 THEN #SS(selector); FI;
///             FI;
///             SS := SegmentSelector(SRC);
///             SS := SegmentDescriptor([SRC]);
///     ELSE IF DS, ES, FS, or GS is loaded with non-NULL segment selector
///         THEN IF Segment selector index is not within descriptor table limits
///             or access rights indicate segment neither data nor readable code segment
///             or segment is data or nonconforming-code segment
///             and (RPL > DPL or CPL > DPL)
///                 THEN #GP(selector); FI;
///                 THEN #NP(selector); FI;
///             FI;
///             SegmentRegister := SegmentSelector(SRC) AND RPL;
///             SegmentRegister := SegmentDescriptor([SRC]);
///         FI;
///     ELSE IF DS, ES, FS, or GS is loaded with a NULL selector:
///         THEN
///             SegmentRegister := NULLSelector;
///             SegmentRegister(DescriptorValidBit) := 0; FI; (* Hidden flag;
///                 not accessible by software *)
///     FI;
///     DEST := Offset(SRC);
/// Real-Address or Virtual-8086 Mode
///     SegmentRegister := SegmentSelector(SRC); FI;
///     DEST := Offset(SRC);
/// ```
#[box_to_static_reference]
pub(super) fn lfs() -> &'static [IrStatement] {
    [exception("lfs")].into()
}

/// # Pseudocode
/// ```text
/// IF Instruction is LIDT
///     THEN
///         IF OperandSize = 16
///             THEN
///                 IDTR(Limit) := SRC[0:15];
///                 IDTR(Base) := SRC[16:47] AND 00FFFFFFH;
///             ELSE IF 32-bit Operand Size
///                 THEN
///                     IDTR(Limit) := SRC[0:15];
///                     IDTR(Base) := SRC[16:47];
///                 FI;
///             ELSE IF 64-bit Operand Size (* In 64-Bit Mode *)
///                 THEN
///                     IDTR(Limit) := SRC[0:15];
///                     IDTR(Base) := SRC[16:79];
///                 FI;
///         FI;
///     ELSE (* Instruction is LGDT *)
///         IF OperandSize = 16
///             THEN
///                 GDTR(Limit) := SRC[0:15];
///                 GDTR(Base) := SRC[16:47] AND 00FFFFFFH;
///             ELSE IF 32-bit Operand Size
///                 THEN
///                     GDTR(Limit) := SRC[0:15];
///                     GDTR(Base) := SRC[16:47];
///                 FI;
///             ELSE IF 64-bit Operand Size (* In 64-Bit Mode *)
///                 THEN
///                     GDTR(Limit) := SRC[0:15];
///                     GDTR(Base) := SRC[16:79];
///                 FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn lgdt() -> &'static [IrStatement] {
    [exception("LGDT")].into()
}

/// # Pseudocode
/// ```text
/// 64-BIT_MODE
///     IF SS is loaded
///         THEN
///             IF SegmentSelector = NULL and ( (RPL = 3) or
///                     (RPL ≠ 3 and RPL ≠ CPL) )
///                 THEN #GP(0);
///             ELSE IF descriptor is in non-canonical space
///                 THEN #GP(selector); FI;
///             ELSE IF Segment selector index is not within descriptor table limits
///                     or segment selector RPL ≠ CPL
///                     or access rights indicate nonwritable data segment
///                     or DPL ≠ CPL
///                 THEN #GP(selector); FI;
///             ELSE IF Segment marked not present
///                 THEN #SS(selector); FI;
///             FI;
///             SS := SegmentSelector(SRC);
///             SS := SegmentDescriptor([SRC]);
///     ELSE IF attempt to load DS, or ES
///         THEN #UD;
///     ELSE IF FS, or GS is loaded with non-NULL segment selector
///         THEN IF Segment selector index is not within descriptor table limits
///             or access rights indicate segment neither data nor readable code segment
///             or segment is data or nonconforming-code segment
///             and ( RPL > DPL or CPL > DPL)
///                 THEN #GP(selector); FI;
///             ELSE IF Segment marked not present
///                 THEN #NP(selector); FI;
///             FI;
///             SegmentRegister := SegmentSelector(SRC) ;
///             SegmentRegister := SegmentDescriptor([SRC]);
///         FI;
///     ELSE IF FS, or GS is loaded with a NULL selector:
///         THEN
///             SegmentRegister := NULLSelector;
///             SegmentRegister(DescriptorValidBit) := 0; FI; (* Hidden flag;
///                 not accessible by software *)
///     FI;
///     DEST := Offset(SRC);
/// PREOTECTED MODE OR COMPATIBILITY MODE;
///     IF SS is loaded
///         THEN
///             IF SegementSelector = NULL
///                 THEN #GP(0);
///             ELSE IF Segment selector index is not within descriptor table limits
///                     or segment selector RPL ≠ CPL
///                     or access rights indicate nonwritable data segment
///                     or DPL ≠ CPL
///                 THEN #GP(selector); FI;
///             ELSE IF Segment marked not present
///                 THEN #SS(selector); FI;
///             FI;
///             SS := SegmentSelector(SRC);
///             SS := SegmentDescriptor([SRC]);
///     ELSE IF DS, ES, FS, or GS is loaded with non-NULL segment selector
///         THEN IF Segment selector index is not within descriptor table limits
///             or access rights indicate segment neither data nor readable code segment
///             or segment is data or nonconforming-code segment
///             and (RPL > DPL or CPL > DPL)
///                 THEN #GP(selector); FI;
///                 THEN #NP(selector); FI;
///             FI;
///             SegmentRegister := SegmentSelector(SRC) AND RPL;
///             SegmentRegister := SegmentDescriptor([SRC]);
///         FI;
///     ELSE IF DS, ES, FS, or GS is loaded with a NULL selector:
///         THEN
///             SegmentRegister := NULLSelector;
///             SegmentRegister(DescriptorValidBit) := 0; FI; (* Hidden flag;
///                 not accessible by software *)
///     FI;
///     DEST := Offset(SRC);
/// Real-Address or Virtual-8086 Mode
///     SegmentRegister := SegmentSelector(SRC); FI;
///     DEST := Offset(SRC);
/// ```
#[box_to_static_reference]
pub(super) fn lgs() -> &'static [IrStatement] {
    [exception("lgs")].into()
}

/// # Pseudocode
/// ```text
/// IF Instruction is LIDT
///     THEN
///         IF OperandSize = 16
///             THEN
///                 IDTR(Limit) := SRC[0:15];
///                 IDTR(Base) := SRC[16:47] AND 00FFFFFFH;
///             ELSE IF 32-bit Operand Size
///                 THEN
///                     IDTR(Limit) := SRC[0:15];
///                     IDTR(Base) := SRC[16:47];
///                 FI;
///             ELSE IF 64-bit Operand Size (* In 64-Bit Mode *)
///                 THEN
///                     IDTR(Limit) := SRC[0:15];
///                     IDTR(Base) := SRC[16:79];
///                 FI;
///         FI;
///     ELSE (* Instruction is LGDT *)
///         IF OperandSize = 16
///             THEN
///                 GDTR(Limit) := SRC[0:15];
///                 GDTR(Base) := SRC[16:47] AND 00FFFFFFH;
///             ELSE IF 32-bit Operand Size
///                 THEN
///                     GDTR(Limit) := SRC[0:15];
///                     GDTR(Base) := SRC[16:47];
///                 FI;
///             ELSE IF 64-bit Operand Size (* In 64-Bit Mode *)
///                 THEN
///                     GDTR(Limit) := SRC[0:15];
///                     GDTR(Base) := SRC[16:79];
///                 FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn lidt() -> &'static [IrStatement] {
    [exception("LIDT")].into()
}

/// # Pseudocode
/// ```text
/// IF SRC(Offset) > descriptor table limit
///     THEN #GP(segment selector); FI;
/// IF segment selector is valid
///     Read segment descriptor;
///     IF SegmentDescriptor(Type) ≠ LDT
///         THEN #GP(segment selector); FI;
///     IF segment descriptor is not present
///         THEN #NP(segment selector); FI;
///     LDTR(SegmentSelector) := SRC;
///     LDTR(SegmentDescriptor) := GDTSegmentDescriptor;
/// ELSE LDTR := INVALID
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn lldt() -> &'static [IrStatement] {
    [exception("LLDT")].into()
}

/// # Pseudocode
/// ```text
/// CR0[0:3] := SRC[0:3];
/// ```
#[box_to_static_reference]
pub(super) fn lmsw() -> &'static [IrStatement] {
    let stmt_0 = assign(o2(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// LOADIWKEY
/// IF CPL > 0
///                     // LOADKWKEY only allowed at ring 0 (supervisor mode)
///     THEN #GP (0); FI;
/// IF EAX[4:1] > 1
///                     // Reserved KeySource encoding used
///     THEN #GP (0); FI;
/// IF EAX[31:5] != 0
///                     // Reserved bit in EAX is set
///     THEN #GP (0); FI;
/// IF EAX[0] AND (CPUID.19H.ECX[0] == 0)
///                         // NoBackup is not supported on this part
///     THEN #GP (0); FI;
/// IF (EAX[4:1] == 1) AND (CPUID.19H.ECX[1] == 0) // KeySource of 1 is not supported on this part
///     THEN #GP (0); FI;
/// IF (EAX[4:1] == 0) // KeySource of 0
///     THEN
///         IWKey.Encryption Key[127:0] := SRC2[127:0]:
///         IWKey.Encryption Key[255:128] := SRC1[127:0];
///         IWKey.IntegrityKey[127:0] := XMM0[127:0];
///         IWKey.NoBackup = EAX [0];
///         IWKey.KeySource = EAX [4:1];
///         RFLAGS.ZF := 0;
///     ELSE
///                     // KeySource of 1. See RDSEED definition for details of randomness
///         IF HW_NRND_GEN.ready == 1
///                             // Full-entropy random data from RDSEED hardware block was received
///             THEN
///                 IWKey.Encryption Key[127:0] := SRC2[127:0] XOR HW_NRND_GEN.data[127:0];
///                 IWKey.Encryption Key[255:128] := SRC1[127:0] XOR HW_NRND_GEN.data[255:128];
///                 IWKey.IntegrityKey[127:0] := XMM0[127:0] XOR HW_NRND_GEN.data[383:256];
///                 IWKey.NoBackup = EAX [0];
///                 IWKey.KeySource = EAX [4:1];
///                 RFLAGS.ZF := 0;
///             ELSE
///                     // Random data was not returned from RDSEED hardware block. IWKey was not loaded
///                 RFLAGS.ZF := 1;
///         FI;
/// FI;
/// RFLAGS.OF, SF, AF, PF, CF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn loadiwkey() -> &'static [IrStatement] {
    [exception("loadiwkey")].into()
}

/// # Pseudocode
/// ```text
/// AssertLOCK#(DurationOfAccompaningInstruction);
/// ```
#[box_to_static_reference]
pub(super) fn lock() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// IF (AddressSize = 32)
///     THEN Count is ECX;
/// ELSE IF (AddressSize = 64)
///     Count is RCX;
/// ELSE Count is CX;
/// FI;
/// Count := Count - 1;
/// IF Instruction is not LOOP
///     THEN
///         IF (Instruction := LOOPE) or (Instruction := LOOPZ)
///             THEN IF (ZF = 1) and (Count ≠ 0)
///                     THEN BranchCond := 1;
///                     ELSE BranchCond := 0;
///                 FI;
///             ELSE (Instruction = LOOPNE) or (Instruction = LOOPNZ)
///                 IF (ZF = 0 ) and (Count ≠ 0)
///                     THEN BranchCond := 1;
///                     ELSE BranchCond := 0;
///                 FI;
///         FI;
///     ELSE (* Instruction = LOOP *)
///         IF (Count ≠ 0)
///             THEN BranchCond := 1;
///             ELSE BranchCond := 0;
///         FI;
/// FI;
/// IF BranchCond = 1
///     THEN
///         IF in 64-bit mode (* OperandSize = 64 *)
///             THEN
///                 tempRIP := RIP + SignExtend(DEST);
///                 IF tempRIP is not canonical
///                     THEN #GP(0);
///                 ELSE RIP := tempRIP;
///                 FI;
///             ELSE
///                 tempEIP := EIP   SignExtend(DEST);
///                 IF OperandSize   16
///                     THEN tempEIP := tempEIP AND 0000FFFFH;
///                 FI;
///                 IF tempEIP is not within code segment limit
///                     THEN #GP(0);
///                     ELSE EIP := tempEIP;
///                 FI;
///         FI;
///     ELSE
///         Terminate loop and continue program execution at (R/E)IP;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn lods() -> &'static [IrStatement] {
    let lods = assign(d(rsi.clone()), rax.clone(), size_architecture());
    [lods].into()
}

/// # Pseudocode
/// ```text
/// IF (AddressSize = 32)
///     THEN Count is ECX;
/// ELSE IF (AddressSize = 64)
///     Count is RCX;
/// ELSE Count is CX;
/// FI;
/// Count := Count - 1;
/// IF Instruction is not LOOP
///     THEN
///         IF (Instruction := LOOPE) or (Instruction := LOOPZ)
///             THEN IF (ZF = 1) and (Count ≠ 0)
///                     THEN BranchCond := 1;
///                     ELSE BranchCond := 0;
///                 FI;
///             ELSE (Instruction = LOOPNE) or (Instruction = LOOPNZ)
///                 IF (ZF = 0 ) and (Count ≠ 0)
///                     THEN BranchCond := 1;
///                     ELSE BranchCond := 0;
///                 FI;
///         FI;
///     ELSE (* Instruction = LOOP *)
///         IF (Count ≠ 0)
///             THEN BranchCond := 1;
///             ELSE BranchCond := 0;
///         FI;
/// FI;
/// IF BranchCond = 1
///     THEN
///         IF in 64-bit mode (* OperandSize = 64 *)
///             THEN
///                 tempRIP := RIP + SignExtend(DEST);
///                 IF tempRIP is not canonical
///                     THEN #GP(0);
///                 ELSE RIP := tempRIP;
///                 FI;
///             ELSE
///                 tempEIP := EIP   SignExtend(DEST);
///                 IF OperandSize   16
///                     THEN tempEIP := tempEIP AND 0000FFFFH;
///                 FI;
///                 IF tempEIP is not within code segment limit
///                     THEN #GP(0);
///                     ELSE EIP := tempEIP;
///                 FI;
///         FI;
///     ELSE
///         Terminate loop and continue program execution at (R/E)IP;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn lodsb() -> &'static [IrStatement] {
    let lods = assign(d(rsi.clone()), rax.clone(), size_result_byte(c(1)));
    [lods].into()
}

/// # Pseudocode
/// ```text
/// IF (AddressSize = 32)
///     THEN Count is ECX;
/// ELSE IF (AddressSize = 64)
///     Count is RCX;
/// ELSE Count is CX;
/// FI;
/// Count := Count - 1;
/// IF Instruction is not LOOP
///     THEN
///         IF (Instruction := LOOPE) or (Instruction := LOOPZ)
///             THEN IF (ZF = 1) and (Count ≠ 0)
///                     THEN BranchCond := 1;
///                     ELSE BranchCond := 0;
///                 FI;
///             ELSE (Instruction = LOOPNE) or (Instruction = LOOPNZ)
///                 IF (ZF = 0 ) and (Count ≠ 0)
///                     THEN BranchCond := 1;
///                     ELSE BranchCond := 0;
///                 FI;
///         FI;
///     ELSE (* Instruction = LOOP *)
///         IF (Count ≠ 0)
///             THEN BranchCond := 1;
///             ELSE BranchCond := 0;
///         FI;
/// FI;
/// IF BranchCond = 1
///     THEN
///         IF in 64-bit mode (* OperandSize = 64 *)
///             THEN
///                 tempRIP := RIP + SignExtend(DEST);
///                 IF tempRIP is not canonical
///                     THEN #GP(0);
///                 ELSE RIP := tempRIP;
///                 FI;
///             ELSE
///                 tempEIP := EIP   SignExtend(DEST);
///                 IF OperandSize   16
///                     THEN tempEIP := tempEIP AND 0000FFFFH;
///                 FI;
///                 IF tempEIP is not within code segment limit
///                     THEN #GP(0);
///                     ELSE EIP := tempEIP;
///                 FI;
///         FI;
///     ELSE
///         Terminate loop and continue program execution at (R/E)IP;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn lodsd() -> &'static [IrStatement] {
    let lods = assign(d(rsi.clone()), rax.clone(), size_result_byte(c(4)));
    [lods].into()
}

/// # Pseudocode
/// ```text
/// IF (AddressSize = 32)
///     THEN Count is ECX;
/// ELSE IF (AddressSize = 64)
///     Count is RCX;
/// ELSE Count is CX;
/// FI;
/// Count := Count - 1;
/// IF Instruction is not LOOP
///     THEN
///         IF (Instruction := LOOPE) or (Instruction := LOOPZ)
///             THEN IF (ZF = 1) and (Count ≠ 0)
///                     THEN BranchCond := 1;
///                     ELSE BranchCond := 0;
///                 FI;
///             ELSE (Instruction = LOOPNE) or (Instruction = LOOPNZ)
///                 IF (ZF = 0 ) and (Count ≠ 0)
///                     THEN BranchCond := 1;
///                     ELSE BranchCond := 0;
///                 FI;
///         FI;
///     ELSE (* Instruction = LOOP *)
///         IF (Count ≠ 0)
///             THEN BranchCond := 1;
///             ELSE BranchCond := 0;
///         FI;
/// FI;
/// IF BranchCond = 1
///     THEN
///         IF in 64-bit mode (* OperandSize = 64 *)
///             THEN
///                 tempRIP := RIP + SignExtend(DEST);
///                 IF tempRIP is not canonical
///                     THEN #GP(0);
///                 ELSE RIP := tempRIP;
///                 FI;
///             ELSE
///                 tempEIP := EIP   SignExtend(DEST);
///                 IF OperandSize   16
///                     THEN tempEIP := tempEIP AND 0000FFFFH;
///                 FI;
///                 IF tempEIP is not within code segment limit
///                     THEN #GP(0);
///                     ELSE EIP := tempEIP;
///                 FI;
///         FI;
///     ELSE
///         Terminate loop and continue program execution at (R/E)IP;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn lodsq() -> &'static [IrStatement] {
    let lods = assign(d(rsi.clone()), rax.clone(), size_result_byte(c(8)));
    [lods].into()
}

/// # Pseudocode
/// ```text
/// IF (AddressSize = 32)
///     THEN Count is ECX;
/// ELSE IF (AddressSize = 64)
///     Count is RCX;
/// ELSE Count is CX;
/// FI;
/// Count := Count - 1;
/// IF Instruction is not LOOP
///     THEN
///         IF (Instruction := LOOPE) or (Instruction := LOOPZ)
///             THEN IF (ZF = 1) and (Count ≠ 0)
///                     THEN BranchCond := 1;
///                     ELSE BranchCond := 0;
///                 FI;
///             ELSE (Instruction = LOOPNE) or (Instruction = LOOPNZ)
///                 IF (ZF = 0 ) and (Count ≠ 0)
///                     THEN BranchCond := 1;
///                     ELSE BranchCond := 0;
///                 FI;
///         FI;
///     ELSE (* Instruction = LOOP *)
///         IF (Count ≠ 0)
///             THEN BranchCond := 1;
///             ELSE BranchCond := 0;
///         FI;
/// FI;
/// IF BranchCond = 1
///     THEN
///         IF in 64-bit mode (* OperandSize = 64 *)
///             THEN
///                 tempRIP := RIP + SignExtend(DEST);
///                 IF tempRIP is not canonical
///                     THEN #GP(0);
///                 ELSE RIP := tempRIP;
///                 FI;
///             ELSE
///                 tempEIP := EIP   SignExtend(DEST);
///                 IF OperandSize   16
///                     THEN tempEIP := tempEIP AND 0000FFFFH;
///                 FI;
///                 IF tempEIP is not within code segment limit
///                     THEN #GP(0);
///                     ELSE EIP := tempEIP;
///                 FI;
///         FI;
///     ELSE
///         Terminate loop and continue program execution at (R/E)IP;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn lodsw() -> &'static [IrStatement] {
    let lods = assign(d(rsi.clone()), rax.clone(), size_result_byte(c(2)));
    [lods].into()
}

/// # Pseudocode
/// ```text
/// IF SRC(Offset) > descriptor table limit
///     THEN ZF := 0; FI;
/// Read segment descriptor;
/// IF SegmentDescriptor(Type) ≠ conforming code segment
/// and (CPL > DPL) OR (RPL > DPL)
/// or Segment type is not valid for instruction
///         THEN
///             ZF := 0;
///         ELSE
///             temp := SegmentLimit([SRC]);
///             IF (SegmentDescriptor(G) = 1)
///                 THEN temp := (temp << 12) OR 00000FFFH;
///             ELSE IF OperandSize = 32
///                 THEN DEST := temp; FI;
///             ELSE IF OperandSize = 64 (* REX.W used *)
///                 THEN DEST := temp(* Zero-extended *); FI;
///             ELSE (* OperandSize = 16 *)
///                 DEST := temp AND FFFFH;
///             FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn lsl() -> &'static [IrStatement] {
    let stmt_0 = assign(c(0), zf.clone(), size_relative(zf.clone()));
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// 64-BIT_MODE
///     IF SS is loaded
///         THEN
///             IF SegmentSelector = NULL and ( (RPL = 3) or
///                     (RPL ≠ 3 and RPL ≠ CPL) )
///                 THEN #GP(0);
///             ELSE IF descriptor is in non-canonical space
///                 THEN #GP(selector); FI;
///             ELSE IF Segment selector index is not within descriptor table limits
///                     or segment selector RPL ≠ CPL
///                     or access rights indicate nonwritable data segment
///                     or DPL ≠ CPL
///                 THEN #GP(selector); FI;
///             ELSE IF Segment marked not present
///                 THEN #SS(selector); FI;
///             FI;
///             SS := SegmentSelector(SRC);
///             SS := SegmentDescriptor([SRC]);
///     ELSE IF attempt to load DS, or ES
///         THEN #UD;
///     ELSE IF FS, or GS is loaded with non-NULL segment selector
///         THEN IF Segment selector index is not within descriptor table limits
///             or access rights indicate segment neither data nor readable code segment
///             or segment is data or nonconforming-code segment
///             and ( RPL > DPL or CPL > DPL)
///                 THEN #GP(selector); FI;
///             ELSE IF Segment marked not present
///                 THEN #NP(selector); FI;
///             FI;
///             SegmentRegister := SegmentSelector(SRC) ;
///             SegmentRegister := SegmentDescriptor([SRC]);
///         FI;
///     ELSE IF FS, or GS is loaded with a NULL selector:
///         THEN
///             SegmentRegister := NULLSelector;
///             SegmentRegister(DescriptorValidBit) := 0; FI; (* Hidden flag;
///                 not accessible by software *)
///     FI;
///     DEST := Offset(SRC);
/// PREOTECTED MODE OR COMPATIBILITY MODE;
///     IF SS is loaded
///         THEN
///             IF SegementSelector = NULL
///                 THEN #GP(0);
///             ELSE IF Segment selector index is not within descriptor table limits
///                     or segment selector RPL ≠ CPL
///                     or access rights indicate nonwritable data segment
///                     or DPL ≠ CPL
///                 THEN #GP(selector); FI;
///             ELSE IF Segment marked not present
///                 THEN #SS(selector); FI;
///             FI;
///             SS := SegmentSelector(SRC);
///             SS := SegmentDescriptor([SRC]);
///     ELSE IF DS, ES, FS, or GS is loaded with non-NULL segment selector
///         THEN IF Segment selector index is not within descriptor table limits
///             or access rights indicate segment neither data nor readable code segment
///             or segment is data or nonconforming-code segment
///             and (RPL > DPL or CPL > DPL)
///                 THEN #GP(selector); FI;
///                 THEN #NP(selector); FI;
///             FI;
///             SegmentRegister := SegmentSelector(SRC) AND RPL;
///             SegmentRegister := SegmentDescriptor([SRC]);
///         FI;
///     ELSE IF DS, ES, FS, or GS is loaded with a NULL selector:
///         THEN
///             SegmentRegister := NULLSelector;
///             SegmentRegister(DescriptorValidBit) := 0; FI; (* Hidden flag;
///                 not accessible by software *)
///     FI;
///     DEST := Offset(SRC);
/// Real-Address or Virtual-8086 Mode
///     SegmentRegister := SegmentSelector(SRC); FI;
///     DEST := Offset(SRC);
/// ```
#[box_to_static_reference]
pub(super) fn lss() -> &'static [IrStatement] {
    [exception("lss")].into()
}

/// # Pseudocode
/// ```text
/// IF SRC is a NULL selector
///     THEN #GP(0);
/// IF SRC(Offset) > descriptor table limit OR IF SRC(type) ≠ global
///     THEN #GP(segment selector); FI;
/// Read segment descriptor;
/// IF segment descriptor is not for an available TSS
///     THEN #GP(segment selector); FI;
/// IF segment descriptor is not present
///     THEN #NP(segment selector); FI;
/// TSSsegmentDescriptor(busy) := 1;
/// (* Locked read-modify-write operation on the entire descriptor when setting busy flag *)
/// TaskRegister(SegmentSelector) := SRC;
/// TaskRegister(SegmentDescriptor) := TSSSegmentDescriptor;
/// ```
#[box_to_static_reference]
pub(super) fn ltr() -> &'static [IrStatement] {
    [exception("LTR")].into()
}

/// # Pseudocode
/// ```text
/// temp := OperandSize - 1
/// DEST := 0
/// WHILE (temp >= 0) AND (Bit(SRC, temp) = 0)
/// DO
///     temp := temp - 1
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
pub(super) fn lzcnt() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    let set_of = assign(c(0), of.clone(), size_relative(of.clone()));
    let set_sf = assign(c(0), sf.clone(), size_relative(sf.clone()));
    let set_af = assign(c(0), af.clone(), size_relative(af.clone()));
    let set_cf = assign(c(0), cf.clone(), size_relative(cf.clone()));
    let set_pf = assign(c(0), pf.clone(), size_relative(pf.clone()));
    let set_zf = condition(b::equal(o1(), c(0), o1_size()), [assign(c(1), zf.clone(), size_relative(zf.clone()))], [assign(c(0), zf.clone(), size_relative(zf.clone()))]);
    [assignment, set_of, set_sf, set_af, set_cf, set_pf, set_zf].into()
}
