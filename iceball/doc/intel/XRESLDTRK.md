# XRESLDTRK

Resume Tracking Load Addresses

The instruction marks the end of an Intel TSX (RTM) suspend load address tracking region.
If the instruction is used inside a suspend load address tracking region it will end the suspend region and all following load addresses will be added to the transaction read set.
If this instruction is used inside an active transaction but not in a suspend region it will cause transaction abort.If the instruction is used outside of a transactional region it behaves like a NOP.Chapter 16, "Programming with Intel® Transactional Synchronization Extensions" in the Intel® 64 and IA-32 ® TSX Suspend Load Architectures Software Developer's Manual, Volume 1 provides additional information on IntelAddress Tracking.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentXRESLDTRK void _xresldtrk(void);

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
XRESLDTRKIF RTM_ACTIVE = 1:IF SUSLDTRK_ACTIVE = 1:SUSLDTRK_ACTIVE := 0ELSE:RTM_ABORTELSE:NOP
```
