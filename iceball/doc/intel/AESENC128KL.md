# AESENC128KL

Perform Ten Rounds of AES Encryption Flow With Key Locker Using 128-Bit Key

1The AESENC128KL instruction performs ten rounds of AES to encrypt the first operand using the 128-bit key indi-cated by the handle from the second operand.
It stores the result in the first operand if the operation succeeds (e.g., does not run into a handle violation failure).

## Flags affected

- ZF is set to 0 if the operation succeeded and set to 1 if the operation failed due to a handle violation. The other arithmetic flags (OF, SF, AF, PF, CF) are cleared to 0.Intel C/C++ Compiler Intrinsic EquivalentAESENC128KL unsigned char _mm_aesenc128kl_u8(__m128i* odata, __m128i idata, const void* h);Exceptions (All Operating Modes)#UD If the LOCK prefix is used.If CPUID.07H:ECX.KL [bit 23] = 0.If CR4.KL = 0.If CPUID.19H:EBX.AESKLE [bit 0] = 0.If CR0.EM = 1.If CR4.OSFXSR = 0.#NM If CR0.TS = 1.#PF If a page fault occurs.#GP(0) If a memory operand effective address is outside the CS, DS, ES, FS, or GS segment limit.If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment selector.If the memory address is in a non-canonical form.#SS(0) If a memory operand effective ad

## Operation

```C
AESENC128KL Handle := UnalignedLoad of 384 bit (SRC); // Load is not guaranteed to be atomic.Illegal Handle = (HandleReservedBitSet (Handle) ||(Handle[0] AND (CPL > 0)) ||Handle [1] ||HandleKeyType (Handle) != HANDLE_KEY_TYPE_AES128);IF (Illegal Handle) {THEN RFLAGS.ZF := 1;ELSE (UnwrappedKey, Authentic) := UnwrapKeyAndAuthenticate384 (Handle[383:0], IWKey);IF (Authentic == 0) THEN RFLAGS.ZF := 1;ELSE DEST := AES128Encrypt (DEST, UnwrappedKey) ;RFLAGS.ZF := 0;FI;FI;RFLAGS.OF, SF, AF, PF, CF := 0;
```
