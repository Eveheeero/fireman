# XGETBV

Get Value of Extended Control Register

Reads the contents of the extended control register (XCR) specified in the ECX register into registers EDX:EAX.
(On processors that support the Intel64 architecture, the high-order 32 bits of RCX are ignored.) The EDX register is loaded with the high-order 32 bits of the XCR and the EAX register is loaded with the low-order 32 bits.
(On proces-sors that support the Intel64 architecture, the high-order 32 bits of each of RAX and RDX are cleared.) If fewer than 64 bits are implemented in the XCR being read, the values returned to EDX:EAX in unimplemented bit loca-tions are undefined.XCR0 is supported on any processor that supports the XGETBV instruction.
If CPUID.(EAX=0DH,ECX=1):EAX.XG1[bit2]= 1, executing XGETBV with ECX= 1 returns in EDX:EAX the logical-AND of XCR0 and the current value of the XINUSE state-component bitmap.
This allows software to discover the state of the init optimization used by XSAVEOPT and XSAVES.
See Chapter 13, "Managing State Using the XSAVE ® 64 and IA-32 Architectures Software Developer's Manual, Volume 1.Feature Set" in IntelUse of any other value for ECX results in a general-protection (#GP) exception.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentXGETBV unsigned __int64 _xgetbv( unsigned int);

## Exceptions

- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #GP(0) - If an invalid XCR is specified in ECX (includes ECX= 1 if
  > CPUID.(EAX=0DH,ECX=1):E
  > AX.XG1[bit 2] = 0).
  - #UD - If CPUID.01H:ECX.XSAVE[bit 26] = 0.
  > If CR4.OSXSAVE[bit 18] = 0.
  > If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #GP(0) - If an invalid XCR is specified in ECX (includes ECX= 1 if
  > CPUID.(EAX=0DH,ECX=1):E
  > AX.XG1[bit 2] = 0).
  - #UD - If CPUID.01H:ECX.XSAVE[bit 26] = 0.
  > If CR4.OSXSAVE[bit 18] = 0.
  > If the LOCK prefix is used.

## Operation

```C
EDX:EAX := XCR[ECX];
```
