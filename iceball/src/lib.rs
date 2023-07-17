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
    /// ### aaa
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
    /// THEN 
    /// #UD; 
    /// ELSE 
    /// tempAL := AL; 
    /// AH := tempAL / imm8; (* imm8 is set to 0AH for the AAM mnemonic *) 
    /// AL := tempAL MOD imm8; 
    /// FI; 
    /// The immediate value (imm8) is taken from the second byte of the instruction. 
    /// ```
    Aam,
    /// # aas
    ///
    /// - aas - ASCII adjust AL after subtraction.
    ///
    /// [Document](https://eveheeero.github.io/book/Intel%C2%AE_64_and_IA-32_Architectures_Developer's_Manual-2/?page=134)
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
    /// THEN 
    /// #UD; 
    /// ELSE 
    /// IF ((AL AND 0FH) > 9) or (AF = 1) 
    /// THEN 
    /// AX := AX – 6; 
    /// AH := AH – 1; 
    /// AF := 1; 
    /// CF := 1; 
    /// AL := AL AND 0FH; 
    /// ELSE 
    /// CF := 0; 
    /// AF := 0; 
    /// AL := AL AND 0FH; 
    /// FI; 
    /// FI;
    /// ```
    Aas,
}

/* origin
/// Move data between general-purpose registers; move data between memory and generalpurpose or segment registers; move immediates to general-purpose registers.
Mov,
/// Conditional move if equal.
Cmove,
/// Conditional move if zero.
Cmovz,
/// Conditional move if not equal.
Cmovne,
/// Conditional move if not zero.
Cmovnz,
/// Conditional move if above.
Cmova,
/// Conditional move if not below or equal.
Cmovnbe,
/// Conditional move if above or equal.
Cmovae,
/// Conditional move if not below.
Cmovnb,
/// Conditional move if below.
Cmovb,
/// Conditional move if not above or equal.
Cmovnae,
/// Conditional move if below or equal.
Cmovbe,
/// Conditional move if not above.
Cmovna,
/// Conditional move if greater.
Cmovg,
/// Conditional move if not less or equal.
Cmovnle,
/// Conditional move if greater or equal.
Cmovge,
/// Conditional move if not less.
Cmovnl,
/// Conditional move if less.
Cmovl,
/// Conditional move if not greater or equal.
Cmovnge,
/// Conditional move if less or equal.
Cmovle,
/// Conditional move if not greater.
Cmovng,
/// Conditional move if carry.
Cmovc,
/// Conditional move if not carry.
Cmovnc,
/// Conditional move if overflow.
Cmovo,
/// Conditional move if not overflow.
Cmovno,
/// Conditional move if sign (negative).
Cmovs,
/// Conditional move if not sign (non-negative).
Cmovns,
/// Conditional move if parity.
Cmovp,
/// Conditional move if parity even.
Cmovpe,
/// Conditional move if not parity.
Cmovnp,
/// Conditional move if parity odd.
Cmovpo,
/// Exchange.
Xchg,
/// Byte swap.
Bswap,
/// Exchange and add.
Xadd,
/// Compare and exchange.
Cmpxchg,
/// Compare and exchange 8 bytes.
Cmpxchg8b,
/// Push onto stack.
Push,
/// Pop off of stack.
Pop,
/// Push general-purpose registers onto stack.
Pusha,
/// Push general-purpose registers onto stack.
Pushad,
/// Pop general-purpose registers from stack.
Popa,
/// Pop general-purpose registers from stack.
Popad,
/// Convert word to doubleword.
Cwd,
/// Convert doubleword to quadword.
Cdq,
/// Convert byte to word.
Cbw,
/// Convert word to doubleword in EAX register.
Cwde,
/// Move and sign extend.
Movsx,
/// Move and zero extend.
Movzx,
/// Unsigned integer add with carry.
Adcx,
/// Unsigned integer add with overflow.
Adox,
/// Integer add.
Add,
/// Add with carry.
Adc,
/// Subtract.
Sub,
/// Subtract with borrow.
Sbb,
/// Signed multiply.
Imul,
/// Unsigned multiply.
Mul,
/// Signed divide.
Idiv,
/// Unsigned divide.
Div,
/// Increment.
Inc,
/// Decrement.
Dec,
/// Negate.
Neg,
/// Compare.
Cmp,
/// Decimal adjust after addition.
Daa,
/// Decimal adjust after subtraction.
Das,
/// ASCII adjust after addition.
Aaa,
/// ASCII adjust after subtraction.
Aas,
/// ASCII adjust after multiplication.
Aam,
/// ASCII adjust before division.
Aad,
/// Perform bitwise logical AND.
And,
/// Perform bitwise logical OR.
Or,
/// Perform bitwise logical exclusive OR.
Xor,
/// Perform bitwise logical NOT.
Not,
/// Shift arithmetic right.
Sar,
/// Shift logical right.
Shr,
/// Shift arithmetic left.
Sal,
/// Shift logical left.
Shl,
/// Shift right double.
Shrd,
/// Shift left double.
Shld,
/// Rotate right.
Ror,
/// Rotate left.
Rol,
/// Rotate through carry right.
Rcr,
/// Rotate through carry left.
Rcl,
/// Bit test.
Bt,
/// Bit test and set.
Bts,
/// Bit test and reset.
Btr,
/// Bit test and complement.
Btc,
/// Bit scan forward.
Bsf,
/// Bit scan reverse.
Bsr,
/// Set byte if equal.
Sete,
/// Set byte if zero.
Setz,
/// Set byte if not equal.
Setne,
/// Set byte if not zero.
Setnz,
/// Set byte if above.
Seta,
/// Set byte if not below or equal.
Setnbe,
/// Set byte if above or equal.
Setae,
/// Set byte if not below
Setnb,
/// Set byte if not carry.
Setnc,
/// Set byte if below.
Setb,
/// Set byte if not above or equal.
Setnae,
/// Set byte if carry.
Setc,
/// Set byte if below or equal.
Setbe,
/// Set byte if not above.
Setna,
/// Set byte if greater.
Setg,
/// Set byte if not less or equal.
Setnle,
/// Set byte if greater or equal.
Setge,
/// Set byte if not less.
Setnl,
/// Set byte if less.
Setl,
/// Set byte if not greater or equal.
Setnge,
/// Set byte if less or equal.
Setle,
/// Set byte if not greater.
Setng,
/// Set byte if sign (negative).
Sets,
/// Set byte if not sign (non-negative).
Setns,
/// Set byte if overflow.
Seto,
/// Set byte if not overflow.
Setno,
/// Set byte if parity even.
Setpe,
/// Set byte if parity.
Setp,
/// Set byte if parity odd.
Setpo,
/// Set byte if not parity.
Setnp,
/// Logical compare.
Test,
/// Provides hardware acceleration to calculate cyclic redundancy checks for fast and efficient implementation of data integrity protocols.
Crc321,
/// This instruction calculates of number of bits set to 1 in the second operand (source) and returns the count in the first operand (a destination register).
Popcnt2,
/// Jump.
Jmp,
/// Jump if equal
Je,
/// Jump if zero.
Jz,
/// Jump if not equal.
Jne,
/// Jump if not zero.
Jnz,
/// Jump if above.
Ja,
/// Jump if not below or equal.
Jnbe,
/// Jump if above or equal.
Jae,
/// Jump if not below.
Jnb,
/// Jump if below.
Jb,
/// Jump if not above or equal.
Jnae,
/// Jump if below or equal.
Jbe,
/// Jump if not above.
Jna,
/// Jump if greater.
Jg,
/// Jump if not less or equal.
Jnle,
/// Jump if greater or equal.
Jge,
/// Jump if not less.
Jnl,
/// Jump if less.
Jl,
/// Jump if not greater or equal.
Jnge,
/// Jump if less or equal.
Jle,
/// Jump if not greater.
Jng,
/// Jump if carry.
Jc,
/// Jump if not carry.
Jnc,
/// Jump if overflow.
Jo,
/// Jump if not overflow.
Jno,
/// Jump if sign (negative).
Js,
/// Jump if not sign (non-negative).
Jns,
/// Jump if parity odd.
Jpo,
/// Jump if not parity.
Jnp,
/// Jump if parity even.
Jpe,
/// Jump if parity.
Jp,
/// Jump register CX zero.
Jcxz,
/// Jump register ECX zero.
Jecxz,
/// Loop with ECX counter.
Loop,
/// Loop with ECX and zero.
Loopz,
/// Loop with ECX and equal.
Loope,
/// Loop with ECX and not zero.
Loopnz,
/// Loop with ECX and not equal.
Loopne,
/// Call procedure.
Call,
/// Return.
Ret,
/// Return from interrupt.
Iret,
/// Software interrupt.
Int,
/// Interrupt on overflow.
Into,
/// Detect value out of range.
Bound,
/// High-level procedure entry.
Enter,
/// High-level procedure exit.
Leave,
/// Move string.
Movs,
/// Move byte string.
Movsb,
/// Move word string.
Movsw,
/// Move doubleword string.
Movsd,
/// Compare string.
Cmps,
/// Compare byte string.
Cmpsb,
/// Compare word string.
Cmpsw,
/// Compare doubleword string.
Cmpsd,
/// Scan string.
Scas,
/// Scan byte string.
Scasb,
/// Scan word string.
Scasw,
/// Scan doubleword string.
Scasd,
/// Load string.
Lods,
/// Load byte string.
Lodsb,
/// Load word string.
Lodsw,
/// Load doubleword string.
Lodsd,
/// Store string.
Stos,
/// Store byte string.
Stosb,
/// Store word string.
Stosw,
/// Store doubleword string.
Stosd,
/// Repeat while ECX not zero.
Rep,
/// Repeat while equal.
Repe,
/// Repeat while zero.
Repz,
/// Repeat while not equal.
Repne,
/// Repeat while not zero.
Repnz,
/// Read from a port.
In,
/// Write to a port.
Out,
/// Input string from port.
Ins,
/// Input byte string from port.
Insb,
/// Input word string from port.
Insw,
/// Input doubleword string from port.
Insd,
/// Output string to port.
Outs,
/// Output byte string to port.
Outsb,
/// Output word string to port.
Outsw,
/// Output doubleword string to port.
Outsd,
/// Set carry flag.
Stc,
/// Clear the carry flag.
Clc,
/// Complement the carry flag.
Cmc,
/// Clear the direction flag.
Cld,
/// Set direction flag.
Std,
/// Load flags into AH register.
Lahf,
/// Store AH register into flags.
Sahf,
/// Push EFLAGS onto stack.
Pushf,
/// Push EFLAGS onto stack.
Pushfd,
/// Pop EFLAGS from stack.
Popf,
/// Pop EFLAGS from stack.
Popfd,
/// Set interrupt flag.
Sti,
/// Clear the interrupt flag.
Cli,
/// Load far pointer using DS.
Lds,
/// Load far pointer using ES.
Les,
/// Load far pointer using FS.
Lfs,
/// Load far pointer using GS.
Lgs,
/// Load far pointer using SS.
Lss,
/// Load effective address.
Lea,
/// No operation.
Nop,
/// Undefined instruction.
Ud,
/// Table lookup translation.
Xlat,
/// Table lookup translation.
Xlatb,
/// Processor identification.
Cpuid,
/// Move data after swapping data bytes.
Movbe1,
/// Prefetch data into cache in anticipation of write.
Prefetchw,
/// Prefetch hint T1 with intent to write.
Prefetchwt1,
/// Flushes and invalidates a memory operand and its associated cache line from all levels of the processor’s cache hierarchy.
Clflush,
/// Flushes and invalidates a memory operand and its associated cache line from all levels of the processor’s cache hierarchy with optimized memory system throughput.
Clflushopt,
/// Save processor extended states to memory.
Xsave,
/// Save processor extended states with compaction to memory.
Xsavec,
/// Save processor extended states to memory, optimized.
Xsaveopt,
/// Restore processor extended states from memory.
Xrstor,
/// Retrieves a random number generated from hardware.
Rdrand,
/// Retrieves a random number generated from hardware.
Rdseed,
/// Bitwise AND of first source with inverted 2nd source operands.
Andn,
/// Contiguous bitwise extract.
Bextr,
/// Extract lowest set bit.
Blsi,
/// Set all lower bits below first set bit to 1.
Blsmsk,
/// Reset lowest set bit.
Blsr,
/// Zero high bits starting from specified bit position.
Bzhi,
/// Count the number leading zero bits.
Lzcnt,
/// Unsigned multiply without affecting arithmetic flags.
Mulx,
/// Parallel deposit of bits using a mask.
Pdep,
/// Parallel extraction of bits using a mask.
Pext,
/// Rotate right without affecting arithmetic flags.
Rorx,
/// Shift arithmetic right.
Sarx,
/// Shift logic left.
Shlx,
/// Shift logic right.
Shrx,
/// Count the number trailing zero bits.
Tzcnt,
/// Load floating-point value.
Fld,
/// Store floating-point value.
Fst,
/// Store floating-point value and pop.
Fstp,
/// Load integer.
Fild,
/// Store integer.
Fist,
/// Store integer and pop.
Fistp1,
/// Load BCD.
Fbld,
/// Store BCD and pop.
Fbstp,
/// Exchange registers.
Fxch,
/// Floating-point conditional move if equal.
Fcmove,
/// Floating-point conditional move if not equal.
Fcmovne,
/// Floating-point conditional move if below.
Fcmovb,
/// Floating-point conditional move if below or equal.
Fcmovbe,
/// Floating-point conditional move if not below.
Fcmovnb,
/// Floating-point conditional move if not below or equal.
Fcmovnbe,
/// Floating-point conditional move if unordered.
Fcmovu,
/// Floating-point conditional move if not unordered.
Fcmovnu,
/// Add floating-point
Fadd,
/// Add floating-point and pop
Faddp,
/// Add integer
Fiadd,
/// Subtract floating-point
Fsub,
/// Subtract floating-point and pop
Fsubp,
/// Subtract integer
Fisub,
/// Subtract floating-point reverse
Fsubr,
/// Subtract floating-point reverse and pop
Fsubrp,
/// Subtract integer reverse
Fisubr,
/// Multiply floating-point
Fmul,
/// Multiply floating-point and pop
Fmulp,
/// Multiply integer
Fimul,
/// Divide floating-point
Fdiv,
/// Divide floating-point and pop
Fdivp,
/// Divide integer
Fidiv,
/// Divide floating-point reverse
Fdivr,
/// Divide floating-point reverse and pop
Fdivrp,
/// Divide integer reverse
Fidivr,
/// Partial remainder
Fprem,
/// IEEE Partial remainder
Fprem1,
/// Absolute value
Fabs,
/// Change sign
Fchs,
/// Round to integer
Frndint,
/// Scale by power of two
Fscale,
/// Square root
Fsqrt,
/// Extract exponent and significand
Fxtract,
/// Compare floating-point.
Fcom,
/// Compare floating-point and pop.
Fcomp,
/// Compare floating-point and pop twice.
Fcompp,
/// Unordered compare floating-point.
Fucom,
/// Unordered compare floating-point and pop.
Fucomp,
/// Unordered compare floating-point and pop twice.
Fucompp,
/// Compare integer.
Ficom,
/// Compare integer and pop.
Ficomp,
/// Compare floating-point and set EFLAGS.
Fcomi,
/// Unordered compare floating-point and set EFLAGS.
Fucomi,
/// Compare floating-point, set EFLAGS, and pop.
Fcomip,
/// Unordered compare floating-point, set EFLAGS, and pop.
Fucomip,
/// Test floating-point (compare with 0.0).
Ftst,
/// Examine floating-point.
Fxam,
/// Sine
Fsin,
/// Cosine
Fcos,
/// Sine and cosine
Fsincos,
/// Partial tangent
Fptan,
/// Partial arctangent
Fpatan,
/// 2x − 1
F2xm1,
/// y∗log2x
Fyl2x,
/// y∗log2(x+1)
Fyl2xp1,
/// Load +1.0
Fld1,
/// Load +0.0
Fldz,
/// Load π
Fldpi,
/// Load log2e
Fldl2e,
/// Load loge2
Fldln2,
/// Load log210
Fldl2t,
/// Load log102
Fldlg2,
/// Increment FPU register stack pointer.
Fincstp,
/// Decrement FPU register stack pointer.
Fdecstp,
/// Free floating-point register.
Ffree,
/// Initialize FPU after checking error conditions.
Finit,
/// Initialize FPU without checking error conditions.
Fninit,
/// Clear floating-point exception flags after checking for error conditions.
Fclex,
/// Clear floating-point exception flags without checking for error conditions.
Fnclex,
/// Store FPU control word after checking error conditions.
Fstcw,
/// Store FPU control word without checking error conditions.
Fnstcw,
/// Load FPU control word.
Fldcw,
/// Store FPU environment after checking error conditions.
Fstenv,
/// Store FPU environment without checking error conditions.
Fnstenv,
/// Load FPU environment.
Fldenv,
/// Save FPU state after checking error conditions.
Fsave,
/// Save FPU state without checking error conditions.
Fnsave,
/// Restore FPU state.
Frstor,
/// Store FPU status word after checking error conditions.
Fstsw,
/// Store FPU status word without checking error conditions.
Fnstsw,
/// Wait for FPU.
Wait,
/// Wait for FPU.
Fwait,
/// FPU no operation.
Fnop,
/// Save x87 FPU and SIMD state.
Fxsave,
/// Restore x87 FPU and SIMD state.
Fxrstor,
/// Move doubleword.
Movd,
/// Move quadword.
Movq,
/// Pack words into bytes with signed saturation.
Packsswb,
/// Pack doublewords into words with signed saturation.
Packssdw,
/// Pack words into bytes with unsigned saturation.
Packuswb,
/// Unpack high-order bytes.
Punpckhbw,
/// Unpack high-order words.
Punpckhwd,
/// Unpack high-order doublewords.
Punpckhdq,
/// Unpack low-order bytes.
Punpcklbw,
/// Unpack low-order words.
Punpcklwd,
/// Unpack low-order doublewords.
Punpckldq,
/// Add packed byte integers.
Paddb,
/// Add packed word integers.
Paddw,
/// Add packed doubleword integers.
Paddd,
/// Add packed signed byte integers with signed saturation.
Paddsb,
/// Add packed signed word integers with signed saturation.
Paddsw,
/// Add packed unsigned byte integers with unsigned saturation.
Paddusb,
/// Add packed unsigned word integers with unsigned saturation.
Paddusw,
/// Subtract packed byte integers.
Psubb,
/// Subtract packed word integers.
Psubw,
/// Subtract packed doubleword integers.
Psubd,
/// Subtract packed signed byte integers with signed saturation.
Psubsb,
/// Subtract packed signed word integers with signed saturation.
Psubsw,
/// Subtract packed unsigned byte integers with unsigned saturation.
Psubusb,
/// Subtract packed unsigned word integers with unsigned saturation.
Psubusw,
/// Multiply packed signed word integers and store high result.
Pmulhw,
/// Multiply packed signed word integers and store low result.
Pmullw,
/// Multiply and add packed word integers.
Pmaddwd,
/// Compare packed bytes for equal.
Pcmpeqb,
/// Compare packed words for equal.
Pcmpeqw,
/// Compare packed doublewords for equal.
Pcmpeqd,
/// Compare packed signed byte integers for greater than.
Pcmpgtb,
/// Compare packed signed word integers for greater than.
Pcmpgtw,
/// Compare packed signed doubleword integers for greater than.
Pcmpgtd,
/// Bitwise logical AND.
Pand,
/// Bitwise logical AND NOT.
Pandn,
/// Bitwise logical OR.
Por,
/// Bitwise logical exclusive OR.
Pxor,
/// Shift packed words left logical.
Psllw,
/// Shift packed doublewords left logical.
Pslld,
/// Shift packed quadword left logical.
Psllq,
/// Shift packed words right logical.
Psrlw,
/// Shift packed doublewords right logical.
Psrld,
/// Shift packed quadword right logical.
Psrlq,
/// Shift packed words right arithmetic.
Psraw,
/// Shift packed doublewords right arithmetic.
Psrad,
/// Move four aligned packed single-precision floating-point values between XMM registers or between and XMM register and memory.
Movaps,
/// Move four unaligned packed single-precision floating-point values between XMM registers or between and XMM register and memory.
Movups,
/// Move two packed single-precision floating-point values to an from the high quadword of an XMM register and memory.
Movhps,
/// Move two packed single-precision floating-point values from the high quadword of an XMM register to the low quadword of another XMM register.
Movhlps,
/// Move two packed single-precision floating-point values to an from the low quadword of an XMM register and memory.
Movlps,
/// Move two packed single-precision floating-point values from the low quadword of an XMM register to the high quadword of another XMM register.
Movlhps,
/// Extract sign mask from four packed single-precision floating-point values.
Movmskps,
/// Move scalar single-precision floating-point value between XMM registers or between an XMM register and memory.
Movss,
/// Add packed single-precision floating-point values.
Addps,
/// Add scalar single-precision floating-point values.
Addss,
/// Subtract packed single-precision floating-point values.
Subps,
/// Subtract scalar single-precision floating-point values.
Subss,
/// Multiply packed single-precision floating-point values.
Mulps,
/// Multiply scalar single-precision floating-point values.
Mulss,
/// Divide packed single-precision floating-point values.
Divps,
/// Divide scalar single-precision floating-point values.
Divss,
/// Compute reciprocals of packed single-precision floating-point values.
Rcpps,
/// Compute reciprocal of scalar single-precision floating-point values.
Rcpss,
/// Compute square roots of packed single-precision floating-point values.
Sqrtps,
/// Compute square root of scalar single-precision floating-point values.
Sqrtss,
/// Compute reciprocals of square roots of packed single-precision floating-point values.
Rsqrtps,
/// Compute reciprocal of square root of scalar single-precision floating-point values.
Rsqrtss,
/// Return maximum packed single-precision floating-point values.
Maxps,
/// Return maximum scalar single-precision floating-point values.
Maxss,
/// Return minimum packed single-precision floating-point values.
Minps,
/// Return minimum scalar single-precision floating-point values.
Minss,
/// Compare packed single-precision floating-point values.
Cmpps,
/// Compare scalar single-precision floating-point values.
Cmpss,
/// Perform ordered comparison of scalar single-precision floating-point values and set flags in
Comiss,
/// Perform unordered comparison of scalar single-precision floating-point values and set flags in EFLAGS register.
Ucomiss,
/// Perform bitwise logical AND of packed single-precision floating-point values.
Andps,
/// Perform bitwise logical AND NOT of packed single-precision floating-point values.
Andnps,
/// Perform bitwise logical OR of packed single-precision floating-point values.
Orps,
/// Perform bitwise logical XOR of packed single-precision floating-point values.
Xorps,
/// Shuffles values in packed single-precision floating-point operands.
Shufps,
/// Unpacks and interleaves the two high-order values from two single-precision floating-point operands.
Unpckhps,
/// Unpacks and interleaves the two low-order values from two single-precision floating-point operands.
Unpcklps,
/// Convert packed doubleword integers to packed single-precision floating-point values.
Cvtpi2ps,
/// Convert doubleword integer to scalar single-precision floating-point value.
Cvtsi2ss,
/// Convert packed single-precision floating-point values to packed doubleword integers.
Cvtps2pi,
/// Convert with truncation packed single-precision floating-point values to packed doubleword integers.
Cvttps2pi,
/// Convert a scalar single-precision floating-point value to a doubleword integer.
Cvtss2si,
/// Convert with truncation a scalar single-precision floating-point value to a scalar doubleword integer.
Cvttss2si,
/// state management instructions allow saving and restoring the state of the MXCSR control and status register.
Mxcsr,
/// Load MXCSR register.
Ldmxcsr,
/// Save MXCSR register state.
Stmxcsr,
/// Compute average of packed unsigned byte integers.
Pavgb,
/// Compute average of packed unsigned word integers.
Pavgw,
/// Insert word.
Pinsrw,
/// Maximum of packed unsigned byte integers.
Pmaxub,
/// Maximum of packed signed word integers.
Pmaxsw,
/// Minimum of packed unsigned byte integers.
Pminub,
/// Minimum of packed signed word integers.
Pminsw,
/// Move byte mask.
Pmovmskb,
/// Multiply packed unsigned integers and store high result.
Pmulhuw,
/// Compute sum of absolute differences.
Psadbw,
/// Shuffle packed integer word in MMX register.
Pshufw,
/// Non-temporal store of selected bytes from an MMX register into memory.
Maskmovq,
/// Non-temporal store of quadword from an MMX register into memory.
Movntq,
/// Non-temporal store of four packed single-precision floating-point values from an XMM register into memory.
Movntps,
/// Serializes store operations.
Sfence,
/// Move two aligned packed double-precision floating-point values between XMM registers or between and XMM register and memory.
Movapd,
/// Move two unaligned packed double-precision floating-point values between XMM registers or between and XMM register and memory.
Movupd,
/// Move high packed double-precision floating-point value to an from the high quadword of an XMM register and memory.
Movhpd,
/// Move low packed single-precision floating-point value to an from the low quadword of an XMM register and memory.
Movlpd,
/// Extract sign mask from two packed double-precision floating-point values.
Movmskpd,
/// Add packed double-precision floating-point values.
Addpd,
/// Add scalar double precision floating-point values.
Addsd,
/// Subtract packed double-precision floating-point values.
Subpd,
/// Subtract scalar double-precision floating-point values.
Subsd,
/// Multiply packed double-precision floating-point values.
Mulpd,
/// Multiply scalar double-precision floating-point values.
Mulsd,
/// Divide packed double-precision floating-point values.
Divpd,
/// Divide scalar double-precision floating-point values.
Divsd,
/// Compute packed square roots of packed double-precision floating-point values.
Sqrtpd,
/// Compute scalar square root of scalar double-precision floating-point values.
Sqrtsd,
/// Return maximum packed double-precision floating-point values.
Maxpd,
/// Return maximum scalar double-precision floating-point values.
Maxsd,
/// Return minimum packed double-precision floating-point values.
Minpd,
/// Return minimum scalar double-precision floating-point values.
Minsd,
/// Perform bitwise logical AND of packed double-precision floating-point values.
Andpd,
/// Perform bitwise logical AND NOT of packed double-precision floating-point values.
Andnpd,
/// Perform bitwise logical OR of packed double-precision floating-point values.
Orpd,
/// Perform bitwise logical XOR of packed double-precision floating-point values.
Xorpd,
/// Compare packed double-precision floating-point values.
Cmppd,
/// Perform ordered comparison of scalar double-precision floating-point values and set flags in EFLAGS register.
Comisd,
/// Perform unordered comparison of scalar double-precision floating-point values and set flags in EFLAGS register.
Ucomisd,
/// Shuffles values in packed double-precision floating-point operands.
Shufpd,
/// Unpacks and interleaves the high values from two packed double-precision floating-point operands.
Unpckhpd,
/// Unpacks and interleaves the low values from two packed double-precision floating-point operands.
Unpcklpd,
/// Convert packed double-precision floating-point values to packed doubleword integers.
Cvtpd2pi,
/// Convert with truncation packed double-precision floating-point values to packed doubleword integers.
Cvttpd2pi,
/// Convert packed doubleword integers to packed double-precision floating-point values.
Cvtpi2pd,
/// Convert packed double-precision floating-point values to packed doubleword integers.
Cvtpd2dq,
/// Convert with truncation packed double-precision floating-point values to packed doubleword integers.
Cvttpd2dq,
/// Convert packed doubleword integers to packed double-precision floating-point values.
Cvtdq2pd,
/// Convert packed single-precision floating-point values to packed double-precision floatingpoint values.
Cvtps2pd,
/// Convert packed double-precision floating-point values to packed single-precision floatingpoint values.
Cvtpd2ps,
/// Convert scalar single-precision floating-point values to scalar double-precision floatingpoint values.
Cvtss2sd,
/// Convert scalar double-precision floating-point values to scalar single-precision floatingpoint values.
Cvtsd2ss,
/// Convert scalar double-precision floating-point values to a doubleword integer.
Cvtsd2si,
/// Convert with truncation scalar double-precision floating-point values to scalar doubleword integers.
Cvttsd2si,
/// Convert doubleword integer to scalar double-precision floating-point value.
Cvtsi2sd,
/// Convert packed doubleword integers to packed single-precision floating-point values.
Cvtdq2ps,
/// Convert packed single-precision floating-point values to packed doubleword integers.
Cvtps2dq,
/// Convert with truncation packed single-precision floating-point values to packed doubleword integers.
Cvttps2dq,
/// Move aligned double quadword.
Movdqa,
/// Move unaligned double quadword.
Movdqu,
/// Move quadword integer from MMX to XMM registers.
Movq2dq,
/// Move quadword integer from XMM to MMX registers.
Movdq2q,
/// Multiply packed unsigned doubleword integers.
Pmuludq,
/// Add packed quadword integers.
Paddq,
/// Subtract packed quadword integers.
Psubq,
/// Shuffle packed low words.
Pshuflw,
/// Shuffle packed high words.
Pshufhw,
/// Shuffle packed doublewords.
Pshufd,
/// Shift double quadword left logical.
Pslldq,
/// Shift double quadword right logical.
Psrldq,
/// Unpack high quadwords.
Punpckhqdq,
/// Unpack low quadwords.
Punpcklqdq,
/// Serializes load operations.
Lfence,
/// Serializes load and store operations.
Mfence,
/// Improves the performance of “spin-wait loops”.
Pause,
/// Non-temporal store of selected bytes from an XMM register into memory.
Maskmovdqu,
/// Non-temporal store of two packed double-precision floating-point values from an XMM register into memory.
Movntpd,
/// Non-temporal store of double quadword from an XMM register into memory.
Movntdq,
/// Non-temporal store of a doubleword from a general-purpose register into memory.
Movnti,
/// Behaves like the FISTP instruction but uses truncation, irrespective of the rounding mode specified in the floating-point control word (FCW).
Fisttp,
/// Special 128-bit unaligned load designed to avoid cache line splits.
Lddqu,
/// Performs single-precision addition on the second and fourth pairs of 32-bit data elements within the operands; single-precision subtraction on the first and third pairs.
Addsubps,
/// Performs double-precision addition on the second pair of quadwords, and double-precision subtraction on the first pair.
Addsubpd,
/// Performs a single-precision addition on contiguous data elements. The first data element of the result is obtained by adding the first and second elements of the first operand; the second element by adding the third and fourth elements of the first operand; the third by adding the first and second elements of the second operand; and the fourth by adding the third and fourth elements of the second operand.
Haddps,
/// Performs a single-precision subtraction on contiguous data elements. The first data element of the result is obtained by subtracting the second element of the first operand from the first element of the first operand; the second element by subtracting the fourth element of the first operand from the third element of the first operand; the third by subtracting the second element of the second operand from the first element of the second operand; and the fourth by subtracting the fourth element of the second operand from the third element of the second operand.
Hsubps,
/// Performs a double-precision addition on contiguous data elements. The first data element of the result is obtained by adding the first and second elements of the first operand; the second element by adding the first and second elements of the second operand.
Haddpd,
/// Performs a double-precision subtraction on contiguous data elements. The first data element of the result is obtained by subtracting the second element of the first operand from the first element of the first operand; the second element by subtracting the second element of the second operand from the first element of the second operand.
Hsubpd,
/// Loads/moves 128 bits; duplicating the second and fourth 32-bit data elements.
Movshdup,
/// Loads/moves 128 bits; duplicating the first and third 32-bit data elements.
Movsldup,
/// Loads/moves 64 bits (bits[63:0] if the source is a register) and returns the same 64 bits in both the lower and upper halves of the 128-bit result register; duplicates the 64 bits from the source.
Movddup,
/// Sets up an address range used to monitor write-back stores.
Monitor,
/// Enables a logical processor to enter into an optimized state while waiting for a write-back store to the address range set up by the MONITOR instruction.
Mwait,
/// Adds two adjacent, signed 16-bit integers horizontally from the source and destination operands and packs the signed 16-bit results to the destination operand.
Phaddw,
/// Adds two adjacent, signed 16-bit integers horizontally from the source and destination operands and packs the signed, saturated 16-bit results to the destination operand.
Phaddsw,
/// Adds two adjacent, signed 32-bit integers horizontally from the source and destination operands and packs the signed 32-bit results to the destination operand.
Phaddd,
/// Performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed 16-bit results are packed and written to the destination operand.
Phsubw,
/// Performs horizontal subtraction on each adjacent pair of 16-bit signed integers by subtracting the most significant word from the least significant word of each pair in the source and destination operands. The signed, saturated 16-bit results are packed and written to the destination operand.
Phsubsw,
/// Performs horizontal subtraction on each adjacent pair of 32-bit signed integers by subtracting the most significant doubleword from the least significant double word of each pair in the source and destination operands. The signed 32-bit results are packed and written to the destination operand.
Phsubd,
/// Computes the absolute value of each signed byte data element.
Pabsb,
/// Computes the absolute value of each signed 16-bit data element.
Pabsw,
/// Computes the absolute value of each signed 32-bit data element.
Pabsd,
/// Multiplies each unsigned byte value with the corresponding signed byte value to produce an intermediate, 16-bit signed integer. Each adjacent pair of 16-bit signed values are added horizontally. The signed, saturated 16-bit results are packed to the destination operand.
Pmaddubsw,
/// Multiplies vertically each signed 16-bit integer from the destination operand with the corresponding signed 16-bit integer of the source operand, producing intermediate, signed 32- bit integers. Each intermediate 32-bit integer is truncated to the 18 most significant bits.
Pmulhrsw,
/// Permutes each byte in place, according to a shuffle control mask. The least significant three or four bits of each shuffle control byte of the control mask form the shuffle index. The shuffle mask is unaffected. If the most significant bit (bit 7) of a shuffle control byte is set, the constant zero is written in the result byte.
Pshufb,
/// Negates each signed integer element of the destination operand if the sign of the corresponding data element in the source operand is less than zero.
Psignb,
/// Negates each signed integer element of the destination operand if the sign of the corresponding data element in the source operand is less than zero.
Psignw,
/// Negates each signed integer element of the destination operand if the sign of the corresponding data element in the source operand is less than zero.
Psignd,
/// Source operand is appended after the destination operand forming an intermediate value of twice the width of an operand. The result is extracted from the intermediate value into the destination operand by selecting the 128 bit or 64 bit value that are right-aligned to the byte offset specified by the immediate value.
Palignr,
/// Returns four lower 32-bits of the 64-bit results of signed 32-bit integer multiplies.
Pmulld,
/// Returns two 64-bit signed result of signed 32-bit integer multiplies.
Pmuldq,
/// Perform double-precision dot product for up to 2 elements and broadcast.
Dppd,
/// Perform single-precision dot products for up to 4 elements and broadcast.
Dpps,
/// Provides a non-temporal hint that can cause adjacent 16-byte items within an aligned 64-byte region (a streaming line) to be fetched and held in a small set of temporary buffers (“streaming load buffers”). Subsequent streaming loads to other aligned 16-byte items in the same streaming line may be supplied from the streaming load buffer and can improve throughput.
Movntdqa,
/// Conditionally copies specified double-precision floating-point data elements in the source operand to the corresponding data elements in the destination, using an immediate byte control.
Blendpd,
/// Conditionally copies specified single-precision floating-point data elements in the source operand to the corresponding data elements in the destination, using an immediate byte control.
Blendps,
/// Conditionally copies specified double-precision floating-point data elements in the source operand to the corresponding data elements in the destination, using an implied mask.
Blendvpd,
/// Conditionally copies specified single-precision floating-point data elements in the source operand to the corresponding data elements in the destination, using an implied mask.
Blendvps,
/// Conditionally copies specified byte elements in the source operand to the corresponding elements in the destination, using an implied mask.
Pblendvb,
/// Conditionally copies specified word elements in the source operand to the corresponding elements in the destination, using an immediate byte control.
Pblendw,
/// Compare packed unsigned word integers.
Pminuw,
/// Compare packed unsigned dword integers.
Pminud,
/// Compare packed signed byte integers.
Pminsb,
/// Compare packed signed dword integers.
Pminsd,
/// Compare packed unsigned word integers.
Pmaxuw,
/// Compare packed unsigned dword integers.
Pmaxud,
/// Compare packed signed byte integers.
Pmaxsb,
/// Compare packed signed dword integers.
Pmaxsd,
/// Round packed single precision floating-point values into integer values and return rounded floating-point values.
Roundps,
/// Round packed double precision floating-point values into integer values and return rounded floating-point values.
Roundpd,
/// Round the low packed single precision floating-point value into an integer value and return a rounded floating-point value.
Roundss,
/// Round the low packed double precision floating-point value into an integer value and return a rounded floating-point value.
Roundsd,
/// Extracts a single-precision floating-point value from a specified offset in an XMM register and stores the result to memory or a general-purpose register.
Extractps,
/// Inserts a single-precision floating-point value from either a 32-bit memory location orselected from a specified offset in an XMM register to a specified offset in the destination
Insertps,
/// Insert a byte value from a register or memory into an XMM register.
Pinsrb,
/// Insert a dword value from 32-bit register or memory into an XMM register.
Pinsrd,
/// Insert a qword value from 64-bit register or memory into an XMM register.
Pinsrq,
/// Extract a byte from an XMM register and insert the value into a general-purpose register or memory.
Pextrb,
/// Extract a word from an XMM register and insert the value into a general-purpose register or memory.
Pextrw,
/// Extract a dword from an XMM register and insert the value into a general-purpose register or memory.
Pextrd,
/// Extract a qword from an XMM register and insert the value into a general-purpose register or memory.
Pextrq,
/// Sign extend the lower 8-bit integer of each packed word element into packed signed word integers.
Pmovsxbw,
/// Zero extend the lower 8-bit integer of each packed word element into packed signed word integers.
Pmovzxbw,
/// Sign extend the lower 8-bit integer of each packed dword element into packed signed dword integers.
Pmovsxbd,
/// Zero extend the lower 8-bit integer of each packed dword element into packed signed dword integers.
Pmovzxbd,
/// Sign extend the lower 16-bit integer of each packed dword element into packed signed dword integers.
Pmovsxwd,
/// Zero extend the lower 16-bit integer of each packed dword element into packed signed dword integers.
Pmovzxwd,
/// Sign extend the lower 8-bit integer of each packed qword element into packed signed qword integers.
Pmovsxbq,
/// Zero extend the lower 8-bit integer of each packed qword element into packed signed qword integers.
Pmovzxbq,
/// Sign extend the lower 16-bit integer of each packed qword element into packed signed qword integers.
Pmovsxwq,
/// Zero extend the lower 16-bit integer of each packed qword element into packed signed qword integers.
Pmovzxwq,
/// Sign extend the lower 32-bit integer of each packed qword element into packed signed qword integers.
Pmovsxdq,
/// Zero extend the lower 32-bit integer of each packed qword element into packed signed qword integers.
Pmovzxdq,
/// Performs eight 4-byte wide Sum of Absolute Differences operations to produce eight word integers.
Mpsadbw,
/// Finds the value and location of the minimum unsigned word from one of 8 horizontally packed unsigned words. The resulting value and location (offset within the source) are packed into the low dword of the destination XMM register.
Phminposuw,
/// Performs a logical AND between the destination with this mask and sets the ZF flag if the result is zero. The CF flag (zero for TEST) is set if the inverted mask AND’d with the destination is all zeroes.
Ptest,
/// 128-bit packed qword equality test.
Pcmpeqq,
/// PACKUSDW packs dword to word with unsigned saturation.
Packusdw,
/// Packed compare explicit-length strings, return index in ECX/RCX.
Pcmpestri,
/// Packed compare explicit-length strings, return mask in XMM0.
Pcmpestrm,
/// Packed compare implicit-length strings, return index in ECX/RCX.
Pcmpistri,
/// Packed compare implicit-length strings, return mask in XMM0.
Pcmpistrm,
/// Performs logical compare of greater-than on packed integer quadwords.
Pcmpgtq,
/// Perform an AES decryption round using an 128-bit state and a round key.
Aesdec,
/// Perform the last AES decryption round using an 128-bit state and a round key.
Aesdeclast,
/// Perform an AES encryption round using an 128-bit state and a round key.
Aesenc,
/// Perform the last AES encryption round using an 128-bit state and a round key.
Aesenclast,
/// Perform an inverse mix column transformation primitive.
Aesimc,
/// Assist the creation of round keys with a key expansion schedule.
Aeskeygenassist,
/// Perform carryless multiplication of two 64-bit numbers.
Pclmulqdq,
/// Convert eight/four data element containing 16-bit floating-point data into eight/four single-precision floating-point data.
Vcvtph2ps,
/// Convert eight/four data element containing single-precision floating-point data into eight/four 16-bit floating-point data.
Vcvtps2ph,
/// Abort an RTM transaction execution.
Xabort,
/// Prefix hint to the beginning of an HLE transaction region.
Xacquire,
/// Prefix hint to the end of an HLE transaction region.
Xrelease,
/// Transaction begin of an RTM transaction region.
Xbegin,
/// Transaction end of an RTM transaction region.
Xend,
/// Test if executing in a transactional region.
Xtest,
/// Perform an intermediate calculation for the next four SHA1 message dwords from the previous message dwords.
Sha1msg1,
/// Perform the final calculation for the next four SHA1 message dwords from the intermediate message dwords.
Sha1msg2,
/// Calculate SHA1 state E after four rounds.
Sha1nexte,
/// Perform four rounds of SHA1 operations.
Sha1rnds4,
/// Perform an intermediate calculation for the next four SHA256 message dwords.
Sha256msg1,
/// Perform the final calculation for the next four SHA256 message dwords.
Sha256msg2,
/// Perform two rounds of SHA256 operations.
Sha256rnds2,
/// Perform dword alignment of two concatenated source vectors.
Valignd,
/// Perform qword alignment of two concatenated source vectors.
Valignq,
/// Replace the VBLENDVPD instructions (using opmask as select control).
Vblendmpd,
/// Replace the VBLENDVPS instructions (using opmask as select control).
Vblendmps,
/// Compress packed DP elements of a vector.
Vcompresspd,
/// Compress packed SP elements of a vector.
Vcompressps,
/// Convert packed DP FP elements of a vector to packed unsigned 32-bit integers.
Vcvtpd2udq,
/// Convert packed DP FP elements of a vector to packed unsigned 32-bit integers.
Vcvttpd2udq,
/// Convert packed SP FP elements of a vector to packed unsigned 32-bit integers.
Vcvtps2udq,
/// Convert packed SP FP elements of a vector to packed unsigned 32-bit integers.
Vcvttps2udq,
/// Convert packed signed 64-bit integers to packed DP FP elements.
Vcvtqq2pd,
/// Convert packed signed 64-bit integers to packed SP FP elements.
Vcvtqq2ps,
/// Convert the low DP FP element of a vector to an unsigned integer.
Vcvtsd2usi,
/// Convert the low DP FP element of a vector to an unsigned integer.
Vcvttsd2usi,
/// Convert the low SP FP element of a vector to an unsigned integer.
Vcvtss2usi,
/// Convert the low SP FP element of a vector to an unsigned integer.
Vcvttss2usi,
/// Convert packed unsigned 32-bit integers to packed DP FP elements.
Vcvtudq2pd,
/// Convert packed unsigned 32-bit integers to packed SP FP elements.
Vcvtudq2ps,
/// Convert an unsigned integer to the low DP FP element and merge to a vector.
Vcvtusi2usd,
/// Convert an unsigned integer to the low SP FP element and merge to a vector.
Vcvtusi2uss,
/// Expand packed DP elements of a vector.
Vexpandpd,
/// Expand packed SP elements of a vector.
Vexpandps,
/// Extract a vector from a full-length vector with 32-bit granular update.
Vextractf32x4,
/// Extract a vector from a full-length vector with 64-bit granular update.
Vextractf64x4,
/// Extract a vector from a full-length vector with 32-bit granular update.
Vextracti32x4,
/// Extract a vector from a full-length vector with 64-bit granular update.
Vextracti64x4,
/// Perform fix-up to special values in DP FP vectors.
Vfixupimmpd,
/// Perform fix-up to special values in SP FP vectors.
Vfixupimmps,
/// Perform fix-up to special values of the low DP FP element.
Vfixupimmsd,
/// Perform fix-up to special values of the low SP FP element.
Vfixupimmss,
/// Convert the exponent of DP FP elements of a vector into FP values.
Vgetexppd,
/// Convert the exponent of SP FP elements of a vector into FP values.
Vgetexpps,
/// Convert the exponent of the low DP FP element in a vector into FP value.
Vgetexpsd,
/// Convert the exponent of the low SP FP element in a vector into FP value.
Vgetexpss,
/// Convert the mantissa of DP FP elements of a vector into FP values.
Vgetmantpd,
/// Convert the mantissa of SP FP elements of a vector into FP values.
Vgetmantps,
/// Convert the mantissa of the low DP FP element of a vector into FP value.
Vgetmantsd,
/// Convert the mantissa of the low SP FP element of a vector into FP value.
Vgetmantss,
/// Insert a 128-bit vector into a full-length vector with 32/64-bit granular update.
Vinsertf32x4,
/// Insert a 256-bit vector into a full-length vector with 32/64-bit granular update.
Vinsertf64x4,
/// VMOVDQA with 32-bit granular conditional update.
Vmovdqa32,
/// VMOVDQA with 64-bit granular conditional update.
Vmovdqa64,
/// VMOVDQU with 32-bit granular conditional update.
Vmovdqu32,
/// VMOVDQU with 64-bit granular conditional update.
Vmovdqu64,
/// Blend dword elements using opmask as select control.
Vpblendmd,
/// Blend qword elements using opmask as select control.
Vpblendmq,
/// Broadcast from general-purpose register to vector register.
Vpbroadcastd,
/// Broadcast from general-purpose register to vector register.
Vpbroadcastq,
/// Compare packed signed dwords using specified primitive.
Vpcmpd,
/// Compare packed unsigned dwords using specified primitive.
Vpcmud,
/// Compare packed signed quadwords using specified primitive.
Vpcmpq,
/// Compare packed unsigned quadwords using specified primitive.
Vpcmuq,
/// Compress packed 64-bit elements of a vector.
Vpcompressq,
/// Compress packed 32-bit elements of a vector.
Vpcompressd,
/// Full permute of two tables of dword elements overwriting the index vector.
Vpermi2d,
/// Full permute of two tables of qword elements overwriting the index vector.
Vpermi2q,
/// Full permute of two tables of DP elements overwriting the index vector.
Vpermi2pd,
/// Full permute of two tables of SP elements overwriting the index vector.
Vpermi2ps,
/// Full permute of two tables of dword elements overwriting one source table.
Vpermt2d,
/// Full permute of two tables of qword elements overwriting one source table.
Vpermt2q,
/// Full permute of two tables of DP elements overwriting one source table.
Vpermt2pd,
/// Full permute of two tables of SP elements overwriting one source table.
Vpermt2ps,
/// Expand packed dword elements of a vector.
Vpexpandd,
/// Expand packed qword elements of a vector.
Vpexpandq,
/// Compute maximum of packed signed 64-bit integer elements.
Vpmaxsq,
/// Compute maximum of packed unsigned 32-bit integer elements.
Vpmaxud,
/// Compute maximum of packed unsigned 64-bit integer elements.
Vpmaxuq,
/// Compute minimum of packed signed 64-bit integer elements.
Vpminsq,
/// Compute minimum of packed unsigned 32-bit integer elements.
Vpminud,
/// Compute minimum of packed unsigned 64-bit integer elements.
Vpminuq,
/// Down convert qword elements in a vector to byte elements using truncation saturation.
Vpmovsqb,
/// Down convert qword elements in a vector to byte elements using truncation unsigned saturation.
Vpmovusqb,
/// Down convert qword elements in a vector to word elements using truncation saturation.
Vpmovsqw,
/// Down convert qword elements in a vector to word elements using truncation unsigned saturation.
Vpmovusqw,
/// Down convert qword elements in a vector to dword elements using truncation saturation.
Vpmovsqd,
/// Down convert qword elements in a vector to dword elements using truncation unsigned saturation.
Vpmovusqd,
/// Down convert dword elements in a vector to byte elements using truncation saturation.
Vpmovsdb,
/// Down convert dword elements in a vector to byte elements using truncation unsigned saturation.
Vpmovusdb,
/// Down convert dword elements in a vector to word elements using truncation saturation.
Vpmovsdw,
/// Down convert dword elements in a vector to word elements using truncation unsigned saturation.
Vpmovusdw,
/// Rotate dword element left by a constant shift count with conditional update.
Vprold,
/// Rotate qword element left by a constant shift count with conditional update.
Vprolq,
/// Rotate dword element left by shift counts specified in a vector with conditional update.
Vprolvd,
/// Rotate qword element left by shift counts specified in a vector with conditional update.
Vprolvq,
/// Rotate dword element right by a constant shift count with conditional update.
Vprord,
/// Rotate qword element right by a constant shift count with conditional update.
Vprorq,
/// Rotate dword element right by shift counts specified in a vector with conditional update.
Vprorrd,
/// Rotate qword element right by shift counts specified in a vector with conditional update.
Vprorrq,
/// Scatter dword elements in a vector to memory using dword indices.
Vpscatterdd,
/// Scatter qword elements in a vector to memory using dword indices.
Vpscatterdq,
/// Scatter dword elements in a vector to memory using qword indices.
Vpscatterqd,
/// Scatter qword elements in a vector to memory using qword indices.
Vpscatterqq,
/// Shift qwords right by a constant shift count and shifting in sign bits.
Vpsraq,
/// Shift qwords right by shift counts in a vector and shifting in sign bits.
Vpsravq,
/// Perform bitwise NAND of dword elements of two vectors and write results to opmask.
Vptestnmd,
/// Perform bitwise NAND of qword elements of two vectors and write results to opmask.
Vptestnmq,
/// Perform bitwise ternary logic operation of three vectors with 32 bit granular conditional update.
Vpterlogd,
/// Perform bitwise ternary logic operation of three vectors with 64 bit granular conditional update.
Vpterlogq,
/// Perform bitwise AND of dword elements of two vectors and write results to opmask.
Vptestmd,
/// Perform bitwise AND of qword elements of two vectors and write results to opmask.
Vptestmq,
/// Compute approximate reciprocals of packed DP FP elements of a vector.
Vrcp14pd,
/// Compute approximate reciprocals of packed SP FP elements of a vector.
Vrcp14ps,
/// Compute the approximate reciprocal of the low DP FP element of a vector.
Vrcp14sd,
/// Compute the approximate reciprocal of the low SP FP element of a vector.
Vrcp14ss,
/// Round packed DP FP elements of a vector to specified number of fraction bits.
Vrndscalepd,
/// Round packed SP FP elements of a vector to specified number of fraction bits.
Vrndscaleps,
/// Round the low DP FP element of a vector to specified number of fraction bits.
Vrndscalesd,
/// Round the low SP FP element of a vector to specified number of fraction bits.
Vrndscaless,
/// Compute approximate reciprocals of square roots of packed DP FP elements of a vector.
Vrsqrt14pd,
/// Compute approximate reciprocals of square roots of packed SP FP elements of a vector.
Vrsqrt14ps,
/// Compute the approximate reciprocal of square root of the low DP FP element of a vector.
Vrsqrt14sd,
/// Compute the approximate reciprocal of square root of the low SP FP element of a vector.
Vrsqrt14ss,
/// Multiply packed DP FP elements of a vector by powers of two with exponents specified in a second vector.
Vscalepd,
/// Multiply packed SP FP elements of a vector by powers of two with exponents specified in a second vector.
Vscaleps,
/// Multiply the low DP FP element of a vector by powers of two with exponent specified in the corresponding element of a second vector.
Vscalesd,
/// Multiply the low SP FP element of a vector by powers of two with exponent specified in the corresponding element of a second vector.
Vscaless,
/// Scatter SP FP elements in a vector to memory using dword indices.
Vscatterdd,
/// Scatter DP FP elements in a vector to memory using dword indices.
Vscatterdq,
/// Scatter SP FP elements in a vector to memory using qword indices.
Vscatterqd,
/// Scatter DP FP elements in a vector to memory using qword indices.
Vscatterqq,
/// Shuffle 128-bit lanes of a vector with 32 bit granular conditional update.
Vshuff32x4,
/// Shuffle 128-bit lanes of a vector with 64 bit granular conditional update.
Vshuff64x2,
/// Shuffle 128-bit lanes of a vector with 32 bit granular conditional update.
Vshufi32x4,
/// Shuffle 128-bit lanes of a vector with 64 bit granular conditional update.
Vshufi64x2,
/// Convert packed DP FP elements of a vector to packed signed 64-bit integers.
Vcvtpd2qq,
/// Convert packed DP FP elements of a vector to packed signed 64-bit integers.
Vcvttpd2qq,
/// Convert packed DP FP elements of a vector to packed unsigned 64-bit integers.
Vcvtpd2uqq,
/// Convert packed DP FP elements of a vector to packed unsigned 64-bit integers.
Vcvttpd2uqq,
/// Convert packed SP FP elements of a vector to packed signed 64-bit integers.
Vcvtps2qq,
/// Convert packed SP FP elements of a vector to packed signed 64-bit integers.
Vcvttps2qq,
/// Convert packed SP FP elements of a vector to packed unsigned 64-bit integers.
Vcvtps2uqq,
/// Convert packed SP FP elements of a vector to packed unsigned 64-bit integers.
Vcvttps2uqq,
/// Convert packed unsigned 64-bit integers to packed DP FP elements.
Vcvtuqq2pd,
/// Convert packed unsigned 64-bit integers to packed SP FP elements.
Vcvtuqq2ps,
/// Extract a vector from a full-length vector with 64-bit granular update.
Vextractf64x2,
/// Extract a vector from a full-length vector with 64-bit granular update.
Vextracti64x2,
/// Test packed DP FP elements in a vector by numeric category.
Vfpclasspd,
/// Test packed SP FP elements in a vector by special-value category.
Vfpclassps,
/// Test the low DP FP element by numeric category.
Vfpclasssd,
/// Test the low SP FP element by special-value category.
Vfpclassss,
/// Insert a 128-bit vector into a full-length vector with 64-bit granular update.
Vinsertf64x2,
/// Insert a 128-bit vector into a full-length vector with 64-bit granular update.
Vinserti64x2,
/// Convert opmask register to vector register in 32-bit granularity.
Vpmovm2d,
/// Convert opmask register to vector register in 64-bit granularity.
Vpmovm2q,
/// Convert a vector register in 32-bit granularity to an opmask register.
Vpmovb2d,
/// Convert a vector register in 64-bit granularity to an opmask register.
Vpmovq2m,
/// Multiply packed signed 64-bit integer elements of two vectors and store low 64-bit signed result.
Vpmullq,
/// Perform RANGE operation on each pair of DP FP elements of two vectors using specified range primitive in imm8.
Vrangeps,
/// Perform RANGE operation on each pair of SP FP elements of two vectors using specified range primitive in imm8.
Vrangepd,
/// Perform RANGE operation on the pair of low DP FP element of two vectors using specified range primitive in imm8.
Vrangesd,
/// Perform RANGE operation on the pair of low SP FP element of two vectors using specified range primitive in imm8.
Vrangess,
/// Perform Reduction operation on packed DP FP elements of a vector using specified reduction primitive in imm8.
Vreducepd,
/// Perform Reduction operation on packed SP FP elements of a vector using specified reduction primitive in imm8.
Vreduceps,
/// Perform Reduction operation on the low DP FP element of a vector using specified reduction primitive in imm8.
Vreducesd,
/// Perform Reduction operation on the low SP FP element of a vector using specified reduction primitive in imm8.
Vreducess,
/// Double block packed Sum-Absolute-Differences on unsigned bytes.
Vdbpsadbw,
/// VMOVDQU with 8/16-bit granular conditional update.
Vmovdqu8,
/// VMOVDQU with 8/16-bit granular conditional update.
Vmovdqu16,
/// Replaces the VPBLENDVB instruction (using opmask as select control).
Vpblendmb,
/// Blend word elements using opmask as select control.
Vpblendmw,
/// Broadcast from general-purpose register to vector register.
Vpbroadcastb,
/// Broadcast from general-purpose register to vector register.
Vpbroadcastw,
/// Compare packed signed bytes using specified primitive.
Vpcmpb,
/// Compare packed unsigned bytes using specified primitive.
Vpcmub,
/// Compare packed signed words using specified primitive.
Vpcmpw,
/// Compare packed unsigned words using specified primitive.
Vpcmuw,
/// Permute packed word elements.
Vpermw,
/// Full permute from two tables of byte elements overwriting the index vector.
Vpermi2b,
/// Full permute from two tables of word elements overwriting the index vector.
Vpermi2w,
/// Convert opmask register to vector register in 8/16-bit granularity.
Vpmovm2b,
/// Convert opmask register to vector register in 8/16-bit granularity.
Vpmovm2w,
/// Convert a vector register in 8-bit granularity to an opmask register.
Vpmovb2m,
/// Convert a vector register in 16-bit granularity to an opmask register.
Vpmovw2m,
/// Down convert word elements in a vector to byte elements using truncation saturation.
Vpmovswb,
/// Down convert word elements in a vector to byte elements using truncation unsigned saturation.
Vpmovuswb,
/// Shift word elements in a vector left by shift counts in a vector.
Vpsllvw,
/// Shift words right by shift counts in a vector and shifting in sign bits.
Vpsravw,
/// Shift word elements in a vector right by shift counts in a vector.
Vpsrlvw,
/// Perform bitwise NAND of byte elements of two vectors and write results to opmask.
Vptestnmb,
/// Perform bitwise NAND of word elements of two vectors and write results to opmask.
Vptestnmw,
/// Perform bitwise AND of byte elements of two vectors and write results to opmask.
Vptestmb,
/// Perform bitwise AND of word elements of two vectors and write results to opmask.
Vptestmw,
/// Broadcast from opmask register to vector register.
Vpbroadcastm,
/// Detect conflicts within a vector of packed 32-bit integers.
Vpconflictd,
/// Detect conflicts within a vector of packed 64-bit integers.
Vpconflictq,
/// Count the number of leading zero bits of packed dword elements.
Vplzcntd,
/// Count the number of leading zero bits of packed qword elements.
Vplzcntq,
/// Add two 8-bit opmasks.
Kaddb,
/// Add two 16-bit opmasks.
Kaddw,
/// Add two 32-bit opmasks.
Kaddd,
/// Add two 64-bit opmasks.
Kaddq,
/// Logical AND two 8-bit opmasks.
Kandb,
/// Logical AND two 16-bit opmasks.
Kandw,
/// Logical AND two 32-bit opmasks.
Kandd,
/// Logical AND two 64-bit opmasks.
Kandq,
/// Logical AND NOT two 8-bit opmasks.
Kandnb,
/// Logical AND NOT two 16-bit opmasks.
Kandnw,
/// Logical AND NOT two 32-bit opmasks.
Kandnd,
/// Logical AND NOT two 64-bit opmasks.
Kandnq,
/// Move from or move to opmask register of 8-bit data.
Kmovb,
/// Move from or move to opmask register of 16-bit data.
Kmovw,
/// Move from or move to opmask register of 32-bit data.
Kmovd,
/// Move from or move to opmask register of 64-bit data.
Kmovq,
/// Bitwise NOT of two 8-bit opmasks.
Knotb,
/// Bitwise NOT of two 16-bit opmasks.
Knotw,
/// Bitwise NOT of two 32-bit opmasks.
Knotd,
/// Bitwise NOT of two 64-bit opmasks.
Knotq,
/// Logical OR two 8-bit opmasks.
Korb,
/// Logical OR two 16-bit opmasks.
Korw,
/// Logical OR two 32-bit opmasks.
Kord,
/// Logical OR two 64-bit opmasks.
Korq,
/// Update EFLAGS according to the result of bitwise OR of two 8-bit opmasks.
Kortestb,
/// Update EFLAGS according to the result of bitwise OR of two 16-bit opmasks.
Kortestw,
/// Update EFLAGS according to the result of bitwise OR of two 32-bit opmasks.
Kortestd,
/// Update EFLAGS according to the result of bitwise OR of two 64-bit opmasks.
Kortestq,
/// Shift left 8-bit opmask by specified count.
Kshiftlb,
/// Shift left 16-bit opmask by specified count.
Kshiftlw,
/// Shift left 32-bit opmask by specified count.
Kshiftld,
/// Shift left 64-bit opmask by specified count.
Kshiftlq,
/// Shift right 8-bit opmask by specified count.
Kshiftrb,
/// Shift right 16-bit opmask by specified count.
Kshiftrw,
/// Shift right 32-bit opmask by specified count.
Kshiftrd,
/// Shift right 64-bit opmask by specified count.
Kshiftrq,
/// Update EFLAGS according to the result of bitwise TEST of two 8-bit opmasks.
Ktestb,
/// Update EFLAGS according to the result of bitwise TEST of two 16-bit opmasks.
Ktestw,
/// Update EFLAGS according to the result of bitwise TEST of two 32-bit opmasks.
Ktestd,
/// Update EFLAGS according to the result of bitwise TEST of two 64-bit opmasks.
Ktestq,
/// Unpack and interleave two 8-bit opmasks into 16-bit mask.
Kunpckbw,
/// Unpack and interleave two 16-bit opmasks into 32-bit mask.
Kunpckwd,
/// Unpack and interleave two 32-bit opmasks into 64-bit mask.
Kunpckdq,
/// Bitwise logical XNOR of two 8-bit opmasks.
Kxnorb,
/// Bitwise logical XNOR of two 16-bit opmasks.
Kxnorw,
/// Bitwise logical XNOR of two 32-bit opmasks.
Kxnord,
/// Bitwise logical XNOR of two 64-bit opmasks.
Kxnorq,
/// Logical XOR of two 8-bit opmasks.
Kxorb,
/// Logical XOR of two 16-bit opmasks.
Kxorw,
/// Logical XOR of two 32-bit opmasks.
Kxord,
/// Logical XOR of two 64-bit opmasks.
Kxorq,
/// Compute approximate base-2 exponential of packed DP FP elements of a vector.
Vexp2pd,
/// Compute approximate base-2 exponential of packed SP FP elements of a vector.
Vexp2ps,
/// Compute approximate base-2 exponential of the low DP FP element of a vector.
Vexp2sd,
/// Compute approximate base-2 exponential of the low SP FP element of a vector.
Vexp2ss,
/// Compute approximate reciprocals to 28 bits of packed DP FP elements of a vector.
Vrcp28pd,
/// Compute approximate reciprocals to 28 bits of packed SP FP elements of a vector.
Vrcp28ps,
/// Compute the approximate reciprocal to 28 bits of the low DP FP element of a vector.
Vrcp28sd,
/// Compute the approximate reciprocal to 28 bits of the low SP FP element of a vector.
Vrcp28ss,
/// Compute approximate reciprocals of square roots to 28 bits of packed DP FP elements of a vector.
Vrsqrt28pd,
/// Compute approximate reciprocals of square roots to 28 bits of packed SP FP elements of a vector.
Vrsqrt28ps,
/// Compute the approximate reciprocal of square root to 28 bits of the low DP FP element of a vector.
Vrsqrt28sd,
/// Compute the approximate reciprocal of square root to 28 bits of the low SP FP element of a vector.
Vrsqrt28ss,
/// Sparse prefetch of packed DP FP vector with T0 hint using dword indices.
Vgatherpf0dpd,
/// Sparse prefetch of packed SP FP vector with T0 hint using dword indices.
Vgatherpf0dps,
/// Sparse prefetch of packed DP FP vector with T0 hint using qword indices.
Vgatherpf0qpd,
/// Sparse prefetch of packed SP FP vector with T0 hint using qword indices.
Vgatherpf0qps,
/// Sparse prefetch of packed DP FP vector with T1 hint using dword indices.
Vgatherpf1dpd,
/// Sparse prefetch of packed SP FP vector with T1 hint using dword indices.
Vgatherpf1dps,
/// Sparse prefetch of packed DP FP vector with T1 hint using qword indices.
Vgatherpf1qpd,
/// Sparse prefetch of packed SP FP vector with T1 hint using qword indices.
Vgatherpf1qps,
/// Sparse prefetch of packed DP FP vector with T0 hint to write using dword indices.
Vscatterpf0dpd,
/// Sparse prefetch of packed SP FP vector with T0 hint to write using dword indices.
Vscatterpf0dps,
/// Sparse prefetch of packed DP FP vector with T0 hint to write using qword indices.
Vscatterpf0qpd,
/// Sparse prefetch of packed SP FP vector with T0 hint to write using qword indices.
Vscatterpf0qps,
/// Sparse prefetch of packed DP FP vector with T1 hint to write using dword indices.
Vscatterpf1dpd,
/// Sparse prefetch of packed SP FP vector with T1 hint to write using dword indices.
Vscatterpf1dps,
/// Sparse prefetch of packed DP FP vector with T1 hint to write using qword indices.
Vscatterpf1qpd,
/// Sparse prefetch of packed SP FP vector with T1 hint to write using qword indices.
Vscatterpf1qps,
/// Clear AC Flag in EFLAGS register.
Clac,
/// Set AC Flag in EFLAGS register.
Stac,
/// Load global descriptor table (GDT) register.
Lgdt,
/// Store global descriptor table (GDT) register.
Sgdt,
/// Load local descriptor table (LDT) register.
Lldt,
/// Store local descriptor table (LDT) register.
Sldt,
/// Load task register.
Ltr,
/// Store task register.
Str,
/// Load interrupt descriptor table (IDT) register.
Lidt,
/// Store interrupt descriptor table (IDT) register.
Sidt,
/// Load machine status word.
Lmsw,
/// Store machine status word.
Smsw,
/// Clear the task-switched flag.
Clts,
/// Adjust requested privilege level.
Arpl,
/// Load access rights.
Lar,
/// Load segment limit.
Lsl,
/// Verify segment for reading
Verr,
/// Verify segment for writing.
Verw,
/// Invalidate cache, no writeback.
Invd,
/// Invalidate cache, with writeback.
Wbinvd,
/// Invalidate TLB Entry.
Invlpg,
/// Invalidate Process-Context Identifier.
Invpcid,
/// (prefix) Perform atomic access to memory (can be applied to a number of general purpose instructions that provide memory source/destination access).
Lock,
/// Halt processor.
Hlt,
/// Return from system management mode (SMM).
Rsm,
/// Read model-specific register.
Rdmsr,
/// Write model-specific register.
Wrmsr,
/// Read performance monitoring counters.
Rdpmc,
/// Read time stamp counter.
Rdtsc,
/// Read time stamp counter and processor ID.
Rdtscp,
/// Fast System Call, transfers to a flat protected mode kernel at CPL = 0.
Sysenter,
/// Fast System Call, transfers to a flat protected mode kernel at CPL = 3.
Sysexit,
/// Save processor supervisor-mode extended states to memory.
Xsaves,
/// Restore processor supervisor-mode extended states from memory.
Xrstors,
/// Reads the state of an extended control register.
Xgetbv,
/// Writes the state of an extended control register.
Xsetbv,
/// Reads from FS base address at any privilege level.
Rdfsbase,
/// Reads from GS base address at any privilege level.
Rdgsbase,
/// Writes to FS base address at any privilege level.
Wrfsbase,
/// Writes to GS base address at any privilege level.
Wrgsbase,
/// Convert doubleword to quadword.
Cdqe,
/// Compare string operands.
Cmpsq,
/// Compare RDX:RAX with m128.
Cmpxchg16b,
/// Load qword at address (R)SI into RAX.
Lodsq,
/// Move qword from address (R)SI to (R)DI.
Movsq,
/// Store RAX at address RDI.
Stosq,
/// Exchanges current GS base register value with value in MSR address C0000102H.
Swapgs,
/// Fast call to privilege level 0 system procedures.
Syscall,
/// Return from fast systemcall.
Sysret,
/// Takes a single 64-bit source operand in memory. It makes the referenced VMCS active and current.
Vmptrld,
/// Takes a single 64-bit destination operand that is in memory. Current-VMCS pointer is stored into the destination operand.
Vmptrst,
/// Takes a single 64-bit operand in memory. The instruction sets the launch state of the VMCS referenced by the operand to “clear”, renders that VMCS inactive, and ensures that data for the VMCS have been written to the VMCS-data area in the referenced VMCS region.
Vmclear,
/// Reads a component from the VMCS (the encoding of that field is given in a register operand) and stores it into a destination operand.
Vmread,
/// Writes a component to the VMCS (the encoding of that field is given in a register operand) from a source operand.
Vmwrite,
/// Launches a virtual machine managed by the VMCS. A VM entry occurs, transferring control to the VM.
Vmlaunch,
/// Resumes a virtual machine managed by the VMCS. A VM entry occurs, transferring control to the VM.
Vmresume,
/// Causes the processor to leave VMX operation.
Vmxoff,
/// Takes a single 64-bit source operand in memory. It causes a logical processor to enter VMX root operation and to use the memory referenced by the operand to support VMX operation.
Vmxon,
/// Invalidate cached Extended Page Table (EPT) mappings in the processor to synchronize address translation in virtual machines with memory-resident EPT pages.
Invept,
/// Invalidate cached mappings of address translation based on the Virtual Processor ID (VPID).
Invvpid,
/// Allows a guest in VMX non-root operation to call the VMM for service. A VM exit occurs, transferring control to the VMM.
Vmcall,
/// This instruction allows software in VMX non-root operation to invoke a VM function, which is processor functionality enabled and configured by software in VMX root operation. No VM , /// exit occurs.
Vmfunc,
/// Returns the available leaf functions of the GETSEC instruction.
GetsecCapabilities,
/// Loads an authenticated code chipset module and enters authenticated code execution mode.
GetsecEnteraccs,
/// Exits authenticated code execution mode.
GetsecExitac,
/// Establishes a Measured Launched Environment (MLE) which has its dynamic root of trust anchored to a chipset supporting Intel Trusted Execution Technology.
GetsecSenter,
/// Exits the MLE.
GetsecSexit,
/// Returns SMX related parameter information.
GetsecParameters,
/// SMX mode control.
GetsecSmcrtl,
/// Wakes up sleeping logical processors inside an MLE.
GetsecWakeup,
/// Create a LowerBound and a UpperBound in a register.
Bndmk,
/// Check the address of a memory reference against a LowerBound.
Bndcl,
/// Check the address of a memory reference against an UpperBound in 1’s compliment form.
Bndcu,
/// Check the address of a memory reference against an UpperBound not in 1’s compliment form.
Bndcn,
/// Copy or load from memory of the LowerBound and UpperBound to a register.
Bndmov,
/// Load bounds using address translation.
Bndldx,
/// Store bounds using address translation.
Bndstx,
/// Clear busy bit in a supervisor shadow stack token.
Clrssbsy,
/// Increment the shadow stack pointer (SSP).
Incssp,
/// Add a page
EnclsEadd,
/// Block an EPC page
EnclsEblock,
/// Create an enclave
EnclsEcreate,
/// Read data by debugger
EnclsEdbgrd,
/// Write data by debugger
EnclsEdbgwr,
/// Extend EPC page measurement
EnclsEextend,
/// Initialize an enclave
EnclsEinit,
/// Load an EPC page as blocked
EnclsEldb,
/// Load an EPC page as unblocked
EnclsEldu,
/// Add version array
EnclsEpa,
/// Remove a page from EPC
EnclsEremove,
/// Activate EBLOCK checks
EnclsEtrack,
/// Write back/invalidate an EPC page
EnclsEwb,
/// Enter an Enclave
EncluEenter,
/// Exit an Enclave
EncluEexit,
/// Create a cryptographic key
EncluEgetkey,
/// Create a cryptographic report
EncluEreport,
/// Re-enter an Enclave
EncluEresume,
/// Read shadow stack point (SSP).
Rdssp,
/// Restore a shadow stack pointer (SSP).
Rstorssp,
/// Save previous shadow stack pointer (SSP).
Saveprevssp,
/// Set busy bit in a supervisor shadow stack token.
Setssbsy,
/// Write to a shadow stack.
Wrss,
/// Write to a user mode shadow stack.
Wruss,
/// Terminate an Indirect Branch in 32-bit and Compatibility Mode.
Endbr32,
/// Terminate an Indirect Branch in 64-bit Mode.
Endbr64,
*/
