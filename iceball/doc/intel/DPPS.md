# DPPS

Dot Product of Packed Single Precision Floating-Point Values

Conditionally multiplies the packed single precision floating-point values in the destination operand (first operand) with the packed single precision floats in the source (second operand) depending on a mask extracted from the high 4 bits of the immediate byte (third operand).
If a condition mask bit in imm8[7:4] is zero, the corresponding ® 64 and IA-32 Archi-multiplication is replaced by a value of 0.0 in the manner described by Section 12.8.4 of Inteltectures Software Developer's Manual, Volume 1.The four resulting single precision values are summed into an intermediate result.
The intermediate result is condi-tionally broadcasted to the destination using a broadcast mask specified by bits [3:0] of the immediate byte.If a broadcast mask bit is "1", the intermediate result is copied to the corresponding dword element in the destina-tion operand.
If a broadcast mask bit is zero, the corresponding element in the destination is set to zero.DPPS follows the NaN forwarding rules stated in the Software Developer's Manual, vol.
1, table 4.7.
These rules do not cover horizontal prioritization of NaNs.
Horizontal propagation of NaNs to the destination and the positioning of those NaNs in the destination is implementation dependent.
NaNs on the input sources or computationally gener-ated NaNs will have at least one NaN propagated to the destination.128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding YMM register destination are unmodified.VEX.128 encoded version: the first source operand is an XMM register or 128-bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding YMM register destination are zeroed.VEX.256 encoded version: The first source operand is a 

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal.
  > Exceptions are determined separately for each add and 
  > multiply operation, in the order of their execution. 
  > Unmasked exceptions will leave the destination operands unchanged.

## Operation

```C
DP_primitive (SRC1, SRC2)IF (imm8[4] = 1) THEN Temp1[31:0] := DEST[31:0] * SRC[31:0]; // update SIMD exception flagsELSE Temp1[31:0] := +0.0; FI;IF (imm8[5] = 1) THEN Temp1[63:32] := DEST[63:32] * SRC[63:32]; // update SIMD exception flagsELSE Temp1[63:32] := +0.0; FI;IF (imm8[6] = 1) THEN Temp1[95:64] := DEST[95:64] * SRC[95:64]; // update SIMD exception flagsELSE Temp1[95:64] := +0.0; FI;IF (imm8[7] = 1) THEN Temp1[127:96] := DEST[127:96] * SRC[127:96]; // update SIMD exception flagsELSE Temp1[127:96] := +0.0; FI;Temp2[31:0] := Temp1[31:0] + Temp1[63:32]; // update SIMD exception flags/* if unmasked exception reported, execute exception handler*/Temp3[31:0] := Temp1[95:64] + Temp1[127:96]; // update SIMD exception flags/* if unmasked exception reported, execute exception handler*/Temp4[31:0] := Temp2[31:0] + Temp3[31:0]; // update SIMD exception flags/* if unmasked exception reported, execute exception handler*/IF (imm8[0] = 1) THEN DEST[31:0] := Temp4[31:0];ELSE DEST[31:0] := +0.0; FI;IF (imm8[1] = 1) THEN DEST[63:32] := Temp4[31:0];ELSE DEST[63:32] := +0.0; FI;IF (imm8[2] = 1) THEN DEST[95:64] := Temp4[31:0];ELSE DEST[95:64] := +0.0; FI;IF (imm8[3] = 1) THEN DEST[127:96] := Temp4[31:0];ELSE DEST[127:96] := +0.0; FI;DPPS (128-bit Legacy SSE Version)DEST[127:0] := DP_Primitive(SRC1[127:0], SRC2[127:0]);DEST[MAXVL-1:128] (Unmodified)VDPPS (VEX.128 Encoded Version)DEST[127:0] := DP_Primitive(SRC1[127:0], SRC2[127:0]);DEST[MAXVL-1:128] := 0VDPPS (VEX.256 Encoded Version)DEST[127:0] := DP_Primitive(SRC1[127:0], SRC2[127:0]);DEST[255:128] := DP_Primitive(SRC1[255:128], SRC2[255:128]);Intel C/C++ Compiler Intrinsic Equivalent(V)DPPS __m128 _mm_dp_ps ( __m128 a, __m128 b, const int mask);VDPPS __m256 _mm256_dp_ps ( __m256 a, __m256 b, const int mask);
```
