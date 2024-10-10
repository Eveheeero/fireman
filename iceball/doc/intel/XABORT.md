# XABORT

Transactional Abort

XABORT forces an RTM abort.
Following an RTM abort, the logical processor resumes execution at the fallback address computed through the outermost XBEGIN instruction.
The EAX register is updated to reflect an XABORT instruction caused the abort, and the imm8 argument will be provided in bits 31:24 of EAX.


## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentXABORT void _xabort( unsigned int);

## Exceptions

- Other Exceptions

## Operation

```C
XABORTIF RTM_ACTIVE = 0THEN Treat as NOP;ELSEGOTO RTM_ABORT_PROCESSING;FI;(* For any RTM abort condition encountered during RTM execution *)RTM_ABORT_PROCESSING:Restore architectural register state;Discard memory updates performed in transaction;Update EAX with status and XABORT argument;RTM_NEST_COUNT:= 0;RTM_ACTIVE:= 0;SUSLDTRK_ACTIVE := 0;IF 64-bit ModeTHENRIP:= fallbackRIP;ELSEEIP := fallbackEIP;FI;END
```
