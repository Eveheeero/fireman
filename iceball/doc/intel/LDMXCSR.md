# LDMXCSR

Load MXCSR Register

Loads the source operand into the MXCSR control/status register.
The source operand is a 32-bit memory location.
® 64 and IA-32 Architectures Software Devel-See "MXCSR Control and Status Register" in Chapter 10, of the Inteloper's Manual, Volume 1, for a description of the MXCSR register and its contents.The LDMXCSR instruction is typically used in conjunction with the (V)STMXCSR instruction, which stores the contents of the MXCSR register in memory.The default MXCSR value at reset is 1F80H.If a (V)LDMXCSR instruction clears a SIMD floating-point exception mask bit and sets the corresponding exception flag bit, a SIMD floating-point exception will not be immediately generated.
The exception will be generated only upon the execution of the next instruction that meets both conditions below: - the instruction must operate on an XMM or YMM register operand, - the instruction causes that particular SIMD floating-point exception to be reported.
This instruction's operation is the same in non-64-bit modes and 64-bit mode.If VLDMXCSR is encoded with VEX.L= 1, an attempt to execute the instruction encoded with VEX.L= 1 will cause an #UD exception.Note: In VEX-encoded versions, VEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.

## Exceptions

- Numeric Exceptions
  > None.
- Other Exceptions
  > See Table2-22, "Type 5 Class Exce
  > ption Conditions," additionally:
  - #GP - For an attempt to set reserved bits in MXCSR.
  - #UD - If VEX.vvvv

## Operation

```C
MXCSR := m32;C/C++ Compiler Intrinsic Equivalent_mm_setcsr(unsigned int i)
```
