# XSUSLDTRK

Suspend Tracking Load Addresses

The instruction marks the start of an Intel TSX (RTM) suspend load address tracking region.
If the instruction is used inside a transactional region, subsequent loads are not added to the read set of the transaction.
If the instruc-tion is used inside a suspend load address tracking region it will cause transaction abort.If the instruction is used outside of a transactional region it behaves like a NOP.Chapter 16, "Programming with Intel® Transactional Synchronization Extensions" in the Intel® 64 and IA-32 ®Architectures Software Developer's Manual, Volume 1 provides additional information on Intel TSX Suspend Load Address Tracking.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentXSUSLDTRK void _xsusldtrk(void);

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
XSUSLDTRKIF RTM_ACTIVE = 1:IF SUSLDTRK_ACTIVE = 0:SUSLDTRK_ACTIVE := 1ELSE:RTM_ABORTELSE:NOP
```
