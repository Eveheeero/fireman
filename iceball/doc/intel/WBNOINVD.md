# WBNOINVD

Write Back and Do Not Invalidate Cache

The WBNOINVD instruction writes back all modified cache lines in the processor's internal cache to main memory but does not invalidate (flush) the internal caches.After executing this instruction, the processor does not wait for the external caches to complete their write-back operation before proceeding with instruction execution.
It is the responsibility of hardware to respond to the cache write-back signal.
The amount of time or cycles for WBNOINVD to complete will vary due to size and other factors of different cache hierarchies.
As a consequence, the use of the WBNOINVD instruction can have an impact on logical processor interrupt/event response time.
The WBNOINVD instruction is a privileged instruction.
When the processor is running in protected mode, the CPL of a program or procedure must be 0 to execute this instruction.
This instruction is also a serializing instruction (see ® 64 and IA-32 Architectures Software Developer's Manual, "Serializing Instructions" in Chapter 9 of the IntelVolume 3A).This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Flags affected

- None.

## Exceptions

- Virtual-8086 Mode Exceptions
  - #GP(0) - WBNOINVD cannot be executed at the virtual-8086 mode.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Protected Mode Exceptions
  - #GP(0) - If the current privilege level is not 0.
  - #UD - If the LOCK prefix is used.

## Operation

```C
WriteBack(InternalCaches);Continue; (* Continue execution *)Intel C/C++ Compiler Intrinsic EquivalentWBNOINVD void _wbnoinvd(void);
```
