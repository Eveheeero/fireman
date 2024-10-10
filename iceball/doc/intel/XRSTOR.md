# XRSTOR

Restore Processor Extended States

Performs a full or partial restore of processor state components from the XSAVE area located at the memory address specified by the source operand.
The implicit EDX:EAX register pair specifies a 64-bit instruction mask.
The specific state components restored correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and XCR0.® 64 and IA-32 Architectures Soft-The format of the XSAVE area is detailed in Section 13.4, "XSAVE Area," of Intelware Developer's Manual, Volume 1.
Like FXRSTOR and FXSAVE, the memory format used for x87 state depends ® 64 and IA-32 Architectures Software Developer's on a REX.W prefix; see Section 13.5.1, "x87 State" of IntelManual, Volume 1.®Section 13.8, "Operation of XRSTOR," of Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1 provides a detailed description of the operation of the XRSTOR instruction.
The following items provide a high-level outline: - Execution of XRSTOR may take one of two forms: standard and compacted.
Bit63 of the XCOMP_BV field in the XSAVE header determines which form is used: value 0 specifies the standard form, while value 1 specifies the compacted form.
- 1If RFBM[i]= 0, XRSTOR does not update state component i.
- If RFBM[i]= 1 and biti is clear in the XSTATE_BV field in the XSAVE header, XRSTOR initializes state component i.
- If RFBM[i]= 1 and XSTATE_BV[i]= 1, XRSTOR loads state componenti from the XSAVE area.
- The standard form of XRSTOR treats MXCSR (which is part of state component 1 - SSE) differently from the XMM registers.
If either form attempts to load MXCSR with an illegal value, a general-protection exception (#GP) occurs.
- XRSTOR loads the internal value XRSTOR_INFO, which may be used to optimize a subsequent execution of XSAVEOPT or XSAVES.
- Immediately following an execution of XRSTOR, the processor tracks as in-use (not in initial configuration) any state componenti for which RFBM[i]= 1 and XSTATE_BV[i]= 1; it tracks as modified any state component ifor which RFBM[i]= 0.Use of a source operand not aligned to 64-byte boundary (for 64-bit and 32-bit modes) results in a general-protec-tion (#GP) exception.
In 64-bit mode, the upper 32 bits of RDX and RAX are ignored.® 64 and IA-32 Architectures Software See Section 13.6, "Processor Tracking of XSAVE-Managed State," of IntelDeveloper's Manual, Volume 1 for discussion of the bitmaps XINUSE and XMODIFIED and of the quantity XRSTOR_INFO.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentXRSTOR void _xrstor( void * , unsigned __int64);XRSTOR void _xrstor64( void * , unsigned __int64);

## Exceptions

- Real-Address Mode Exceptions
  - #GP - If a memory operand is not aligned on
  > a 64-byte boundary, regardless of segment.
  > If any part of the operand lies outside 
  > the effective address space from 0 to FFFFH.
  > If bit63 of the XCOMP_BV field of the XSAVE header is 1 and 
  > CPUID.(EAX=0DH,ECX=1):E
  > AX.XSAVEC[bit 1]= 0.
  > If the standard form is executed and a bit in
  >  XCR0 is 0 and the corresponding bit in the 
  > XSTATE_BV field of the XSAVE header is 1.
  > If the standard form is executed and bytes 
  > 23:8 of the XSAVE header are not all zero.
  > If the compacted form is executed and a bit in
  >  the XCOMP_BV field in the XSAVE header is 0 
  > and the corresponding bit in the XSTATE_BV field is 1.
  > If the compacted form is executed and bytes 
  > 63:16 of the XSAVE header are not all zero.
  > If attempting to write any reserved bits of the MXCSR register with 1.
  - #NM - If CR0.TS[bit 3] = 1.
  - #UD - If CPUID.01H:ECX.XSAVE[bit 26] = 0.
  > If CR4.OSXSAVE[bit 18] = 0.
  > If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- 64-Bit Mode Exceptions
  - #GP(0) - If a memory address is in a non-canonical form.
  > If a memory operand is not aligned on 
  > a 64-byte boundary, regardless of segment.
  > If bit63 of the XCOMP_BV field of the XSAVE header is 1 and 
  > CPUID.(EAX=0DH,ECX=1):EAX.XSAVEC[bit 1]= 0.
  > If the standard form is executed and a bit in
  >  XCR0 is 0 and the corresponding bit in the 
  > XSTATE_BV field of the XSAVE header is 1.
  > If the standard form is executed and bytes 
  > 23:8 of the XSAVE header are not all zero.
  > If the compacted form is executed and a bit in
  >  XCR0 is 0 and the corresponding bit in the 
  > XCOMP_BV field of the XSAVE header is 1.
  > If the compacted form is executed and a bit in
  >  the XCOMP_BV field in the XSAVE header is 0 
  > and the corresponding bit in the XSTATE_BV field is 1.
  > If the compacted form is executed and bytes 
  > 63:16 of the XSAVE header are not all zero.
  > If attempting to write any reserved bits of the MXCSR register with 1.
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
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If a memory operand is not aligned on 
  > a 64-byte boundary, regardless of segment.
  > If bit63 of the XCOMP_BV field of the XSAVE header is 1 and 
  > CPUID.(EAX=0DH,ECX=1):E
  > AX.XSAVEC[bit 1]= 0.
  > If the standard form is executed and a bit in
  >  XCR0 is 0 and the corresponding bit in the 
  > XSTATE_BV field of the XSAVE header is 1.
  > If the standard form is executed and bytes 
  > 23:8 of the XSAVE header are not all zero.
  > If the compacted form is executed and a bit in XCR0 is 0 and the corresponding bit in the 
  > XCOMP_BV field of the XSAVE header is 1.
  > If the compacted form is executed and a bit in
  >  the XCOMP_BV field in the XSAVE header is 0 
  > and the corresponding bit in the XSTATE_BV field is 1.
  > If the compacted form is executed and bytes 
  > 63:16 of the XSAVE header are not all zero.
  > If attempting to write any reserved bits of the MXCSR register with 1.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
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
  > other misalignments (4-, 8-, or 16-byte 
  > misalignments).

## Operation

```C
RFBM := XCR0 AND EDX:EAX;/* bitwise logical AND */COMPMASK := XCOMP_BV field from XSAVE header;RSTORMASK := XSTATE_BV field from XSAVE header;IF COMPMASK[63] = 0THEN/* Standard form of XRSTOR */TO_BE_RESTORED := RFBM AND RSTORMASK;TO_BE_INITIALIZED := RFBM AND NOT RSTORMASK;IF TO_BE_RESTORED[0] = 1THENXINUSE[0] := 1;load x87 state from legacy region of XSAVE area;ELSIF TO_BE_INITIALIZED[0] = 1THENXINUSE[0] := 0;initialize x87 state;FI;IF RFBM[1] = 1 OR RFBM[2]= 1THEN load MXCSR from legacy region of XSAVE area;FI;IF TO_BE_RESTORED[1] = 1THENXINUSE[1] := 1;load XMM registers from legacy region of XSAVE area; // this step does not load MXCSRELSIF TO_BE_INITIALIZED[1] = 1THENXINUSE[1] := 0;set all XMM registers to 0; // this step does not initialize MXCSRFI;FOR i := 2 TO 62 IF TO_BE_RESTORED[i] = 1THENXINUSE[i] := 1;load XSAVE state component i at offset n from base of XSAVE area;// n enumerated by CPUID(EAX=0DH,ECX=i):EBX)ELSIF TO_BE_INITIALIZED[i] = 1THENXINUSE[i] := 0;initialize XSAVE state component i;FI;ENDFOR;ELSE/* Compacted form of XRSTOR */IF CPUID.(EAX=0DH,ECX=1):EAX.XSAVEC[bit1]= 0THEN/* compacted form not supported */FORMAT = COMPMASK AND 7FFFFFFF_FFFFFFFFH;RESTORE_FEATURES = FORMAT AND RFBM;TO_BE_RESTORED := RESTORE_FEATURES AND RSTORMASK;FORCE_INIT := RFBM AND NOT FORMAT;TO_BE_INITIALIZED = (RFBM AND NOT RSTORMASK) OR FORCE_INIT;IF TO_BE_RESTORED[0] = 1THENXINUSE[0] := 1;load x87 state from legacy region of XSAVE area;ELSIF TO_BE_INITIALIZED[0] = 1THENXINUSE[0] := 0;initialize x87 state;FI;IF TO_BE_RESTORED[1] = 1THENXINUSE[1] := 1;load SSE state from legacy region of XSAVE area; // this step loads the XMM registers and MXCSRELSIF TO_BE_INITIALIZED[1] = 1THENset all XMM registers to 0;XINUSE[1] := 0;MXCSR := 1F80H;FI;NEXT_FEATURE_OFFSET = 576;// Legacy area and XSAVE header consume 576 bytesFOR i := 2 TO 62 IF FORMAT[i] = 1THENIF TO_BE_RESTORED[i] = 1THENXINUSE[i] := 1;load XSAVE state component i at offset NEXT_FEATURE_OFFSET from base of XSAVE area;FI;NEXT_FEATURE_OFFSET = NEXT_FEATURE_OFFSET + n (n enumerated by CPUID(EAX=0DH,ECX=i):EAX);FI;IF TO_BE_INITIALIZED[i] = 1THENXINUSE[i] := 0;initialize XSAVE state component i;FI;ENDFOR;FI;XMODIFIED := NOT RFBM;IF in VMX non-root operationTHEN VMXNR := 1;ELSE VMXNR := 0; ¢ ²XRSTOR_INFO := CPL,VMXNR,LAXA,COMPMASK;
```
