# VUCOMISH

Unordered Compare Scalar FP16 Values and Set EFLAGS

This instruction compares the FP16 values in the low word of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal).
The OF, SF and AF flags in the EFLAGS register are set to 0.
The unordered result is returned if either source operand is a NaN (QNaN or SNaN).Operand 1 is an XMM register; operand 2 can be an XMM register or a 16-bit memory location.The VUCOMISH instruction differs from the VCOMISH instruction in that it signals a SIMD floating-point invalid oper-ation exception (#I) only if a source operand is an SNaN.
The COMISS instruction signals an invalid numeric excep-tion when a source operand is either a QNaN or SNaN.The EFLAGS register is not updated if an unmasked SIMD floating-point exception is generated.
EVEX.vvvv are reserved and must be 1111b, otherwise instructions will #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Denormal.

## Operation

```C
VUCOMISHRESULT := UnorderedCompare(SRC1.fp16[0],SRC2.fp16[0])if RESULT is UNORDERED:ZF, PF, CF := 1, 1, 1else if RESULT is GREATER_THAN:ZF, PF, CF := 0, 0, 0else if RESULT is LESS_THAN:ZF, PF, CF := 0, 0, 1else: // RESULT is EQUALSZF, PF, CF := 1, 0, 0OF, AF, SF := 0, 0, 0 Intel C/C++ Compiler Intrinsic EquivalentVUCOMISH int _mm_ucomieq_sh (__m128h a, __m128h b);VUCOMISH int _mm_ucomige_sh (__m128h a, __m128h b);VUCOMISH int _mm_ucomigt_sh (__m128h a, __m128h b);VUCOMISH int _mm_ucomile_sh (__m128h a, __m128h b);VUCOMISH int _mm_ucomilt_sh (__m128h a, __m128h b);VUCOMISH int _mm_ucomineq_sh (__m128h a, __m128h b);
```
