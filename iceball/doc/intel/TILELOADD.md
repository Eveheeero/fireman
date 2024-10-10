# TILELOADD/TILELOADDT1

Load Tile

This instruction is required to use SIB addressing.
The index register serves as a stride indicator.
If the SIB encoding omits an index register, the value zero is assumed for the content of the index register.This instruction loads a tile destination with rows and columns as specified by the tile configuration.
The "T1" version provides a hint to the implementation that the data would be reused but does not need to be resident in the nearest cache levels.The TILECFG.start_row in the TILECFG data should be initialized to '0' in order to load the entire tile and is set to zero on successful completion of the TILELOADD instruction.
TILELOADD is a restartable instruction and the TILECFG.start_row will be non-zero when restartable events occur during the instruction execution.
Only memory operands are supported and they can only be accessed using a SIB addressing mode, similar to the V[P]GATHER*/V[P]SCATTER* instructions.Any attempt to execute the TILELOADD/TILELOADDT1 instructions inside an Intel TSX transaction will result in a transaction abort.

## Flags affected

- None.

## Operation

```C
TILELOADD[,T1] tdest, tsib start := tilecfg.start_rowzero_upper_rows(tdest,start)membegin := tsib.base + displacement// if no index register in the SIB encoding, the value zero is used. stride := tsib.index << tsib.scalenbytes := tdest.colsb while start < tdest.rows:memptr := membegin + start * stride write_row_and_zero(tdest, start, read_memory(memptr, nbytes), nbytes)start := start + 1 zero_tilecfg_start()// In the case of a memory fault in the middle of an instruction, the tilecfg.start_row := startIntel C/C++ Compiler Intrinsic EquivalentTILELOADD void _tile_loadd(__tile dst, const void *base, int stride);
```
