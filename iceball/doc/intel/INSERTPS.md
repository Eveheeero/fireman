# INSERTPS

Insert Scalar Single Precision Floating-Point Value

InstructionMode Feature SupportFlag66 0F 3A 21 /r ibAV/VSSE4_1Insert a single precision floating-point value selected by INSERTPS xmm1, xmm2/m32, imm8imm8 from xmm2/m32 into xmm1 at the specified destination element specified by imm8 and zero out destination elements in xmm1 as indicated in imm8.VEX.128.66.0F3A.WIG 21 /r ibBV/VAVXInsert a single precision floating-point value selected by imm8 from xmm3/m32 and merge with values in xmm2 VINSERTPS xmm1, xmm2, xmm3/m32, at the specified destination element specified by imm8 imm8and write out the result and zero out destination elements in xmm1 as indicated in imm8.CV/VAVX512FInsert a single precision floating-point value selected by EVEX.128.66.0F3A.W0 21 /r ibimm8 from xmm3/m32 and merge with values in xmm2 VINSERTPS xmm1, xmm2, xmm3/m32, at the specified destination element specified by imm8 imm8and write out the result and zero out destination elements in xmm1 as indicated in imm8.Instruction Operand EncodingOp/EnTuple TypeOperand 1Operand 2Operand 3Operand 4AN/AModRM:reg (r, w)ModRM:r/m (r)imm8N/ABN/AModRM:reg (w)VEX.vvvv (r)ModRM:r/m (r)imm8CTuple1 ScalarModRM:reg (w)EVEX.vvvv (r)ModRM:r/m (r)imm8(register source form)Copy a single precision scalar floating-point element into a 128-bit vector register.
The immediate operand has three fields, where the ZMask bits specify which elements of the destination will be set to zero, the Count_D bits specify which element of the destination will be overwritten with the scalar value, and for vector register sources the Count_S bits specify which element of the source will be copied.
When the scalar source is a memory operand the Count_S bits are ignored.
(memory source form)Load a floating-point element from a 32-bit memory location and destination operand it into the first source at the location indicated by the Count_D bits of the immediate operand.
Store in the destination and zero out destination elements based on the ZMask bits of the immediate operand.
128-bit Legacy SSE version: The first source register is an XMM register.
The second source operand is either an XMM register or a 32-bit memory location.
The destination is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding register destination are unmodified.VEX.128 and EVEX encoded version: The destination and first source register is an XMM register.
The second source operand is either an XMM register or a 32-bit memory location.
The upper bits (MAXVL-1:128) of the corre-sponding register destination are zeroed.If VINSERTPS is encoded with VEX.L= 1, an attempt to 

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-22, "Type 5 Class Exception Conditions," additionally:

## Operation

```C
VINSERTPS (VEX.128 and EVEX Encoded Version)IF (SRC = REG) THEN COUNT_S := imm8[7:6]ELSE COUNT_S := 0COUNT_D := imm8[5:4]ZMASK := imm8[3:0]CASE (COUNT_S) OF0: TMP := SRC2[31:0]1: TMP := SRC2[63:32]2: TMP := SRC2[95:64]3: TMP := SRC2[127:96]ESAC;CASE (COUNT_D) OF0: TMP2[31:0] := TMPTMP2[127:32] := SRC1[127:32]1: TMP2[63:32] := TMPTMP2[31:0] := SRC1[31:0]TMP2[127:64] := SRC1[127:64]2: TMP2[95:64] := TMPTMP2[63:0] := SRC1[63:0]TMP2[127:96] := SRC1[127:96]3: TMP2[127:96] := TMPTMP2[95:0] := SRC1[95:0]ESAC;IF (ZMASK[0] = 1) THEN DEST[31:0] := 00000000HELSE DEST[31:0] := TMP2[31:0]IF (ZMASK[1] = 1) THEN DEST[63:32] := 00000000HELSE DEST[63:32] := TMP2[63:32]IF (ZMASK[2] = 1) THEN DEST[95:64] := 00000000HELSE DEST[95:64] := TMP2[95:64]IF (ZMASK[3] = 1) THEN DEST[127:96] := 00000000HELSE DEST[127:96] := TMP2[127:96]DEST[MAXVL-1:128] := 0INSERTPS (128-bit Legacy SSE Version)IF (SRC = REG) THEN COUNT_S :=imm8[7:6]ELSE COUNT_S :=0COUNT_D := imm8[5:4]ZMASK := imm8[3:0]CASE (COUNT_S) OF0: TMP := SRC[31:0]1: TMP := SRC[63:32]2: TMP := SRC[95:64]3: TMP := SRC[127:96]ESAC;CASE (COUNT_D) OF0: TMP2[31:0] := TMPTMP2[127:32] := DEST[127:32]1: TMP2[63:32] := TMPTMP2[31:0] := DEST[31:0]TMP2[63:0] := DEST[63:0]TMP2[127:96] := DEST[127:96]3: TMP2[127:96] := TMPTMP2[95:0] := DEST[95:0]ESAC;IF (ZMASK[0] = 1) THEN DEST[31:0] := 00000000HELSE DEST[31:0] := TMP2[31:0]IF (ZMASK[1] = 1) THEN DEST[63:32] := 00000000HELSE DEST[63:32] := TMP2[63:32]IF (ZMASK[2] = 1) THEN DEST[95:64] := 00000000HELSE DEST[95:64] := TMP2[95:64]IF (ZMASK[3] = 1) THEN DEST[127:96] := 00000000HELSE DEST[127:96] := TMP2[127:96]DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVINSERTPS __m128 _mm_insert_ps(__m128 dst, __m128 src, const int nidx);INSETRTPS __m128 _mm_insert_ps(__m128 dst, __m128 src, const int nidx);
```
