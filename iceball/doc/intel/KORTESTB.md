# KORTESTW/KORTESTB/KORTESTQ/KORTESTD

OR Masks and Set Flags

Performs a bitwise OR between the vector mask register k2, and the vector mask register k1, and sets CF and ZF based on the operation result.
ZF flag is set if both sources are 0x0.
CF is set if, after the OR operation is done, the operation result is all 1's.

## Flags affected

- The ZF flag is set if the result of OR-ing both sources is all 0s.The CF flag is set if the result of OR-ing both sources is all 1s.The OF, SF, AF, and PF flags are set to 0.

## Exceptions

- Other Exceptions

## Operation

```C
KORTESTW TMP[15:0] := DEST[15:0] BITWISE OR SRC[15:0]IF(TMP[15:0]=0)THEN ZF := 1ELSE ZF := 0FI;IF(TMP[15:0]=FFFFh)THEN CF := 1ELSE CF := 0FI;KORTESTB TMP[7:0] := DEST[7:0] BITWISE OR SRC[7:0]IF(TMP[7:0]=0)THEN ZF := 1ELSE ZF := 0FI;IF(TMP[7:0]==FFh)THEN CF := 1ELSE CF := KORTESTQ TMP[63:0] := DEST[63:0] BITWISE OR SRC[63:0]IF(TMP[63:0]=0)THEN ZF := 1ELSE ZF := 0FI;IF(TMP[63:0]==FFFFFFFF_FFFFFFFFh)THEN CF := 1ELSE CF := 0FI;KORTESTD TMP[31:0] := DEST[31:0] BITWISE OR SRC[31:0]IF(TMP[31:0]=0)THEN ZF := 1ELSE ZF := 0FI;IF(TMP[31:0]=FFFFFFFFh)THEN CF := 1ELSE CF := 0FI;Intel C/C++ Compiler Intrinsic EquivalentKORTESTW __mmask16 _mm512_kortest[cz](__mmask16 a, __mmask16 b);
```
