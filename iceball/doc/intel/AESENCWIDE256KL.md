# AESENCWIDE256KL

Perform 14 Rounds of AES Encryption Flow With Key Locker on 8 Blocks Using 256-Bit Key

1The AESENCWIDE256KL instruction performs 14 rounds of AES to encrypt each of the eight blocks in XMM0-7 using the 256-bit key indicated by the handle from the second operand.
It replaces each input block in XMM0-7 with its corresponding encrypted block if the operation succeeds (e.g., does not run into a handle violation failure).

## Flags affected

- ZF is set to 0 if the operation succeeded and set to 1 if the operation failed due to a handle violation. The other arithmetic flags (OF, SF, AF, PF, CF) are cleared to 0.Intel C/C++ Compiler Intrinsic EquivalentAESENCWIDE256KLunsigned char _mm_aesencwide256kl_u8(__m128i odata[8], const __m128i idata[8], const void* h);Exceptions (All Operating Modes)#UD If the LOCK prefix is used.If CPUID.07H:ECX.KL [bit 23] = 0.If CR4.KL = 0.If CPUID.19H:EBX.AESKLE [bit 0] = 0.If CR0.EM = 1.If CR4.OSFXSR = 0.If CPUID.19H:EBX.WIDE_KL [bit 2] = 0.#NM If CR0.TS = 1.#PF If a page fault occurs.#GP(0) If a memory operand effective address is outside the CS, DS, ES, FS, or GS segment limit.If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment selector.If the memory address is in a non-canonical form.#SS(0) If a memory operand effective ad

## Operation

```C
AESENCWIDE256KL Handle := UnalignedLoad of 512 bit (SRC); // Load is not guaranteed to be atomic.Illegal Handle = (HandleReservedBitSet (Handle) ||(Handle[0] AND (CPL > 0)) ||Handle [1] ||HandleKeyType (Handle) != HANDLE_KEY_TYPE_AES256);IF (Illegal Handle) THEN RFLAGS.ZF := 1;ELSE (UnwrappedKey, Authentic) := UnwrapKeyAndAuthenticate512 (Handle[511:0], IWKey);IF (Authentic == 0) THEN RFLAGS.ZF := 1;ELSE XMM0 := AES256Encrypt (XMM0, UnwrappedKey) ;XMM1 := AES256Encrypt (XMM1, UnwrappedKey) ;XMM2 := AES256Encrypt (XMM2, UnwrappedKey) ;XMM3 := AES256Encrypt (XMM3, UnwrappedKey) ;XMM4 := AES256Encrypt (XMM4, UnwrappedKey) ;XMM5 := AES256Encrypt (XMM5, UnwrappedKey) ;XMM6 := AES256Encrypt (XMM6, UnwrappedKey) ;XMM7 := AES256Encrypt (XMM7, UnwrappedKey) ;RFLAGS.ZF := 0;FI;FI;RFLAGS.OF, SF, AF, PF, CF := 0;
```
