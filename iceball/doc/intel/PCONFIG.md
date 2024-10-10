# PCONFIG

Platform Configuration

The PCONFIG instruction allows software to configure certain platform features.
It supports these features with multiple leaf functions, selecting a leaf function using the value in EAX.Depending on the leaf function, the registers RBX, RCX, and RDX may be used to provide input information or for the instruction to report output information.
Addresses and operands are 32 bits outside 64-bit mode and are 64 bits in 64-bit mode.
The value of CS.D does not affect operand size or address size.Executions of PCONFIG may fail for platform-specific reasons.
An execution reports failure by setting the ZF flag and loading EAX with a non-zero failure reason; a successful execution clears ZF and EAX.Each PCONFIG leaf function applies to a specific hardware block called a PCONFIG target.
The leaf function is supported only if the processor supports that target.
Each target is associated with a numerical target identifier, and CPUID leaf 1BH (PCONFIG information) enumerates the identifiers of the supported targets.
An attempt to execute an undefined leaf function, or a leaf function that applies to an unsupported target identifier, results in a general-protection exception (#GP).Leaf Function MKTME_KEY_PROGRAMAs of this writing, the only defined PCONFIG leaf function is used for key programming for total memory encryp-1tion-multi-key (TME-MK).
This leaf function is called MKTME_KEY_PROGRAM and it pertains to the TME-MK target, which has target identifier 1.
The leaf function is selected by loading EAX with value 0.
The MKTME_KEY_PROGRAM leaf function uses the EBX (or RBX) register for additional input information.Software uses the MKTME_KEY_PROGRAM leaf function to manage the encryption key associated with a particular key identifier (KeyID).
The leaf function uses a data structure called the TME-MK key programming structure (MKTME_KEY_PROGRAM_STRUCT).
Software provides the address of the structure (as an offset in the DS segment) in EBX (or RBX).
The format of the structure is given in Table 4-15.Table 4-15.
 MKTME_KEY_PROGRAM_STRUCT FormatFieldOffset (bytes)Size (bytes)CommentsKEYID02Key Identifier.KEYID_CTRL24KeyID control: - Bits 7:0: key-programming command (COMMAND) - Bits 23:8: encryption algorithm (ENC_ALG) - Bits 31:24: Reserved, must be zero (RSVD)Ignored658Not used.KEY_FIELD_16464Software supplied data key or entropy for data key.KEY_FIELD_212864Software supplied tweak key or entropy for tweak key.

## Exceptions

- 64-Bit Mode Exceptions
  - #GP(0) - If input value in EAX encode
  > s an unsupported leaf function.
  > If a memory operand is non-canonical form.
  > MKTME_KEY_PROGRAM leaf function:
  > If IA32_TME_ACTIVATE MSR is not locked.
  > If hardware encryption and TME-MK capabilit
  > y are not enabled in IA32_TME_ACTIVATE MSR.
  > If a memory operand is not 256B aligned.
  > If any of the reserved bits in the KEYID_CT
  > RL field of the MKTME_KEY_PROGRAM_STRUCT are 
  > set or that field indicates an unsupported Ke
  > yID, key-programming command, or encryption 
  > algorithm.
  - #PF(fault-code) - If a page fault occurs in accessing memory operands.
  - #UD - If any of the LOCK/REP/Operand
  >  Size/VEX prefixes are used.
- Real-Address Mode Exceptions
  - #GP - If input value in EAX encode
  > s an unsupported leaf function.
  > MKTME_KEY_PROGRAM leaf function:
  > If IA32_TME_ACTIVATE MSR is not locked.
  > If hardware encryption and TME-MK capabilit
  > y are not enabled in IA32_TME_ACTIVATE MSR.
  > If a memory operand is not 256B aligned.
  > If any of the reserved bits in the KEYID_CT
  > RL field of the MKTME_KEY_PROGRAM_STRUCT are 
  > set or that field indicates an unsupported Ke
  > yID, key-programming command, or encryption 
  > algorithm.
  - #UD - If any of the LOCK/REP/Opera
  > nd Size/VEX prefixes are used.
- Protected Mode Exceptions
  - #GP(0) - If input value in EAX enco
  > des an unsupported leaf function.
  > If a memory operand effective address is outside the relevant segment limit.
  > MKTME_KEY_PROGRAM leaf function:
  > If IA32_TME_ACTIVATE MSR is not locked.
  > If hardware encryption and TME-MK capabilit
  > y are not enabled in IA32_TME_ACTIVATE MSR.
  > If the memory operand is not 256B aligned.
  > If any of the reserved bits in the KEYID_CT
  > RL field of the MKTME_KEY_PROGRAM_STRUCT are 
  > set or that field indicates an unsupported Ke
  > yID, key-programming command, or encryption 
  > algorithm.
  - #PF(fault-code) - If a page fault occurs in accessing memory operands.
  - #UD - If any of the LOCK/REP/Opera
  > nd Size/VEX prefixes are used.
  > If current privilege level is not 0.
  > If CPUID.7.0:EDX[bit 18] = 0
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  - #UD - PCONFIG instruction is not recognized in virtual-8086 mode.

## Operation

```C
(* #UD if PCONFIG is not enumerated or CPL > 0 *)IF CPUID.7.0:EDX[18] = 0 OR CPL > 0THEN #UD; FI;(* #GP(0) for an unsupported leaf function *)IF EAX != 0THEN #GP(0); FI;CASE (EAX)(* operation based on selected leaf function *)0 (MKTME_KEY_PROGRAM):(* Confirm that TME-MK is properly enabled by the IA32_TME_ACTIVATE MSR *)(* The MSR must be locked, encryption enabled, and a non-zero number of KeyID bits specified *)IF IA32_TME_ACTIVATE[0] = 0 OR IA32_TME_ACTIVATE[1] = 0 OR IA32_TME_ACTIVATE[35:32] = 0THEN #GP(0); FI;IF DS:RBX is not 256-byte alignedTHEN #GP(0); FI;Load TMP_KEY_PROGRAM_STRUCT from 192 bytes at linear address DS:RBX;IF TMP_KEY_PROGRAM_STRUCT.KEYID_CTRL sets any reserved bitsTHEN #GP(0); FI;(* Check for a valid command *)IF TMP_KEY_PROGRAM_STRUCT. KEYID_CTRL.COMMAND > 3THEN #GP(0); FI;(* Check that the KEYID being operated upon is a valid KEYID *)IF TMP_KEY_PROGRAM_STRUCT.KEYID = 0 ORTMP_KEY_PROGRAM_STRUCT.KEYID > 2^IA32_TME_ACTIVATE.MK_TME_KEYID_BITS - 1 ORTMP_KEY_PROGRAM_STRUCT.KEYID > IA(* Check that only one encryption algorithm is requested for the KeyID and it is one of the activated algorithms *)IF TMP_KEY_PROGRAM_STRUCT.KEYID_CTRL.ENC_ALG does not set exactly one bit OR(TMP_KEY_PROGRAM_STRUCT.KEYID_CTRL.ENC_ALG & IA32_TME_ACTIVATE[63:48]) = 0THEN #GP(0); FI:Attempt to acquire lock to gain exclusive access to platform key table;IF attempt is unsuccessfulTHEN (* PCONFIG failure *)RFLAGS.ZF := 1;RAX := DEVICE_BUSY;(* failure reason 5 *)GOTO EXIT;FI;CASE (TMP_KEY_PROGRAM_STRUCT.KEYID_CTRL.COMMAND) OF0 (KEYID_SET_KEY_DIRECT):Update TME-MK table for TMP_KEY_PROGRAM_STRUCT.KEYID as follows:Encrypt with the selected keyUse the encryption algorithm selected by TMP_KEY_PROGRAM_STRUCT.KEYID_CTRL.ENC_ALG(* The number of bytes used by the next two lines depends on selected encryption algorithm *)DATA_KEY is TMP_KEY_PROGRAM_STRUCT.KEY_FIELD_1TWEAK_KEY is TMP_KEY_PROGRAM_STRUCT.KEY_FIELD_2BREAK;1 (KEYID_SET_KEY_RANDOM):Load TMP_RND_DATA_KEY with a random key using hardware RNG; (* key size depends on selected encryption algorithm *)IF there was insufficient entropyTHEN (* PCONFIG failure *)RFLAGS.ZF := 1;RAX := ENTROPY_ERROR;(* failure reason 2 *)Release lock on platform key table;GOTO EXIT;FI;Load TMP_RND_TWEAK_KEY with a random key using hardware RNG; (* key size depends on selected encryption algorithm *)IF there was insufficient entropyTHEN (* PCONFIG failure *)RFLAGS.ZF := 1;RAX := ENTROPY_ERROR;(* failure reason 2 *)Release lock on platform key table;GOTO EXIT;FI;(* Combine software-supplied entropy to the data key and tweak key *)(* The number of bytes used by the next two lines depends on selected encryption algorithm *)TMP_RND_DATA_KEY := TMP_RND_KEY XOR TMP_KEY_PROGRAM_STRUCT.KEY_FIELD_1;TMP_RND_TWEAK_KEY := TMP_RND_TWEAK_KEY XOR TMP_KEY_PROGRAM_STRUCT.KEY_FIELD_2;Update TME-MK table for TMP_KEY_PROGRAM_STRUCT.KEYID as follows:Encrypt with the selected keyUse the encryption algorithm selected by TMP_KEY_PROGRAM_STRUCT.KEYID_CTRL.ENC_ALG(* The number of bytes used by the next two lines depends on selected encryption algorithm *)DATA_KEY is TMP_RND_DATA_KEYTWEAK_KEY is TMP_RND_TWEAK_KEYUpdate TME-MK table for TMP_KEY_PROGRAM_STRUCT.KEYID as follows:Encrypt (or not) using the current configuration for TMEThe specified encryption algorithm and key values are not used.BREAK;3 (KEYID_NO_ENCRYPT):Update TME-MK table for TMP_KEY_PROGRAM_STRUCT.KEYID as follows:Do not encryptThe specified encryption algorithm and key values are not used.BREAK;ESAC;Release lock on platform key table;ESAC;RAX := 0;RFLAGS.ZF := 0;EXIT:RFLAGS.CF := 0;RFLAGS.PF := 0;RFLAGS.AF := 0;RFLAGS.OF := 0;RFLAGS.SF := 0;
```
