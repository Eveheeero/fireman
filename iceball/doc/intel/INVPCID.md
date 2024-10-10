# INVPCID

Invalidate Process-Context Identifier

Invalidates mappings in the translation lookaside buffers (TLBs) and paging-structure caches based on process-context identifier (PCID).
(See Section 4.10, "Caching Translation Information," in the Intel64 and IA-32 Architec-ture Software Developer's Manual, Volume 3A.) Invalidation is based on the INVPCID type specified in the register operand and the INVPCID descriptor specified in the memory operand.Outside 64-bit mode, the register operand is always 32 bits, regardless of the value of CS.D.
In 64-bit mode the register operand has 64 bits.There are four INVPCID types currently defined: - Individual-address invalidation: If the INVPCID type is 0, the logical processor invalidates mappings-except 1 In some cases, the global translations-for the linear address and PCID specified in the INVPCID descriptor.instruction may invalidate global translations or mappings for other linear addresses (or other PCIDs) as well.
- Single-context invalidation: If the INVPCID type is 1, the logical processor invalidates all mappings-except global translations-associated with the PCID specified in the INVPCID descriptor.
In some cases, the instruction may invalidate global translations or mappings for other PCIDs as well.
- All-context invalidation, including global translations: If the INVPCID type is 2, the logical processor invalidates all mappings-including global translations-associated with any PCID.
 - All-context invalidation: If the INVPCID type is 3, the logical processor invalidates all mappings-except global translations-associated with any PCID.
In some case, the instruction may invalidate global translations as well.
The INVPCID descriptor comprises 128 bits and consists of a PCID and a linear address as shown in Figure 3-25.
For INVPCID type 0, the processor uses the full 64 bits of the linear address even outside 64-bit mode; the linear address is not used for other INVPCID types.127646301211Linear AddressPCIDReserved (must be zero)Figure 3-25.
 INVPCID Descriptor

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If the current privilege level is not 0.
  > If the memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register contains an unusable segment.
  > If the source operand is located in an execute-only code segment.
  > If an invalid type is specified in the register operand, i.e., INVPCID_TYPE > 3.
  > If bits 63:12 of INVPCID_DESC are not all zero.
  > If INVPCID_TYPE is either 0 or 1 and INVPCID_DESC[11:0] is not zero.
  > If INVPCID_TYPE is 0 and the linear addre
  > ss in INVPCID_DESC[127:64] is not canonical.
  - #PF(fault-code) - If a page fault occurs
  >  in accessing the memory operand.
  - #SS(0) - If the memory operand effective address is outside the SS segment limit.
  > If the SS register contains an unusable segment.
  - #UD - If if CPUID.(EAX=07H, ECX=
- Virtual-8086 Mode Exceptions
  - #GP(0) - The INVPCID instruction is not recognized in virtual-8086 mode.
- SIMD Floating-Point Exceptions
  > None.
- 64-Bit Mode Exceptions
  - #GP(0) - If the current privilege level is not 0.
  > If the memory operand is in the CS, DS, ES, FS
  > , or GS segments and the memory address is 
  > in a non-canonical form.
  > If an invalid type is specified in the 
  > register operand, i.e., INVPCID_TYPE > 3.
  > If bits 63:12 of INVPCID_DESC are not all zero.
  > If CR4.PCIDE=0, INVPCID_TYPE is either 0 
  > or 1, and INVPCID_DESC[11:0] is not zero.
  > If INVPCID_TYPE is 0 and the linear addres
  > s in INVPCID_DESC[127:64] is not canonical.
  - #PF(fault-code) - If a page fault occurs
  >  in accessing the memory operand.
  - #SS(0) - If the memory destination operand is in th
  > e SS segment and the memory address is in a non-
  > canonical form.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  - #GP - If an invalid type is specified in the register operand, i.e., INVPCID_TYPE > 3.
  > If bits 63:12 of INVPCID_DESC are not all zero.
  > If INVPCID_TYPE is either 0 or 1 and INVPCID_DESC[11:0] is not zero.
  > If INVPCID_TYPE is 0 and the linear addres
  > s in INVPCID_DESC[127:64] is not canonical.
  - #UD - If CPUID.(EAX=07H, ECX=0H
  > ):EBX.INVPCID (bit 10) = 0.
  > If the LOCK prefix is used.

## Operation

```C
INVPCID_TYPE := value of register operand; // must be in the range of 0-3INVPCID_DESC := value of memory operand;CASE INVPCID_TYPE OF0:// individual-address invalidationPCID := INVPCID_DESC[11:0];L_ADDR := INVPCID_DESC[127:64];Invalidate mappings for L_ADDR associated with PCID except global translations;BREAK;1:// single PCID invalidationPCID := INVPCID_DESC[11:0];Invalidate all mappings associated with PCID except global translations;BREAK;2:// all PCID invalidation including global translationsInvalidate all mappings for all PCIDs, including global translations;BREAK;3:// all PCID invalidation retaining global translationsInvalidate all mappings for all PCIDs except global translations;BREAK;ESAC;Intel C/C++ Compiler Intrinsic EquivalentINVPCID void _invpcid(unsigned __int32 type, void * descriptor);
```
