# BNDSTX

Store Extended Bounds Using Address Translation

InstructionMode Feature SupportFlagNP 0F 1B /rMRV/VMPXStore the bounds in bnd and the pointer value in the index register BNDSTX mib, bndof mib to a bound table entry (BTE) with address translation using the base of mib.Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3SIB.base (r): Address of pointerModRM:reg (r)N/AMRSIB.index(r)BNDSTX uses the linear address constructed from the displacement and base register of the SIB-addressing form of the memory operand (mib) to perform address translation to store to a bound table entry.
The bounds in the source operand bnd are written to the lower and upper bounds in the BTE.
The content of the index register of mib is written to the pointer value field in the BTE.
This instruction does not cause memory access to the linear address of mib nor the effective address referenced by the base, and does not read or write any flags.
Segment overrides apply to the linear address computation with the base of mib, and are used during address translation to generate the address of the bound table entry.
By default, the address of the BTE is assumed to be linear address.
There are no segmentation checks performed on the base of mib.
The base of mib will not be checked for canonical address violation as it does not access memory.
Any encoding of this instruction that does not specify base or index register will treat those registers as zero (constant).
The reg-reg form of this instruction will remain a NOP.The scale field of the SIB byte has no effect on these instructions and is ignored.The bound register may be partially updated on memory faults.
The order in which memory operands are loaded is implementation specific.

## Flags affected

- None.

## Exceptions

- 64-Bit Mode Exceptions
  - #BR - If the bound directory entry is invalid.
  - #UD - If ModRM is RIP relative.
  > If the LOCK prefix is used.
  > If ModRM.r/m and REX encodes BND4-B
  > ND15 when Intel MPX is enabled.
  - #GP(0) - If the memory address (A_BDE or A_BTE) is in a non-canonical form.
- Protected Mode Exceptions
  - #BR - If the bound directory entry is invalid.
  - #UD - If the LOCK prefix is used.
  > If ModRM.r/m encodes BND4-BND7 when Intel MPX is enabled.
  > If 67H prefix is not used and CS.D=0.
  > If 67H prefix is used and CS.D=1.
  - #GP(0) - If a destination effective address of the Bo
  > und Table entry is outside the DS segment limit.
  > If DS register contains a NULL segment selector.
  > If the destination operand points to a non-writable segment
  - #PF(fault code) - If a page fault occurs.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If ModRM.r/m encodes BND4-BND7 when Intel MPX is enabled.
  > If 16-bit addressing is used.
  - #GP(0) - If a destination effective address of the Bo
  > und Table entry is outside the DS segment limit.
- Virtual-8086 Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If ModRM.r/m encodes BND4-BND7 when Intel MPX is enabled.
  > If 16-bit addressing is used.
  - #GP(0) - If a destination effective address of the Bo
  > und Table entry is outside the DS segment limit.
  - #PF(fault code) - If a page fault occurs.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
base := mib.SIB.base ? mib.SIB.base + Disp: 0;ptr_value := mib.SIB.index ? mib.SIB.index : 0;Outside 64-bit ModeA_BDE[31:0] := (Zero_extend32(base[31:12] « 2) + (BNDCFG[31:12] «12 );A_BT[31:0] := LoadFrom(A_BDE);IF A_BT[0] equal 0 ThenBNDSTATUS := A_BDE | 02H; #BR; FI;A_DEST[31:0] := (Zero_extend32(base[11:2] « 4) + (A_BT[31:2] « 2 ); // address of Bound table entryA_DEST[8][31:0] := ptr_value; A_DEST[0][31:0] := BND.LB; In 64-bit Mode1A_BDE[63:0] := (Zero_extend64(base[47+MAWA:20] « 3) + (BNDCFG[63:12] «12 );A_BT[63:0] := LoadFrom(A_BDE);IF A_BT[0] equal 0 ThenBNDSTATUS := A_BDE | 02H; #BR; FI;A_DEST[63:0] := (Zero_extend64(base[19:3] « 5) + (A_BT[63:3] « 3 ); // address of Bound table entryA_DEST[16][63:0] := ptr_value; A_DEST[0][63:0] := BND.LB; A_DEST[8][63:0] := BND.UB; Intel C/C++ Compiler Intrinsic EquivalentBNDSTX: _bnd_store_ptr_bounds(const void **ptr_addr, const void *ptr_val); 
```
