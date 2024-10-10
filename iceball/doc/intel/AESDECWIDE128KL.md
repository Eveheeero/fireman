# AESDECWIDE128KL

Perform Ten Rounds of AES Decryption Flow With Key Locker on 8 Blocks Using 128-Bit Key

1The AESDECWIDE128KL instruction performs ten rounds of AES to decrypt each of the eight blocks in XMM0-7 using the 128-bit key indicated by the handle from the second operand.
It replaces each input block in XMM0-7 with its corresponding decrypted block if the operation succeeds (e.g., does not run into a handle violation failure).

## Flags affected

- ZF is set to 0 if the operation succeeded and set to 1 if the operation failed due to a handle violation. The other arithmetic flags (OF, SF, AF, PF, CF) are cleared to 0.

## Operation

```C
AESDECWIDE128KLHandle := UnalignedLoad of 384 bit (SRC);// Load is not guaranteed to be atomic.Illegal Handle = (HandleReservedBitSet (Handle) ||(Handle[0] AND (CPL > 0)) ||Handle [2] ||HandleKeyType (Handle) != HANDLE_KEY_TYPE_AES128);IF (Illegal Handle) THEN RFLAGS.ZF := 1;ELSE (UnwrappedKey, Authentic) := UnwrapKeyAndAuthenticate384 (Handle[383:0], IWKey);IF Authentic == 0 {THEN RFLAGS.ZF := 1;ELSE XMM0 := AES128Decrypt (XMM0, UnwrappedKey) ;XMM1 := AES128Decrypt (XMM1, UnwrappedKey) ;XMM2 := AES128Decrypt (XMM2, UnwrappedKey) ;XMM3 := AES128Decrypt (XMM3, UnwrappedKey) ;XMM4 := AES128Decrypt (XMM4, UnwrappedKey) ;XMM5 := AES128Decrypt (XMM5, UnwrappedKey) ;XMM6 := AES128Decrypt (XMM6, UnwrappedKey) ;XMM7 := AES128Decrypt (XMM7, UnwrappedKey) ;RFLAGS.ZF := 0;FI;FI;RFLAGS.OF, SF, AF, PF, CF := 0;
```
