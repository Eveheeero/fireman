# LDTILECFG

Load Tile Configuration

The LDTILECFG instruction takes an operand containing a pointer to a 64-byte memory location containing the description of the tiles to be supported.
In order to configure the tiles, the AMX-TILE bit in CPUID must be set and the operating system has to have enabled the tiles architecture.The memory area contains the palette and describes how many tiles are being used and defines each tile in terms of rows and column bytes.
Requests must be compatible with the restrictions provided by CPUID; see Table 3-10 below.
Table 3-10.
Memory Area LayoutByte(s)Field NameDescription0palettePalette selects the supported configuration of the tiles that will be used.1start_rowstart_row is used for storing the restart values for interrupted operations.2-15reserved, must be zero16-17tile0.colsbTile 0 bytes per row.18-19tile1.colsbTile 1 bytes per row.20-21tile2.colsbTile 2 bytes per row....(sequence continues)30-31tile7.colsbTile 7 bytes per row.32-47reserved, must be zero48tile0.rowsTile 0 rows.49tile1.rowsTile 1 rows.50tile2.rowsTile 2 rows....(sequence continues)55tile7.rowsTile 7 rows.56-63reserved, must be zeroIf a tile row and column pair is not used to specify tile parameters, they must have the value zero.
All enabled tiles (based on the palette) must be configured.
Specifying tile parameters for more tiles than the implementation limit or the palette limit results in a #GP fault.
If the palette_id is zero, that signifies the INIT state for both TILECFG and TILEDATA.
Tiles are zeroed in the INIT 

## Flags affected

- None.

## Operation

```C
LDTILECFG memerror :=Falsebuf := read_memory(mem, 64) temp_tilecfg.palette_id := buf.byte[0]if temp_tilecfg.palette_id > max_palette: error := Trueif not xcr0_supports_palette(temp_tilecfg.palette_id): error := Trueif temp_tilecfg.palette_id !=0: temp_tilecfg.start_row := buf.byte[1] if buf.byte[2..15] is nonzero:error := Truep := 16# configure columnsfor n in 0 ... palette_table[temp_tilecfg.palette_id].max_names-1:temp_tilecfg.t[n].colsb:= buf.word[p/2]p := p + 2if temp_tilecfg.t[n].colsb > palette_table[temp_tilecfg.palette_id].bytes_per_row:error := Trueif nonzero(buf[p...47]):error := True# configure rowsp := 48for n in 0 ... palette_table[temp_tilecfg.palette_id].max_names-1:temp_tilecfg.t[n].rows:= buf.byte[p]if temp_tilecfg.t[n].rows > palette_table[temp_tilecfg.palette_id].max_rows:error := Truep := p + 1if nonzero(buf[p...63]):error := True# validate each tile's row & col configs are reasonable and enable the valid tilesfor n in 0 ... palette_table[temp_tilecfg.palette_id].max_names-1:if temp_tilecfg.t[n].rows !=0 and temp_tilecfg.t[n].colsb != 0:temp_tilecfg.t[n].valid := 1elif temp_tilecfg.t[n].rows == 0 and temp_tilecfg.t[n].colsb == 0:temp_tilecfg.t[n].valid := 0else:error := True// one of rows or colsbwas 0 but not both.if error:#GPelif temp_tilecfg.palette_id == 0:TILES_CONFIGURED := 0// init statetilecfg := 0// equivalent to 64B of zeroszero_all_tile_data()else:tilecfg := temp_tilecfgIntel C/C++ Compiler Intrinsic EquivalentLDTILECFG void _tile_loadconfig(const void *);
```
