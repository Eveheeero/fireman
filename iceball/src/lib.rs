#[derive(Debug, Clone)]
pub struct Instruction {
    /// aka. opcode
    pub statement: Statement,
    /// aka. mnemnonic
    pub arguments: Option<Arguments>,
}

#[derive(Debug, Clone, Copy)]
pub enum Statement {
    X64(X64Statement),
}

#[derive(Debug, Clone)]
pub enum Arguments {
    Register(u8),
    RegisterRegister { from: u8, to: u8 },
    Memory(u64),
}

/// From intel manual, chapter 5. Instruction Set Summary
///
/// ### register <-> hex transform documentation
/// - <https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=43>
/// - <https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=44>
/// - <https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=45>
///
/// - [Opcode definition](https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=112)
/// - [Instruction definition](https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=115)
#[derive(Debug, Clone, Copy)]
pub enum X64Statement {
    /// # aaa
    ///
    /// ASCII Adjust After Addition
    ///
    /// - aaa - ASCII adjust AL after addition.
    ///
    /// [Document](https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=129)
    ///
    /// Adjusts the sum of two unpacked BCD values to create an unpacked BCD result. The AL register is the implied
    /// source and destination operand for this instruction. The AAA instruction is only useful when it follows an ADD
    /// instruction that adds (binary addition) two unpacked BCD values and stores a byte result in the AL register. The
    /// AAA instruction then adjusts the contents of the AL register to contain the correct 1-digit unpacked BCD result.
    ///
    /// If the addition produces a decimal carry, the AH register increments by 1, and the CF and AF flags are set. If there
    /// was no decimal carry, the CF and AF flags are cleared and the AH register is unchanged. In either case, bits 4
    /// through 7 of the AL register are set to 0.
    ///
    /// This instruction executes as described in compatibility mode and legacy mode. It is not valid in 64-bit mode.
    ///
    /// ## Compatibility
    ///
    /// ### aaa
    /// - 64Bit mode: Invalid
    /// - Compat/Leg mode: Valid
    ///
    /// ## Opcode
    /// - aaa - 37
    ///
    /// ## Flags
    /// The AF and CF flags are set to 1 if the adjustment results in a decimal carry; otherwise they are set to 0. The OF,
    /// SF, ZF, and PF flags are undefined.
    ///
    /// ## Exceptions
    ///
    /// ### Protection Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    ///
    /// ### Real-Address Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    ///
    /// ### Virtual-8086 Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    ///
    /// ### Compatibility Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    ///
    /// ### 64-Bit Mode Exceptions
    /// - UD: If in 64-bit mode.
    ///
    /// ## Operation
    /// ```ignore
    /// IF 64-Bit Mode
    ///     THEN
    ///         #UD;
    ///     ELSE
    ///     IF ((AL AND 0FH) > 9) or (AF = 1)
    ///         THEN
    ///             AX := AX + 106H;
    ///             AF := 1;
    ///             CF := 1;
    ///         ELSE
    ///             AF := 0;
    ///             CF := 0;
    ///     FI;
    ///     AL := AL AND 0FH;
    /// FI;
    /// ```
    Aaa,
    /// # aad
    ///
    /// ASCII Adjust AX Before Division
    ///
    /// - aad - ASCII adjust AX before division.
    /// - aad imm8 - Adjust AX before division to number base imm8.
    ///
    /// [Document](https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=131)
    ///
    /// Adjusts two unpacked BCD digits (the least-significant digit in the AL register and the most-significant digit in the
    /// AH register) so that a division operation performed on the result will yield a correct unpacked BCD value. The AAD
    /// instruction is only useful when it precedes a DIV instruction that divides (binary division) the adjusted value in the
    /// AX register by an unpacked BCD value.
    ///
    /// The AAD instruction sets the value in the AL register to (AL + (10 * AH)), and then clears the AH register to 00H.
    /// The value in the AX register is then equal to the binary equivalent of the original unpacked two-digit (base 10)
    /// number in registers AH and AL.
    ///
    /// The generalized version of this instruction allows adjustment of two unpacked digits of any number base (see the
    /// “Operation” section below), by setting the imm8 byte to the selected number base (for example, 08H for octal, 0AH
    /// for decimal, or 0CH for base 12 numbers). The AAD mnemonic is interpreted by all assemblers to mean adjust
    /// ASCII (base 10) values. To adjust values in another number base, the instruction must be hand coded in machine
    /// code (D5 imm8).
    ///
    /// This instruction executes as described in compatibility mode and legacy mode. It is not valid in 64-bit mode.
    ///
    /// ## Compatibility
    ///
    /// ### aad
    /// - 64Bit mode: Invalid
    /// - Compat/Leg mode: Valid
    ///
    /// ### aad imm8
    /// - 64Bit mode: Invalid
    /// - Compat/Leg mode: Valid
    ///
    /// ## Opcode
    /// - aad - d5 0a
    /// - aad imm8 - d5 ib
    ///
    /// ## Flags
    /// The SF, ZF, and PF flags are set according to the resulting binary value in the AL register; the OF, AF, and CF flags
    /// are undefined.
    ///
    /// ## Exceptions
    ///
    /// ### Protection Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    ///
    /// ### Real-Address Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    ///
    /// ### Virtual-8086 Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    ///
    /// ### Compatibility Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    ///
    /// ### 64-Bit Mode Exceptions
    /// - UD: If in 64-bit mode.
    ///
    /// ## Operation
    /// ```ignore
    /// IF 64-Bit Mode
    ///     THEN
    ///         #UD;
    ///     ELSE
    ///         tempAL := AL;
    ///         tempAH := AH;
    ///         AL := (tempAL + (tempAH ∗ imm8)) AND FFH;
    ///         (* imm8 is set to 0AH for the AAD mnemonic.*)
    ///         AH := 0;
    /// FI;
    /// The immediate value (imm8) is taken from the second byte of the instruction.
    /// ```
    Aad,
    /// # aam
    ///
    /// ASCII Adjust AX After Multiply
    ///
    /// - aam - ASCII adjust AX after multiply.
    /// - aam imm8 - Adjust AX after multiply to number base imm8.
    ///
    /// [Document](https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=133)
    ///
    /// Adjusts the result of the multiplication of two unpacked BCD values to create a pair of unpacked (base 10) BCD
    /// values. The AX register is the implied source and destination operand for this instruction. The AAM instruction is
    /// only useful when it follows an MUL instruction that multiplies (binary multiplication) two unpacked BCD values and
    /// stores a word result in the AX register. The AAM instruction then adjusts the contents of the AX register to contain
    /// the correct 2-digit unpacked (base 10) BCD result.
    ///
    /// The generalized version of this instruction allows adjustment of the contents of the AX to create two unpacked
    /// digits of any number base (see the “Operation” section below). Here, the imm8 byte is set to the selected number
    /// base (for example, 08H for octal, 0AH for decimal, or 0CH for base 12 numbers). The AAM mnemonic is interpreted
    /// by all assemblers to mean adjust to ASCII (base 10) values. To adjust to values in another number base, the
    /// instruction must be hand coded in machine code (D4 imm8).
    ///
    /// This instruction executes as described in compatibility mode and legacy mode. It is not valid in 64-bit mode.
    ///
    /// ## Compatibility
    ///
    /// ### aam
    /// - 64Bit mode: Invalid
    /// - Compat/Leg mode: Valid
    ///
    /// ### aam imm8
    /// - 64Bit mode: Invalid
    /// - Compat/Leg mode: Valid
    ///
    /// ## Opcode
    /// - aam - d4 0a
    /// - aam imm8 - d4 ib
    ///
    /// ## Flags
    /// The SF, ZF, and PF flags are set according to the resulting binary value in the AL register. The OF, AF, and CF flags
    /// are undefined.
    ///
    /// ## Exceptions
    ///
    /// ### Protection Mode Exceptions
    /// - DE: If an immediate value of 0 is used.
    /// - UD: If the LOCK prefix is used.
    ///
    /// ### Real-Address Mode Exceptions
    /// - DE: If an immediate value of 0 is used.
    /// - UD: If the LOCK prefix is used.
    ///
    /// ### Virtual-8086 Mode Exceptions
    /// - DE: If an immediate value of 0 is used.
    /// - UD: If the LOCK prefix is used.
    ///
    /// ### Compatibility Mode Exceptions
    /// - DE: If an immediate value of 0 is used.
    /// - UD: If the LOCK prefix is used.
    ///
    /// ### 64-Bit Mode Exceptions
    /// - UD: If in 64-bit mode.
    ///
    /// ## Operation
    /// ```ignore
    /// IF 64-Bit Mode
    ///     THEN
    ///         #UD;
    ///     ELSE
    ///         tempAL := AL;
    ///         AH := tempAL / imm8; (* imm8 is set to 0AH for the AAM mnemonic *)
    ///         AL := tempAL MOD imm8;
    /// FI;
    /// The immediate value (imm8) is taken from the second byte of the instruction.
    /// ```
    Aam,
    /// # aas
    ///
    /// ASCII Adjust AL After Subtraction
    ///
    /// - aas - ASCII adjust AL after subtraction.
    ///
    /// [Document](https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=135)
    ///
    /// Adjusts the result of the subtraction of two unpacked BCD values to create a unpacked BCD result. The AL register
    /// is the implied source and destination operand for this instruction. The AAS instruction is only useful when it follows
    /// a SUB instruction that subtracts (binary subtraction) one unpacked BCD value from another and stores a byte
    /// result in the AL register. The AAA instruction then adjusts the contents of the AL register to contain the correct 1-
    /// digit unpacked BCD result.
    ///
    /// If the subtraction produced a decimal carry, the AH register decrements by 1, and the CF and AF flags are set. If no
    /// decimal carry occurred, the CF and AF flags are cleared, and the AH register is unchanged. In either case, the AL
    /// register is left with its top four bits set to 0.
    ///
    /// This instruction executes as described in compatibility mode and legacy mode. It is not valid in 64-bit mode.
    ///
    /// ## Compatibility
    ///
    /// ### aas
    /// - 64Bit mode: Invalid
    /// - Compat/Leg mode: Valid
    ///
    /// ## Opcode
    /// - aas - 3f
    ///
    /// ## Flags
    /// The AF and CF flags are set to 1 if there is a decimal borrow; otherwise, they are cleared to 0. The OF, SF, ZF, and
    /// PF flags are undefined.
    ///
    /// ## Exceptions
    ///
    /// ### Protection Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    ///
    /// ### Real-Address Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    ///
    /// ### Virtual-8086 Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    ///
    /// ### Compatibility Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    ///
    /// ### 64-Bit Mode Exceptions
    /// - UD: If in 64-bit mode.
    ///
    /// ## Operation
    /// ```ignore
    /// IF 64-bit mode
    ///     THEN
    ///         #UD;
    ///     ELSE
    ///         IF ((AL AND 0FH) > 9) or (AF = 1)
    ///             THEN
    ///                 AX := AX – 6;
    ///                 AH := AH – 1;
    ///                 AF := 1;
    ///                 CF := 1;
    ///                 AL := AL AND 0FH;
    ///             ELSE
    ///                 CF := 0;
    ///                 AF := 0;
    ///                 AL := AL AND 0FH;
    ///         FI;
    /// FI;
    /// ```
    Aas,
    /// # adc
    ///
    /// Add With Carry
    ///
    /// - adc al, imm8 - Add with carry imm8 to AL.
    /// - adc ax, imm16 - Add with carry imm16 to AX.
    /// - adc eax, imm32 - Add with carry imm32 to EAX.
    /// - adc rax, imm32 - Add with carry imm32 sign extended to 64-bits to RAX.
    /// - adc r/m8, imm8 - Add with carry imm8 to r/m8.
    /// - adc r/m8*, imm8 - Add with carry imm8 to r/m8.
    /// - adc r/m16, imm16 - Add with carry imm16 to r/m16.
    /// - adc r/m32, imm32 - Add with CF imm32 to r/m32.
    /// - adc r/m64, imm32 - Add with CF imm32 sign extended to 64-bits to r/m64.
    /// - adc r/m16, imm8 - Add with CF sign-extended imm8 to r/m16.
    /// - adc r/m32, imm8 - Add with CF sign-extended imm8 into r/m32.
    /// - adc r/m64, imm8 - Add with CF sign-extended imm8 into r/m64.
    /// - adc r/m8, r8 - Add with carry byte register to r/m8.
    /// - adc r/m8*, r8* - Add with carry byte register to r/m64.
    /// - adc r/m16, r16 - Add with carry r16 to r/m16.
    /// - adc r/m32, r32 - Add with CF r32 to r/m32.
    /// - adc r/m64, r64 - Add with CF r64 to r/m64.
    /// - adc r8, r/m8 - Add with carry r/m8 to byte register.
    /// - adc r8*, r/m8* - Add with carry r/m64 to byte register.
    /// - adc r16, r/m16 - Add with carry r/m16 to r16.
    /// - adc r32, r/m32 - Add with CF r/m32 to r32.
    /// - adc r64, r/m64 - Add with CF r/m64 to r64.
    ///
    /// [Document](https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=137)
    ///
    /// Adds the destination operand (first operand), the source operand (second operand), and the carry (CF) flag and
    /// stores the result in the destination operand. The destination operand can be a register or a memory location; the
    /// source operand can be an immediate, a register, or a memory location. (However, two memory operands cannot be
    /// used in one instruction.) The state of the CF flag represents a carry from a previous addition. When an immediate
    /// value is used as an operand, it is sign-extended to the length of the destination operand format.
    ///
    /// The ADC instruction does not distinguish between signed or unsigned operands. Instead, the processor evaluates
    /// the result for both data types and sets the OF and CF flags to indicate a carry in the signed or unsigned result,
    /// respectively. The SF flag indicates the sign of the signed result.
    ///
    /// The ADC instruction is usually executed as part of a multibyte or multiword addition in which an ADD instruction is
    /// followed by an ADC instruction.
    ///
    /// This instruction can be used with a LOCK prefix to allow the instruction to be executed atomically.
    ///
    /// In 64-bit mode, the instruction’s default operation size is 32 bits. Using a REX prefix in the form of REX.R permits
    /// access to additional registers (R8-R15). Using a REX prefix in the form of REX.W promotes operation to 64 bits. See
    /// the summary chart at the beginning of this section for encoding data and limits.
    ///
    /// ## Compatibility
    ///
    /// ### adc al, imm8
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### adc ax, imm16
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### adc eax, imm32
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### adc rax, imm32
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: N.E.
    ///
    /// ### adc r/m8, imm8
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### adc r/m8* , imm8
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: N.E.
    ///
    /// ### adc r/m16, imm16
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### adc r/m32, imm32
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### adc r/m64, imm32
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: N.E.
    ///
    /// ### adc r/m16, imm8
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### adc r/m32, imm8
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### adc r/m64, imm8
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: N.E.
    ///
    /// ### adc r/m8, r8
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### adc r/m8*, r8*
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: N.E.
    ///
    /// ### adc r/m16, r16
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### adc r/m32, r32
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### adc r/m64, r64
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: N.E.
    ///
    /// ### adc r8, r/m8
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### adc r8*, r/m8*
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: N.E.
    ///
    /// ### adc r16, r/m16
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### adc r32, r/m32
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### adc r64, r/m64
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: N.E.
    ///
    /// ## Notes
    /// - In 64-bit mode, r/m8 can not be encoded to access the following byte registers if a REX prefix is used: AH, BH, CH, DH.
    ///
    /// ## Opcode
    /// - adc al, imm8 - 14 ib
    /// - adc ax, imm16 - 15 iw
    /// - adc eax, imm32 - 15 id
    /// - adc rax, imm32 - REX.W + 15 id
    /// - adc r/m8, imm8 - 80 /2 ib
    /// - adc r/m8*, imm8 - REX + 80 /2 ib
    /// - adc r/m16, imm16 - 81 /2 iw
    /// - adc r/m32, imm32 - 81 /2 id
    /// - adc r/m64, imm32 - REX.W + 81 /2 id
    /// - adc r/m16, imm8 - 83 /2 ib
    /// - adc r/m32, imm8 - 83 /2 ib
    /// - adc r/m64, imm8 - REX.W + 83 /2 ib
    /// - adc r/m8, r8 - 10 /r
    /// - adc r/m8*, r8* - REX + 10 /r
    /// - adc r/m16, r16 - 11 /r
    /// - adc r/m32, r32 - 11 /r
    /// - adc r/m64, r64 - REX.W + 11 /r
    /// - adc r8, r/m8 - 12 /r
    /// - adc r8*, r/m8* - REX + 12 /r
    /// - adc r16, r/m16 - 13 /r
    /// - adc r32, r/m32 - 13 /r
    /// - adc r64, r/m64 - REX.W + 13 /r
    ///
    /// ## Flags
    /// The OF, SF, ZF, AF, CF, and PF flags are set according to the result.
    ///
    /// ## Exceptions
    ///
    /// ### Protection Mode Exceptions
    /// - GP(0) : If the destination is located in a non-writable segment.
    /// - GP(0) : If a memory operand effective address is outside the CS, DS, ES, FS, or GS segment limit.
    /// - GP(0) : If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment selector.
    /// - SS(0) : If a memory operand effective address is outside the SS segment limit.
    /// - PF(fault-code) : If a page fault occurs.
    /// - AC(0) : If alignment checking is enabled and an unaligned memory reference is made while the current privilege level is 3.
    /// - UD : If the LOCK prefix is used but the destination is not a memory operand.
    ///
    /// ### Real-Address Mode Exceptions
    /// - GP : If a memory operand effective address is outside the CS, DS, ES, FS, or GS segment limit.
    /// - SS : If a memory operand effective address is outside the SS segment limit.
    /// - UD : If the LOCK prefix is used but the destination is not a memory operand.
    ///
    /// ### Virtual-8086 Mode Exceptions
    /// - GP(0) : If a memory operand effective address is outside the CS, DS, ES, FS, or GS segment limit.
    /// - SS(0) : If a memory operand effective address is outside the SS segment limit.
    /// - PF(fault-code) : If a page fault occurs.
    /// - AC(0) : If alignment checking is enabled and an unaligned memory reference is made.
    /// - UD : If the LOCK prefix is used but the destination is not a memory operand.
    ///
    /// ### Compatibility Mode Exceptions
    /// - GP(0) : If the destination is located in a non-writable segment.
    /// - GP(0) : If a memory operand effective address is outside the CS, DS, ES, FS, or GS segment limit.
    /// - GP(0) : If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment selector.
    /// - SS(0) : If a memory operand effective address is outside the SS segment limit.
    /// - PF(fault-code) : If a page fault occurs.
    /// - AC(0) : If alignment checking is enabled and an unaligned memory reference is made while the current privilege level is 3.
    /// - UD : If the LOCK prefix is used but the destination is not a memory operand.
    ///
    /// ### 64-Bit Mode Exceptions
    /// - SS(0) : If a memory address referencing the SS segment is in a non-canonical form.
    /// - GP(0) : If the memory address is in a non-canonical form.
    /// - PF(fault-code) : If a page fault occurs.
    /// - AC(0) : If alignment checking is enabled and an unaligned memory reference is made while the current privilege level is 3.
    /// - UD : If the LOCK prefix is used but the destination is not a memory operand.
    ///
    /// ## Intel C/C++ Compiler Intrinsic Equivalent
    /// - ADC extern unsigned char _addcarry_u8(unsigned char c_in, unsigned char src1, unsigned char src2, unsigned char *sum_out);
    /// - ADC extern unsigned char _addcarry_u16(unsigned char c_in, unsigned short src1, unsigned short src2, unsigned short *sum_out);
    /// - ADC extern unsigned char _addcarry_u32(unsigned char c_in, unsigned int src1, unsigned char int, unsigned int *sum_out);
    /// - ADC extern unsigned char _addcarry_u64(unsigned char c_in, unsigned __int64 src1, unsigned __int64 src2, unsigned __int64 *sum_out);
    ///
    /// ## Operation
    /// ```ignore
    /// DEST := DEST + SRC + CF;
    /// ```
    Adc,
    /// # adcx
    ///
    /// Unsigned Integer Addition of Two Operands With Carry Flag
    ///
    /// - adcx r32, r/m32 - Unsigned addition of r32 with CF, r/m32 to r32, writes CF.
    /// - adcx r64, r/m64 - Unsigned addition of r64 with CF, r/m64 to r64, writes CF.
    ///
    /// [Document](https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=140)
    ///
    /// Performs an unsigned addition of the destination operand (first operand), the source operand (second operand)
    /// and the carry-flag (CF) and stores the result in the destination operand. The destination operand is a general-
    /// purpose register, whereas the source operand can be a general-purpose register or memory location. The state of
    /// CF can represent a carry from a previous addition. The instruction sets the CF flag with the carry generated by the
    /// unsigned addition of the operands.
    ///
    /// The ADCX instruction is executed in the context of multi-precision addition, where we add a series of operands with
    /// a carry-chain. At the beginning of a chain of additions, we need to make sure the CF is in a desired initial state.
    /// Often, this initial state needs to be 0, which can be achieved with an instruction to zero the CF (e.g. XOR).
    ///
    /// This instruction is supported in real mode and virtual-8086 mode. The operand size is always 32 bits if not in 64-bit
    /// mode.
    ///
    /// In 64-bit mode, the default operation size is 32 bits. Using a REX Prefix in the form of REX.R permits access to addi-
    /// tional registers (R8-15). Using REX Prefix in the form of REX.W promotes operation to 64 bits.
    ///
    /// ADCX executes normally either inside or outside a transaction region.
    ///
    /// Note: ADCX defines the OF flag differently than the ADD/ADC instructions as defined in the Intel® 64 and IA-32
    /// Architectures Software Developer’s Manual, Volume 2A.
    ///
    /// ## Compatibility
    ///
    /// ### adcx r32, r/m32
    /// - 64Bit mode support: V/V
    /// - CPUID Feature Flag: ADX
    ///
    /// ### adcx r64, r/m64
    /// - 64/32Bit mode support: V/NE
    /// - CPUID Feature Flag: ADX
    ///
    /// ## Opcode
    /// - adcx r32, r/m32 - 66 0f 38 f6 /r
    /// - adcx r64, r/m64 - 66 REX.w 0f 38 f6 /r
    ///
    /// ## Flags
    /// CF is updated based on result. OF, SF, ZF, AF, and PF flags are unmodified.
    ///
    /// ## Exceptions
    ///
    /// ### SIMD Floating-Point Exceptions
    /// - None
    ///
    /// ### Protection Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    /// - UD: IF CPUID.(EAX=07H, ECX=0H):EBX.ADX[bit 19] = 0.
    /// - SS(0): For an illegal address in the SS segments.
    /// - GP(0): For an illegal memory operand effective address in the CS, DS, ES, FS, or GS segments.
    /// - GP(0): If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment selector.
    /// - PF(fault-code): For a page fault.
    /// - AC(0): If alignment checking is enabled and an unaligned memory reference is made while the current privilege level is 3.
    ///
    /// ### Real-Address Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    /// - UD: IF CPUID.(EAX=07H, ECX=0H):EBX.ADX[bit 19] = 0.
    /// - SS(0): For an illegal address in the SS segments.
    /// - GP(0): If any part of the operand lies outside the effective address space from 0 to FFFFH.
    ///
    /// ### Virtual-8086 Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    /// - UD: IF CPUID.(EAX=07H, ECX=0H):EBX.ADX[bit 19] = 0.
    /// - SS(0): For an illegal address in the SS segments.
    /// - GP(0): If any part of the operand lies outside the effective address space from 0 to FFFFH.
    /// - PF(fault-code): For a page fault.
    /// - AC(0): If alignment checking is enabled and an unaligned memory reference is made.
    ///
    /// ### Compatibility Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    /// - UD: IF CPUID.(EAX=07H, ECX=0H):EBX.ADX[bit 19] = 0.
    /// - SS(0): For an illegal address in the SS segments.
    /// - GP(0): For an illegal memory operand effective address in the CS, DS, ES, FS, or GS segments.
    /// - GP(0): If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment selector.
    /// - PF(fault-code): For a page fault.
    /// - AC(0): If alignment checking is enabled and an unaligned memory reference is made while the current privilege level is 3.
    ///
    /// ### 64-Bit Mode Exceptions
    /// - UD: If the LOCK prefix is used.
    /// - UD: IF CPUID.(EAX=07H, ECX=0H):EBX.ADX[bit 19] = 0.
    /// - SS(0): If a memory address referencing the SS segment is in a non-canonical form.
    /// - GP(0): If a memory address is in a non-canonical form.
    /// - PF(fault-code): For a page fault.
    /// - AC(0): If alignment checking is enabled and an unaligned memory reference is made while the current privilege level is 3.
    ///
    /// ## Intel C/C++ Compiler Intrinsic Equivalent
    /// unsigned char _addcarryx_u32 (unsigned char c_in, unsigned int src1, unsigned int src2, unsigned int *sum_out);
    /// unsigned char _addcarryx_u64 (unsigned char c_in, unsigned __int64 src1, unsigned __int64 src2, unsigned __int64 *sum_out);
    ///
    /// ## Operation
    /// ```ignore
    /// IF OperandSize is 64-bit
    ///     THEN CF:DEST[63:0] := DEST[63:0] + SRC[63:0] + CF;
    ///     ELSE CF:DEST[31:0] := DEST[31:0] + SRC[31:0] + CF;
    /// FI;
    /// ```
    Adcx,
    /// # add
    ///
    /// Add
    ///
    /// - add al, imm8  - Add imm8 to AL.
    /// - add ax, imm16 - Add imm16 to AX.
    /// - add eax, imm32 - Add imm32 to EAX.
    /// - add rax, imm32 - Add imm32 sign extended to 64-bits to RAX.
    /// - add r/m8, imm8 - Add imm8 to r/m8.
    /// - add r/m8*, imm8 - Add sign-extended imm8 to r/m8.
    /// - add r/m16, imm16 - Add imm16 to r/m16.
    /// - add r/m32, imm32 - Add imm32 to r/m32.
    /// - add r/m64, imm32 - Add imm32 sign extended to 64-bits to r/m64.
    /// - add r/m16, imm8 - Add sign-extended imm8 to r/m16.
    /// - add r/m32, imm8 - Add sign-extended imm8 to r/m32.
    /// - add r/m64, imm8 - Add sign-extended imm8 to r/m64.
    /// - add r/m8, r8 - Add r8 to r/m8.
    /// - add r/m8*, r8* - Add r8 to r/m8.
    /// - add r/m16, r16 - Add r16 to r/m16.
    /// - add r/m32, r32 - Add r32 to r/m32.
    /// - add r/m64, r64 - Add r64 to r/m64.
    /// - add r8, r/m8 - Add r/m8 to r8.
    /// - add r8*, r/m8* - Add r/m8 to r8.
    /// - add r16, r/m16 - Add r/m16 to r16.
    /// - add r32, r/m32 - Add r/m32 to r32.
    /// - add r64, r/m64 - Add r/m64 to r64.
    ///
    /// [Document](https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=142)
    ///
    /// Adds the destination operand (first operand) and the source operand (second operand) and then stores the result
    /// in the destination operand. The destination operand can be a register or a memory location; the source operand
    /// can be an immediate, a register, or a memory location. (However, two memory operands cannot be used in one
    /// instruction.) When an immediate value is used as an operand, it is sign-extended to the length of the destination
    /// operand format.
    ///
    /// The ADD instruction performs integer addition. It evaluates the result for both signed and unsigned integer oper-
    /// ands and sets the OF and CF flags to indicate a carry (overflow) in the signed or unsigned result, respectively. The
    /// SF flag indicates the sign of the signed result.
    ///
    /// This instruction can be used with a LOCK prefix to allow the instruction to be executed atomically.
    ///
    /// In 64-bit mode, the instruction’s default operation size is 32 bits. Using a REX prefix in the form of REX.R permits
    /// access to additional registers (R8-R15). Using a REX prefix in the form of REX.W promotes operation to 64 bits. See
    /// the summary chart at the beginning of this section for encoding data and limits.
    ///
    /// ## Compatibility
    ///
    /// ### add al, imm8
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### add ax, imm16
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### add eax, imm32
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### add rax, imm32
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: N.E.
    ///
    /// ### add r/m8, imm8
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### add r/m8* , imm8
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: N.E.
    ///
    /// ### add r/m16, imm16
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### add r/m32, imm32
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### add r/m64, imm32
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: N.E.
    ///
    /// ### add r/m16, imm8
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### add r/m32, imm8
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### add r/m64, imm8
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: N.E.
    ///
    /// ### add r/m8, r8
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### add r/m8*, r8*
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: N.E.
    ///
    /// ### add r/m16, r16
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### add r/m32, r32
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### add r/m64, r64
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: N.E.
    ///
    /// ### add r8, r/m8
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### add r8*, r/m8*
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: N.E.
    ///
    /// ### add r16, r/m16
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### add r32, r/m32
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: Valid
    ///
    /// ### add r64, r/m64
    /// - 64Bit mode: Valid
    /// - Compat/Leg mode: N.E.
    ///
    /// ## Notes
    /// - In 64-bit mode, r/m8 can not be encoded to access the following byte registers if a REX prefix is used: AH, BH, CH, DH.
    ///
    /// ## Opcode
    /// - add al, imm8 - 04 ib
    /// - add ax, imm16 - 05 iw
    /// - add eax, imm32 - 05 id
    /// - add rax, imm32 - REX.W + 05 id
    /// - add r/m8, imm8 - 80 /0 ib
    /// - add r/m8*, imm8 - REX + 80 /0 ib
    /// - add r/m16, imm16 - 81 /0 iw
    /// - add r/m32, imm32 - 81 /0 id
    /// - add r/m64, imm32 - REX.W + 81 /0 id
    /// - add r/m16, imm8 - 83 /0 ib
    /// - add r/m32, imm8 - 83 /0 ib
    /// - add r/m64, imm8 - REX.W + 83 /0 ib
    /// - add r/m8, r8 - 00 /r
    /// - add r/m8*, r8* - REX + 00 /r
    /// - add r/m16, r16 - 01 /r
    /// - add r/m32, r32 - 01 /r
    /// - add r/m64, r64 - REX.W + 01 /r
    /// - add r8, r/m8 - 02 /r
    /// - add r8*, r/m8* - REX + 02 /r
    /// - add r16, r/m16 - 03 /r
    /// - add r32, r/m32 - 03 /r
    /// - add r64, r/m64 - REX.W + 03 /r
    ///
    /// ## Flags
    /// The OF, SF, ZF, AF, CF, and PF flags are set according to the result.
    ///
    /// ## Exceptions
    ///
    /// ### Protection Mode Exceptions
    /// - GP(0): If the destination is located in a non-writable segment.
    /// - GP(0): If a memory operand effective address is outside the CS, DS, ES, FS, or GS segment limit.
    /// - GP(0): If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment selector.
    /// -SS(0): If a memory operand effective address is outside the SS segment limit.
    /// -PF(fault-code): If a page fault occurs.
    /// -AC(0): If alignment checking is enabled and an unaligned memory reference is made while the current privilege level is 3.
    /// -UD: If the LOCK prefix is used but the destination is not a memory operand.
    ///
    /// ### Real-Address Mode Exceptions
    /// - GP: If a memory operand effective address is outside the CS, DS, ES, FS, or GS segment limit.
    /// - SS: If a memory operand effective address is outside the SS segment limit.
    /// - UD: If the LOCK prefix is used but the destination is not a memory operand.
    ///
    /// ### Virtual-8086 Mode Exceptions
    /// - GP(0): If a memory operand effective address is outside the CS, DS, ES, FS, or GS segment limit.
    /// - SS(0): If a memory operand effective address is outside the SS segment limit.
    /// - PF(fault-code): If a page fault occurs.
    /// - AC(0): If alignment checking is enabled and an unaligned memory reference is made.
    /// - UD: If the LOCK prefix is used but the destination is not a memory operand.
    ///
    /// ### Compatibility Mode Exceptions
    /// - GP(0): If the destination is located in a non-writable segment.
    /// - GP(0): If a memory operand effective address is outside the CS, DS, ES, FS, or GS segment limit.
    /// - GP(0): If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment selector.
    /// -SS(0): If a memory operand effective address is outside the SS segment limit.
    /// -PF(fault-code): If a page fault occurs.
    /// -AC(0): If alignment checking is enabled and an unaligned memory reference is made while the current privilege level is 3.
    /// -UD: If the LOCK prefix is used but the destination is not a memory operand.
    ///
    /// ### 64-Bit Mode Exceptions
    /// - SS(0): If a memory address referencing the SS segment is in a non-canonical form.
    /// - GP(0): If a memory address is in a non-canonical form.
    /// - PF(fault-code): If a page fault occurs.
    /// - AC(0): If alignment checking is enabled and an unaligned memory reference is made while the current privilege level is 3.
    /// - UD: If the LOCK prefix is used but the destination is not a memory operand.
    ///
    /// ## Operation
    /// ```ignore
    /// DEST := DEST + SRC;
    /// ```
    Add,
    /// # addpd
    ///
    /// Add Packed Double Precision Floating-Point Values
    ///
    /// - addpd xmm1, xmm2/m128 - Add packed double precision floating-point values from xmm2/mem to xmm1 and store result in xmm1.
    /// - vaddpd xmm1, xmm2, xmm3/m128 - Add packed double precision floating-point values from xmm3/mem to xmm2 and store result in xmm1.
    /// - vaddpd ymm1, ymm2, ymm3/m256 - Add packed double precision floating-point values from ymm3/mem to ymm2 and store result in ymm1.
    /// - vaddpd xmm1 {k1}{z}, xmm2, xmm3/m128/m64bcst - Add packed double precision floating-point values from xmm3/m128/m64bcst to xmm2 and store result in xmm1 with writemask k1.
    /// - vaddpd ymm1 {k1}{z}, ymm2, ymm3/m256/m64bcst - Add packed double precision floating-point values from ymm3/m256/m64bcst to ymm2 and store result in ymm1 with writemask k1.
    /// - vaddpd zmm1 {k1}{z}, zmm2, zmm3/m512/m64bcst{er} - Add packed double precision floating-point values from zmm3/m512/m64bcst to zmm2 and store result in zmm1 with writemask k1.
    ///
    /// [Document](https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=144)
    ///
    /// Adds two, four or eight packed double precision floating-point values from the first source operand to the second
    /// source operand, and stores the packed double precision floating-point result in the destination operand.
    ///
    /// EVEX encoded versions: The first source operand is a ZMM/YMM/XMM register. The second source operand can be
    /// a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a
    /// 64-bit memory location. The destination operand is a ZMM/YMM/XMM register conditionally updated with
    /// writemask k1.
    ///
    /// VEX.256 encoded version: The first source operand is a YMM register. The second source operand can be a YMM
    /// register or a 256-bit memory location. The destination operand is a YMM register. The upper bits (MAXVL-1:256) of
    /// the corresponding ZMM register destination are zeroed.
    ///
    /// VEX.128 encoded version: the first source operand is a XMM register. The second source operand is an XMM
    /// register or 128-bit memory location. The destination operand is an XMM register. The upper bits (MAXVL-1:128) of
    /// the corresponding ZMM register destination are zeroed.
    ///
    /// 128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location. The desti-
    /// nation is not distinct from the first source XMM register and the upper Bits (MAXVL-1:128) of the corresponding
    /// ZMM register destination are unmodified.
    ///
    /// ## Compatibility
    ///
    /// ### addpd xmm1, xmm2/m128
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: SSE2
    ///
    /// ### vaddpd xmm1, xmm2, xmm3/m128
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: AVX
    ///
    /// ### vaddpd ymm1, ymm2, ymm3/m256
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: AVX
    ///
    /// ### vaddpd xmm1 {k1}{z}, xmm2, xmm3/m128/m64bcst
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: AVX512VL + AVX512F
    ///
    /// ### vaddpd ymm1 {k1}{z}, ymm2, ymm3/m256/m64bcst
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: AVX512VL + AVX512F
    ///
    /// ### vaddpd zmm1 {k1}{z}, zmm2, zmm3/m512/m64bcst{er}
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: AVX512F
    ///
    /// ## Opcode
    /// - addpd xmm1, xmm2/m128 - 66 0f 58 /r
    /// - vddpd xmm1, xmm2, xmm3/m128 - VEX.128.66.0f.WIG 58 /r
    /// - vaddpd ymm1, ymm2, ymm3/m256 - VEX.256.66.0f.WIG 58 /r
    /// - vaddpd xmm1 {k1}{z}, xmm2, xmm3/m128/m64bcst - EVEX.128.66.0f.w1 58 /r
    /// - vaddpd ymm1 {k1}{z}, ymm2, ymm3/m256/m64bcst - EVEX.256.66.0f.w1 58 /r
    /// - vaddpd zmm1 {k1}{z}, zmm2, zmm3/m512/m64bcst{er} - EVEX.512.66.0f.w1 58 /r
    ///
    /// ## Exceptions
    ///
    /// ### SIMD Floating-Point Exceptions
    /// - Overflow, Underflow, Invalid, Precision, Denormal.
    ///
    /// ### Other Exceptions
    /// - VEX-encoded instruction, see Table 2-19, “Type 2 Class Exception Conditions.”
    /// - EVEX-encoded instruction, see Table 2-46, “Type E2 Class Exception Conditions.”
    ///
    /// ## Intel C/C++ Compiler Intrinsic Equivalent
    /// VADDPD __m512d _mm512_add_pd (__m512d a, __m512d b);
    /// VADDPD __m512d _mm512_mask_add_pd (__m512d s, __mmask8 k, __m512d a, __m512d b);
    /// VADDPD __m512d _mm512_maskz_add_pd (__mmask8 k, __m512d a, __m512d b);
    /// VADDPD __m256d _mm256_mask_add_pd (__m256d s, __mmask8 k, __m256d a, __m256d b);
    /// VADDPD __m256d _mm256_maskz_add_pd (__mmask8 k, __m256d a, __m256d b);
    /// VADDPD __m128d _mm_mask_add_pd (__m128d s, __mmask8 k, __m128d a, __m128d b);
    /// VADDPD __m128d _mm_maskz_add_pd (__mmask8 k, __m128d a, __m128d b);
    /// VADDPD __m512d _mm512_add_round_pd (__m512d a, __m512d b, int);
    /// VADDPD __m512d _mm512_mask_add_round_pd (__m512d s, __mmask8 k, __m512d a, __m512d b, int);
    /// VADDPD __m512d _mm512_maskz_add_round_pd (__mmask8 k, __m512d a, __m512d b, int);
    /// ADDPD __m256d _mm256_add_pd (__m256d a, __m256d b);
    /// ADDPD __m128d _mm_add_pd (__m128d a, __m128d b);
    ///
    /// ## Operation
    ///
    /// ### VADDPD (EVEX Encoded Versions) When SRC2 Operand is a Vector Register
    /// ```ignore
    /// (KL, VL) = (2, 128), (4, 256), (8, 512)
    /// IF (VL = 512) AND (EVEX.b = 1)
    ///     THEN
    ///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
    ///     ELSE
    ///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
    /// FI;
    /// FOR j := 0 TO KL-1
    ///     i := j * 64
    ///     IF k1[j] OR *no writemask*
    ///         THEN DEST[i+63:i] := SRC1[i+63:i] + SRC2[i+63:i]
    ///         ELSE
    ///             IF *merging-masking* ; merging-masking
    ///                 THEN *DEST[i+63:i] remains unchanged*
    ///                 ELSE ; zeroing-masking
    ///                     DEST[i+63:i] := 0
    ///             FI
    ///     FI;
    /// ENDFOR
    /// DEST[MAXVL-1:VL] := 0
    /// ```
    ///
    /// ### VADDPD (EVEX Encoded Versions) When SRC2 Operand is a Memory Source
    /// ```ignore
    /// (KL, VL) = (2, 128), (4, 256), (8, 512)
    /// FOR j := 0 TO KL-1
    ///     i := j * 64
    ///     IF k1[j] OR *no writemask*
    ///         THEN
    ///             IF (EVEX.b = 1)
    ///                 THEN
    ///                     DEST[i+63:i] := SRC1[i+63:i] + SRC2[63:0]
    ///                 ELSE
    ///                     DEST[i+63:i] := SRC1[i+63:i] + SRC2[i+63:i]
    ///             FI;
    ///         ELSE
    ///             IF *merging-masking* ; merging-masking
    ///                 THEN *DEST[i+63:i] remains unchanged*
    ///                 ELSE ; zeroing-masking
    ///                     DEST[i+63:i] := 0
    ///             FI
    ///     FI;
    /// ENDFOR
    /// DEST[MAXVL-1:VL] := 0
    /// ```
    ///
    /// ### VADDPD (VEX.256 Encoded Version)
    /// ```ignore
    /// DEST[63:0] := SRC1[63:0] + SRC2[63:0]
    /// DEST[127:64] := SRC1[127:64] + SRC2[127:64]
    /// DEST[191:128] := SRC1[191:128] + SRC2[191:128]
    /// DEST[255:192] := SRC1[255:192] + SRC2[255:192]
    /// DEST[MAXVL-1:256] := 0
    /// ```
    ///
    /// ### VADDPD (VEX.128 Encoded Version)
    /// ```ignore
    /// DEST[63:0] := SRC1[63:0] + SRC2[63:0]
    /// DEST[127:64] := SRC1[127:64] + SRC2[127:64]
    /// DEST[MAXVL-1:128] := 0
    /// ```
    ///
    /// ### ADDPD (128-bit Legacy SSE Version)
    /// ```ignore
    /// DEST[63:0] := DEST[63:0] + SRC[63:0]
    /// DEST[127:64] := DEST[127:64] + SRC[127:64]
    /// DEST[MAXVL-1:128] (Unmodified)
    /// ```
    Addpd,
    /// # addps
    ///
    /// Add Packed Single Precision Floating-Point Values
    ///
    /// - addps xmm1, xmm2/m128 - Add packed single precision floating-point values from xmm2/m128 to xmm1 and store result in xmm1.
    /// - vaddps xmm1, xmm2, xmm3/m128 - Add packed single precision floating-point values from xmm3/m128 to xmm2 and store result in xmm1.
    /// - vaddps ymm1, ymm2, ymm3/m256 - Add packed single precision floating-point values from ymm3/m256 to ymm2 and store result in ymm1.
    /// - vaddps xmm1 {k1}{z}, xmm2, xmm3/m128/m32bcst - Add packed single precision floating-point values from xmm3/m128/m32bcst to xmm2 and store result in xmm1 with writemask k1.
    /// - vaddps ymm1 {k1}{z}, ymm2, ymm3/m256/m32bcst - Add packed single precision floating-point values from ymm3/m256/m32bcst to ymm2 and store result in ymm1 with writemask k1.
    /// - vaddps zmm1 {k1}{z}, zmm2, zmm3/m512/m32bcst{er} - Add packed single precision floating-point values from zmm3/m512/m32bcst to zmm2 and store result in zmm1 with writemask k1.
    ///
    /// [Document](https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=147)
    ///
    /// Adds four, eight or sixteen packed single precision floating-point values from the first source operand with the
    /// second source operand, and stores the packed single precision floating-point result in the destination operand.
    ///
    /// EVEX encoded versions: The first source operand is a ZMM/YMM/XMM register. The second source operand can be
    /// a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a
    /// 32-bit memory location. The destination operand is a ZMM/YMM/XMM register conditionally updated with
    /// writemask k1.
    ///
    /// VEX.256 encoded version: The first source operand is a YMM register. The second source operand can be a YMM
    /// register or a 256-bit memory location. The destination operand is a YMM register. The upper bits (MAXVL-1:256) of
    /// the corresponding ZMM register destination are zeroed.
    ///
    /// VEX.128 encoded version: the first source operand is a XMM register. The second source operand is an XMM
    /// register or 128-bit memory location. The destination operand is an XMM register. The upper bits (MAXVL-1:128) of
    /// the corresponding ZMM register destination are zeroed.
    ///
    /// 128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location. The desti-
    /// nation is not distinct from the first source XMM register and the upper Bits (MAXVL-1:128) of the corresponding
    /// ZMM register destination are unmodified.
    ///
    /// ## Compatibility
    ///
    /// ### addps xmm1, xmm2/m128
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: SSE
    ///
    /// ### vaddps xmm1, xmm2, xmm3/m128
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: AVX
    ///
    /// ### vaddps ymm1, ymm2, ymm3/m256
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: AVX
    ///
    /// ### vaddps xmm1 {k1}{z}, xmm2, xmm3/m128/m32bcst
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: AVX512VL + AVX512F
    ///
    /// ### vaddps ymm1 {k1}{z}, ymm2, ymm3/m256/m32bcst
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: AVX512VL + AVX512F
    ///
    /// ### vaddps zmm1 {k1}{z}, zmm2, zmm3/m512/m32bcst{er}
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: AVX512F
    ///
    /// ## Opcode
    /// - addps xmm1, xmm2/m128 - np 0f 58 /r
    /// - vaddps xmm1, xmm2, xmm3/m128 - VEX.128.0f.WIG 58 /r
    /// - vaddps ymm1, ymm2, ymm3/m256 - VEX.256.0f.WIG 58 /r
    /// - vaddps xmm1 {k1}{z}, xmm2, xmm3/m128/m32bcst - EVEX.128.0f.w0 58 /r
    /// - vaddps ymm1 {k1}{z}, ymm2, ymm3/m256/m32bcst - EVEX.256.0f.w0 58 /r
    /// - vaddps zmm1 {k1}{z}, zmm2, zmm3/m512/m32bcst{er} - EVEX.512.0f.w0 58 /r
    ///
    /// ## Exceptions
    ///
    /// ### SIMD Floating-Point Exceptions
    /// - Overflow, Underflow, Invalid, Precision, Denormal.
    ///
    /// ### Other Exceptions
    /// - VEX-encoded instruction, see Table 2-19, “Type 2 Class Exception Conditions.”
    /// - EVEX-encoded instruction, see Table 2-46, “Type E2 Class Exception Conditions.”
    ///
    /// ## Intel C/C++ Compiler Intrinsic Equivalent
    /// VADDPS __m512 _mm512_add_ps (__m512 a, __m512 b);
    /// VADDPS __m512 _mm512_mask_add_ps (__m512 s, __mmask16 k, __m512 a, __m512 b);
    /// VADDPS __m512 _mm512_maskz_add_ps (__mmask16 k, __m512 a, __m512 b);
    /// VADDPS __m256 _mm256_mask_add_ps (__m256 s, __mmask8 k, __m256 a, __m256 b);
    /// VADDPS __m256 _mm256_maskz_add_ps (__mmask8 k, __m256 a, __m256 b);
    /// VADDPS __m128 _mm_mask_add_ps (__m128d s, __mmask8 k, __m128 a, __m128 b);
    /// VADDPS __m128 _mm_maskz_add_ps (__mmask8 k, __m128 a, __m128 b);
    /// VADDPS __m512 _mm512_add_round_ps (__m512 a, __m512 b, int);
    /// VADDPS __m512 _mm512_mask_add_round_ps (__m512 s, __mmask16 k, __m512 a, __m512 b, int);
    /// VADDPS __m512 _mm512_maskz_add_round_ps (__mmask16 k, __m512 a, __m512 b, int);
    /// ADDPS __m256 _mm256_add_ps (__m256 a, __m256 b);
    /// ADDPS __m128 _mm_add_ps (__m128 a, __m128 b);
    ///
    /// ## Operation
    ///
    /// ### VADDPS (EVEX Encoded Versions) When SRC2 Operand is a Register
    /// ```ignore
    /// (KL, VL) = (4, 128), (8, 256), (16, 512)
    /// IF (VL = 512) AND (EVEX.b = 1)
    ///     THEN
    ///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
    ///     ELSE
    ///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
    /// FI;
    /// FOR j := 0 TO KL-1
    ///     i := j * 32
    ///     IF k1[j] OR *no writemask*
    ///         THEN DEST[i+31:i] := SRC1[i+31:i] + SRC2[i+31:i]
    ///         ELSE
    ///             IF *merging-masking* ; merging-masking
    ///                 THEN *DEST[i+31:i] remains unchanged*
    ///                 ELSE ; zeroing-masking
    ///                     DEST[i+31:i] := 0
    ///             FI
    ///     FI;
    /// ENDFOR;
    /// DEST[MAXVL-1:VL] := 0
    /// ```
    ///
    /// ### VADDPS (EVEX Encoded Versions) When SRC2 Operand is a Memory Source
    /// ```ignore
    /// (KL, VL) = (4, 128), (8, 256), (16, 512)
    /// FOR j := 0 TO KL-1
    ///     i := j * 32
    ///     IF k1[j] OR *no writemask*
    ///     THEN
    ///         IF (EVEX.b = 1)
    ///             THEN
    ///                 DEST[i+31:i] := SRC1[i+31:i] + SRC2[31:0]
    ///             ELSE
    ///                 DEST[i+31:i] := SRC1[i+31:i] + SRC2[i+31:i]
    ///         FI;
    ///     ELSE
    ///         IF *merging-masking* ; merging-masking
    ///             THEN *DEST[i+31:i] remains unchanged*
    ///             ELSE ; zeroing-masking
    ///                 DEST[i+31:i] := 0
    ///         FI
    ///     FI;
    /// ENDFOR;
    /// DEST[MAXVL-1:VL] := 0
    /// ```
    ///
    /// ### VADDPS (VEX.256 Encoded Version)
    /// ```ignore
    /// DEST[31:0] := SRC1[31:0] + SRC2[31:0]
    /// DEST[63:32] := SRC1[63:32] + SRC2[63:32]
    /// DEST[95:64] := SRC1[95:64] + SRC2[95:64]
    /// DEST[127:96] := SRC1[127:96] + SRC2[127:96]
    /// DEST[159:128] := SRC1[159:128] + SRC2[159:128]
    /// DEST[191:160]:= SRC1[191:160] + SRC2[191:160]
    /// DEST[223:192] := SRC1[223:192] + SRC2[223:192]
    /// DEST[255:224] := SRC1[255:224] + SRC2[255:224].
    /// DEST[MAXVL-1:256] := 0
    /// ```
    ///
    /// ### VADDPS (VEX.128 Encoded Version)
    /// ```ignore
    /// DEST[31:0] := SRC1[31:0] + SRC2[31:0]
    /// DEST[63:32] := SRC1[63:32] + SRC2[63:32]
    /// DEST[95:64] := SRC1[95:64] + SRC2[95:64]
    /// DEST[127:96] := SRC1[127:96] + SRC2[127:96]
    /// DEST[MAXVL-1:128] := 0
    /// ```
    ///
    /// ### ADDPS (128-bit Legacy SSE Version)
    /// ```ignore
    /// DEST[31:0] := SRC1[31:0] + SRC2[31:0]
    /// DEST[63:32] := SRC1[63:32] + SRC2[63:32]
    /// DEST[95:64] := SRC1[95:64] + SRC2[95:64]
    /// DEST[127:96] := SRC1[127:96] + SRC2[127:96]
    /// DEST[MAXVL-1:128] (Unmodified)
    /// ```
    Addps,
    /// # addss
    ///
    /// Add Scalar Single Precision Floating-Point Values
    ///
    /// - addss xmm1, xmm2/m32 - Add the low single precision floating-point value from xmm2/mem to xmm1 and store the result in xmm1.
    /// - vaddss xmm1, xmm2, xmm3/m32 - Add the low single precision floating-point value from xmm3/mem to xmm2 and store the result in xmm1.
    /// - vaddss xmm1 {k1}{z}, xmm2, xmm3/m32{er} - Add the low single precision floating-point value from xmm3/m32 to xmm2 and store the result in xmm1 with writemask k1.
    ///
    /// [Document](https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=152)
    ///
    /// Adds the low single precision floating-point values from the second source operand and the first source operand,
    /// and stores the double precision floating-point result in the destination operand.
    ///
    /// The second source operand can be an XMM register or a 64-bit memory location. The first source and destination
    /// operands are XMM registers.
    ///
    /// 128-bit Legacy SSE version: The first source and destination operands are the same. Bits (MAXVL-1:32) of the
    /// corresponding the destination register remain unchanged.
    ///
    /// EVEX and VEX.128 encoded version: The first source operand is encoded by EVEX.vvvv/VEX.vvvv. Bits (127:32) of
    /// the XMM register destination are copied from corresponding bits in the first source operand. Bits (MAXVL-1:128) of
    /// the destination register are zeroed.
    ///
    /// EVEX version: The low doubleword element of the destination is updated according to the writemask.
    ///
    /// Software should ensure VADDSS is encoded with VEX.L=0. Encoding VADDSS with VEX.L=1 may encounter unpre-
    /// dictable behavior across different processor generations.
    ///
    /// ## Compatibility
    ///
    /// ### addss xmm1, xmm2/m32
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: SSE
    ///
    /// ### vaddss xmm1, xmm2, xmm3/m32
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: AVX
    ///
    /// ### vaddss xmm1 {k1}{z}, xmm2, xmm3/m32{er}
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: AVX512F
    ///
    /// ## Opcode
    /// - addss xmm1, xmm2/m32 - F3 0f 58 /r
    /// - vaddss xmm1, xmm2, xmm3/m32 - VEX.LIG.F3.0f.WIG 58 /r
    /// - vaddss xmm1 {k1}{z}, xmm2, xmm3/m32{er} - EVEX.LIG.F3.0f.W0 58 /r
    ///
    /// ## Exceptions
    ///
    /// ### SIMD Floating-Point Exceptions
    /// - Overflow, Underflow, Invalid, Precision, Denormal.
    ///
    /// ### Other Exceptions
    /// VEX-encoded instruction, see Table 2-20, “Type 3 Class Exception Conditions.”
    /// EVEX-encoded instruction, see Table 2-47, “Type E3 Class Exception Conditions.”
    ///
    /// ## Intel C/C++ Compiler Intrinsic Equivalent
    /// VADDSS __m128 _mm_mask_add_ss (__m128 s, __mmask8 k, __m128 a, __m128 b);
    /// VADDSS __m128 _mm_maskz_add_ss (__mmask8 k, __m128 a, __m128 b);
    /// VADDSS __m128 _mm_add_round_ss (__m128 a, __m128 b, int);
    /// VADDSS __m128 _mm_mask_add_round_ss (__m128 s, __mmask8 k, __m128 a, __m128 b, int);
    /// VADDSS __m128 _mm_maskz_add_round_ss (__mmask8 k, __m128 a, __m128 b, int);
    /// ADDSS __m128 _mm_add_ss (__m128 a, __m128 b);
    ///
    /// ## Operation
    ///
    /// ###VADDSS (EVEX Encoded Versions)
    /// ```ignore
    /// IF (EVEX.b = 1) AND SRC2 *is a register*
    ///     THEN
    ///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
    ///     ELSE
    ///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
    /// FI;
    /// IF k1[0] or *no writemask*
    ///     THEN DEST[31:0] := SRC1[31:0] + SRC2[31:0]
    ///     ELSE
    ///         IF *merging-masking* ; merging-masking
    ///             THEN *DEST[31:0] remains unchanged*
    ///             ELSE ; zeroing-masking
    ///                 THEN DEST[31:0] := 0
    ///     FI;
    /// FI;
    /// DEST[127:32] := SRC1[127:32]
    /// DEST[MAXVL-1:128] := 0
    /// ```
    ///
    /// ### VADDSS DEST, SRC1, SRC2 (VEX.128 Encoded Version)
    /// ```ignore
    /// DEST[31:0] := SRC1[31:0] + SRC2[31:0]
    /// DEST[127:32] := SRC1[127:32]
    /// DEST[MAXVL-1:128] := 0
    /// ```
    ///
    /// ### ADDSS DEST, SRC (128-bit Legacy SSE Version)
    /// ```ignore
    /// DEST[31:0] := DEST[31:0] + SRC[31:0]
    /// DEST[MAXVL-1:32] (Unmodified)
    /// ```
    Addss,
    /// # addsubpd
    ///
    /// Packed Double Precision Floating-Point Add/Subtract
    ///
    /// - addsubpd xmm1, xmm2/m128 - Add/subtract packed double precision floating-point values from xmm2/m128 to xmm1.
    /// - vaddsubpd xmm1, xmm2, xmm3/m128 - Add/subtract packed double precision floating-point values from xmm3/m128 to xmm2 and store result in xmm1.
    /// - vaddsubpd ymm1, ymm2, ymm3/m256 - Add/subtract packed double precision floating-point values from ymm3/m256 to ymm2 and store result in ymm1.
    ///
    /// [Document](https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=154)
    ///
    /// Adds odd-numbered double precision floating-point values of the first source operand (second operand) with the
    /// corresponding double precision floating-point values from the second source operand (third operand); stores the
    /// result in the odd-numbered values of the destination operand (first operand). Subtracts the even-numbered double
    /// precision floating-point values from the second source operand from the corresponding double precision floating
    /// values in the first source operand; stores the result into the even-numbered values of the destination operand.
    ///
    /// In 64-bit mode, using a REX prefix in the form of REX.R permits this instruction to access additional registers
    /// (XMM8-XMM15).
    ///
    /// 128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location. The desti-
    /// nation is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding
    /// YMM register destination are unmodified. See Figure 3-3.
    ///
    /// VEX.128 encoded version: the first source operand is an XMM register or 128-bit memory location. The destination
    /// operand is an XMM register. The upper bits (MAXVL-1:128) of the corresponding YMM register destination are
    /// zeroed.
    ///
    /// VEX.256 encoded version: The first source operand is a YMM register. The second source operand can be a YMM
    /// register or a 256-bit memory location. The destination operand is a YMM register.
    ///
    /// ## Compatibility
    ///
    /// ### addsuppd xmm1, xmm2/m128
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: SSE3
    ///
    /// ### vaddsubpd xmm1, xmm2, xmm3/m128
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: AVX
    ///
    /// ### vaddsubpd ymm1, ymm2, ymm3/m256
    /// - 64/32Bit mode support: V/V
    /// - CPUID Feature Flag: AVX
    ///
    /// ## Opcode
    /// - addsubpd xmm1, xmm2/m128 - 66 0f d0 /r
    /// - vaddsubpd xmm1, xmm2, xmm3/m128 - VEX.128.66.0f.WIG d0 /r
    /// - vaddsubpd ymm1, ymm2, ymm3/m256 - VEX.256.66.0f.WIG d0 /r
    ///
    /// ## Exceptions
    /// - When the source operand is a memory operand, it must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
    ///
    /// ### SIMD Floating-Point Exceptions
    /// - Overflow, Underflow, Invalid, Precision, Denormal.
    ///
    /// ### Other Exceptions
    /// See Table 2-19, “Type 2 Class Exception Conditions.”
    ///
    /// ## Intel C/C++ Compiler Intrinsic Equivalent
    /// ADDSUBPD __m128d _mm_addsub_pd(__m128d a, __m128d b)
    /// VADDSUBPD __m256d _mm256_addsub_pd (__m256d a, __m256d b)
    ///
    /// ## Operation
    ///
    /// ### ADDSUBPD (128-bit Legacy SSE Version)
    /// ```ignore
    /// DEST[63:0] := DEST[63:0] - SRC[63:0]
    /// DEST[127:64] := DEST[127:64] + SRC[127:64]
    /// DEST[MAXVL-1:128] (Unmodified)
    /// ```
    ///
    /// ### VADDSUBPD (VEX.128 Encoded Version)
    /// ```ignore
    /// DEST[63:0] := SRC1[63:0] - SRC2[63:0]
    /// DEST[127:64] := SRC1[127:64] + SRC2[127:64]
    /// DEST[MAXVL-1:128] := 0
    /// ```
    ///
    /// ### VADDSUBPD (VEX.256 Encoded Version)
    /// ```ignore
    /// DEST[63:0] := SRC1[63:0] - SRC2[63:0]
    /// DEST[127:64] := SRC1[127:64] + SRC2[127:64]
    /// DEST[191:128] := SRC1[191:128] - SRC2[191:128]
    /// DEST[255:192] := SRC1[255:192] + SRC2[255:192]
    /// ```
    Addsubpd,
}
