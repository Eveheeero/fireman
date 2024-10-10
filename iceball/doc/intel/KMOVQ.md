# KMOVW/KMOVB/KMOVQ/KMOVD

Move From and to Mask Registers

InstructionMode Feature FlagSupportVEX.L0.0F.W0 90 /rRMV/VAVX512FMove 16 bits mask from k2/m16 and store the result in k1.KMOVW k1, k2/m16VEX.L0.66.0F.W0 90 /r RMV/VAVX512DQMove 8 bits mask from k2/m8 and store the result in k1.
KMOVB k1, k2/m8VEX.L0.0F.W1 90 /r RMV/VAVX512BWMove 64 bits mask from k2/m64 and store the result in k1.KMOVQ k1, k2/m64VEX.L0.66.0F.W1 90 /r RMV/VAVX512BWMove 32 bits mask from k2/m32 and store the result in k1.
KMOVD k1, k2/m32VEX.L0.0F.W0 91 /r MRV/VAVX512FMove 16 bits mask from k1 and store the result in m16.KMOVW m16, k1VEX.L0.66.0F.W0 91 /r MRV/VAVX512DQMove 8 bits mask from k1 and store the result in m8.
KMOVB m8, k1VEX.L0.0F.W1 91 /r MRV/VAVX512BWMove 64 bits mask from k1 and store the result in m64.KMOVQ m64, k1VEX.L0.66.0F.W1 91 /r MRV/VAVX512BWMove 32 bits mask from k1 and store the result in m32.KMOVD m32, k1VEX.L0.0F.W0 92 /r RRV/VAVX512FMove 16 bits mask from r32 to k1.KMOVW k1, r32VEX.L0.66.0F.W0 92 /r RRV/VAVX512DQMove 8 bits mask from r32 to k1.KMOVB k1, r32VEX.L0.F2.0F.W1 92 /r RRV/IAVX512BWMove 64 bits mask from r64 to k1.KMOVQ k1, r64VEX.L0.F2.0F.W0 92 /r RRV/VAVX512BWMove 32 bits mask from r32 to k1.KMOVD k1, r32VEX.L0.0F.W0 93 /r RRV/VAVX512FMove 16 bits mask from k1 to r32.KMOVW r32, k1VEX.L0.66.0F.W0 93 /r RRV/VAVX512DQMove 8 bits mask from k1 to r32.KMOVB r32, k1VEX.L0.F2.0F.W1 93 /r RRV/IAVX512BWMove 64 bits mask from k1 to r64.KMOVQ r64, k1VEX.L0.F2.0F.W0 93 /r RRV/VAVX512BWMove 32 bits mask from k1 to r32.KMOVD r32, k1Instruction Operand EncodingOp/EnOperand 1Operand 2RMModRM:reg (w)ModRM:r/m (r)MRModRM:r/m (w, ModRM:[7:6] must not be 11b)ModRM:reg (r)RRModRM:reg (w)ModRM:r/m (r, ModRM:[7:6] must be 11b) Copies values from the source operand (second operand) to the destination operand (first operand).
The source and destination operands can be mask registers, memory location or general purpose.
The instruction cannot be When moving to a mask register, the result is zero extended to MAX_KL size (i.e., 64 bits currently).
When moving to a general-purpose register (GPR), the result is zero-extended to the size of the destination.
In 32-bit mode, the default GPR destination's size is 32 bits.
In 64-bit mode, the default GPR destination's size is 64 bits.
Note that VEX.W can only be used to modify the size of the GPR operand in 64b mode.

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > Instructions with RR operand encoding, see Table2-63, 
  > "TYPE K20 Exception Definiti
  > on (VEX-Encoded OpMask 
  > Instructions w/o Memory Arg)."
  > Instructions with RM or MR operand encoding, see Ta

## Operation

```C
KMOVWIF *destination is a memory location*DEST[15:0] := SRC[15:0]IF *destination is a mask register or a GPR *DEST := ZeroExtension(SRC[15:0])KMOVBIF *destination is a memory location*DEST[7:0] := SRC[7:0]IF *destination is a mask register or a GPR *DEST := ZeroExtension(SRC[7:0])KMOVQIF *destination is a memory location or a GPR*DEST[63:0] := SRC[63:0]IF *destination is a mask register*DEST := ZeroExtension(SRC[63:0])KMOVDIF *destination is a memory location*DEST[31:0] := SRC[31:0]IF *destination is a mask register or a GPR *DEST := ZeroExtension(SRC[31:0])Intel C/C++ Compiler Intrinsic EquivalentKMOVW __mmask16 _mm512_kmov(__mmask16 a);
```
