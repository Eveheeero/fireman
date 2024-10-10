# VMOVSH

Move Scalar FP16 Value

This instruction moves a FP16 value to a register or memory location.The two register-only forms are aliases and differ only in where their operands are encoded; this is a side effect of the encodings selected.

## Exceptions

- SIMD Floating-Point Exceptions
  > None

## Operation

```C
VMOVSH dest, src (two operand load)IF k1[0] or no writemask:DEST.fp16[0] := SRC.fp16[0]ELSE IF *zeroing*:DEST.fp16[0] := 0// ELSE DEST.fp16[0] remains unchangedDEST[MAXVL:16] := 0VMOVSH dest, src (two operand store)IF k1[0] or no writemask:VMOVSH dest, src1, src2 (three operand copy)IF k1[0] or no writemask:DEST.fp16[0] := SRC2.fp16[0]ELSE IF *zeroing*:DEST.fp16[0] := 0// ELSE DEST.fp16[0] remains unchangedDEST[127:16] := SRC1[127:16]DEST[MAXVL:128] := 0 Intel C/C++ Compiler Intrinsic EquivalentVMOVSH __m128h _mm_load_sh (void const* mem_addr);VMOVSH __m128h _mm_mask_load_sh (__m128h src, __mmask8 k, void const* mem_addr);VMOVSH __m128h _mm_maskz_load_sh (__mmask8 k, void const* mem_addr);VMOVSH __m128h _mm_mask_move_sh (__m128h src, __mmask8 k, __m128h a, __m128h b);VMOVSH __m128h _mm_maskz_move_sh (__mmask8 k, __m128h a, __m128h b);VMOVSH __m128h _mm_move_sh (__m128h a, __m128h b);VMOVSH void _mm_mask_store_sh (void * mem_addr, __mmask8 k, __m128h a);VMOVSH void _mm_store_sh (void * mem_addr, __m128h a);
```
