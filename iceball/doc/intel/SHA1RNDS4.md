# SHA1RNDS4

Perform Four Rounds of SHA1 Operation

InstructionMode Feature SupportFlagRMIV/VSHAPerforms four rounds of SHA1 operation operating on SHA1 state NP 0F 3A CC /r ib(A,B,C,D) from xmm1, with a pre-computed sum of the next 4 round SHA1RNDS4 xmm1, message dwords and state variable E from xmm2/m128.
The xmm2/m128, imm8immediate byte controls logic functions and round constants.Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RMIModRM:reg (r, w)ModRM:r/m (r)imm8The SHA1RNDS4 instruction performs four rounds of SHA1 operation using an initial SHA1 state (A,B,C,D) from the first operand (which is a source operand and the destination operand) and some pre-computed sum of the next 4 round message dwords, and state variable E from the second operand (a source operand).
The updated SHA1 state (A,B,C,D) after four rounds of processing is stored in the destination operand.

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
SHA1RNDS4 The function f() and Constant K are dependent on the value of the immediate.IF ( imm8[1:0] = 0 )THEN f() := f0(), K := K; 0ELSE IF ( imm8[1:0] = 1 ) THEN f() := f1(), K := K; 1ELSE IF ( imm8[1:0] = 2 ) THEN f() := f2(), K := K; 2ELSE IF ( imm8[1:0] = 3 ) THEN f() := f3(), K := K3; FI;A := SRC1[127:96]; B := SRC1[95:64]; C := SRC1[63:32]; D := SRC1[31:0]; E := SRC2[127:96]; W0W := SRC2[95:64]; 1W := SRC2[63:32]; 2W := SRC2[31:0]; 3Round i = 0 operation:A_1 := f (B, C, D) + (A ROL 5) +WE +K; 0B_1 := A; C_1 := B ROL 30; D_1 := C; E_1 := D; FOR i = 1 to 3+ E_i +K; B_(i +1) := A_i; C_(i +1) := B_i ROL 30; D_(i +1) := C_i; E_(i +1) := D_i; ENDFORDEST[127:96] := A_4; DEST[95:64] := B_4; DEST[63:32] := C_4; DEST[31:0] := D_4; Intel C/C++ Compiler Intrinsic EquivalentSHA1RNDS4 __m128i _mm_sha1rnds4_epu32(__m128i, __m128i, const int);
```
