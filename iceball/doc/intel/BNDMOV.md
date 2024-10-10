# BNDMOV

Move Bounds

InstructionMode Feature SupportFlag66 0F 1A /rRMNE/VMPXMove lower and upper bound from bnd2/m64 to bound register BNDMOV bnd1, bnd2/m64bnd1.66 0F 1A /rRMV/NEMPXMove lower and upper bound from bnd2/m128 to bound register BNDMOV bnd1, bnd2/m128bnd1.66 0F 1B /rMRNE/VMPXMove lower and upper bound from bnd2 to bnd1/m64.BNDMOV bnd1/m64, bnd266 0F 1B /rMRV/NEMPXMove lower and upper bound from bnd2 to bound register BNDMOV bnd1/m128, bnd2bnd1/m128.Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RMModRM:reg (w)ModRM:r/m (r)N/AMRModRM:r/m (w)ModRM:reg (r)N/ABNDMOV moves a pair of lower and upper bound values from the source operand (the second operand) to the destination (the first operand).
Each operation is 128-bit move.
The exceptions are same as the MOV instruction.
The memory format for loading/store bounds in 64-bit mode is shown in Figure3-5.BNDMOV to memory in 32-bit mode4016Byte offsetBNDMOV to memory in 64-bit modeUpper Bound (UB)Lower Bound (LB)8Figure 3-5.
 Memory Layout of BNDMOV to/from Memory016Byte offsetUpper Bound (UB)Lower Bound (LB)8This instruction does not change flags.

## Flags affected

- None.

## Exceptions

- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used but the destination is not a memory operand.
  > If ModRM.r/m encodes BND4-BND7 when Intel MPX is enabled.
  > If 67H prefix is not used and CS.D=0.
  > If 67H prefix is used and CS.D=1.
  - #SS(0) - If the memory operand effective address is outside the SS segment limit.
  - #GP(0) - If the memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If the destination operand points to a non-writable segment
  > If the DS, ES, FS, or GS segment register contains a NULL segment selector.
  - #AC(0) - If alignment checking is enabled and an una
  > ligned memory reference is made while CPL is 3.
  - #PF(fault code) - If a page fault occurs.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  - #UD - If the LOCK prefix is used but the destination is not a memory operand.
  > If ModRM.r/m encodes BND4-BND7 when Intel MPX is enabled.
  > If 16-bit addressing is used.
  - #GP(0) - If the memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If the memory operand effective ad
  > dress is outside the SS segment limit.
  - #AC(0) - If alignment checking is enabled and an un
  > aligned memory reference is made while CPL is 3.
  - #PF(fault code) - If a page fault occurs.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used but the destination is not a memory operand.
  > If ModRM.r/m encodes BND4-BND7 when Intel MPX is enabled.
  > If 16-bit addressing is used.
  - #GP(0) - If the memory operand effective address is
- 64-Bit Mode Exceptions
  - #UD - If the LOCK prefix is used but the destination is not a memory operand.
  > If ModRM.r/m and REX encodes BND4-BND15 when Intel MPX is enabled.
  - #SS(0) - If the memory address referencing th
  > e SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #AC(0) - If alignment checking is enabled and an un

## Operation

```C
BNDMOV register to registerBNDMOV from memoryIF 64-bit mode THENDEST.LB := LOAD_QWORD(SRC); DEST.UB := LOAD_QWORD(SRC+8); ELSEDEST.LB := LOAD_DWORD_ZERO_EXT(SRC); DEST.UB := LOAD_DWORD_ZERO_EXT(SRC+4); FI;BNDMOV to memoryIF 64-bit mode THENDEST[63:0] := SRC.LB; DEST[127:64] := SRC.UB; ELSEDEST[31:0] := SRC.LB; DEST[63:32] := SRC.UB; FI;Intel C/C++ Compiler Intrinsic EquivalentBNDMOV void * _bnd_copy_ptr_bounds(const void *q, const void *r)
```
