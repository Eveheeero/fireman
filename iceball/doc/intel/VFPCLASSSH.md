# VFPCLASSSH

Test Types of Scalar FP16 Values

This instruction checks the low FP16 value in the source operand for special categories, specified by the set bits in the imm8 byte.
Each set bit in imm8 specifies a category of floating-point values that the input data element is clas-sified against; see Table 5-8 for the categories.
The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element.
The result is written to the low bit in the desti-nation mask register according to the writemask.
The other bits in the destination mask register are zeroed.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VFPCLASSSH dest{k2}, src, imm8 IF k2[0] or *no writemask*:DEST.bit[0] := check_fp_class_fp16(src.fp16[0], imm8) // see VFPCLASSPHELSE:DEST.bit[0] := 0DEST[MAXKL-1:1] := 0 Intel C/C++ Compiler Intrinsic EquivalentVFPCLASSSH __mmask8 _mm_fpclass_sh_mask (__m128h a, int imm8);VFPCLASSSH __mmask8 _mm_mask_fpclass_sh_mask (__mmask8 k1, __m128h a, int imm8);
```
