# TDPBSSD/TDPBSUD/TDPBUSD/TDPBUUD

Dot Product of Signed/Unsigned Bytes with Dword Accumulation

For each possible combination of (row of tmm2, column of tmm3), the instruction performs a set of SIMD dot-prod-ucts on all corresponding four byte elements, one from tmm2 and one from tmm3, adds the results of those dot-products, and then accumulates the result into the corresponding row and column of tmm1.
Each dword in input tiles tmm2 and tmm3 is interpreted as four byte elements.
These may be signed or unsigned.
Each letter in the two-letter pattern SU, US, SS, UU indicates the signed/unsigned nature of the values in tmm2 and tmm3, respec-tively.Any attempt to execute the TDPBSSD/TDPBSUD/TDPBUSD/TDPBUUD instructions inside an Intel TSX transaction will result in a transaction abort.

## Flags affected

- None.

## Operation

```C
define DPBD(c,x,y):// arguments are dwords if *x operand is signed*:extend_src1 := SIGN_EXTEND else:extend_src1 := ZERO_EXTENDif *y operand is signed*: extend_src2 := SIGN_EXTENDelse:extend_src2 := ZERO_EXTENDp0dword := extend_src1(x.byte[0]) * extend_src2(y.byte[0]) p1dword := extend_src1(x.byte[1]) * extend_src2(y.byte[1]) p2dword := extend_src1(x.byte[2]) * extend_src2(y.byte[2]) TDPBSSD, TDPBSUD, TDPBUSD, TDPBUUD tsrcdest, tsrc1, tsrc2 (Register Only Version)// C = m x n (tsrcdest), A = m x k (tsrc1), B = k x n (tsrc2)tsrc1_elements_per_row := tsrc1.colsb / 4 tsrc2_elements_per_row := tsrc2.colsb / 4 tsrcdest_elements_per_row := tsrcdest.colsb / 4for m in 0 ... tsrcdest.rows-1: tmp := tsrcdest.row[m]for k in 0 ... tsrc1_elements_per_row-1:for n in 0 ... tsrcdest_elements_per_row-1: DPBD( tmp.dword[n], tsrc1.row[m].dword[k], tsrc2.row[k].dword[n] ) write_row_and_zero(tsrcdest, m, tmp, tsrcdest.colsb)zero_upper_rows(tsrcdest, tsrcdest.rows) zero_tilecfg_start()Intel C/C++ Compiler Intrinsic EquivalentTDPBSSD void _tile_dpbssd(__tile dst, __tile src1, __tile src2);TDPBSUD void _tile_dpbsud(__tile dst, __tile src1, __tile src2);TDPBUSD void _tile_dpbusd(__tile dst, __tile src1, __tile src2);TDPBUUD void _tile_dpbuud(__tile dst, __tile src1, __tile src2);
```
