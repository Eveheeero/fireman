# FXTRACT

Extract Exponent and Significand

Separates the source value in the ST(0) register into its exponent and significand, stores the exponent in ST(0), and pushes the significand onto the register stack.
Following this operation, the new top-of-stack register ST(0) contains the value of the original significand expressed as a floating-point value.
The sign and significand of this value are the same as those found in the source operand, and the exponent is 3FFFH (biased value for a true expo-nent of zero).
The ST(1) register contains the value of the original operand's true (unbiased) exponent expressed as a floating-point value.
(The operation performed by this instruction is a superset of the IEEE-recommended logb(x) function.)This instruction and the F2XM1 instruction are useful for performing power and range scaling operations.
The FXTRACT instruction is also useful for converting numbers in double extended-precision floating-point format to decimal representations (e.g., for printing or displaying).If the floating-point zero-divide exception (#Z) is masked and the source operand is zero, an exponent value of - is stored in register ST(1) and 0 with the sign of the source operand is stored in register ST(0).This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Floating-Point Exceptions
  - #IS - Stack underflow or overflow occurred.
  - #IA - Source operand is an SNaN value or unsupported format.
  - #Z - ST(0) operand is
  > ±
  > 0.
  - #D - Source operand is a denormal value.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
TEMP := Significand(ST(0));ST(0) := Exponent(ST(0));- 1;TOP := TOP ST(0) := TEMP;FPU Flags AffectedC1Set to 0 if stack underflow occurred; set to 1 if stack overflow occurred.C0, C2, C3 Undefined.
```
