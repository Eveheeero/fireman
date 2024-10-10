# AESDEC256KL

Perform 14 Rounds of AES Decryption Flow With Key Locker Using 256-Bit Key

1The AESDEC256KL instruction performs 14 rounds of AES to decrypt the first operand using the 256-bit key indi-cated by the handle from the second operand.
It stores the result in the first operand if the operation succeeds (e.g., does not run into a handle violation failure).

## Flags affected

- ZF is set to 0 if the operation succeeded and set to 1 if the operation failed due to a handle violation. The other arithmetic flags (OF, SF, AF, PF, CF) are cleared to 0.Intel C/C++ Compiler Intrinsic EquivalentAESDEC256KLunsigned char _mm_aesdec256kl_u8(__m128i* odata, __m128i idata, const void* h);

## Operation

```C
AESDEC256KL Handle := UnalignedLoad of 512 bit (SRC); // Load is not guaranteed to be atomic.Illegal Handle = (HandleReservedBitSet (Handle) ||(Handle[0] AND (CPL > 0)) ||Handle [2] ||HandleKeyType (Handle) != HANDLE_KEY_TYPE_AES256);IF (Illegal Handle) THEN RFLAGS.ZF := 1;ELSE (UnwrappedKey, Authentic) := UnwrapKeyAndAuthenticate512 (Handle[511:0], IWKey);IF (Authentic == 0) THEN RFLAGS.ZF := 1;ELSE DEST := AES256Decrypt (DEST, UnwrappedKey) ;RFLAGS.ZF := 0;FI;FI;RFLAGS.OF, SF, AF, PF, CF := 0;
```
