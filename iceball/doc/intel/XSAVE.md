# XSAVE

Save Processor Extended States

Performs a full or partial save of processor state components to the XSAVE area located at the memory address specified by the destination operand.
The implicit EDX:EAX register pair specifies a 64-bit instruction mask.
The specific state components saved correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and XCR0.® 64 and IA-32 Architectures Soft-The format of the XSAVE area is detailed in Section 13.4, "XSAVE Area," of Intelware Developer's Manual, Volume 1.
Like FXRSTOR and FXSAVE, the memory format used for x87 state depends ® 64 and IA-32 Architectures Software Developer's on a REX.W prefix; see Section 13.5.1, "x87 State" of IntelManual, Volume 1.®Section 13.7, "Operation of XSAVE," of Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1 provides a detailed description of the operation of the XSAVE instruction.
The following items provide a high-level outline: - 1XSAVE saves state component i if and only if RFBM[i]= 1.
- XSAVE does not modify bytes 511:464 of the legacy region of the XSAVE area (see Section 13.4.1, "Legacy ® 64 and IA-32 Architectures Software Developer's Manual, Volume 1).Region of an XSAVE Area" of Intel - ®XSAVE reads the XSTATE_BV field of the XSAVE header (see Section 13.4.2, "XSAVE Header" of Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1) and writes a modified value back to memory as follows.
If RFBM[i]= 1, XSAVE writes XSTATE_BV[i] with the value of XINUSE[i].
(XINUSE is a bitmap by which the processor tracks the status of various state components.
See Section 13.6, "Processor Tracking of XSAVE-® 64 and IA-32 Architectures Software Developer's Manual, Volume 1.) If RFBM[i]= 0, Managed State" of IntelXSAVE writes XSTATE_BV[i] with the value that it read from memory (it does not modify the bit).
XSAVE does not write to any part of the XSAVE header other than the XSTATE_BV field.
- XSAVE always uses the standard format of the extended region of the XSAVE area (see Section 13.4.3, ®"Extended Region of an XSAVE Area" of Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1).Use of a destination operand not aligned to 64-byte boundary (in either 64-bit or 32-bit modes) results in a general-protection (#GP) exception.
In 64-bit mode, the upper 32 bits of RDX and RAX are ignored.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentXSAVE void _xsave( void * , unsigned __int64);XSAVE void _xsave64( void * , unsigned __int64);

## Exceptions

- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  - #GP - If a memory operand is not aligned on
  > a 64-byte boundary, regardless of segment.
  > If any part of the operand lies outside 
  > the effective address space from 0 to FFFFH.
  - #NM - If CR0.TS[bit 3] = 1.
  - #UD - If CPUID.01H:ECX.XSAVE[bit 26] = 0.
  > If CR4.OSXSAVE[bit 18] = 0.
  > If the LOCK prefix is used.
- Protected Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If a memory operand is not aligned on 
  > a 64-byte boundary, regardless of segment.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #NM - If CR0.TS[bit 3] = 1.
  - #UD - If CPUID.01H:ECX.XSAVE[bit 26] = 0.
  > If CR4.OSXSAVE[bit 18] = 0.
  > If the LOCK prefix is used.
  - #AC - If this exception is disabled a general protec
  > tion exception (#GP) is signaled if the memory 
  > operand is not aligned on a 64-byte boundary, 
  > as described above. If the alignment check 
  > exception (#AC) is enabled (and the CPL is 3)
  > , signaling of #AC is not guaranteed and may 
  > vary with implementation, as follows. In all implementations where #AC is not signaled, a 
  > general protection exception is signaled in its 
  > place. In addition, the width of the alignment 
  > check may also vary with implementation. For instance, for a given implementation, an align-
  > ment check exception might be signaled for a 2-byte misalignment, whereas a general protec-
  > tion exception might be signaled for all 
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- 64-Bit Mode Exceptions
  - #GP(0) - If the memory address is in a non-canonical form.
  > If a memory operand is not aligned on 
  > a 64-byte boundary, regardless of segment.
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #NM - If CR0.TS[bit 3] = 1.
  - #UD - If CPUID.01H:ECX.XSAVE[bit 26] = 0.
  > If CR4.OSXSAVE[bit 18] = 0.
  > If the LOCK prefix is used.
  - #AC - If this exception is disabled a general protec
  > tion exception (#GP) is signaled if the memory 
  > operand is not aligned on a 64-byte boundary, 
  > as described above. If the alignment check 
  > exception (#AC) is enabled (and the CPL is 3)
  > , signaling of #AC is not guaranteed and may 
  > vary with implementation, as follows. In all implementations where #AC is not signaled, a 
  > general protection exception is signaled in its 
  > place. In addition, the width of the alignment 
  > check may also vary with implementation. For instance, for a given implementation, an align-
  > ment check exception might be signaled for a 2-byte misalignment, whereas a general protec-
  > tion exception might be signaled for all 

## Operation

```C
RFBM := XCR0 AND EDX:EAX;/* bitwise logical AND */OLD_BV := XSTATE_BV field from XSAVE header;IF RFBM[0]= 1THEN store x87 state into legacy region of XSAVE area;FI;IF RFBM[1]= 1THEN store XMM registers into legacy region of XSAVE area; // this step does not save MXCSR or MXCSR_MASKFI;IF RFBM[1]= 1 OR RFBM[2]= 1THEN store MXCSR and MXCSR_MASK into legacy region of XSAVE area;FI;FOR i := 2 TO 62IF RFBM[i] = 1THEN save XSAVE state component i at offset n from base of XSAVE area (n enumerated by CPUID(EAX=0DH,ECX=i):EBX);FI;ENDFOR;XSTATE_BV field in XSAVE header := (OLD_BV AND NOT RFBM) OR (XINUSE AND RFBM);
```
