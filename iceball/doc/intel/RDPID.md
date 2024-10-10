# RDPID

Read Processor ID

Reads the value of the IA32_TSC_AUX MSR (address C0000103H) into the destination register.
The value of CS.D and operand-size prefixes (66H and REX.W) do not affect the behavior of the RDPID instruction.

## Flags affected

- None.

## Exceptions

- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CPUID.7H.0:ECX.RDPID[bit 22] = 0.
- 64-Bit Mode Exceptions
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
DEST := IA32_TSC_AUX 
```
