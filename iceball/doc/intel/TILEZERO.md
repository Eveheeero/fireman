# TILEZERO

Zero Tile

This instruction zeroes the destination tile.Any attempt to execute the TILEZERO instruction inside an Intel TSX transaction will result in a transaction abort.

## Flags affected

- None.

## Operation

```C
TILEZERO tdestnbytes := palette_table[palette_id].bytes_per_row for i in 0 ... palette_table[palette_id].max_rows-1:for j in 0 ... nbytes-1: tdest.row[i].byte[j] := 0zero_tilecfg_start()Intel C/C++ Compiler Intrinsic EquivalentTILEZERO void _tile_zero(__tile dst);
```
