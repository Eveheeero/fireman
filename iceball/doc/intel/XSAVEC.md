# XSAVEC

Save Processor Extended States With Compaction

Performs a full or partial save of processor state components to the XSAVE area located at the memory address specified by the destination operand.
The implicit EDX:EAX register pair specifies a 64-bit instruction mask.
The specific state components saved correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and XCR0.® 64 and IA-32 Architectures Soft-The format of the XSAVE area is detailed in Section 13.4, "XSAVE Area," of Intelware Developer's Manual, Volume 1.
Like FXRSTOR and FXSAVE, the memory format used for x87 state depends ® 64 and IA-32 Architectures Software Developer's on a REX.W prefix; see Section 13.5.1, "x87 State" of IntelManual, Volume 1.®Section 13.10, "Operation of XSAVEC," of Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1 provides a detailed description of the operation of the XSAVEC instruction.
The following items provide a high-level outline: - Execution of XSAVEC is similar to that of XSAVE.
XSAVEC differs from XSAVE in that it uses compaction and that it may use the init optimization.
- 1XSAVEC saves state component i if and only if RFBM[i]= 1 and XINUSE[i]= 1.
(XINUSE is a bitmap by which the processor tracks the status of various state components.
See Section 13.6, "Processor Tracking of XSAVE-® 64 and IA-32 Architectures Software Developer's Manual, Volume 1.)Managed State" of Intel - XSAVEC does not modify bytes 511:464 of the legacy region of the XSAVE area (see Section 13.4.1, "Legacy ® 64 and IA-32 Architectures Software Developer's Manual, Volume 1).Region of an XSAVE Area" of Intel - 2,3XSAVEC writes the logical AND of RFBM and XINUSE to the XSTATE_BV field of the XSAVE header.
(See ® 64 and IA-32 Architectures Software Developer's Manual, Volume 1.) Section 13.4.2, "XSAVE Header" of IntelXSAVEC sets bit63 of the XCOMP_BV field and sets bits62:0 of that field to RFBM[62:0].
XSAVEC does not write to any parts of the XSAVE header other than the XSTATE_BV and XCOMP_BV fields.
- XSAVEC always uses the compacted format of the extended region of the XSAVE area (see Section 13.4.3, ®"Extended Region of an XSAVE Area" of Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1).Use of a destination operand not aligned to 64-byte boundary (in either 64-bit or 32-bit modes) results in a general-protection (#GP) exception.
In 64-bit mode, the upper 32 bits of RDX and RAX are ignored.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentXSAVEC void _xsavec( void * , unsigned __int64);XSAVEC64 void _xsavec64( void * , unsigned __int64);

## Exceptions

- Real-Address Mode Exceptions
  - #GP - If a memory operand is not aligned on
  > a 64-byte boundary, regardless of segment.
  > If any part of the operand lies outside th
  > e effective address space from 0 to FFFFH.
  - #NM - If CR0.TS[bit 3] = 1.
  - #UD - If CPUID.01H:ECX.XSAVE[bit26] = 0 or
  > CPUID.(EAX=0DH,ECX=1):EAX.XSAVEC[bit1]= 0.
  > If CR4.OSXSAVE[bit 18] = 0.
  > If the LOCK prefix is used.
- Protected Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If a memory operand is not aligned on 
  > a 64-byte boundary, regardless of segment.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #NM - If CR0.TS[bit 3] = 1.
  - #UD - If CPUID.01H:ECX.XSAVE[bit26] = 0 or
  > CPUID.(EAX=0DH,ECX=1):EAX.XSAVEC[bit1]= 0.
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
  > other misalignments (4-, 8-, or 16-byte 
  > misalignments).
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- 64-Bit Mode Exceptions
  - #GP(0) - If the memory address is in a non-canonical form.
  > If a memory operand is not aligned on 
  > a 64-byte boundary, regardless of segment.
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #NM - If CR0.TS[bit 3] = 1.
  - #UD - If CPUID.01H:ECX.XSAVE[bit26] = 0 or
  > CPUID.(EAX=0DH,ECX=1):EAX.XSAVEC[bit1]= 0.
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
RFBM := XCR0 AND EDX:EAX;/* bitwise logical AND */TO_BE_SAVED := RFBM AND XINUSE;/* bitwise logical AND */If MXCSR   1F80H AND RFBM[1]TO_BE_SAVED[1] = 1;FI;IF TO_BE_SAVED[0]= 1THEN store x87 state into legacy region of XSAVE area;FI;IF TO_BE_SAVED[1]= 1THEN store SSE state into legacy region of XSAVE area; // this step saves the XMM registers, MXCSR, and MXCSR_MASKFI;NEXT_FEATURE_OFFSET = 576;// Legacy area and XSAVE header consume 576 bytesFOR i := 2 TO 62IF RFBM[i] = 1THENIF TO_BE_SAVED[i]THEN save XSAVE state component i at offset NEXT_FEATURE_OFFSET from base of XSAVE area;FI;NEXT_FEATURE_OFFSET = NEXT_FEATURE_OFFSET + n (n enumerated by CPUID(EAX=0DH,ECX=i):EAX);FI;ENDFOR;XSTATE_BV field in XSAVE header := TO_BE_SAVED;XCOMP_BV field in XSAVE header := RFBM OR 80000000_00000000H;
```
