# VCOMISH

Compare Scalar Ordered FP16 Values and Set EFLAGS

This instruction compares the FP16 values in the low word of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal).
The OF, SF and AF flags in the EFLAGS register are set to 0.
The unordered result is returned if either source operand is a NaN (QNaN or SNaN).Operand 1 is an XMM register; operand 2 can be an XMM register or a 16-bit memory location.The VCOMISH instruction differs from the VUCOMISH instruction in that it signals a SIMD floating-point invalid oper-ation exception (#I) when a source operand is either a QNaN or SNaN.
The VUCOMISH instruction signals an invalid numeric exception only if a source operand is an SNaN.The EFLAGS register is not updated if an unmasked SIMD floating-point exception is generated.
EVEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.

## Operation

```C
VCOMISH SRC1, SRC2 RESULT := OrderedCompare(SRC1.fp16[0],SRC2.fp16[0])IF RESULT is UNORDERED:ZF, PF, CF := 1, 1, 1ELSE IF RESULT is GREATER_THAN:ZF, PF, CF := 0, 0, 0ELSE IF RESULT is LESS_THAN:ZF, PF, CF := 0, 0, 1ELSE: // RESULT is EQUALSZF, PF, CF := 1, 0, 0OF, AF, SF := 0, 0, 0 Intel C/C++ Compiler Intrinsic EquivalentVCOMISH int _mm_comi_round_sh (__m128h a, __m128h b, const int imm8, const int sae);VCOMISH int _mm_comi_sh (__m128h a, __m128h b, const int imm8);VCOMISH int _mm_comieq_sh (__m128h a, __m128h b);VCOMISH int _mm_comige_sh (__m128h a, __m128h b);VCOMISH int _mm_comigt_sh (__m128h a, __m128h b);VCOMISH int _mm_comile_sh (__m128h a, __m128h b);VCOMISH int _mm_comilt_sh (__m128h a, __m128h b);VCOMISH int _mm_comineq_sh (__m128h a, __m128h b);
```
