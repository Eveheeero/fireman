# RDSEED

Read Random SEED

Loads a hardware generated random value and store it in the destination register.
The random value is generated from an Enhanced NRBG (Non Deterministic Random Bit Generator) that is compliant to NIST SP800-90B and NIST SP800-90C in the XOR construction mode.
The size of the random value is determined by the destination register size and operating mode.
The Carry Flag indicates whether a random value is available at the time the instruction is executed.
CF=1 indicates that the data in the destination is valid.
Otherwise CF=0 and the data in the destination operand will be returned as zeros for the specified width.
All other flags are forced to 0 in either situation.
Software must check the state of CF=1 for determining if a valid random seed value has been returned, otherwise it is expected to loop and retry execution of RDSEED (see Section 1.2).The RDSEED instruction is available at all privilege levels.
The RDSEED instruction executes normally either inside or outside a transaction region.In 64-bit mode, the instruction's default operand size is 32 bits.
Using a REX prefix in the form of REX.B permits access to additional registers (R8-R15).
Using a REX prefix in the form of REX.W promotes operation to 64 bit oper-ands.
See the summary chart at the beginning of this section for encoding data and limits.

## Flags affected

- The CF flag is set according to the result (see the "Operation" section above). The OF, SF, ZF, AF, and PF flags are set to 0.C/C++ Compiler Intrinsic EquivalentRDSEED int _rdseed16_step( unsigned short * );RDSEED int _rdseed32_step( unsigned int * );RDSEED int _rdseed64_step( unsigned __int64 *);

## Exceptions

- Compatibility Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CPUID.(EAX=07H, ECX=0H
  > ):EBX.RDSEED[bit 18] = 0.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CPUID.(EAX=07H, ECX=0H
  > ):EBX.RDSEED[bit 18] = 0.
- Virtual-8086 Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CPUID.(EAX=07H, ECX=0H
  > ):EBX.RDSEED[bit 18] = 0.
- 64-Bit Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CPUID.(EAX=07H, ECX=0H
  > ):EBX.RDSEED[bit 18] = 0.

## Operation

```C
IF HW_NRND_GEN.ready = 1THEN CASE ofoperand size is 64: DEST[63:0] := HW_NRND_GEN.data;operand size is 32: DEST[31:0] := HW_NRND_GEN.data;operand size is 16: DEST[15:0] := HW_NRND_GEN.data;ESAC;CF := 1;ELSECASE ofoperand size is 64: DEST[63:0] := 0;operand size is 32: DEST[31:0] := 0;operand size is 16: DEST[15:0] := 0;ESAC;CF := 0;
```
