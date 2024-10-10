# ENCODEKEY256

Encode 256-Bit Key With Key Locker

1The ENCODEKEY256 instruction wraps a 256-bit AES key from the implicit operand XMM1:XMM0 into a key handle that is then stored in the implicit destination operands XMM0-3.The explicit source operand is a general-purpose register and specifies what handle restrictions should be built into the handle.The explicit destination operand is populated with information on the source of the key and its attributes.
XMM4 through XMM6 are reserved for future usages and software should not rely upon them being zeroed.

## Flags affected

- All arithmetic flags (OF, SF, ZF, AF, PF, CF) are cleared to 0. Although they are cleared for the currently defined oper-ations, future extensions may report information in the flags.Intel C/C++ Compiler Intrinsic EquivalentENCODEKEY256unsigned int _mm_encodekey256_u32(unsigned int htype, __m128i key_lo, __m128i key_hi, void* h);Exceptions (All Operating Modes)#GP If reserved bit is set in source register value.#UD If the LOCK prefix is used.If CPUID.07H:ECX.KL [bit 23] = 0.If CR4.KL = 0.If CPUID.19H:EBX.AESKLE [bit 0] = 0.If CR0.EM = 1.

## Operation

```C
ENCODEKEY256 2#GP (0) if a reserved bit in SRC[31:0] is setInputKey[255:0] := XMM1:XMM0;KeyMetadata[2:0] = SRC[2:0];KeyMetadata[23:3] = 0; // Reserved for future usageKeyMetadata[27:24] = 1; // KeyType is AES-256 (value of 1)KeyMetadata[127:28] = 0; // Reserved for future usage// KeyMetadata is the AAD input and InputKey is the Plaintext input for WrapKey256Handle[511:0] := WrapKey256(InputKey[255:0], KeyMetadata[127:0], IWKey.Integrity Key[127:0], IWKey.Encryption Key[255:0]);DEST[0] := IWKey.NoBackup;DEST[4:1] := IWKey.KeySource[3:0];DEST[31:5] = 0;XMM0 := Handle[127:0];  // AADXMM1 := Handle[255:128]; // Integrity TagXMM2 := Handle[383:256]; // CipherText[127:0]XMM3 := Handle[511:384]; // CipherText[255:128]XMM4 := 0; // Reserved for future usageXMM5 := 0; // Reserved for future usageXMM6 := 0; // Reserved for future usageRFLAGS.OF, SF, ZF, AF, PF, CF := 0;
```
