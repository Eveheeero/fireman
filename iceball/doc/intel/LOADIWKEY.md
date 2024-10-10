# LOADIWKEY

Load Internal Wrapping Key With Key Locker

1The LOADIWKEY instruction writes the Key Locker internal wrapping key, which is called IWKey.
This IWKey is used by the ENCODEKEY* instructions to wrap keys into handles.
Conversely, the AESENC/DEC*KL instructions use IWKey to unwrap those keys from the handles and help verify the handle integrity.
For security reasons, no instruc-tion is designed to allow software to directly read the IWKey value.IWKey includes two cryptographic keys as well as metadata.
The two cryptographic keys are loaded from register sources so that LOADIWKEY can be executed without the keys ever being in memory.The key input operands are: - The 256-bit encryption key is loaded from the two explicit operands.
- The 128-bit integrity key is loaded from the implicit operand XMM0.The implicit operand EAX specifies the KeySource and whether backing up the key is permitted: - EAX[0] - When set, the wrapping key being initialized is not permitted to be backed up to platform-scoped storage.
- EAX[4:1] - This specifies the KeySource, which is the type of key.
Currently only two encodings are supported.
A KeySource of 0 indicates that the key input operands described above should be directly stored as the internal wrapping keys.
LOADIWKEY with a KeySource of 1 will have random numbers from the on-chip random number generator XORed with the source registers (including XMM0) so that the software that executes the LOADIWKEY does not know the actual IWKey encryption and integrity keys.
Software can choose to put additional random data into the source registers so that other sources of random data are combined with the hardware random number generator supplied value.
Software should always check ZF after executing LOADIWKEY with KeySource of 1 as this operation may fail due to it being unable to get sufficient full-entropy data from the on-chip random number generator.
Both KeySource of 0 and 1 specify that IWKey be used with the AES-GCM-SIV algorithm.
CPUID.19H.ECX[1] enumerates support for KeySource of 1.
All other KeySource encodings are reserved.
- EAX[31:5] - Reserved.

## Flags affected

- ZF is set to 0 if the operation succeeded and set to 1 if the operation failed due to full-entropy random data not being received from RDSEED. The other arithmetic flags (OF, SF, AF, PF, CF) are cleared to 0.Intel C/C++ Compiler Intrinsic EquivalentExceptions (All Operating Modes)#GP If CPL > 0. (Does not apply in real-address mode.)If EAX[4:1] > 1.If EAX[31:5] != 0.If (EAX[0] == 1) AND (CPUID.19H.ECX[0] == 0).If (EAX[4:1] == 1) AND (CPUID.19H.ECX[1] == 0).#UD If the LOCK prefix is used.If CPUID.07H:ECX.KL [bit 23] = 0.If CR4.KL = 0.If CR0.EM = 1.

## Operation

```C
LOADIWKEY IF CPL > 0 // LOADKWKEY only allowed at ring 0 (supervisor mode)THEN #GP (0); FI;IF EAX[4:1] > 1 // Reserved KeySource encoding usedTHEN #GP (0); FI;IF EAX[31:5] != 0 // Reserved bit in EAX is setTHEN #GP (0); FI;IF EAX[0] AND (CPUID.19H.ECX[0] == 0) // NoBackup is not supported on this partTHEN #GP (0); FI;IF (EAX[4:1] == 1) AND (CPUID.19H.ECX[1] == 0) // KeySource of 1 is not supported on this partTHEN #GP (0); FI;IF (EAX[4:1] == 0) // KeySource of 0THENIWKey.Encryption Key[127:0] := SRC2[127:0]:IWKey.Encryption Key[255:128] := SRC1[127:0];IWKey.IntegrityKey[127:0] := XMM0[127:0];IWKey.NoBackup = EAX [0];IWKey.KeySource = EAX [4:1];RFLAGS.ZF := 0; ELSE // KeySource of 1. See RDSEED definition for details of randomnessIF HW_NRND_GEN.ready == 1 // Full-entropy random data from RDSEED hardware block was receivedTHENIWKey.Encryption Key[127:0] := SRC2[127:0] XOR HW_NRND_GEN.data[127:0];IWKey.Encryption Key[255:128] := SRC1[127:0] XOR HW_NRND_GEN.data[255:128];IWKey.IntegrityKey[127:0] := XMM0[127:0] XOR HW_NRND_GEN.data[383:256];IWKey.NoBackup = EAX [0];IWKey.KeySource = EAX [4:1];RFLAGS.ZF := 0; ELSE // Random data was not returned from RDSEED hardware block. IWKey was not loadedRFLAGS.ZF := 1; FI;FI;RFLAGS.OF, SF, AF, PF, CF := 0;
```
