# TDPBF16PS

Dot Product of BF16 Tiles Accumulated into Packed Single Precision Tile

This instruction performs a set of SIMD dot-products of two BF16 elements and accumulates the results into a packed single precision tile.
Each dword element in input tiles tmm2 and tmm3 is interpreted as a BF16 pair.
For each possible combination of (row of tmm2, column of tmm3), the instruction performs a set of SIMD dot-products on all corresponding BF16 pairs (one pair from tmm2 and one pair from tmm3), adds the results of those dot-prod-ucts, and then accumulates the result into the corresponding row and column of tmm1."Round to nearest even" rounding mode is used when doing each accumulation of the FMA.
Output denormals are always flushed to zero and input denormals are always treated as zero.
MXCSR is not consulted nor updated.
Any attempt to execute the TDPBF16PS instruction inside a TSX transaction will result in a transaction abort.

## Flags affected

- None.

## Operation

```C
define make_fp32(x):// The x parameter is bfloat16. Pack it in to upper 16b of a dword.// The bit pattern is a legal fp32 value. Return that bit pattern. dword: = 0dword[31:16] := x return dwordTDPBF16PS tsrcdest, tsrc1, tsrc2// C = m x n (tsrcdest), A = m x k (tsrc1), B = k x n (tsrc2)# src1 and src2 elements are pairs of bfloat16elements_src1 := tsrc1.colsb / 4elements_src2 := tsrc2.colsb / 4elements_dest := tsrcdest.colsb / 4elements_temp := tsrcdest.colsb / 2// Count is in bfloat16 prior to horizontalfor m in 0 ... tsrcdest.rows-1:temp1[ 0 ... elements_temp-1 ] := 0for k in 0 ... elements_src1-1:for n in 0 ... elements_dest-1:// FP32 FMA with DAZ=FTZ=1, RNE rounding.// MXCSR is neither consulted nor updated.// No exceptions raised or denoted.for n in 0 ... elements_dest-1:// DAZ=FTZ=1, RNE rounding.// MXCSR is neither consulted nor updated.// No exceptions raised or denoted.tmpf32 := temp1.fp32[2*n] + temp1.fp32[2*n+1]tsrcdest.row[m].fp32[n] := tsrcdest.row[m].fp32[n] + tmpf32write_row_and_zero(tsrcdest, m, tmp, tsrcdest.colsb)zero_upper_rows(tsrcdest, tsrcdest.rows)zero_tilecfg_start()Intel C/C++ Compiler Intrinsic EquivalentTDPBF16PS void _tile_dpbf16ps(__tile dst, __tile src1, __tile src2);
```
