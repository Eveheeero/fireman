# MOVAPD

Move Aligned Packed Double Precision Floating-Point Values

InstructionMode Feature SupportFlag66 0F 28 /rAV/VSSE2Move aligned packed double precision floating-MOVAPD xmm1, xmm2/m128point values from xmm2/mem to xmm1.
66 0F 29 /rBV/VSSE2Move aligned packed double precision floating-MOVAPD xmm2/m128, xmm1point values from xmm1 to xmm2/mem.
VEX.128.66.0F.WIG 28 /rAV/VAVXMove aligned packed double precision floating-VMOVAPD xmm1, xmm2/m128point values from xmm2/mem to xmm1.
VEX.128.66.0F.WIG 29 /rBV/VAVXMove aligned packed double precision floating-VMOVAPD xmm2/m128, xmm1point values from xmm1 to xmm2/mem.
VEX.256.66.0F.WIG 28 /rAV/VAVXMove aligned packed double precision floating-VMOVAPD ymm1, ymm2/m256point values from ymm2/mem to ymm1.
VEX.256.66.0F.WIG 29 /rBV/VAVXMove aligned packed double precision floating-VMOVAPD ymm2/m256, ymm1point values from ymm1 to ymm2/mem.
EVEX.128.66.0F.W1 28 /rCV/VAVX512VLMove aligned packed double precision floating-VMOVAPD xmm1 {k1}{z}, xmm2/m128AVX512Fpoint values from xmm2/m128 to xmm1 using writemask k1.EVEX.256.66.0F.W1 28 /rCV/VAVX512VLMove aligned packed double precision floating-VMOVAPD ymm1 {k1}{z}, ymm2/m256 AVX512Fpoint values from ymm2/m256 to ymm1 using writemask k1.EVEX.512.66.0F.W1 28 /rCV/VAVX512FMove aligned packed double precision floating-VMOVAPD zmm1 {k1}{z}, zmm2/m512point values from zmm2/m512 to zmm1 using writemask k1.EVEX.128.66.0F.W1 29 /rDV/VAVX512VLMove aligned packed double precision floating-VMOVAPD xmm2/m128 {k1}{z}, xmm1AVX512Fpoint values from xmm1 to xmm2/m128 using writemask k1.EVEX.256.66.0F.W1 29 /rDV/VAVX512VLMove aligned packed double precision floating-VMOVAPD ymm2/m256 {k1}{z}, ymm1AVX512Fpoint values from ymm1 to ymm2/m256 using writemask k1.EVEX.512.66.0F.W1 29 /rDV/VAVX512FMove aligned packed double precision floating-VMOVAPD zmm2/m512 {k1}{z}, zmm1point values from zmm1 to zmm2/m512 using writemask k1.Instruction Operand EncodingOp/EnTuple TypeOperand 1Operand 2Operand 3Operand 4AN/AModRM:reg (w)ModRM:r/m (r)N/AN/ABN/AModRM:r/m (w)ModRM:reg (r)N/AN/ACFull MemModRM:reg (w)ModRM:r/m (r)N/AN/ADFull MemModRM:r/m (w)ModRM:reg (r)N/AN/AMoves 2, 4 or 8 double precision floating-point values from the source operand (second operand) to the destination operand (first operand).
This instruction can be used to load an XMM, YMM or ZMM register from an 128-bit, 256-bit or 512-bit memory location, to store the contents of an XMM, YMM or ZMM register into a 128-bit, 256-bit or 512-bit memory location, or to move data between two XMM, two YMM or two ZMM registers.
When the source or destination operand is a memory operexception (#GP) will be generated.
For EVEX encoded versions, the operand must be aligned to the size of the memory operand.
To move double precision floating-point values to and from unaligned memory locations, use the VMOVUPD instruction.Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.EVEX.512 encoded version:Moves 512 bits of packed double precision floating-point values from the source operand (second operand) to the destination operand (first operand).
This instruction can be used to load a ZMM register from a 512-bit float64 memory location, to store the contents of a ZMM register into a 512-bit float64 memory location, or to move data between two ZMM registers.
When the source or destination operand is a memory operand, the operand must be aligned on a 64-byte boundary or a general-protection exception (#GP) will be generated.
To move single precision floating-point values to and from unaligned memory locations, use the VMOVUPD instruction.VEX.256 and EVEX.256 encoded versions:Moves 256 bits of packed double precision floating-point values from the source operand (second operand) to the destination operand (first operand).
This instruction can be used to load a YMM register from a 256-bit memory location, to store the contents of a YMM register into a 256-bit memory location, or to move data between two YMM registers.
When the source or destination operand is a memory operand, the operand must be aligned on a 32-byte boundary or a general-protection exception (#GP) will be generated.
To move double precision floating-point values to and from unaligned memory locations, use the VMOVUPD instruction.128-bit versions:Moves 128 bits of packed double precision floating-point values from the source operand (second operand) to the destination operand (first operand).
This instruction can be used to load an XMM register from a 128-bit memory location, to store the contents of an XMM register into a 128-bit memory location, or to move data between two XMM registers.
When the source or destination operand is a memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
To move single precision floating-point values to and from unaligned memory locations, use the VMOVUPD instruction.128-bit Legacy SSE version: Bits (MAXVL-1:128) of the corresponding ZMM destination register remain unchanged.
(E)VEX.128 encoded version: Bits (MAXVL-1:128) of the destination ZMM register destination are zeroed.


## Exceptions

- Other Exceptions
  > Non-EVEX-encoded instruction, see Exceptions Type1.SS
  > E2 in Table2-18, "Type 1 Class Exception Conditions."
  > EVEX-encoded instruction, see Table2-44, "Type E1 Class Exception Conditions."
  > Additionally:
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VMOVAPD (EVEX Encoded Versions, Register-Copy Form)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN DEST[i+63:i] := SRC[i+63:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE  DEST[i+63:i] := 0 ; zeroing-maskingFIFI;VMOVAPD (EVEX Encoded Versions, Store-Form) (KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN DEST[i+63:i] := SRC[i+63:i]ELSE ELSE *DEST[i+63:i] remains unchanged*; merging-maskingFI;ENDFOR;VMOVAPD (EVEX Encoded Versions, Load-Form) (KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN DEST[i+63:i] := SRC[i+63:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE  DEST[i+63:i] := 0 ; zeroing-maskingFIFI;ENDFORDEST[MAXVL-1:VL] := 0VMOVAPD (VEX.256 Encoded Version, Load - and Register Copy)DEST[255:0] := SRC[255:0]DEST[MAXVL-1:256] := 0VMOVAPD (VEX.256 Encoded Version, Store-Form)DEST[255:0] := SRC[255:0]VMOVAPD (VEX.128 Encoded Version, Load - and Register Copy)DEST[127:0] := SRC[127:0]DEST[MAXVL-1:128] := 0MOVAPD (128-bit Load- and Register-Copy- Form Legacy SSE Version)DEST[127:0] := SRC[127:0]DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVMOVAPD __m512d _mm512_load_pd( void * m);VMOVAPD __m512d _mm512_mask_load_pd(__m512d s, __mmask8 k, void * m);VMOVAPD __m512d _mm512_maskz_load_pd( __mmask8 k, void * m);VMOVAPD void _mm512_store_pd( void * d, __m512d a);VMOVAPD void _mm512_mask_store_pd( void * d, __mmask8 k, __m512d a);VMOVAPD __m256d _mm256_mask_load_pd(__m256d s, __mmask8 k, void * m);VMOVAPD __m256d _mm256_maskz_load_pd( __mmask8 k, void * m);VMOVAPD void _mm256_mask_store_pd( void * d, __mmask8 k, __m256d a);VMOVAPD __m128d _mm_mask_load_pd(__m128d s, __mmask8 k, void * m);VMOVAPD __m128d _mm_maskz_load_pd( __mmask8 k, void * m);VMOVAPD void _mm_mask_store_pd( void * d, __mmask8 k, __m128d a);MOVAPD __m256d _mm256_load_pd (double * p);MOVAPD void _mm256_store_pd(double * p, __m256d a);MOVAPD __m128d _mm_load_pd (double * p);MOVAPD void _mm_store_pd(double * p, __m128d a);
```
