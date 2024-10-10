# TILESTORED

Store Tile

This instruction is required to use SIB addressing.
The index register serves as a stride indicator.
If the SIB encoding omits an index register, the value zero is assumed for the content of the index register.This instruction stores a tile source of rows and columns as specified by the tile configuration.The TILECFG.start_row in the TILECFG data should be initialized to '0' in order to store the entire tile and are set to zero on successful completion of the TILESTORED instruction.
TILESTORED is a restartable instruction and the TILECFG.start_row will be non-zero when restartable events occur during the instruction execution.Only memory operands are supported and they can only be accessed using a SIB addressing mode, similar to the V[P]GATHER*/V[P]SCATTER* instructions.
Any attempt to execute the TILESTORED instruction inside an Intel TSX transaction will result in a transaction abort.

## Flags affected

- None.

## Operation

```C
TILESTORED tsib, tsrcstart := tilecfg.start_rowmembegin := tsib.base + displacement// if no index register in the SIB encoding, the value zero is used. stride := tsib.index << tsib.scalewhile start < tdest.rows:memptr := membegin + start * stride write_memory(memptr, tsrc.colsb, tsrc.row[start]) start := start + 1zero_tilecfg_start()// In the case of a memory fault in the middle of an instruction, the tilecfg.start_row := startIntel C/C++ Compiler Intrinsic EquivalentTILESTORED void _tile_stored(__tile src, void *base, int stride);
```
