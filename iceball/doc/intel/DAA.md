# DAA

Decimal Adjust AL After Addition

Adjusts the sum of two packed BCD values to create a packed BCD result.
The AL register is the implied source and destination operand.
The DAA instruction is only useful when it follows an ADD instruction that adds (binary addi-tion) two 2-digit, packed BCD values and stores a byte result in the AL register.
The DAA instruction then adjusts the contents of the AL register to contain the correct 2-digit, packed BCD result.
If a decimal carry is detected, the CF and AF flags are set accordingly.This instruction executes as described above in compatibility mode and legacy mode.
It is not valid in 64-bit mode.

## Flags affected

- The CF and AF flags are set if the adjustment of the value results in a decimal carry in either digit of the result (see the "Operation" section above). The SF, ZF, and PF flags are set according to the result. The OF flag is undefined.

## Exceptions

- Compatibility Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.

## Operation

```C
IF 64-Bit ModeTHEN#UD;ELSEold_AL := AL;old_CF := CF;CF := 0;=  9) or AF 1)IF (((AL AND 0FH) > THEN+ AL := AL  6;+ CF := old_CF or (Carry from AL := AL  6); AF := 1;  ELSE  AF := 0;FI;>= 99H) or (old_CF  1))IF ((old_AL  THEN + AL := AL 60H;  CF := 1;ELSE CF := 0;FI;FI;ExampleADD AL, BL Before: AL=79H BL=35H EFLAGS(OSZAPC)=XXXXXXAfter: AL=AEH BL=35H EFLAGS(0SZAPC)=110000DAABefore: AL=AEH BL=35H EFLAGS(OSZAPC)=110000After: AL=14H BL=35H EFLAGS(0SZAPC)=X00111DAABefore: AL=2EH BL=35H EFLAGS(OSZAPC)=110000After: AL=34H BL=35H EFLAGS(0SZAPC)
```
