# PHMINPOSUW

Packed Horizontal Word Minimum

Determine the minimum unsigned word value in the source operand (second operand) and place the unsigned word in the low word (bits 0-15) of the destination operand (first operand).
The word index of the minimum value is stored in bits 16-18 of the destination operand.
The remaining upper bits of the destination are set to zero.
128-bit Legacy SSE version: Bits (MAXVL-1:128) of the corresponding XMM destination register remain unchanged.VEX.128 encoded version: Bits (MAXVL-1:128) of the destination XMM register are zeroed.
VEX.vvvv is reserved and must be 1111b, VEX.L must be 0, otherwise the instruction will #UD.

## Flags affected

- None.

## Exceptions

- Other Exceptions
  > See Table2-21, "Type 4 Class Exception Conditions," additionally:
  - #UD - If VEX.L = 1.
  > If VEX.vvvv 
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
PHMINPOSUW (128-bit Legacy SSE Version)INDEX := 0;MIN := SRC[15:0]IF (SRC[31:16] < MIN) THEN INDEX := 1;  MIN := SRC[31:16]; FI;IF (SRC[47:32] < MIN) THEN INDEX := 2;  MIN := SRC[47:32]; FI;* Repeat operation for words 3 through 6IF (SRC[127:112] < MIN) THEN INDEX := 7;  MIN := SRC[127:112]; FI;DEST[15:0] := MIN;DEST[18:16] := INDEX;DEST[127:19] := 0000000000000000000000000000H;VPHMINPOSUW (VEX.128 Encoded Version)INDEX := 0MIN := SRC[15:0]IF (SRC[31:16] < MIN) THEN INDEX := 1; MIN := SRC[31:16]IF (SRC[47:32] < MIN) THEN INDEX := 2; MIN := SRC[47:32]* Repeat operation for words 3 through 6IF (SRC[127:112] < MIN) THEN INDEX := 7; MIN := SRC[127:112]DEST[15:0] := MINDEST[18:16] := INDEXIntel C/C++ Compiler Intrinsic EquivalentPHMINPOSUW __m128i _mm_minpos_epu16( __m128i packed_words);
```
