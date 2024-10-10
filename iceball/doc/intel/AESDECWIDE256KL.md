# AESDECWIDE256KL

Perform 14 Rounds of AES Decryption Flow With Key Locker on 8 Blocks Using 256-Bit Key

1The AESDECWIDE256KL instruction performs 14 rounds of AES to decrypt each of the eight blocks in XMM0-7 using the 256-bit key indicated by the handle from the second operand.
It replaces each input block in XMM0-7 with its corresponding decrypted block if the operation succeeds (e.g., does not run into a handle violation failure).

## Flags affected

- ZF is set to 0 if the operation succeeded and set to 1 if the operation failed due to a handle violation. The other arithmetic flags (OF, SF, AF, PF, CF) are cleared to 0.

## Operation

```C
AESDECWIDE256KLHandle := UnalignedLoad of 512 bit (SRC); // Load is not guaranteed to be atomic.Illegal Handle = (HandleReservedBitSet (Handle) ||(Handle[0] AND (CPL > 0)) ||Handle [2] ||HandleKeyType (Handle) != HANDLE_KEY_TYPE_AES256);IF (Illegal Handle) {THEN RFLAGS.ZF := 1;ELSE (UnwrappedKey, Authentic) := UnwrapKeyAndAuthenticate512 (Handle[511:0], IWKey);IF (Authentic == 0) THEN RFLAGS.ZF := 1;ELSE XMM0 := AES256Decrypt (XMM0, UnwrappedKey) ;XMM1 := AES256Decrypt (XMM1, UnwrappedKey) ;XMM2 := AES256Decrypt (XMM2, UnwrappedKey) ;XMM3 := AES256Decrypt (XMM3, UnwrappedKey) ;XMM4 := AES256Decrypt (XMM4, UnwrappedKey) ;XMM5 := AES256Decrypt (XMM5, UnwrappedKey) ;XMM6 := AES256Decrypt (XMM6, UnwrappedKey) ;XMM7 := AES256Decrypt (XMM7, UnwrappedKey) ;RFLAGS.ZF := 0;FI;FI;RFLAGS.OF, SF, AF, PF, CF := 0;
```
