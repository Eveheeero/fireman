# AESDEC128KL

Perform Ten Rounds of AES Decryption Flow With Key Locker Using 128-Bit Key

1The AESDEC128KL instruction performs 10 rounds of AES to decrypt the first operand using the 128-bit key indi-cated by the handle from the second operand.
It stores the result in the first operand if the operation succeeds (e.g., does not run into a handle violation failure).

## Flags affected

- ZF is set to 0 if the operation succeeded and set to 1 if the operation failed due to a handle violation. The other arithmetic flags (OF, SF, AF, PF, CF) are cleared to 0.Intel C/C++ Compiler Intrinsic EquivalentAESDEC128KLunsigned char _mm_aesdec128kl_u8(__m128i* odata, __m128i idata, const void* h);

## Operation

```C
AESDEC128KL Handle := UnalignedLoad of 384 bit (SRC); // Load is not guaranteed to be atomic.Illegal Handle = (HandleReservedBitSet (Handle) ||(Handle[0] AND (CPL > 0)) ||Handle [2] ||HandleKeyType (Handle) != HANDLE_KEY_TYPE_AES128);IF (Illegal Handle) {THEN RFLAGS.ZF := 1;ELSE (UnwrappedKey, Authentic) := UnwrapKeyAndAuthenticate384 (Handle[383:0], IWKey);IF (Authentic == 0) THEN RFLAGS.ZF := 1;ELSE DEST := AES128Decrypt (DEST, UnwrappedKey) ;RFLAGS.ZF := 0;FI;FI;RFLAGS.OF, SF, AF, PF, CF := 0;
```
