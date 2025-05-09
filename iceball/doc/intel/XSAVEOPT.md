# XSAVEOPT

Save Processor Extended States Optimized

Performs a full or partial save of processor state components to the XSAVE area located at the memory address specified by the destination operand.
The implicit EDX:EAX register pair specifies a 64-bit instruction mask.
The specific state components saved correspond to the bits set in the requested-feature bitmap (RFBM), which is the logical-AND of EDX:EAX and XCR0.® 64 and IA-32 Architectures Soft-The format of the XSAVE area is detailed in Section 13.4, "XSAVE Area," of Intelware Developer's Manual, Volume 1.
Like FXRSTOR and FXSAVE, the memory format used for x87 state depends ® 64 and IA-32 Architectures Software Developer's on a REX.W prefix; see Section 13.5.1, "x87 State" of IntelManual, Volume 1.®Section 13.9, "Operation of XSAVEOPT," of Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1 provides a detailed description of the operation of the XSAVEOPT instruction.
The following items provide a high-level outline: - Execution of XSAVEOPT is similar to that of XSAVE.
XSAVEOPT differs from XSAVE in that it may use the init and modified optimizations.
The performance of XSAVEOPT will be equal to or better than that of XSAVE.
- 1XSAVEOPT saves state component i only if RFBM[i]= 1 and XINUSE[i]= 1.
(XINUSE is a bitmap by which the processor tracks the status of various state components.
See Section 13.6, "Processor Tracking of XSAVE-® 64 and IA-32 Architectures Software Developer's Manual, Volume 1.) Even if both Managed State," of the Intelbits are 1, XSAVEOPT may optimize and not save state component i if (1)state component i has not been modified since the last execution of XRSTOR or XRSTORS; and (2)this execution of XSAVES corresponds to that last execution of XRSTOR or XRSTORS as determined by the internal value XRSTOR_INFO (see the Operation section below).
- XSAVEOPT does not modify bytes 511:464 of the legacy region of the XSAVE area (see Section 13.4.1, "Legacy ® 64 and IA-32 Architectures Software Developer's Manual, Volume 1).Region of an XSAVE Area" of Intel - ®XSAVEOPT reads the XSTATE_BV field of the XSAVE header (see Section 13.4.2, "XSAVE Header," of the Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1) and writes a modified value back to memory as follows.
If RFBM[i]= 1, XSAVEOPT writes XSTATE_BV[i] with the value of XINUSE[i].
If RFBM[i]= 0, XSAVEOPT writes XSTATE_BV[i] with the value that it read from memory (it does not modify the bit).
XSAVEOPT does not write to any part of the XSAVE header other than the XSTATE_BV field.
- XSAVEOPT always uses the standard format of the extended region of the XSAVE area (see Section 13.4.3, ® 64 and IA-32 Architectures Software Developer's Manual, "Extended Region of an XSAVE Area" of IntelVolume 1).Use of a destination operand not aligned to 64-byte boundary (in either 64-bit or 32-bit modes) will result in a general-protection (#GP) exception.
In 64-bit mode, the upper 32 bits of RDX and RAX are ignored.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentXSAVEOPT void _xsaveopt( void * , unsigned __int64);XSAVEOPT void _xsaveopt64( void * , unsigned __int64);

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If a memory operand is not aligned on 
  > a 64-byte boundary, regardless of segment.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #UD - If CPUID.01H:ECX.XSAVE[bit26] = 0 or
  > CPUID.(EAX=0DH,ECX=1):EAX.XSAVEOPT[bit0]= 
  > 0.
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
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  - #GP - If a memory operand is not aligned on
  > a 64-byte boundary, regardless of segment.
  > If any part of the operand lies outside 
  > the effective address space from 0 to FFFFH.
  - #NM - If CR0.TS[bit 3] = 1.
  - #UD - If CPUID.01H:ECX.XSAVE[bit26] = 0 or
  > CPUID.(EAX=0DH,ECX=1):EAX.XSAVEOPT[bit0]= 
  > 0.
  > If CR4.OSXSAVE[bit 18] = 0.
  > If the LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  > If a memory operand is not aligned on 
  > a 64-byte boundary, regardless of segment.
  - #PF(fault-code) - If a page fault occurs.
  - #NM - If CR0.TS[bit 3] = 1.
  - #UD - If CPUID.01H:ECX.XSAVE[bit26] = 0 or
  > CPUID.(EAX=0DH,ECX=1):EAX.XSAVEOPT[bit0]= 
  > 0.
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
RFBM := XCR0 AND EDX:EAX;/* bitwise logical AND */OLD_BV := XSTATE_BV field from XSAVE header;TO_BE_SAVED := RFBM AND XINUSE;IF in VMX non-root operationTHEN VMXNR := 1;ELSE VMXNR := 0;FI;LAXA := linear address of XSAVE area; ¢ ²CPL,VMXNR,LAXA,00000000_00000000HIF XRSTOR_INFO= THEN TO_BE_SAVED := TO_BE_SAVED AND XMODIFIED;FI;IF TO_BE_SAVED[0]= 1THEN store x87 state into legacy region of XSAVE area;FI;IF TO_BE_SAVED[1]THEN store XMM registers into legacy region of XSAVE area; // this step does not save MXCSR or MXCSR_MASKFI;IF RFBM[1]= 1 or RFBM[2]= 1THEN store MXCSR and MXCSR_MASK into legacy region of XSAVE area;FI;FOR i := 2 TO 62IF TO_BE_SAVED[i] = 1THEN save XSAVE state component i at offset n from base of XSAVE area (n enumerated by CPUID(EAX=0DH,ECX=i):EBX);FI;ENDFOR;XSTATE_BV field in XSAVE header := (OLD_BV AND NOT RFBM) OR (XINUSE AND RFBM);
```
