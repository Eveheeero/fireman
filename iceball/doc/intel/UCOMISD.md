# UCOMISD

Unordered Compare Scalar Double Precision Floating-Point Values and Set EFLAGS

Performs an unordered compare of the double precision floating-point values in the low quadwords of operand 1 (first operand) and operand 2 (second operand), and sets the ZF, PF, and CF flags in the EFLAGS register according to the result (unordered, greater than, less than, or equal).
The OF, SF, and AF flags in the EFLAGS register are set to 0.
The unordered result is returned if either source operand is a NaN (QNaN or SNaN).Operand 1 is an XMM register; operand 2 can be an XMM register or a 64 bit memorylocation.
The UCOMISD instruction differs from the COMISD instruction in that it signals a SIMD floating-point invalid oper-ation exception (#I) only when a source operand is an SNaN.
The COMISD instruction signals an invalid operation exception only if a source operand is either an SNaN or a QNaN.The EFLAGS register is not updated if an unmasked SIMD floating-point exception is generated.Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b, otherwise instructions will #UD.Software should ensure VCOMISD is encoded with VEX.L=0.
Encoding VCOMISD with VEX.L=1 may encounter unpredictable behavior across different processor generations.

## Exceptions

- Other Exceptions
  > VEX-encoded instructions, see Table2-20, "Type 3 Class Exception Conditions," additionally:
- SIMD Floating-Point Exceptions
  > Invalid (if SNaN operands), Denormal.

## Operation

```C
(V)UCOMISD (All Versions)RESULT := UnorderedCompare(DEST[63:0] <> SRC[63:0]) {(* Set EFLAGS *) CASE (RESULT) OFUNORDERED: ZF,PF,CF := 111;GREATER_THAN: ZF,PF,CF := 000;LESS_THAN: ZF,PF,CF := 001;EQUAL: ZF,PF,CF := 100;Intel C/C++ Compiler Intrinsic EquivalentVUCOMISD int _mm_comi_round_sd(__m128d a, __m128d b, int imm, int sae); UCOMISD int _mm_ucomieq_sd(__m128d a, __m128d b)UCOMISD int _mm_ucomilt_sd(__m128d a, __m128d b)UCOMISD int _mm_ucomile_sd(__m128d a, __m128d b)UCOMISD int _mm_ucomigt_sd(__m128d a, __m128d b)UCOMISD int _mm_ucomige_sd(__m128d a, __m128d b)UCOMISD int _mm_ucomineq_sd(__m128d a, __m128d b)
```
