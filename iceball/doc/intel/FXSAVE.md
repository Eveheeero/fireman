# FXSAVE

Save x87 FPU, MMX Technology, and SSE State

Saves the current state of the x87 FPU, MMX technology, XMM, and MXCSR registers to a 512-byte memory loca-tion specified in the destination operand.
The content layout of the 512 byte region depends on whether the processor is operating in non-64-bit operating modes or 64-bit sub-mode of IA-32e mode.
Bytes 464:511 are available to software use.
The processor does not write to bytes 464:511 of an FXSAVE area.
The operation of FXSAVE in non-64-bit modes is described first.Non-64-Bit Mode OperationTable 3-43 shows the layout of the state information in memory when the processor is operating in legacy modes.Table 3-43.
 Non-64-Bit-Mode Layout of FXSAVE and FXRSTOR Memory Region151413 1211109 8765 43210RsvdFCSFIP[31:0]FOPRsvdFSWFCWFTW0MXCSR_MASKMXCSRRsrvdFDSFDP[31:0]16ReservedST0/MM032ReservedST1/MM148ReservedST2/MM264ReservedST3/MM380ReservedST4/MM496ReservedST5/MM5112ReservedST6/MM6128ReservedST7/MM7144XMM0160XMM1176XMM2192XMM3208XMM4224XMM5240XMM6256XMM7272Table 3-43.
 Non-64-Bit-Mode Layout of FXSAVE and FXRSTOR Memory Region (Contd.)151413 1211109 8765 43210Reserved304Reserved320Reserved336Reserved352Reserved368Reserved384Reserved400Reserved416Reserved432Reserved448Available464Available480Available496The destination operand contains the first byte of the memory image, and it must be aligned on a 16-byte boundary.
A misaligned destination operand will result in a general-protection (#GP) exception being generated (or in some cases, an alignment check exception [#AC]).The FXSAVE instruction is used when an operating system needs to perform a context switch or when an exception handler needs to save and examine the current state of the x87 FPU, MMX technology, and/or XMM and MXCSR registers.The fields in Table 3-43 are defined in Table 3-44.Table 3-44.
 Field Definitions FieldDefinition®FCWx87 FPU Control Word (16 bits).
See Figure 8-6 in the Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for the layout of the x87 FPU control word.®FSWx87 FPU Status Word (16 bits).
See Figure 8-4 in the Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for the layout of the x87 FPU status word.Abridged FTWx87 FPU Tag Word (8 bits).
The tag information saved here is abridged, as described in the following paragraphs.FOPx87 FPU Opcode (16 bits).
The lower 11 bits of this field contain the opcode, upper 5 bits are reserved.
®See Figure 8-8 in the Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for the layout of the x87 FPU opcode field.FIPx87 FPU Instruction Pointer Offset (64 bits).
The contents of this field differ depending on the current addressing mode (32-bit, 16-bit, or 64-bit) of the processor when the FXSAVE instruction was executed:32-bit mode - 32-bit IP offset.16-bit mode - low 16 bits are IP offset; high 16 bits are reserved.64-bit mode with REX.W - 64-bit IP offset.64-bit mode without REX.W - 32-bit IP offset.See "x87 FPU Instruction and Operand (Data) Pointers" in Chapter 8 of the Intel® 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for a description of the x87 FPU instruction pointer.FCSx87 FPU Instruction Pointer Selector (16 bits).Table 3-44.
 Field Definitions  (Contd.)FieldDefinitionFDPx87 FPU Instruction Operand (Data) Pointer Offset (64 bits).
The contents of this field differ depending on the current addressing mode (32-bit, 16-bit, or 64-bit) of the processor when the FXSAVE instruction was executed:32-bit mode - 32-bit DP offset.16-bit mode - low 16 bits are DP offset; high 16 bits are reserved.64-bit mode with REX.W - 64-bit DP offset.64-bit mode without REX.W - 32-bit DP offset.® 64 and IA-32 See "x87 FPU Instruction and Operand (Data) Pointers" in Chapter 8 of the IntelArchitectures Software Developer's Manual, Volume 1, for a description of the x87 FPU operand pointer.FDSx87 FPU Instruction Operand (Data) Pointer Selector (16 bits).
If CPUID.(EAX=07H,ECX=0H):EBX[bit 13]= 1, the processor deprecates FCS and FDS, and this field is saved as 0000H.®MXCSRMXCSR Register State (32 bits).
See Figure 10-3 in the Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for the layout of the MXCSR register.
If the OSFXSR bit in control register CR4 is not set, the FXSAVE instruction may not save this register.
This behavior is implementation dependent.MXCSR_MXCSR_MASK (32 bits).
This mask can be used to adjust values written to the MXCSR register, MASKensuring that reserved bits are set to 0.
Set the mask bits and flags in MXCSR to the mode of operation desired for SSE and SSE2 SIMD floating-point instructions.
See "Guidelines for Writing to the ® 64 and IA-32 Architectures Software Developer's Manual, MXCSR Register" in Chapter 11 of the IntelVolume 1, for instructions for how to determine and use the MXCSR_MASK value.ST0/MM0 through x87 FPU or MMX technology registers.
These 80-bit fields contain the x87 FPU data registers or the ST7/MM7MMX technology registers, depending on the state of the processor prior to the execution of the FXSAVE instruction.
If the processor had been executing x87 FPU instruction prior to the FXSAVE instruction, the x87 FPU data registers are saved; if it had been executing MMX instructions (or SSE or SSE2 instructions that operated on the MMX technology registers), the MMX technology registers are saved.
When the MMX technology registers are saved, the high 16 bits of the field are reserved.XMM0 through XMM7XMM registers (128 bits per field).
If the OSFXSR bit in control register CR4 is not set, the FXSAVE instruction may not save these registers.
This behavior is implementation dependent.The FXSAVE instruction saves an abridged version of the x87 FPU tag word in the FTW field (unlike the FSAVE instruction, which saves the complete tag word).
The tag information is saved in physical register order (R0 through R7), rather than in top-of-stack (TOS) order.
With the FXSAVE instruction, however, only a single bit (1 for valid or 0 for empty) is saved for each tag.
For example, assume that the tag word is currently set as follows:R7R6R5R4R3R2R1R011xxxxxx11111111Here, 11B indicates empty stack elements and "xx" indicates valid (00B), zero (01B), or special (10B).
For this example, the FXSAVE instruction saves only the following 8 bits of information:R7R6R5R4R3R2R1R001110000Here, a 1 is saved for any valid, zero, or special tag, and a 0 is saved for any empty tag.The operation of the FXSAVE instruction differs from that of the FSAVE instruction, the as follows: - FXSAVE instruction does not check for pending unmasked floating-point exceptions.
(The FXSAVE operation in this regard is similar to the operation of the FNSAVE instruction).
 - After the FXSAVE instruction has saved the state of the x87 FPU, MMX technology, XMM, and MXCSR registers, the processor retains the contents of the registers.
Because of this behavior, the FXSAVE instruction cannot be used by an application program to pass a "clean" x87 FPU state to a procedure, since it retains the current state.
To clean the x87 FPU state, an application must  - The format of the memory image saved with the FXSAVE instruction is the same regardless of the current addressing mode (32-bit or 16-bit) and operating mode (protected, real address, or system management).
This behavior differs from the FSAVE instructions, where the memory image format is different depending on the addressing mode and operating mode.
Because of the different image formats, the memory image saved with the FXSAVE instruction cannot be restored correctly with the FRSTOR instruction, and likewise the state saved with the FSAVE instruction cannot be restored correctly with the FXRSTOR instruction.The FSAVE format for FTW can be recreated from the FTW valid bits and the stored 80-bit floating-point data (assuming the stored data was not the contents of MMX technology registers) using Table 3-45.Table 3-45.
 Recreating FSAVE Format ExponentExponentFractionJ and MFTW valid bitall 1'sall 0'sall 0'sbitsx87 FTW0000x110Special000001x1Valid00100110Special00001101Valid0100x110Special100101x1Special01011001Zero01110110Special101001x1Special101001x1Special10100110Special10101101Special011For all legal combinations above.EmptyThe J-bit is defined to be the 1-bit binary integer to the left of the decimal place in the significand.
The M-bit is defined to be the most significant bit of the fractional portion of the significand (i.e., the bit immediately to the right of the decimal place).When the M-bit is the most significant bit of the fractional portion of the significand, it must be 0 if the fraction is all 0's.IA-32e Mode OperationIn compatibility sub-mode of IA-32e mode, legacy SSE registers, XMM0 through XMM7, are saved according to the legacy FXSAVE map.
In 64-bit mode, all of the SSE registers, XMM0 through XMM15, are saved.
Additionally, there are two different layouts of the FXSAVE map in 64-bit mode, corresponding to FXSAVE64 (which requires REX.W=1) and FXSAVE (REX.W=0).
In Table 3-46.
 Layout of the 64-Bit Mode FXSAVE64 Map (Requires REX.W = 1)15141312111098765 43210FIPFOPFTWFSWFCW0ReservedMXCSR_MASKMXCSRFDP16ReservedST0/MM032ReservedST1/MM148ReservedST2/MM264ReservedST3/MM380ReservedST4/MM496ReservedST5/MM5112ReservedST6/MM6128ReservedST7/MM7144XMM0160XMM1176XMM2192XMM3208XMM4224XMM5240XMM6256XMM7272XMM8288XMM9304XMM10320XMM11336XMM12352XMM13368XMM14384XMM15400Reserved416Reserved432Reserved448Available464Available480Table 3-47.
 Layout of the 64-Bit Mode FXSAVE Map (REX.W = 0)151413 12111098765 43210FCSFIP[31:0]FOPFTWFSWFCW0ReservedReservedMXCSR_MASKMXCSRFDP[31:0]FDS16ReservedReservedST0/MM032ReservedST1/MM148ReservedST2/MM264ReservedST3/MM380ReservedST4/MM496ReservedST5/MM5112ReservedST6/MM6128ReservedST7/MM7144XMM0160XMM1176XMM2192XMM3208XMM4224XMM5240XMM6256XMM7272XMM8288XMM9304XMM10320XMM11336XMM12352XMM13368XMM14384XMM15400Reserved416Reserved432Reserved448Available464Available480

## Exceptions

- Virtual-8086 Mode Exceptions
  > Same exceptions as in real address mode.
  - #PF(fault-code) - For a page fault.
  - #AC - For unaligned memory reference.
  - #UD - If the LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  > If memory operand is not aligned on a 
  > 16-byte boundary, regardless of segment.
  - #PF(fault-code) - For a page fault.
  - #NM - If CR0.TS[bit 3] = 1.
  > If CR0.EM[bit 2] = 1.
  - #UD - If CPUID.01H:EDX.FXSR[bit 24] = 0.
  > If the LOCK prefix is used.
  - #AC - If this exception is disabled a general protec
  > tion exception (#GP) is signaled if the memory 
  > operand is not aligned on a 16-byte boundary, 
  > as described above. If the alignment check 
  > exception (#AC) is enabled (and the CPL is 3)
  > , signaling of #AC is not guaranteed and may 
  > vary with implementation, as follows. In all implementations where #AC is not signaled, a 
  > general protection exception is signaled in its 
  > place. In addition, the width of the alignment 
  > check may also vary with implementation. For instance, for a given implementation, an align-
  > ment check exception might be signaled for a 2-byte misalignment, whereas a general protec-
  > tion exception might be signaled for all 
  > other misalignments (4-, 8-, or 16-byte 
  > misalignments).
  > Implementation Note
  > The order in which the processor signals general-protection
  >  (#GP) and page-fault (#PF) exceptions when they both 
  > ®
  >  64 and IA-32 Architectures Software Devel-
  > occur on an instruction boundary is given in Table 5-2 in the Intel
- Real-Address Mode Exceptions
  - #GP - If a memory operand is not aligned on
  > a 16-byte boundary, regardless of segment.
  > If any part of the operand lies outside th
  > e effective address space from 0 to FFFFH.
  - #NM - If CR0.TS[bit 3] = 1.
  > If CR0.EM[bit 2] = 1.
  - #UD - If CPUID.01H:EDX.FXSR[bit 24] = 0.
  > If the LOCK prefix is used.
- Protected Mode Exceptions
  - #GP(0) - For an illegal memory operand effective address in the CS, DS, ES, FS or GS segments.
  > If a memory operand is not aligned on a 16-by
  > te boundary, regardless of segment. (See the 
  > description of the alignment check exception [#AC] below.)
  - #SS(0) - For an illegal address in the SS segment.
  - #PF(fault-code) - For a page fault.
  - #NM - If CR0.TS[bit 3] = 1.
  > If CR0.EM[bit 2] = 1.
  - #UD - If CPUID.01H:EDX.FXSR[bit 24] = 0.
  - #UD - If the LOCK prefix is used.
  - #AC - If this exception is disabled a general protec
  > tion exception (#GP) is signaled if the memory 
  > operand is not aligned on a 16-byte boundary, 
  > as described above. If the alignment check 
  > exception (#AC) is enabled (and the CPL is 3)
  > , signaling of #AC is not guaranteed and may 
  > vary with implementation, as follows. In all implementations where #AC is not signaled, a 
  > general protection exception is signaled in its 
  > place. In addition, the width of the alignment 
  > check may also vary with implementation. For instance, for a given implementation, an align-
  > ment check exception might be signaled for a 2-byte misalignment, whereas a general protec-
  > tion exception might be signaled for all 
  > other misalignments (4-, 8-, or 16-byte 
  > misalignments).

## Operation

```C
IF 64-Bit ModeTHENIF REX.W = 1THENDEST := Save64BitPromotedFxsave(x87 FPU, MMX, XMM15-XMM0,MXCSR);ELSEDEST := Save64BitDefaultFxsave(x87 FPU, MMX, XMM15-XMM0, MXCSR);FI;ELSEDEST := SaveLegacyFxsave(x87 FPU, MMX, XMM7-XMM0, MXCSR);FI;
```
