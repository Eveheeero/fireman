# XSAVES

Save Processor Extended States Supervisor

Performs a full or partial save of processor state components to the XSAVE area located at the memory address specified by the destination operand.
The implicit EDX:EAX register pair specifies a 64-bit instruction mask.
The specific state components saved correspond to the bits set in the requested-feature bitmap (RFBM), the logical-AND of EDX:EAX and the logical-OR of XCR0 with the IA32_XSS MSR.
XSAVES may be executed only if CPL = 0.® 64 and IA-32 Architectures The format of the XSAVE area is detailed in Section 13.4, "XSAVE Area," of the IntelSoftware Developer's Manual, Volume 1.
Like FXRSTOR and FXSAVE, the memory format used for x87 state ® 64 and IA-32 Architectures Software depends on a REX.W prefix; see Section 13.5.1, "x87 State," of the IntelDeveloper's Manual, Volume 1.®Section 13.11, "Operation of XSAVES," of the Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1 provides a detailed description of the operation of the XSAVES instruction.
The following items provide a high-level outline: - Execution of XSAVES is similar to that of XSAVEC.
XSAVES differs from XSAVEC in that it can save state components corresponding to bits set in the IA32_XSS MSR and that it may use the modified optimization.
- 1XSAVES saves state component i only if RFBM[i]= 1 and XINUSE[i]= 1.
(XINUSE is a bitmap by which the processor tracks the status of various state components.
See Section 13.6, "Processor Tracking of XSAVE-® 64 and IA-32 Architectures Software Developer's Manual, Volume 1.) Even if Managed State," of the Intelboth bits are 1, XSAVES may optimize and not save state component i if (1)state component i has not been modified since the last execution of XRSTOR or XRSTORS; and (2)this execution of XSAVES correspond to that last execution of XRSTOR or XRSTORS as determined by XRSTOR_INFO (see the Operation section below).
- XSAVES does not modify bytes 511:464 of the legacy region of the XSAVE area (see Section 13.4.1, "Legacy ® 64 and IA-32 Architectures Software Developer's Manual, Volume 1).Region of an XSAVE Area," of the Intel - 2XSAVES writes the logical AND of RFBM and XINUSE to the XSTATE_BV field of the XSAVE header.
(See Section ®13.4.2, "XSAVE Header," of the Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1.) XSAVES sets bit63 of the XCOMP_BV field and sets bits62:0 of that field to RFBM[62:0].
XSAVES does not write to any parts of the XSAVE header other than the XSTATE_BV and XCOMP_BV fields.
- XSAVES always uses the compacted format of the extended region of the XSAVE area (see Section 13.4.3, ®"Extended Region of an XSAVE Area," of the Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1).Use of a destination operand not aligned to 64-byte boundary (in either 64-bit or 32-bit modes) results in a general-protection (#GP) exception.
In 64-bit mode, the upper 32 bits of RDX and RAX are ignored.

## Exceptions

- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #GP(0) - If CPL> 0.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If a memory operand is not aligned on 
  > a 64-byte boundary, regardless of segment.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #NM - If CR0.TS[bit 3] = 1.
  - #UD - If CPUID.01H:ECX.XSAVE[bit26] = 0 or CPUID.(EAX=0DH,ECX=1):EAX.XSS[bit3]= 0.
  > If CR4.OSXSAVE[bit 18] = 0.
  > If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- 64-Bit Mode Exceptions
  - #GP(0) - If CPL> 0.
  > If the memory address is in a non-canonical form.
  > If a memory operand is not aligned on 
  > a 64-byte boundary, regardless of segment.
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #NM - If CR0.TS[bit 3] = 1.
  - #UD - If CPUID.01H:ECX.XSAVE[bit26] = 0 or CPUID.(EAX=0DH,ECX=1):EAX.XSS[bit3]= 0.
- Real-Address Mode Exceptions
  - #GP - If a memory operand is not aligned on
  > a 64-byte boundary, regardless of segment.
  > If any part of the operand lies outside th
  > e effective address space from 0 to FFFFH.
  - #NM - If CR0.TS[bit 3] = 1.
  - #UD - If CPUID.01H:ECX.XSAVE[bit26] = 0 or CPUID.(EAX=0DH,ECX=1):EAX.XSS[bit3]= 0.
  > If CR4.OSXSAVE[bit 18] = 0.
  > If the LOCK prefix is used.

## Operation

```C
RFBM := (XCR0 OR IA32_XSS) AND EDX:EAX;/* bitwise logical OR and AND */IF in VMX non-root operationTHEN VMXNR := 1;ELSE VMXNR := 0;FI;LAXA := linear address of XSAVE area;COMPMASK := RFBM OR 80000000_00000000H;TO_BE_SAVED := RFBM AND XINUSE; ¢ ²CPL,VMXNR,LAXA,COMPMASKIF XRSTOR_INFO= THEN TO_BE_SAVED := TO_BE_SAVED AND XMODIFIED;FI;IF MXCSR   1F80H AND RFBM[1]THEN TO_BE_SAVED[1] = 1;FI;IF TO_BE_SAVED[0]= 1THEN store x87 state into legacy region of XSAVE area;FI;IF TO_BE_SAVED[1]= 1THEN store SSE state into legacy region of XSAVE area; // this step saves the XMM registers, MXCSR, and MXCSR_MASKFI;NEXT_FEATURE_OFFSET = 576;// Legacy area and XSAVE header consume 576 bytesFOR i := 2 TO 62IF RFBM[i] = 1THENIF TO_BE_SAVED[i]THENsave XSAVE state component i at offset NEXT_FEATURE_OFFSET from base of XSAVE area;IF i = 8// state component 8 is for PT stateTHEN IA32_RTIT_CTL.TraceEn[bit0] := 0;FI;FI;NEXT_FEATURE_OFFSET = NEXT_FEATURE_OFFSET + n (n enumerated by CPUID(EAX=0DH,ECX=i):EAX);FI;ENDFOR;NEW_HEADER := RFBM AND XINUSE;IF MXCSR   1F80H AND RFBM[1]THEN NEW_HEADER[1] = 1;FI;XSTATE_BV field in XSAVE header := NEW_HEADER;XCOMP_BV field in XSAVE header := COMPMASK;Intel C/C++ Compiler Intrinsic EquivalentXSAVES void _xsaves( void * , unsigned __int64);XSAVES64 void _xsaves64( void * , unsigned __int64);
```
