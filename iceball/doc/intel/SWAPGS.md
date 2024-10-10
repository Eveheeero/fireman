# SWAPGS

Swap GS Base Register

SWAPGS exchanges the current GS base register value with the value contained in MSR address C0000102H (IA32_KERNEL_GS_BASE).
The SWAPGS instruction is a privileged instruction intended for use by system soft-ware.
When using SYSCALL to implement system calls, there is no kernel stack at the OS entry point.
Neither is there a straightforward method to obtain a pointer to kernel structures from which the kernel stack pointer could be read.
Thus, the kernel cannot save general purpose registers or reference memory.
By design, SWAPGS does not require any general purpose registers or memory operands.
No registers need to be saved before using the instruction.
SWAPGS exchanges the CPL 0 data pointer from the IA32_KERNEL_GS_BASE MSR with the GS base register.
The kernel can then use the GS prefix on normal memory references to access kernel data structures.
Similarly, when the OS kernel is entered using an interrupt or exception (where the kernel stack is already set up), SWAPGS can be used to quickly get a pointer to the kernel data structures.The IA32_KERNEL_GS_BASE MSR itself is only accessible using RDMSR/WRMSR instructions.
Those instructions are only accessible at privilege level 0.
The WRMSR instruction ensures that the IA32_KERNEL_GS_BASE MSR contains a canonical address.

## Flags affected

- None.

## Exceptions

- Compatibility Mode Exceptions
  > 
  >  
  - #UD - If Mode
  > 64-Bit.
- Real-Address Mode Exceptions
  > 
  >  
  - #UD - If Mode
  > 64-Bit.
- Protected Mode Exceptions
  > 
  >  
  - #UD - If Mode
  > 64-Bit.
- Virtual-8086 Mode Exceptions
  > 
  >  
- 64-Bit Mode Exceptions
  > 
  >  
  - #GP(0) - If CPL

## Operation

```C
 IF CS.L 1 (* Not in 64-Bit Mode *)THEN#UD; FI;IF CPL  0THEN #GP(0); FI;tmp := GS.base;GS.base := IA32_KERNEL_GS_BASE;IA32_KERNEL_GS_BASE := tmp;
```
