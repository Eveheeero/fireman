# SHA256RNDS2

Perform Two Rounds of SHA256 Operation

InstructionMode Feature SupportFlagRMIV/VSHAPerform 2 rounds of SHA256 operation using an initial SHA256 NP 0F 38 CB /r state (C,D,G,H) from xmm1, an initial SHA256 state (A,B,E,F) from SHA256RNDS2 xmm1, xmm2/m128, and a pre-computed sum of the next 2 round mes-xmm2/m128, <XMM0>sage dwords and the corresponding round constants from the implicit operand XMM0, storing the updated SHA256 state (A,B,E,F) result in xmm1.Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RMIModRM:reg (r, w)ModRM:r/m (r)Implicit XMM0 (r)The SHA256RNDS2 instruction performs 2 rounds of SHA256 operation using an initial SHA256 state (C,D,G,H) from the first operand, an initial SHA256 state (A,B,E,F) from the second operand, and a pre-computed sum of the next 2 round message dwords and the corresponding round constants from the implicit operand xmm0.
Note that only the two lower dwords of XMM0 are used by the instruction.The updated SHA256 state (A,B,E,F) is written to the first operand, and the second operand can be used as the updated state (C,D,G,H) in later rounds.

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
SHA256RNDS2 A_0 := SRC2[127:96]; B_0 := SRC2[95:64]; C_0 := SRC1[127:96]; D_0 := SRC1[95:64]; E_0 := SRC2[63:32]; F_0 := SRC2[31:0]; G_0 := SRC1[63:32]; H_0 := SRC1[31:0];  := XMM0[31: 0]; WK0WK := XMM0[63: 32]; 1FOR i = 0 to 1A_(i +1) := Ch (E_i, F_i, G_i) + ( E_i) +WK+ H_i + Maj(A_i , B_i, C_i) + ( A_i); 1i0B_(i +1) := A_i; C_(i +1) := B_i ; D_(i +1) := C_i; ( E_i) +WK+ H_i + D_i; E_(i +1) := Ch (E_i, F_i, G_i) + 1iF_(i +1) := E_i ; G_(i +1) := F_i; H_(i +1) := G_i; ENDFORDEST[127:96] := A_2; DEST[95:64] := B_2; Intel C/C++ Compiler Intrinsic EquivalentSHA256RNDS2 __m128i _mm_sha256rnds2_epu32(__m128i, __m128i, __m128i);
```
