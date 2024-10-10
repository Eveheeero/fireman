# RDSSPD/RDSSPQ

Read Shadow Stack Pointer

Copies the current shadow stack pointer (SSP) register to the register destination.
This opcode is a NOP when CET shadow stacks are not enabled and on processors that do not support CET.

## Flags affected

- None.C/C++ Compiler Intrinsic EquivalentRDSSPD__int32 _rdsspd_i32(void);RDSSPQ__int64 _rdsspq_i64(void);

## Exceptions

- Compatibility Mode Exceptions
  > None.
- Virtual-8086 Mode Exceptions
  > None.
- Real-Address Mode Exceptions
  > None.

## Operation

```C
IF CPL = 3IF CR4.CET & IA32_U_CET.SH_STK_ENIF (operand size is 64 bit)THENDest := SSP;ELSEDest := SSP[31:0];FI;FI;ELSEIF CR4.CET & IA32_S_CET.SH_STK_ENIF (operand size is 64 bit)THENDest := SSP;ELSEDest := SSP[31:0];FI;FI;FI;
```
