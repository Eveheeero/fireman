# XSETBV

Set Extended Control Register

Writes the contents of registers EDX:EAX into the 64-bit extended control register (XCR) specified in the ECX register.
(On processors that support the Intel64 architecture, the high-order 32 bits of RCX are ignored.) The contents of the EDX register are copied to high-order 32 bits of the selected XCR and the contents of the EAX register are copied to low-order 32 bits of the XCR.
(On processors that support the Intel64 architecture, the high-order 32 bits of each of RAX and RDX are ignored.) Undefined or reserved bits in an XCR should be set to values previously read.This instruction must be executed at privilege level 0 or in real-address mode; otherwise, a general protection exception #GP(0) is generated.
Specifying a reserved or unimplemented XCR in ECX will also cause a general protection exception.
The processor will also generate a general protection exception if software attempts to write to reserved bits in an XCR.Currently, only XCR0 is supported.
Thus, all other values of ECX are reserved and will cause a #GP(0).
Note that bit0 of XCR0 (corresponding to x87 state) must be set to 1; the instruction will cause a #GP(0) if an attempt is made to clear this bit.
In addition, the instruction causes a #GP(0) if an attempt is made to set XCR0[2] (AVX state) while clearing XCR0[1] (SSE state); it is necessary to set both bits to use AVX instructions; Section 13.3, "Enabling ® 64 and IA-32 Architectures Software Developer's the XSAVE Feature Set and XSAVE-Enabled Features," of IntelManual, Volume 1.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentXSETBV void _xsetbv( unsigned int, unsigned __int64);

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If the current privilege level is not 0.
  > If an invalid XCR is specified in ECX.
  > If the value in EDX:EAX sets bits that 
  > are reserved in the XCR specified by ECX.
  > If an attempt is made to clear bit 0 of XCR0.
  > If an attempt is made to set XCR0[2:1] to 10b.
  - #UD - If CPUID.01H:ECX.XSAVE[bit 26] = 0.
- Virtual-8086 Mode Exceptions
  - #GP(0) - The XSETBV instruction is not recognized in virtual-8086 mode.
- Real-Address Mode Exceptions
  - #GP - If an invalid XCR is specified in ECX.
  > If the value in EDX:EAX sets bits that 
  > are reserved in the XCR specified by ECX.
  > If an attempt is made to clear bit 0 of XCR0.
  > If an attempt is made to set XCR0[2:1] to 10b.
  - #UD - If CPUID.01H:ECX.XSAVE[bit 26] = 0.
  > If CR4.OSXSAVE[bit 18] = 0.
  > If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
XCR[ECX] := EDX:EAX;
```
