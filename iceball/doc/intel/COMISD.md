# COMISD

Compare Scalar Ordered Double Precision Floating-Point Values and Set EFLAGS

Compares the double precision floating-point values in the low quadwords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal).
The OF, SF, and AF flags in the EFLAGS register are set to 0.
The unordered result is returned if either source operand is a NaN (QNaN or SNaN).Operand 1 is an XMM register; operand 2 can be an XMM register or a 64 bit memory location.
The COMISD instruc-tion differs from the UCOMISD instruction in that it signals a SIMD floating-point invalid operation exception (#I) when a source operand is either a QNaN or SNaN.
The UCOMISD instruction signals an invalid operation exception only if a source operand is an SNaN.The EFLAGS register is not updated if an unmasked SIMD floating-point exception is generated.VEX.vvvv and EVEX.vvvv are reserved and must be 1111b, otherwise instructions will #UD.Software should ensure VCOMISD is encoded with VEX.L=0.
Encoding VCOMISD with VEX.L=1 may encounter unpredictable behavior across different processor generations.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid (if SNaN or QNaN operands), Denormal.
- Other Exceptions
  > VEX-encoded instructions, see Table2-20, "Type 3 Class Exception Conditions."
  > EVEX-encoded instructions, see Table2-48, "Type E3NF Class Exception Conditions."

## Operation

```C
COMISD (All Versions)RESULT := OrderedCompare(DEST[63:0] <> SRC[63:0]) {(* Set EFLAGS *) CASE (RESULT) OFUNORDERED: ZF,PF,CF := 111;GREATER_THAN: ZF,PF,CF := 000;LESS_THAN: ZF,PF,CF := 001;EQUAL: ZF,PF,CF := 100;Intel C/C++ Compiler Intrinsic EquivalentVCOMISD int _mm_comi_round_sd(__m128d a, __m128d b, int imm, int sae); VCOMISD int _mm_comieq_sd (__m128d a, __m128d b)VCOMISD int _mm_comilt_sd (__m128d a, __m128d b)VCOMISD int _mm_comile_sd (__m128d a, __m128d b)VCOMISD int _mm_comigt_sd (__m128d a, __m128d b)VCOMISD int _mm_comige_sd (__m128d a, __m128d b)VCOMISD int _mm_comineq_sd (__m128d a, __m128d b)
```
