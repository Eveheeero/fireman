# STTILECFG

Store Tile Configuration

The STTILECFG instruction takes a pointer to a 64-byte memory location (described in Table 3-10 in the "LDTI-LECFG-Load Tile Configuration" entry) that will, after successful execution of this instruction, contain the descrip-tion of the tiles that were configured.
In order to configure tiles, the AMX-TILE bit in CPUID must be set and the operating system has to have enabled the tiles architecture.If the tiles are not configured, then STTILECFG stores 64B of zeros to the indicated memory location.
Any attempt to execute the STTILECFG instruction inside an Intel TSX transaction will result in a transaction abort.

## Operation

```C
STTILECFG memif TILES_CONFIGURED == 0://write 64 bytes of zeros at mem pointer buf[0..63] := 0write_memory(mem, 64, buf) else:buf.byte[0] := tilecfg.palette_id buf.byte[1] := tilecfg.start_row buf.byte[2..15] := 0p := 16for n in 0 ... palette_table[tilecfg.palette_id].max_names-1: buf.word[p/2] := tilecfg.t[n].colsbp := p + 2 if p < 47:buf.byte[p..47] := 0p := 48for n in 0 ... palette_table[tilecfg.palette_id].max_names-1: buf.byte[p++] := tilecfg.t[n].rowsif p < 63:buf.byte[p..63] := 0write_memory(mem, 64, buf)Intel C/C++ Compiler Intrinsic EquivalentSTTILECFGvoid _tile_storeconfig(void *);
```
