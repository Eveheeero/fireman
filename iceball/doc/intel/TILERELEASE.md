# TILERELEASE

Release Tile

This instruction returns TILECFG and TILEDATA to the INIT state.Any attempt to execute the TILERELEASE instruction inside an Intel TSX transaction will result in a transaction abort.

## Flags affected

- None.

## Operation

```C
zero_all_tile_data()tilecfg := 0// equivalent to 64B of zeros TILES_CONFIGURED := 0Intel C/C++ Compiler Intrinsic EquivalentTILERELEASE void _tile_release(void);
```
