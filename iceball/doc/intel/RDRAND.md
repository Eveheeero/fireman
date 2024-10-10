# RDRAND

Read Random Number

Loads a hardware generated random value and store it in the destination register.
The size of the random value is determined by the destination register size and operating mode.
The Carry Flag indicates whether a random value is available at the time the instruction is executed.
CF=1 indicates that the data in the destination is valid.
Other-wise CF=0 and the data in the destination operand will be returned as zeros for the specified width.
All other flags are forced to 0 in either situation.
Software must check the state of CF=1 for determining if a valid random value ® 64 and IA-32 Archi-has been returned, otherwise it is expected to loop and retry execution of RDRAND (see Inteltectures Software Developer's Manual, Volume 1, Section 7.3.17, "Random Number Generator Instructions").This instruction is available at all privilege levels.In 64-bit mode, the instruction's default operand size is 32 bits.
Using a REX prefix in the form of REX.B permits access to additional registers (R8-R15).
Using a REX prefix in the form of REX.W promotes operation to 64 bit oper-ands.
See the summary chart at the beginning of this section for encoding data and limits.

## Flags affected

- The CF flag is set according to the result (see the "OpeIntel C/C++ Compiler Intrinsic EquivalentRDRAND int _rdrand16_step( unsigned short * );RDRAND int _rdrand32_step( unsigned int * );RDRAND int _rdrand64_step( unsigned __int64 *);

## Exceptions

- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CPUID.01H:ECX.RDRAND[bit 30] = 0.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
IF HW_RND_GEN.ready = 1THEN CASE ofoperand size is 64: DEST[63:0] := HW_RND_GEN.data;operand size is 32: DEST[31:0] := HW_RND_GEN.data;operand size is 16: DEST[15:0] := HW_RND_GEN.data;ESACCF := 1;ELSECASE ofoperand size is 64: DEST[63:0] := 0;operand size is 32: DEST[31:0] := 0;operand size is 16: DEST[15:0] := 0;ESACCF := 0;FIOF, SF, ZF, AF, PF := 0;
```
