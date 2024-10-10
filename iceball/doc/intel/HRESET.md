# HRESET

History Reset

Requests the processor to selectively reset selected components of hardware history maintained by the current logical processor.
HRESET operation is controlled by the implicit EAX operand.
The value of the explicit imm8 operand is ignored.
This instruction can only be executed at privilege level 0.The HRESET instruction can be used to request reset of multiple components of hardware history.
Prior to the execution of HRESET, the system software must take the following steps:

## Flags affected

- None.

## Exceptions

- Virtual-8086 Mode Exceptions
  - #GP(0) - HRESET instruction is not
  > recognized in virtual-8086 mode.
- Protected Mode Exceptions
  > B
  - #GP(0) - If CPL > 0 or (EAX
  > AND NOT IA32_HRESET_ENABLE) 
  > 0.
  - #UD - If CPUID.07H.01H:EAX.HRESET[bit 22] = 0.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
IF EAX = 0    THEN NOP    ELSE         FOREACH i such that EAX[i] = 1             Reset prediction history for feature iFI
```
