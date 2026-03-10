use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// IF near call
///     THEN IF near relative call
///         THEN
///             IF OperandSize = 64
///                 THEN
///                     tempDEST := SignExtend(DEST); (* DEST is rel32 *)
///                     tempRIP := RIP + tempDEST;
///                     IF stack not large enough for a 8-byte return address
///                         THEN #SS(0); FI;
///                     Push(RIP);
///                     IF ShadowStackEnabled(CPL) AND DEST != 0
///                         ShadowStackPush8B(RIP);
///                     FI;
///                     RIP := tempRIP;
///             FI;
///             IF OperandSize = 32
///                 THEN
///                     tempEIP := EIP + DEST; (* DEST is rel32 *)
///                     IF tempEIP is not within code segment limit THEN #GP(0); FI;
///                     IF stack not large enough for a 4-byte return address
///                         THEN #SS(0); FI;
///                     Push(EIP);
///                     IF ShadowStackEnabled(CPL) AND DEST != 0
///                         ShadowStackPush4B(EIP);
///                     FI;
///                     EIP := tempEIP;
///             FI;
///             IF OperandSize = 16
///                 THEN
///                     tempEIP := (EIP + DEST) AND 0000FFFFH; (* DEST is rel16 *)
///                     IF tempEIP is not within code segment limit THEN #GP(0); FI;
///                     IF stack not large enough for a 2-byte return address
///                         THEN #SS(0); FI;
///                     Push(IP);
///                     IF ShadowStackEnabled(CPL) AND DEST != 0
///                         (* IP is zero extended and pushed as a 32 bit value on shadow stack *)
///                         ShadowStackPush4B(IP);
///                     FI;
///                     EIP := tempEIP;
///             FI;
///             IF OperandSize = 64
///                 THEN
///                     tempRIP := DEST; (* DEST is r/m64 *)
///                     IF stack not large enough for a 8-byte return address
///                         THEN #SS(0); FI;
///                     Push(RIP);
///                     IF ShadowStackEnabled(CPL)
///                         ShadowStackPush8B(RIP);
///                     FI;
///                     RIP := tempRIP;
///             FI;
///             IF OperandSize = 32
///                 THEN
///                     tempEIP := DEST; (* DEST is r/m32 *)
///                     IF tempEIP is not within code segment limit THEN #GP(0); FI;
///                     IF stack not large enough for a 4-byte return address
///                         THEN #SS(0); FI;
///                     Push(EIP);
///                     IF ShadowStackEnabled(CPL)
///                         ShadowStackPush4B(EIP);
///                     FI;
///                     EIP := tempEIP;
///             FI;
///             IF OperandSize = 16
///                 THEN
///                     tempEIP := DEST AND 0000FFFFH; (* DEST is r/m16 *)
///                     IF tempEIP is not within code segment limit THEN #GP(0); FI;
///                     IF stack not large enough for a 2-byte return address
///                         THEN #SS(0); FI;
///                     Push(IP);
///                     IF ShadowStackEnabled(CPL)
///                         (* IP is zero extended and pushed as a 32 bit value on shadow stack *)
///                         ShadowStackPush4B(IP);
///                     FI;
///                     EIP := tempEIP;
///             FI;
///     FI;rel/abs
///     IF (Call near indirect, absolute indirect)
///         IF EndbranchEnabledAndNotSuppressed(CPL)
///             IF CPL = 3
///                 THEN
///                     IF ( no 3EH prefix OR IA32_U_CET.NO_TRACK_EN == 0 )
///                         THEN
///                             IA32_U_CET.TRACKER = WAIT_FOR_ENDBRANCH
///                     FI;
///                 ELSE
///                     IF ( no 3EH prefix OR IA32_S_CET.NO_TRACK_EN == 0 )
///                         THEN
///                             IA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCH
///                     FI;
///             FI;
///         FI;
///     FI;
/// FI; near
/// IF far call and (PE = 0 or (PE = 1 and VM = 1)) (* Real-address or virtual-8086 mode *)
///     THEN
///         IF OperandSize = 32
///             THEN
///                 IF stack not large enough for a 6-byte return address
///                     THEN #SS(0); FI;
///                 IF DEST[31:16] is not zero THEN #GP(0); FI;
///                 Push(CS); (* Padded with 16 high-order bits *)
///                 Push(EIP);
///                 CS := DEST[47:32]; (* DEST is ptr16:32 or [m16:32] *)
///                 EIP := DEST[31:0]; (* DEST is ptr16:32 or [m16:32] *)
///             ELSE (* OperandSize = 16 *)
///                 IF stack not large enough for a 4-byte return address
///                     THEN #SS(0); FI;
///                 Push(CS);
///                 Push(IP);
///                 CS := DEST[31:16]; (* DEST is ptr16:16 or [m16:16] *)
///                 EIP := DEST[15:0]; (* DEST is ptr16:16 or [m16:16]; clear upper 16 bits *)
///         FI;
/// FI;
/// IF far call and (PE = 1 and VM = 0) (* Protected mode or IA-32e Mode, not virtual-8086 mode*)
///     THEN
///         IF segment selector in target operand NULL
///             THEN #GP(0); FI;
///         IF segment selector index not within descriptor table limits
///             THEN #GP(new code segment selector); FI;
///         Read type and access rights of selected segment descriptor;
///         IF IA32_EFER.LMA = 0
///             THEN
///                 IF segment type is not a conforming or nonconforming code segment, call
///                 gate, task gate, or TSS
///                     THEN #GP(segment selector); FI;
///             ELSE
///                 IF segment type is not a conforming or nonconforming code segment or
///                 64-bit call gate,
///                     THEN #GP(segment selector); FI;
///         FI;
///         Depending on type and access rights:
///             GO TO CONFORMING-CODE-SEGMENT;
///             GO TO NONCONFORMING-CODE-SEGMENT;
///             GO TO CALL-GATE;
///             GO TO TASK-GATE;
///             GO TO TASK-STATE-SEGMENT;
/// FI;
/// CONFORMING-CODE-SEGMENT:
///     IF L bit = 1 and D bit = 1 and IA32_EFER.LMA = 1
///         THEN GP(new code segment selector); FI;
///     IF DPL > CPL
///         THEN #GP(new code segment selector); FI;
///     IF segment not present
///         THEN #NP(new code segment selector); FI;
///     IF stack not large enough for return address
///         THEN #SS(0); FI;
///     tempEIP := DEST(Offset);
///     IF target mode = Compatibility mode
/// 
///         THEN tempEIP := tempEIP AND 00000000_FFFFFFFFH; FI;
///     IF OperandSize = 16
///         THEN
///             tempEIP := tempEIP AND 0000FFFFH; FI; (* Clear upper 16 bits *)
///     IF (IA32_EFER.LMA = 0 or target mode = Compatibility mode) and (tempEIP outside new code segment limit)
///         THEN #GP(0); FI;
///     IF tempEIP is non-canonical
///         THEN #GP(0); FI;
///     IF ShadowStackEnabled(CPL)
///         IF OperandSize = 32
///             THEN
///                 tempPushLIP = CSBASE + EIP;
///             ELSE
///                 IF OperandSize = 16
///                     THEN
///                         tempPushLIP = CSBASE + IP;
///                     ELSE (* OperandSize = 64 *)
///                         tempPushLIP = RIP;
///                 FI;
///         FI;
///         tempPushCS = CS;
///     FI;
///     IF OperandSize = 32
///         THEN
///             Push(CS); (* Padded with 16 high-order bits *)
///             Push(EIP);
///             CS := DEST(CodeSegmentSelector);
///             (* Segment descriptor information also loaded *)
///             CS(RPL) := CPL;
///             EIP := tempEIP;
///         ELSE
///             IF OperandSize = 16
///                 THEN
///                     Push(CS);
///                     Push(IP);
///                     CS := DEST(CodeSegmentSelector);
///                     (* Segment descriptor information also loaded *)
///                     CS(RPL) := CPL;
///                     EIP := tempEIP;
///                 ELSE (* OperandSize = 64 *)
///                     Push(CS); (* Padded with 48 high-order bits *)
///                     Push(RIP);
///                     CS := DEST(CodeSegmentSelector);
///                     (* Segment descriptor information also loaded *)
///                     CS(RPL) := CPL;
///                     RIP := tempEIP;
///             FI;
///     FI;
///     IF ShadowStackEnabled(CPL)
///             (* If target is legacy or compatibility mode then the SSP must be in low 4GB *)
///             IF (SSP & 0xFFFFFFFF00000000 != 0)
///                 THEN #GP(0); FI;
///         FI;
///         (* align to 8 byte boundary if not already aligned *)
///         tempSSP = SSP;
///         Shadow_stack_store 4 bytes of 0 to (SSP - 4)
///         SSP = SSP & 0xFFFFFFFFFFFFFFF8H
///         ShadowStackPush8B(tempPushCS); (* Padded with 48 high-order bits of 0 *)
///         ShadowStackPush8B(tempPushLIP); (* Padded with 32 high-order bits of 0 for 32 bit LIP*)
///         ShadowStackPush8B(tempSSP);
///     FI;
///     IF EndbranchEnabled(CPL)
///         IF CPL = 3
///             THEN
///                 IA32_U_CET.TRACKER = WAIT_FOR_ENDBRANCH
///                 IA32_U_CET.SUPPRESS = 0
///             ELSE
///                 IA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCH
///                 IA32_S_CET.SUPPRESS = 0
///         FI;
///     FI;
/// END;
/// NONCONFORMING-CODE-SEGMENT:
///     IF L-Bit = 1 and D-BIT = 1 and IA32_EFER.LMA = 1
///         THEN GP(new code segment selector); FI;
///     IF (RPL > CPL) or (DPL ≠ CPL)
///         THEN #GP(new code segment selector); FI;
///     IF segment not present
///         THEN #NP(new code segment selector); FI;
///     IF stack not large enough for return address
///         THEN #SS(0); FI;
///     tempEIP := DEST(Offset);
///     IF target mode = Compatibility mode
/// 
///         THEN tempEIP := tempEIP AND 00000000_FFFFFFFFH; FI;
///     IF OperandSize = 16
///         THEN tempEIP := tempEIP AND 0000FFFFH; FI; (* Clear upper 16 bits *)
///     IF (IA32_EFER.LMA = 0 or target mode = Compatibility mode) and (tempEIP outside new code segment limit)
///         THEN #GP(0); FI;
///     IF tempEIP is non-canonical
///         THEN #GP(0); FI;
///     IF ShadowStackEnabled(CPL)
///         IF IA32_EFER.LMA & CS.L
/// 
///             tempPushLIP = RIP
/// 
///         ELSE
/// 
///             tempPushLIP = CSBASE + EIP;
/// 
///         FI;
///         tempPushCS = CS;
///     FI;
///     IF OperandSize = 32
///         THEN
///             Push(CS); (* Padded with 16 high-order bits *)
///             Push(EIP);
///             CS := DEST(CodeSegmentSelector);
///             (* Segment descriptor information also loaded *)
///             CS(RPL) := CPL;
///             EIP := tempEIP;
///         ELSE
///             IF OperandSize = 16
///                 THEN
///                     Push(CS);
///                     Push(IP);
///                     CS := DEST(CodeSegmentSelector);
///                     (* Segment descriptor information also loaded *)
///                     CS(RPL) := CPL;
///                     EIP := tempEIP;
///                 ELSE (* OperandSize = 64 *)
///                     Push(CS); (* Padded with 48 high-order bits *)
///                     Push(RIP);
///                     CS := DEST(CodeSegmentSelector);
///                     (* Segment descriptor information also loaded *)
///                     CS(RPL) := CPL;
///                     RIP := tempEIP;
///             FI;
///     FI;
///     IF ShadowStackEnabled(CPL)
///         IF (IA32_EFER.LMA and DEST(CodeSegmentSelector).L) = 0
///             (* If target is legacy or compatibility mode then the SSP must be in low 4GB *)
///             IF (SSP & 0xFFFFFFFF00000000 != 0)
///                 THEN #GP(0); FI;
///         FI;
///     (* align to 8 byte boundary if not already aligned *)
///     tempSSP = SSP;
///     Shadow_stack_store 4 bytes of 0 to (SSP - 4)
///     SSP = SSP & 0xFFFFFFFFFFFFFFF8H
///     ShadowStackPush8B(tempPushCS); (* Padded with 48 high-order 0 bits *)
///     ShadowStackPush8B(tempPushLIP); (* Padded 32 high-order bits of 0 for 32 bit LIP*)
///     ShadowStackPush8B(tempSSP);
///     FI;
///     IF EndbranchEnabled(CPL)
///         IF CPL = 3
///             THEN
///                 IA32_U_CET.TRACKER = WAIT_FOR_ENDBRANCH
///                 IA32_U_CET.SUPPRESS = 0
///             ELSE
///                 IA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCH
///                 IA32_S_CET.SUPPRESS = 0
///         FI;
///     FI;
/// END;
/// CALL-GATE:
///     IF call gate (DPL < CPL) or (RPL > DPL)
///         THEN #GP(call-gate selector); FI;
///     IF call gate not present
///         THEN #NP(call-gate selector); FI;
///         THEN #GP(0); FI;
///     IF call-gate code-segment selector index is outside descriptor table limits
///         THEN #GP(call-gate code-segment selector); FI;
///     Read call-gate code-segment descriptor;
///     IF call-gate code-segment descriptor does not indicate a code segment
///     or call-gate code-segment descriptor DPL > CPL
///         THEN #GP(call-gate code-segment selector); FI;
///     IF IA32_EFER.LMA = 1 AND (call-gate code-segment descriptor is
///     not a 64-bit code segment or call-gate code-segment descriptor has both L-bit and D-bit set)
///         THEN #GP(call-gate code-segment selector); FI;
///     IF call-gate code segment not present
///         THEN #NP(call-gate code-segment selector); FI;
///     IF call-gate code segment is non-conforming and DPL < CPL
///         THEN go to MORE-PRIVILEGE;
///         ELSE go to SAME-PRIVILEGE;
///     FI;
/// END;
/// MORE-PRIVILEGE:
///     IF current TSS is 32-bit
///         THEN
///             TSSstackAddress := (new code-segment DPL * 8) + 4;
///             IF (TSSstackAddress + 5) > current TSS limit
///                 THEN #TS(current TSS selector); FI;
///             NewSS := 2 bytes loaded from (TSS base + TSSstackAddress + 4);
///             NewESP := 4 bytes loaded from (TSS base + TSSstackAddress);
///         ELSE
///             IF current TSS is 16-bit
///                 THEN
///                     TSSstackAddress := (new code-segment DPL * 4) + 2
///                     IF (TSSstackAddress + 3) > current TSS limit
///                         THEN #TS(current TSS selector); FI;
///                     NewSS := 2 bytes loaded from (TSS base + TSSstackAddress + 2);
///                     NewESP := 2 bytes loaded from (TSS base + TSSstackAddress);
///                 ELSE (* current TSS is 64-bit *)
///                     TSSstackAddress := (new code-segment DPL * 8) + 4;
///                     IF (TSSstackAddress + 7) > current TSS limit
///                         THEN #TS(current TSS selector); FI;
///                     NewSS := new code-segment DPL; (* NULL selector with RPL = new CPL *)
///                     NewRSP := 8 bytes loaded from (current TSS base + TSSstackAddress);
///             FI;
///     FI;
///     IF IA32_EFER.LMA = 0 and NewSS is NULL
///         THEN #TS(NewSS); FI;
///     Read new stack-segment descriptor;
///     IF IA32_EFER.LMA = 0 and (NewSS RPL ≠ new code-segment DPL
///     or new stack-segment DPL ≠ new code-segment DPL or new stack segment is not a
///     writable data segment)
///         THEN #TS(NewSS); FI
///     IF IA32_EFER.LMA = 0 and new stack segment not present
///         THEN #SS(NewSS); FI;
///     IF CallGateSize = 32
///         THEN
///             IF new stack does not have room for parameters plus 16 bytes
///                 THEN #SS(NewSS); FI;
///             IF CallGate(InstructionPointer) not within new code-segment limit
///                 THEN #GP(0); FI;
///             SS := newSS; (* Segment descriptor information also loaded *)
///             ESP := newESP;
///             CS:EIP := CallGate(CS:InstructionPointer);
///             (* Segment descriptor information also loaded *)
///             Push(oldSS:oldESP); (* From calling procedure *)
///             temp := parameter count from call gate, masked to 5 bits;
///             Push(parameters from calling procedure's stack, temp)
///             Push(oldCS:oldEIP); (* Return address to calling procedure *)
///         ELSE
///             IF CallGateSize = 16
///                 THEN
///                     IF new stack does not have room for parameters plus 8 bytes
///                         THEN #SS(NewSS); FI;
///                     IF (CallGate(InstructionPointer) AND FFFFH) not in new code-segment limit
///                         THEN #GP(0); FI;
///                     SS := newSS; (* Segment descriptor information also loaded *)
///                     ESP := newESP;
///                     CS:IP := CallGate(CS:InstructionPointer);
///                     (* Segment descriptor information also loaded *)
///                     Push(oldSS:oldESP); (* From calling procedure *)
///                     temp := parameter count from call gate, masked to 5 bits;
///                     Push(parameters from calling procedure's stack, temp)
///                     Push(oldCS:oldEIP); (* Return address to calling procedure *)
///                 ELSE (* CallGateSize = 64 *)
///                     IF pushing 32 bytes on the stack would use a non-canonical address
///                         THEN #SS(NewSS); FI;
///                     IF (CallGate(InstructionPointer) is non-canonical)
///                         THEN #GP(0); FI;
///                     SS := NewSS; (* NewSS is NULL)
///                     RSP := NewESP;
///                     CS:IP := CallGate(CS:InstructionPointer);
///                     (* Segment descriptor information also loaded *)
///                     Push(oldSS:oldESP); (* From calling procedure *)
///                     Push(oldCS:oldEIP); (* Return address to calling procedure *)
///             FI;
///     FI;
///     IF ShadowStackEnabled(CPL) AND CPL = 3
///         THEN
///             IF IA32_EFER.LMA = 0
///                 THEN IA32_PL3_SSP := SSP;
///                 ELSE (* adjust so bits 63:N get the value of bit N-1, where N is the CPU's maximum linear-address width *)
///                     IA32_PL3_SSP := LA_adjust(SSP);
///             FI;
///     FI;
///     CPL := CodeSegment(DPL)
///     CS(RPL) := CPL
///     IF ShadowStackEnabled(CPL)
///         oldSSP := SSP
///         SSP := IA32_PLi_SSP; (* where i is the CPL *)
///         IF SSP & 0x07 != 0 (* if SSP not aligned to 8 bytes then #GP *)
///         (* Token and CS:LIP:oldSSP pushed on shadow stack must be contained in a naturally aligned 32-byte region*)
///         IF (SSP & ~0x1F) != ((SSP - 24) & ~0x1F)
///             #GP(0); FI;
///         IF ((IA32_EFER.LMA and CS.L) = 0 AND SSP[63:32] != 0)
///             THEN #GP(0); FI;
///         expected_token_value = SSP
///                                 (* busy bit - bit position 0 - must be clear *)
///         new_token_value = SSP | BUSY_BIT   (* Set the busy bit *)
///         IF shadow_stack_lock_cmpxchg8b(SSP, new_token_value, expected_token_value) != expected_token_value
///             THEN #GP(0); FI;
///         IF oldSS.DPL != 3
///             ShadowStackPush8B(oldCS); (* Padded with 48 high-order bits of 0 *)
///             ShadowStackPush8B(oldCSBASE+oldRIP); (* Padded with 32 high-order bits of 0 for 32 bit LIP*)
///             ShadowStackPush8B(oldSSP);
///         FI;
///     FI;
///     IF EndbranchEnabled (CPL)
///         IA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCH
///         IA32_S_CET.SUPPRESS = 0
///     FI;
/// END;
/// SAME-PRIVILEGE:
///     IF CallGateSize = 32
///         THEN
///             IF stack does not have room for 8 bytes
///                 THEN #SS(0); FI;
///             IF CallGate(InstructionPointer) not within code segment limit
///                 THEN #GP(0); FI;
///             CS:EIP := CallGate(CS:EIP) (* Segment descriptor information also loaded *)
///             Push(oldCS:oldEIP); (* Return address to calling procedure *)
///         ELSE
///             If CallGateSize = 16
///                 THEN
///                     IF stack does not have room for 4 bytes
///                         THEN #SS(0); FI;
///                     IF CallGate(InstructionPointer) not within code segment limit
///                         THEN #GP(0); FI;
///                     CS:IP := CallGate(CS:instruction pointer);
///                     (* Segment descriptor information also loaded *)
///                     Push(oldCS:oldIP); (* Return address to calling procedure *)
///                 ELSE (* CallGateSize = 64)
///                     IF pushing 16 bytes on the stack touches non-canonical addresses
///                         THEN #SS(0); FI;
///                     IF RIP non-canonical
///                         THEN #GP(0); FI;
///                     CS:IP := CallGate(CS:instruction pointer);
///                     (* Segment descriptor information also loaded *)
///                     Push(oldCS:oldIP); (* Return address to calling procedure *)
///             FI;
///     FI;
///     CS(RPL) := CPL
///     IF ShadowStackEnabled(CPL)
///         (* Align to next 8 byte boundary *)
///         Shadow_stack_store 4 bytes of 0 to (SSP - 4)
///         SSP = SSP & 0xFFFFFFFFFFFFFFF8H;
///         (* push cs:lip:ssp on shadow stack *)
///         ShadowStackPush8B(oldCS); (* Padded with 48 high-order bits of 0 *)
///         ShadowStackPush8B(oldCSBASE + oldRIP); (* Padded with 32 high-order bits of 0 for 32 bit LIP*)
///         ShadowStackPush8B(tempSSP);
///     FI;
///     IF EndbranchEnabled (CPL)
///         IF CPL = 3
///             THEN
///                 IA32_U_CET.TRACKER = WAIT_FOR_ENDBRANCH;
///                 IA32_U_CET.SUPPRESS = 0
///             ELSE
///                 IA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCH;
///                 IA32_S_CET.SUPPRESS = 0
///         FI;
///     FI;
/// END;
/// TASK-GATE:
///     IF task gate DPL < CPL or RPL
///         THEN #GP(task gate selector); FI;
///     IF task gate not present
///         THEN #NP(task gate selector); FI;
///     Read the TSS segment selector in the task-gate descriptor;
///     IF TSS segment selector local/global bit is set to local
///     or index not within GDT limits
///         THEN #GP(TSS selector); FI;
///     Access TSS descriptor in GDT;
///     IF descriptor is not a TSS segment
/// THEN #GP(TSS selector); FI;
///     IF TSS descriptor specifies that the TSS is busy
/// THEN #GP(TSS selector); FI;
///     IF TSS not present
///         THEN #NP(TSS selector); FI;
///     SWITCH-TASKS (with nesting) to TSS;
///     IF EIP not within code segment limit
///         THEN #GP(0); FI;
/// END;
/// TASK-STATE-SEGMENT:
///     IF TSS DPL < CPL or RPL
///     or TSS descriptor indicates TSS not available
///         THEN #GP(TSS selector); FI;
///     IF TSS is not present
///         THEN #NP(TSS selector); FI;
///     SWITCH-TASKS (with nesting) to TSS;
///     IF EIP not within code segment limit
///         THEN #GP(0); FI;
/// END;
/// ```
#[box_to_static_reference]
pub(super) fn call() -> &'static [IrStatement] {
    let set_sp = assign(b::sub(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    let ret_address = b::add(rip.clone(), instruction_byte_size());
    let save_ret = assign(ret_address, d(rsp.clone()), size_architecture());
    let call = jump_by_call(o1());
    let type1 = type_specified(o1(), o1_size(), DataType::Address);
    let type2 = type_specified(rsp.clone(), size_architecture(), DataType::Address);
    let type3 = type_specified(rip.clone(), size_architecture(), DataType::Address);
    [set_sp, save_ret, call, type1, type2, type3].into()
}

/// # Pseudocode
/// ```text
/// IF OperandSize = 16 (* Instruction = CBW *)
///     THEN
///         AX := SignExtend(AL);
///     ELSE IF (OperandSize = 32, Instruction = CWDE)
///         EAX := SignExtend(AX); FI;
///     ELSE (* 64-Bit Mode, OperandSize = 64, Instruction = CDQE*)
///         RAX := SignExtend(EAX);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cbw() -> &'static [IrStatement] {
    let ext = assign(u::sign_extend(al.clone()), ax.clone(), size_relative(ax.clone()));
    let type1 = type_specified(al.clone(), size_relative(al.clone()), DataType::Int);
    let type2 = type_specified(ax.clone(), size_relative(ax.clone()), DataType::Int);
    [ext, type1, type2].into()
}

/// # Pseudocode
/// ```text
/// IF OperandSize = 16 (* CWD instruction *)
///     THEN
///         DX := SignExtend(AX);
///     ELSE IF OperandSize = 32 (* CDQ instruction *)
///         EDX := SignExtend(EAX); FI;
///     ELSE IF 64-Bit Mode and OperandSize = 64 (* CQO instruction*)
///         RDX := SignExtend(RAX); FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cdq() -> &'static [IrStatement] {
    let set_tmp = assign(u::sign_extend(eax.clone()), tmp64.clone(), size_relative(tmp64.clone()));
    let set_dx = assign(b::shr(tmp64.clone(), c(16)), edx.clone(), size_relative(edx.clone()));
    let set_ax = assign(tmp64.clone(), eax.clone(), size_relative(eax.clone()));
    let type1 = type_specified(eax.clone(), size_relative(eax.clone()), DataType::Int);
    let type2 = type_specified(edx.clone(), size_relative(edx.clone()), DataType::Int);
    [set_tmp, set_dx, set_ax, type1, type2].into()
}

/// # Pseudocode
/// ```text
/// IF OperandSize = 16 (* Instruction = CBW *)
///     THEN
///         AX := SignExtend(AL);
///     ELSE IF (OperandSize = 32, Instruction = CWDE)
///         EAX := SignExtend(AX); FI;
///     ELSE (* 64-Bit Mode, OperandSize = 64, Instruction = CDQE*)
///         RAX := SignExtend(EAX);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cdqe() -> &'static [IrStatement] {
    let ext = assign(u::sign_extend(eax.clone()), rax.clone(), size_relative(rax.clone()));
    let type1 = type_specified(eax.clone(), size_relative(eax.clone()), DataType::Int);
    let type2 = type_specified(rax.clone(), size_relative(rax.clone()), DataType::Int);
    [ext, type1, type2].into()
}

/// # Pseudocode
/// ```text
/// EFLAGS.AC := 0;
/// ```
#[box_to_static_reference]
pub(super) fn clac() -> &'static [IrStatement] {
    [exception("clac")].into()
}

/// # Pseudocode
/// ```text
/// CF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn clc() -> &'static [IrStatement] {
    let set = assign(c(0), cf.clone(), size_relative(cf.clone()));
    [set].into()
}

/// # Pseudocode
/// ```text
/// DF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn cld() -> &'static [IrStatement] {
    let set = assign(c(0), df.clone(), size_relative(df.clone()));
    [set].into()
}

/// # Pseudocode
/// ```text
/// Cache_Line_Demote(m8);
/// ```
#[box_to_static_reference]
pub(super) fn cldemote() -> &'static [IrStatement] {
    [exception("cldemote")].into()
}

/// # Pseudocode
/// ```text
/// Flush_Cache_Line(SRC);
/// ```
#[box_to_static_reference]
pub(super) fn clflush() -> &'static [IrStatement] {
    [exception("CLFLUSH")].into()
}

/// # Pseudocode
/// ```text
/// Flush_Cache_Line_Optimized(SRC);
/// ```
#[box_to_static_reference]
pub(super) fn clflushopt() -> &'static [IrStatement] {
    [exception("CLFLUSHOPT")].into()
}

/// # Pseudocode
/// ```text
/// IF CR0.PE = 0
///     THEN IF := 0; (* Reset Interrupt Flag *)
///     ELSE
///         IF IOPL ≥ CPL(* CPL = 3 if EFLAGS.VM = 1 *)
///             THEN IF := 0; (* Reset Interrupt Flag *)
///             ELSE
///                 IF VME mode OR PVI mode
///                     THEN VIF := 0; (* Reset Virtual Interrupt Flag *)
///                     ELSE #GP(0);
///                 FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cli() -> &'static [IrStatement] {
    let set = assign(c(0), if_.clone(), size_relative(if_.clone()));
    [set].into()
}

/// # Pseudocode
/// ```text
/// IF (CR4.CET = 0)
///     THEN #UD; FI;
/// IF (IA32_S_CET.SH_STK_EN = 0)
///     THEN #UD; FI;
/// IF CPL > 0
///     THEN GP(0); FI;
/// SSP_LA = Linear_Address(mem operand)
/// IF SSP_LA not aligned to 8 bytes
///     THEN #GP(0); FI;
/// expected_token_value = SSP_LA | BUSY_BIT(* busy bit - bit position 0 - must be set *)
/// new_token_value = SSP_LA
///         (* Clear the busy bit *)
/// IF shadow_stack_lock_cmpxchg8b(SSP_LA, new_token_value, expected_token_value) != expected_token_value
///     invalid_token := 1; FI
/// (* Set the CF if invalid token was detected *)
/// RFLAGS.CF = (invalid_token == 1) ? 1 : 0;
/// RFLAGS.ZF,PF,AF,OF,SF := 0;
/// SSP := 0
/// ```
#[box_to_static_reference]
pub(super) fn clrssbsy() -> &'static [IrStatement] {
    [exception("clrssbsy")].into()
}

/// # Pseudocode
/// ```text
/// CR0.TS[bit 3] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn clts() -> &'static [IrStatement] {
    [exception("clts")].into()
}

/// # Pseudocode
/// ```text
/// UIF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn clui() -> &'static [IrStatement] {
    let stmt_0 = assign(c(0), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// Cache_Line_Write_Back(m8);
/// 1.The Mod field of the ModR/M byte cannot have value 11B.
/// ```
#[box_to_static_reference]
pub(super) fn clwb() -> &'static [IrStatement] {
    [exception("CLWB")].into()
}

/// # Pseudocode
/// ```text
/// temp := SRC
/// IF condition TRUE
///     THEN DEST := temp;
/// ELSE IF (OperandSize = 32 and IA-32e mode active)
///     THEN DEST[63:32] := 0;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cmc() -> &'static [IrStatement] {
    let set = assign(u::not(cf.clone()), cf.clone(), size_relative(cf.clone()));
    [set].into()
}

/// # Pseudocode
/// ```text
/// temp := SRC1 - SignExtend(SRC2);
/// ModifyStatusFlags; (* Modify status flags in the same manner as the SUB instruction*)
/// ```
#[box_to_static_reference]
pub(super) fn cmp() -> &'static [IrStatement] {
    let sub = b::sub(o1(), u::sign_extend(o2()));
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, type1, type2].into()
}

/// # Pseudocode
/// ```text
/// CASE (COMPARISON PREDICATE) OF
/// 0: OP3 := EQ_OQ; OP5 := EQ_OQ;
///     1: OP3 := LT_OS; OP5 := LT_OS;
///     2: OP3 := LE_OS; OP5 := LE_OS;
///     3: OP3 := UNORD_Q; OP5 := UNORD_Q;
///     4: OP3 := NEQ_UQ; OP5 := NEQ_UQ;
///     5: OP3 := NLT_US; OP5 := NLT_US;
///     6: OP3 := NLE_US; OP5 := NLE_US;
///     7: OP3 := ORD_Q; OP5 := ORD_Q;
///     8: OP5 := EQ_UQ;
///     9: OP5 := NGE_US;
///     10: OP5 := NGT_US;
///     11: OP5 := FALSE_OQ;
///     12: OP5 := NEQ_OQ;
///     13: OP5 := GE_OS;
///     14: OP5 := GT_OS;
///     15: OP5 := TRUE_UQ;
///     16: OP5 := EQ_OS;
///     17: OP5 := LT_OQ;
///     18: OP5 := LE_OQ;
///     19: OP5 := UNORD_S;
///     20: OP5 := NEQ_US;
///     21: OP5 := NLT_UQ;
///     22: OP5 := NLE_UQ;
///     23: OP5 := ORD_S;
///     24: OP5 := EQ_US;
///     25: OP5 := NGE_UQ;
///     26: OP5 := NGT_UQ;
///     27: OP5 := FALSE_OS;
///     28: OP5 := NEQ_OS;
///     29: OP5 := GE_OQ;
///     30: OP5 := GT_OQ;
///     31: OP5 := TRUE_US;
///     DEFAULT: Reserved;
/// ESAC;
/// VCMPPD (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k2[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     CMP := SRC1[i+63:i] OP5 SRC2[63:0]
///                 ELSE
///                     CMP := SRC1[i+63:i] OP5 SRC2[i+63:i]
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                         ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VCMPPD (VEX.256 Encoded Version)
/// CMP0 := SRC1[63:0] OP5 SRC2[63:0];
/// CMP1 := SRC1[127:64] OP5 SRC2[127:64];
/// CMP2 := SRC1[191:128] OP5 SRC2[191:128];
/// CMP3 := SRC1[255:192] OP5 SRC2[255:192];
/// IF CMP0 = TRUE
///     THEN DEST[63:0] := FFFFFFFFFFFFFFFFH;
///     ELSE DEST[63:0] := 0000000000000000H; FI;
/// IF CMP1 = TRUE
///     THEN DEST[127:64] := FFFFFFFFFFFFFFFFH;
///     ELSE DEST[127:64] := 0000000000000000H; FI;
/// IF CMP2 = TRUE
///     THEN DEST[191:128] := FFFFFFFFFFFFFFFFH;
///     ELSE DEST[191:128] := 0000000000000000H; FI;
/// IF CMP3 = TRUE
///     THEN DEST[255:192] := FFFFFFFFFFFFFFFFH;
///     ELSE DEST[255:192] := 0000000000000000H; FI;
/// DEST[MAXVL-1:256] := 0
/// VCMPPD (VEX.128 Encoded Version)
/// CMP0 := SRC1[63:0] OP5 SRC2[63:0];
/// CMP1 := SRC1[127:64] OP5 SRC2[127:64];
/// IF CMP0 = TRUE
///     THEN DEST[63:0] := FFFFFFFFFFFFFFFFH;
///     ELSE DEST[63:0] := 0000000000000000H; FI;
/// IF CMP1 = TRUE
///     THEN DEST[127:64] := FFFFFFFFFFFFFFFFH;
///     ELSE DEST[127:64] := 0000000000000000H; FI;
/// DEST[MAXVL-1:128] := 0
/// CMPPD (128-bit Legacy SSE Version)
/// CMP0 := SRC1[63:0] OP3 SRC2[63:0];
/// CMP1 := SRC1[127:64] OP3 SRC2[127:64];
/// IF CMP0 = TRUE
///     THEN DEST[63:0] := FFFFFFFFFFFFFFFFH;
///     ELSE DEST[63:0] := 0000000000000000H; FI;
/// IF CMP1 = TRUE
///     THEN DEST[127:64] := FFFFFFFFFFFFFFFFH;
///     ELSE DEST[127:64] := 0000000000000000H; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn cmppd() -> &'static [IrStatement] {
    let assignment = assign(b::equal(o2(), o3(), o1_size()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CASE (COMPARISON PREDICATE) OF
///     0: OP3 := EQ_OQ; OP5 := EQ_OQ;
///     1: OP3 := LT_OS; OP5 := LT_OS;
///     2: OP3 := LE_OS; OP5 := LE_OS;
///     3: OP3 := UNORD_Q; OP5 := UNORD_Q;
///     4: OP3 := NEQ_UQ; OP5 := NEQ_UQ;
///     5: OP3 := NLT_US; OP5 := NLT_US;
///     6: OP3 := NLE_US; OP5 := NLE_US;
///     7: OP3 := ORD_Q; OP5 := ORD_Q;
///     8: OP5 := EQ_UQ;
///     9: OP5 := NGE_US;
///     10: OP5 := NGT_US;
///     11: OP5 := FALSE_OQ;
///     12: OP5 := NEQ_OQ;
///     13: OP5 := GE_OS;
///     14: OP5 := GT_OS;
///     15: OP5 := TRUE_UQ;
///     16: OP5 := EQ_OS;
///     17: OP5 := LT_OQ;
///     18: OP5 := LE_OQ;
///     19: OP5 := UNORD_S;
///     20: OP5 := NEQ_US;
///     21: OP5 := NLT_UQ;
///     22: OP5 := NLE_UQ;
///     23: OP5 := ORD_S;
///     24: OP5 := EQ_US;
///     25: OP5 := NGE_UQ;
///     26: OP5 := NGT_UQ;
///     27: OP5 := FALSE_OS;
///     28: OP5 := NEQ_OS;
///     29: OP5 := GE_OQ;
///     30: OP5 := GT_OQ;
///     31: OP5 := TRUE_US;
///     DEFAULT: Reserved
/// ESAC;
/// VCMPPS (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k2[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     CMP := SRC1[i+31:i] OP5 SRC2[31:0]
///                 ELSE
///                     CMP := SRC1[i+31:i] OP5 SRC2[i+31:i]
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                         ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VCMPPS (VEX.256 Encoded Version)
/// CMP0 := SRC1[31:0] OP5 SRC2[31:0];
/// CMP1 := SRC1[63:32] OP5 SRC2[63:32];
/// CMP2 := SRC1[95:64] OP5 SRC2[95:64];
/// CMP3 := SRC1[127:96] OP5 SRC2[127:96];
/// CMP4 := SRC1[159:128] OP5 SRC2[159:128];
/// CMP5 := SRC1[191:160] OP5 SRC2[191:160];
/// CMP6 := SRC1[223:192] OP5 SRC2[223:192];
/// CMP7 := SRC1[255:224] OP5 SRC2[255:224];
/// IF CMP0 = TRUE
///     THEN DEST[31:0] :=FFFFFFFFH;
///     ELSE DEST[31:0] := 000000000H; FI;
/// IF CMP1 = TRUE
///     THEN DEST[63:32] := FFFFFFFFH;
///     ELSE DEST[63:32] :=000000000H; FI;
/// IF CMP2 = TRUE
///     THEN DEST[95:64] := FFFFFFFFH;
///     ELSE DEST[95:64] := 000000000H; FI;
/// IF CMP3 = TRUE
///     THEN DEST[127:96] := FFFFFFFFH;
///     ELSE DEST[127:96] := 000000000H; FI;
/// IF CMP4 = TRUE
///     THEN DEST[159:128] := FFFFFFFFH;
///     ELSE DEST[159:128] := 000000000H; FI;
/// IF CMP5 = TRUE
///     THEN DEST[191:160] := FFFFFFFFH;
///     ELSE DEST[191:160] := 000000000H; FI;
/// IF CMP6 = TRUE
///     THEN DEST[223:192] := FFFFFFFFH;
///     ELSE DEST[223:192] :=000000000H; FI;
/// IF CMP7 = TRUE
///     THEN DEST[255:224] := FFFFFFFFH;
///     ELSE DEST[255:224] := 000000000H; FI;
/// VCMPPS (VEX.128 Encoded Version)
/// CMP0 := SRC1[31:0] OP5 SRC2[31:0];
/// CMP1 := SRC1[63:32] OP5 SRC2[63:32];
/// CMP2 := SRC1[95:64] OP5 SRC2[95:64];
/// CMP3 := SRC1[127:96] OP5 SRC2[127:96];
/// IF CMP0 = TRUE
///     THEN DEST[31:0] :=FFFFFFFFH;
///     ELSE DEST[31:0] := 000000000H; FI;
/// IF CMP1 = TRUE
///     THEN DEST[63:32] := FFFFFFFFH;
///     ELSE DEST[63:32] := 000000000H; FI;
/// IF CMP2 = TRUE
///     THEN DEST[95:64] := FFFFFFFFH;
///     ELSE DEST[95:64] := 000000000H; FI;
/// IF CMP3 = TRUE
///     THEN DEST[127:96] := FFFFFFFFH;
///     ELSE DEST[127:96] :=000000000H; FI;
/// DEST[MAXVL-1:128] := 0
/// CMPPS (128-bit Legacy SSE Version)
/// CMP0 := SRC1[31:0] OP3 SRC2[31:0];
/// CMP1 := SRC1[63:32] OP3 SRC2[63:32];
/// CMP2 := SRC1[95:64] OP3 SRC2[95:64];
/// CMP3 := SRC1[127:96] OP3 SRC2[127:96];
/// IF CMP0 = TRUE
///     THEN DEST[31:0] :=FFFFFFFFH;
///     ELSE DEST[31:0] := 000000000H; FI;
/// IF CMP1 = TRUE
///     THEN DEST[63:32] := FFFFFFFFH;
///     ELSE DEST[63:32] := 000000000H; FI;
/// IF CMP2 = TRUE
///     THEN DEST[95:64] := FFFFFFFFH;
///     ELSE DEST[95:64] := 000000000H; FI;
/// IF CMP3 = TRUE
///     THEN DEST[127:96] := FFFFFFFFH;
///     ELSE DEST[127:96] :=000000000H; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn cmpps() -> &'static [IrStatement] {
    let assignment = assign(b::equal(o2(), o3(), o1_size()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// temp := SRC1 - SRC2;
/// SetStatusFlags(temp);
/// IF (64-Bit Mode)
///     THEN
///         IF (Byte comparison)
///         THEN IF DF = 0
///             THEN
///                 (R|E)SI := (R|E)SI + 1;
///                 (R|E)DI := (R|E)DI + 1;
///             ELSE
///                 (R|E)SI := (R|E)SI - 1;
///                 (R|E)DI := (R|E)DI - 1;
///             FI;
///         ELSE IF (Word comparison)
///             THEN IF DF = 0
///                 THEN
///                     (R|E)SI := (R|E)SI + 2;
///                     (R|E)DI := (R|E)DI + 2;
///                 ELSE
///                     (R|E)SI := (R|E)SI - 2;
///                     (R|E)DI := (R|E)DI - 2;
///                 FI;
///         ELSE IF (Doubleword comparison)
///             THEN IF DF = 0
///                 THEN
///                     (R|E)SI := (R|E)SI + 4;
///                     (R|E)DI := (R|E)DI + 4;
///                 ELSE
///                     (R|E)SI := (R|E)SI - 4;
///                     (R|E)DI := (R|E)DI - 4;
///                 FI;
///         ELSE (* Quadword comparison *)
///             THEN IF DF = 0
///                 (R|E)SI := (R|E)SI + 8;
///                 (R|E)DI := (R|E)DI + 8;
///             ELSE
///                 (R|E)SI := (R|E)SI - 8;
///                 (R|E)DI := (R|E)DI - 8;
///             FI;
///         FI;
///     ELSE (* Non-64-bit Mode *)
///         IF (byte comparison)
///         THEN IF DF = 0
///             THEN
///                 (E)SI := (E)SI + 1;
///                 (E)DI := (E)DI + 1;
///             ELSE
///                 (E)SI := (E)SI - 1;
///                 (E)DI := (E)DI - 1;
///             FI;
///         ELSE IF (Word comparison)
///             THEN IF DF = 0
///                 (E)SI := (E)SI + 2;
///                 (E)DI := (E)DI + 2;
///             ELSE
///                 (E)SI := (E)SI - 2;
///                 (E)DI := (E)DI - 2;
///             FI;
///         ELSE (* Doubleword comparison *)
///             THEN IF DF = 0
///                 (E)SI := (E)SI + 4;
///                 (E)DI := (E)DI + 4;
///             ELSE
///                 (E)SI := (E)SI - 4;
///                 (E)DI := (E)DI - 4;
///             FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cmps() -> &'static [IrStatement] {
    let source = d(rsi.clone());
    let destination = d(rdi.clone());
    let sub = b::sub(source.clone(), u::sign_extend(destination.clone()));
    let calc_flags = calc_flags_automatically(sub, size_architecture(), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(source, size_architecture(), DataType::Int);
    let type2 = type_specified(destination, size_architecture(), DataType::Int);
    let type3 = type_specified(rsi.clone(), size_architecture(), DataType::Address);
    let type4 = type_specified(rdi.clone(), size_architecture(), DataType::Address);
    [calc_flags, type1, type2, type3, type4].into()
}

/// # Pseudocode
/// ```text
/// temp := SRC1 - SRC2;
/// SetStatusFlags(temp);
/// IF (64-Bit Mode)
///     THEN
///         IF (Byte comparison)
///         THEN IF DF = 0
///             THEN
///                 (R|E)SI := (R|E)SI + 1;
///                 (R|E)DI := (R|E)DI + 1;
///             ELSE
///                 (R|E)SI := (R|E)SI - 1;
///                 (R|E)DI := (R|E)DI - 1;
///             FI;
///         ELSE IF (Word comparison)
///             THEN IF DF = 0
///                 THEN
///                     (R|E)SI := (R|E)SI + 2;
///                     (R|E)DI := (R|E)DI + 2;
///                 ELSE
///                     (R|E)SI := (R|E)SI - 2;
///                     (R|E)DI := (R|E)DI - 2;
///                 FI;
///         ELSE IF (Doubleword comparison)
///             THEN IF DF = 0
///                 THEN
///                     (R|E)SI := (R|E)SI + 4;
///                     (R|E)DI := (R|E)DI + 4;
///                 ELSE
///                     (R|E)SI := (R|E)SI - 4;
///                     (R|E)DI := (R|E)DI - 4;
///                 FI;
///         ELSE (* Quadword comparison *)
///             THEN IF DF = 0
///                 (R|E)SI := (R|E)SI + 8;
///                 (R|E)DI := (R|E)DI + 8;
///             ELSE
///                 (R|E)SI := (R|E)SI - 8;
///                 (R|E)DI := (R|E)DI - 8;
///             FI;
///         FI;
///     ELSE (* Non-64-bit Mode *)
///         IF (byte comparison)
///         THEN IF DF = 0
///             THEN
///                 (E)SI := (E)SI + 1;
///                 (E)DI := (E)DI + 1;
///             ELSE
///                 (E)SI := (E)SI - 1;
///                 (E)DI := (E)DI - 1;
///             FI;
///         ELSE IF (Word comparison)
///             THEN IF DF = 0
///                 (E)SI := (E)SI + 2;
///                 (E)DI := (E)DI + 2;
///             ELSE
///                 (E)SI := (E)SI - 2;
///                 (E)DI := (E)DI - 2;
///             FI;
///         ELSE (* Doubleword comparison *)
///             THEN IF DF = 0
///                 (E)SI := (E)SI + 4;
///                 (E)DI := (E)DI + 4;
///             ELSE
///                 (E)SI := (E)SI - 4;
///                 (E)DI := (E)DI - 4;
///             FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cmpsb() -> &'static [IrStatement] {
    let source = d(rsi.clone());
    let destination = d(rdi.clone());
    let sub = b::sub(source.clone(), u::sign_extend(destination.clone()));
    let calc_flags = calc_flags_automatically(sub, size_result_byte(c(1)), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(source, size_architecture(), DataType::Int);
    let type2 = type_specified(destination, size_architecture(), DataType::Int);
    let type3 = type_specified(rsi.clone(), size_architecture(), DataType::Address);
    let type4 = type_specified(rdi.clone(), size_architecture(), DataType::Address);
    [calc_flags, type1, type2, type3, type4].into()
}

/// # Pseudocode
/// ```text
/// CASE (COMPARISON PREDICATE) OF
///     0: OP3 := EQ_OQ; OP5 := EQ_OQ;
///     1: OP3 := LT_OS; OP5 := LT_OS;
///     2: OP3 := LE_OS; OP5 := LE_OS;
///     3: OP3 := UNORD_Q; OP5 := UNORD_Q;
///     4: OP3 := NEQ_UQ; OP5 := NEQ_UQ;
///     5: OP3 := NLT_US; OP5 := NLT_US;
///     6: OP3 := NLE_US; OP5 := NLE_US;
///     7: OP3 := ORD_Q; OP5 := ORD_Q;
///     8: OP5 := EQ_UQ;
///     9: OP5 := NGE_US;
///     10: OP5 := NGT_US;
///     11: OP5 := FALSE_OQ;
///     12: OP5 := NEQ_OQ;
///     13: OP5 := GE_OS;
///     14: OP5 := GT_OS;
///     15: OP5 := TRUE_UQ;
///     16: OP5 := EQ_OS;
///     17: OP5 := LT_OQ;
///     18: OP5 := LE_OQ;
///     19: OP5 := UNORD_S;
///     20: OP5 := NEQ_US;
///     22: OP5 := NLE_UQ;
///     23: OP5 := ORD_S;
///     24: OP5 := EQ_US;
///     25: OP5 := NGE_UQ;
///     26: OP5 := NGT_UQ;
///     27: OP5 := FALSE_OS;
///     28: OP5 := NEQ_OS;
///     29: OP5 := GE_OQ;
///     30: OP5 := GT_OQ;
///     31: OP5 := TRUE_US;
///     DEFAULT: Reserved
/// ESAC;
/// VCMPSD (EVEX Encoded Version)
/// CMP0 := SRC1[63:0] OP5 SRC2[63:0];
/// IF k2[0] or *no writemask*
///     THENIF CMP0 = TRUE
///         THEN DEST[0] := 1;
///         ELSE DEST[0] := 0; FI;
///     ELSE DEST[0] := 0
///             ; zeroing-masking only
/// FI;
/// DEST[MAX_KL-1:1] := 0
/// CMPSD (128-bit Legacy SSE Version)
/// CMP0 := DEST[63:0] OP3 SRC[63:0];
/// IF CMP0 = TRUE
/// THEN DEST[63:0] := FFFFFFFFFFFFFFFFH;
/// ELSE DEST[63:0] := 0000000000000000H; FI;
/// DEST[MAXVL-1:64] (Unmodified)
/// VCMPSD (VEX.128 Encoded Version)
/// CMP0 := SRC1[63:0] OP5 SRC2[63:0];
/// IF CMP0 = TRUE
/// THEN DEST[63:0] := FFFFFFFFFFFFFFFFH;
/// ELSE DEST[63:0] := 0000000000000000H; FI;
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn cmpsd() -> &'static [IrStatement] {
    let source = d(rsi.clone());
    let destination = d(rdi.clone());
    let sub = b::sub(source.clone(), u::sign_extend(destination.clone()));
    let calc_flags = calc_flags_automatically(sub, size_result_byte(c(4)), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(source, size_architecture(), DataType::Int);
    let type2 = type_specified(destination, size_architecture(), DataType::Int);
    let type3 = type_specified(rsi.clone(), size_architecture(), DataType::Address);
    let type4 = type_specified(rdi.clone(), size_architecture(), DataType::Address);
    [calc_flags, type1, type2, type3, type4].into()
}

/// # Pseudocode
/// ```text
/// temp := SRC1 - SRC2;
/// SetStatusFlags(temp);
/// IF (64-Bit Mode)
///     THEN
///         IF (Byte comparison)
///         THEN IF DF = 0
///             THEN
///                 (R|E)SI := (R|E)SI + 1;
///                 (R|E)DI := (R|E)DI + 1;
///             ELSE
///                 (R|E)SI := (R|E)SI - 1;
///                 (R|E)DI := (R|E)DI - 1;
///             FI;
///         ELSE IF (Word comparison)
///             THEN IF DF = 0
///                 THEN
///                     (R|E)SI := (R|E)SI + 2;
///                     (R|E)DI := (R|E)DI + 2;
///                 ELSE
///                     (R|E)SI := (R|E)SI - 2;
///                     (R|E)DI := (R|E)DI - 2;
///                 FI;
///         ELSE IF (Doubleword comparison)
///             THEN IF DF = 0
///                 THEN
///                     (R|E)SI := (R|E)SI + 4;
///                     (R|E)DI := (R|E)DI + 4;
///                 ELSE
///                     (R|E)SI := (R|E)SI - 4;
///                     (R|E)DI := (R|E)DI - 4;
///                 FI;
///         ELSE (* Quadword comparison *)
///             THEN IF DF = 0
///                 (R|E)SI := (R|E)SI + 8;
///                 (R|E)DI := (R|E)DI + 8;
///             ELSE
///                 (R|E)SI := (R|E)SI - 8;
///                 (R|E)DI := (R|E)DI - 8;
///             FI;
///         FI;
///     ELSE (* Non-64-bit Mode *)
///         IF (byte comparison)
///         THEN IF DF = 0
///             THEN
///                 (E)SI := (E)SI + 1;
///                 (E)DI := (E)DI + 1;
///             ELSE
///                 (E)SI := (E)SI - 1;
///                 (E)DI := (E)DI - 1;
///             FI;
///         ELSE IF (Word comparison)
///             THEN IF DF = 0
///                 (E)SI := (E)SI + 2;
///                 (E)DI := (E)DI + 2;
///             ELSE
///                 (E)SI := (E)SI - 2;
///                 (E)DI := (E)DI - 2;
///             FI;
///         ELSE (* Doubleword comparison *)
///             THEN IF DF = 0
///                 (E)SI := (E)SI + 4;
///                 (E)DI := (E)DI + 4;
///             ELSE
///                 (E)SI := (E)SI - 4;
///                 (E)DI := (E)DI - 4;
///             FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cmpsq() -> &'static [IrStatement] {
    let source = d(rsi.clone());
    let destination = d(rdi.clone());
    let sub = b::sub(source.clone(), u::sign_extend(destination.clone()));
    let calc_flags = calc_flags_automatically(sub, size_result_byte(c(8)), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(source, size_architecture(), DataType::Int);
    let type2 = type_specified(destination, size_architecture(), DataType::Int);
    let type3 = type_specified(rsi.clone(), size_architecture(), DataType::Address);
    let type4 = type_specified(rdi.clone(), size_architecture(), DataType::Address);
    [calc_flags, type1, type2, type3, type4].into()
}

/// # Pseudocode
/// ```text
/// CASE (COMPARISON PREDICATE) OF
///     0: OP3 := EQ_OQ; OP5 := EQ_OQ;
///     1: OP3 := LT_OS; OP5 := LT_OS;
///     2: OP3 := LE_OS; OP5 := LE_OS;
///     3: OP3 := UNORD_Q; OP5 := UNORD_Q;
///     4: OP3 := NEQ_UQ; OP5 := NEQ_UQ;
///     5: OP3 := NLT_US; OP5 := NLT_US;
///     6: OP3 := NLE_US; OP5 := NLE_US;
///     7: OP3 := ORD_Q; OP5 := ORD_Q;
///     8: OP5 := EQ_UQ;
///     9: OP5 := NGE_US;
///     10: OP5 := NGT_US;
///     11: OP5 := FALSE_OQ;
///     12: OP5 := NEQ_OQ;
///     13: OP5 := GE_OS;
///     14: OP5 := GT_OS;
///     15: OP5 := TRUE_UQ;
///     16: OP5 := EQ_OS;
///     17: OP5 := LT_OQ;
///     19: OP5 := UNORD_S;
///     20: OP5 := NEQ_US;
///     21: OP5 := NLT_UQ;
///     22: OP5 := NLE_UQ;
///     23: OP5 := ORD_S;
///     24: OP5 := EQ_US;
///     25: OP5 := NGE_UQ;
///     26: OP5 := NGT_UQ;
///     27: OP5 := FALSE_OS;
///     28: OP5 := NEQ_OS;
///     29: OP5 := GE_OQ;
///     30: OP5 := GT_OQ;
///     31: OP5 := TRUE_US;
///     DEFAULT: Reserved
/// ESAC;
/// VCMPSS (EVEX Encoded Version)
/// CMP0 := SRC1[31:0] OP5 SRC2[31:0];
/// IF k2[0] or *no writemask*
///     THENIF CMP0 = TRUE
///         THEN DEST[0] := 1;
///         ELSE DEST[0] := 0; FI;
///     ELSE DEST[0] := 0
///             ; zeroing-masking only
/// FI;
/// DEST[MAX_KL-1:1] := 0
/// CMPSS (128-bit Legacy SSE Version)
/// CMP0 := DEST[31:0] OP3 SRC[31:0];
/// IF CMP0 = TRUE
/// THEN DEST[31:0] := FFFFFFFFH;
/// ELSE DEST[31:0] := 00000000H; FI;
/// DEST[MAXVL-1:32] (Unmodified)
/// VCMPSS (VEX.128 Encoded Version)
/// CMP0 := SRC1[31:0] OP5 SRC2[31:0];
/// IF CMP0 = TRUE
/// THEN DEST[31:0] := FFFFFFFFH;
/// ELSE DEST[31:0] := 00000000H; FI;
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn cmpss() -> &'static [IrStatement] {
    let assignment = assign(b::equal(o2(), o3(), o1_size()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// temp := SRC1 - SRC2;
/// SetStatusFlags(temp);
/// IF (64-Bit Mode)
///     THEN
///         IF (Byte comparison)
///         THEN IF DF = 0
///             THEN
///                 (R|E)SI := (R|E)SI + 1;
///                 (R|E)DI := (R|E)DI + 1;
///             ELSE
///                 (R|E)SI := (R|E)SI - 1;
///                 (R|E)DI := (R|E)DI - 1;
///             FI;
///         ELSE IF (Word comparison)
///             THEN IF DF = 0
///                 THEN
///                     (R|E)SI := (R|E)SI + 2;
///                     (R|E)DI := (R|E)DI + 2;
///                 ELSE
///                     (R|E)SI := (R|E)SI - 2;
///                     (R|E)DI := (R|E)DI - 2;
///                 FI;
///         ELSE IF (Doubleword comparison)
///             THEN IF DF = 0
///                 THEN
///                     (R|E)SI := (R|E)SI + 4;
///                     (R|E)DI := (R|E)DI + 4;
///                 ELSE
///                     (R|E)SI := (R|E)SI - 4;
///                     (R|E)DI := (R|E)DI - 4;
///                 FI;
///         ELSE (* Quadword comparison *)
///             THEN IF DF = 0
///                 (R|E)SI := (R|E)SI + 8;
///                 (R|E)DI := (R|E)DI + 8;
///             ELSE
///                 (R|E)SI := (R|E)SI - 8;
///                 (R|E)DI := (R|E)DI - 8;
///             FI;
///         FI;
///     ELSE (* Non-64-bit Mode *)
///         IF (byte comparison)
///         THEN IF DF = 0
///             THEN
///                 (E)SI := (E)SI + 1;
///                 (E)DI := (E)DI + 1;
///             ELSE
///                 (E)SI := (E)SI - 1;
///                 (E)DI := (E)DI - 1;
///             FI;
///         ELSE IF (Word comparison)
///             THEN IF DF = 0
///                 (E)SI := (E)SI + 2;
///                 (E)DI := (E)DI + 2;
///             ELSE
///                 (E)SI := (E)SI - 2;
///                 (E)DI := (E)DI - 2;
///             FI;
///         ELSE (* Doubleword comparison *)
///             THEN IF DF = 0
///                 (E)SI := (E)SI + 4;
///                 (E)DI := (E)DI + 4;
///             ELSE
///                 (E)SI := (E)SI - 4;
///                 (E)DI := (E)DI - 4;
///             FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cmpsw() -> &'static [IrStatement] {
    let source = d(rsi.clone());
    let destination = d(rdi.clone());
    let sub = b::sub(source.clone(), u::sign_extend(destination.clone()));
    let calc_flags = calc_flags_automatically(sub, size_result_byte(c(2)), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(source, size_architecture(), DataType::Int);
    let type2 = type_specified(destination, size_architecture(), DataType::Int);
    let type3 = type_specified(rsi.clone(), size_architecture(), DataType::Address);
    let type4 = type_specified(rdi.clone(), size_architecture(), DataType::Address);
    [calc_flags, type1, type2, type3, type4].into()
}

/// # Pseudocode
/// ```text
/// (* Accumulator = AL, AX, EAX, or RAX depending on whether a byte, word, doubleword, or quadword comparison is being performed *)
/// TEMP := DEST
/// IF accumulator = TEMP
///     THEN
///         ZF := 1;
///         DEST := SRC;
///     ELSE
///         ZF := 0;
///         accumulator := TEMP;
///         DEST := TEMP;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cmpxchg() -> &'static [IrStatement] {
    let cond = b::equal(rax.clone(), d(o1()), o1_size());
    let true_b = [assign(o2(), d(o1()), o1_size())];
    let false_b = [assign(d(o1()), rax.clone(), o1_size())];
    let cmpxchg = condition(cond.clone(), true_b, false_b);
    let calc_flags = calc_flags_automatically(cond, size_result_byte(c(1)), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    let type3 = type_specified(rax.clone(), size_relative(rax.clone()), DataType::Int);
    [calc_flags, cmpxchg, type1, type2, type3].into()
}

/// # Pseudocode
/// ```text
/// IF (64-Bit Mode and OperandSize = 64)
///     THEN
///         TEMP128 := DEST
///         IF (RDX:RAX = TEMP128)
///             THEN
///                 ZF := 1;
///                 DEST := RCX:RBX;
///             ELSE
///                 ZF := 0;
///                 RDX:RAX := TEMP128;
///                 DEST := TEMP128;
///                 FI;
///         FI
///     ELSE
///         TEMP64 := DEST;
///         IF (EDX:EAX = TEMP64)
///             THEN
///                 ZF := 1;
///                 DEST := ECX:EBX;
///             ELSE
///                 ZF := 0;
///                 EDX:EAX := TEMP64;
///                 DEST := TEMP64;
///                 FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cmpxchg16b() -> &'static [IrStatement] {
    let cond = b::equal(rax.clone(), d(o1()), o1_size());
    let true_b = [assign(o2(), d(o1()), o1_size())];
    let false_b = [assign(d(o1()), rax.clone(), o1_size())];
    let cmpxchg = condition(cond.clone(), true_b, false_b);
    let calc_flags = calc_flags_automatically(cond, size_result_byte(c(1)), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    let type3 = type_specified(rax.clone(), size_relative(rax.clone()), DataType::Int);
    [calc_flags, cmpxchg, type1, type2, type3].into()
}

/// # Pseudocode
/// ```text
/// IF (64-Bit Mode and OperandSize = 64)
///     THEN
///         TEMP128 := DEST
///         IF (RDX:RAX = TEMP128)
///             THEN
///                 ZF := 1;
///                 DEST := RCX:RBX;
///             ELSE
///                 ZF := 0;
///                 RDX:RAX := TEMP128;
///                 DEST := TEMP128;
///                 FI;
///         FI
///     ELSE
///         TEMP64 := DEST;
///         IF (EDX:EAX = TEMP64)
///             THEN
///                 ZF := 1;
///                 DEST := ECX:EBX;
///             ELSE
///                 ZF := 0;
///                 EDX:EAX := TEMP64;
///                 DEST := TEMP64;
///                 FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cmpxchg8b() -> &'static [IrStatement] {
    let cond = b::equal(rax.clone(), d(o1()), o1_size());
    let true_b = [assign(o2(), d(o1()), o1_size())];
    let false_b = [assign(d(o1()), rax.clone(), o1_size())];
    let cmpxchg = condition(cond.clone(), true_b, false_b);
    let calc_flags = calc_flags_automatically(cond, size_result_byte(c(1)), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    let type3 = type_specified(rax.clone(), size_relative(rax.clone()), DataType::Int);
    [calc_flags, cmpxchg, type1, type2, type3].into()
}

/// # Pseudocode
/// ```text
/// COMISD (All Versions)
/// RESULT :=OrderedCompare(DEST[63:0] <> SRC[63:0]) {
/// (* Set EFLAGS *) CASE (RESULT) OF
///     UNORDERED: ZF,PF,CF := 111;
///     GREATER_THAN: ZF,PF,CF := 000;
///     LESS_THAN: ZF,PF,CF := 001;
///     EQUAL: ZF,PF,CF := 100;
/// ESAC;
/// OF, AF, SF := 0; }
/// ```
#[box_to_static_reference]
pub(super) fn comisd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o1(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// COMISS (All Versions)
/// RESULT :=OrderedCompare(DEST[31:0] <> SRC[31:0]) {
/// (* Set EFLAGS *) CASE (RESULT) OF
///     UNORDERED: ZF,PF,CF := 111;
///     GREATER_THAN: ZF,PF,CF := 000;
///     LESS_THAN: ZF,PF,CF := 001;
///     EQUAL: ZF,PF,CF := 100;
/// ESAC;
/// OF, AF, SF := 0; }
/// ```
#[box_to_static_reference]
pub(super) fn comiss() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o1(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IA32_BIOS_SIGN_ID MSR := Update with installed microcode revision number;
/// CASE (EAX) OF
///     EAX = 0:
///         EAX := Highest basic function input value understood by CPUID;
///         EBX := Vendor identification string;
///         EDX := Vendor identification string;
///         ECX := Vendor identification string;
///     BREAK;
///     EAX = 1H:
///         EAX[3:0] := Stepping ID;
///         EAX[7:4] := Model;
///         EAX[11:8] := Family;
///         EAX[13:12] := Processor type;
///         EAX[15:14] := Reserved;
///         EAX[19:16] := Extended Model;
///         EAX[27:20] := Extended Family;
///         EAX[31:28] := Reserved;
///         EBX[7:0] := Brand Index; (* Reserved if the value is zero. *)
///         EBX[15:8] := CLFLUSH Line Size;
///         EBX[16:23] := Reserved; (* Number of threads enabled = 2 if MT enable fuse set. *)
///         EBX[24:31] := Initial APIC ID;
///         ECX := Feature flags; (* See Figure3-7. *)
///         EDX := Feature flags; (* See Figure3-8. *)
///     BREAK;
///     EAX = 2H:
///         EAX := Cache and TLB information;
///             := Cache and TLB information;
/// EBX
///             := Cache and TLB information;
/// ECX
///         EDX := Cache and TLB information;
///     BREAK;
///     EAX = 3H:
///         EAX := Reserved;
///             := Reserved;
/// EBX
///             := ProcessorSerialNumber[31:0];
/// ECX
///         (* Pentium III processors only, otherwise reserved. *)
///         EDX := ProcessorSerialNumber[63:32];
///         (* Pentium III processors only, otherwise reserved. *
///     BREAK
///     EAX = 4H:
///         EAX := Deterministic Cache Parameters Leaf; (* See Table 3-8. *)
///         EBX := Deterministic Cache Parameters Leaf;
///             := Deterministic Cache Parameters Leaf;
/// ECX
///         EDX := Deterministic Cache Parameters Leaf;
///     BREAK;
///     EAX = 5H:
///         EAX := MONITOR/MWAIT Leaf; (* See Table 3-8. *)
///             := MONITOR/MWAIT Leaf;
/// EBX
///             := MONITOR/MWAIT Leaf;
/// ECX
///         EDX := MONITOR/MWAIT Leaf;
///     BREAK;
///     EAX = 6H:
///         EAX := Thermal and Power Management Leaf; (* See Table 3-8. *)
///             := Thermal and Power Management Leaf;
/// EBX
///             := Thermal and Power Management Leaf;
/// ECX
///         EDX := Thermal and Power Management Leaf;
///     BREAK;
///     EAX = 7H:
///         EAX := Structured Extended Feature Flags Enumeration Leaf; (* See Table 3-8. *)
///         EBX := Structured Extended Feature Flags Enumeration Leaf;
///             := Structured Extended Feature Flags Enumeration Leaf;
/// ECX
///         EDX := Structured Extended Feature Flags Enumeration Leaf;
///     BREAK;
///     EAX = 8H:
///         EAX := Reserved = 0;
///             := Reserved = 0;
/// EBX
///             := Reserved = 0;
/// ECX
///         EDX := Reserved = 0;
///     BREAK;
///     EAX = 9H:
///         EAX := Direct Cache Access Information Leaf; (* See Table 3-8. *)
///             := Direct Cache Access Information Leaf;
/// EBX
///             := Direct Cache Access Information Leaf;
/// ECX
///         EDX := Direct Cache Access Information Leaf;
///     BREAK;
///     EAX = AH:
///         EAX := Architectural Performance Monitoring Leaf; (* See Table 3-8. *)
///             := Architectural Performance Monitoring Leaf;
/// EBX
///             := Architectural Performance Monitoring Leaf;
/// ECX
///         EDX := Architectural Performance Monitoring Leaf;
///         BREAK
///     EAX = BH:
///         EAX := Extended Topology Enumeration Leaf; (* See Table 3-8. *)
///         EBX := Extended Topology Enumeration Leaf;
///             := Extended Topology Enumeration Leaf;
/// ECX
///         EDX := Extended Topology Enumeration Leaf;
///     BREAK;
///     EAX = CH:
///         EAX := Reserved = 0;
///             := Reserved = 0;
/// EBX
///             := Reserved = 0;
/// ECX
///         EDX := Reserved = 0;
///     BREAK;
///     EAX = DH:
///         EAX := Processor Extended State Enumeration Leaf; (* See Table 3-8. *)
///             := Processor Extended State Enumeration Leaf;
/// EBX
///             := Processor Extended State Enumeration Leaf;
/// ECX
///         EDX := Processor Extended State Enumeration Leaf;
///     BREAK;
///     EAX = EH:
///         EAX := Reserved = 0;
///             := Reserved = 0;
/// EBX
///             := Reserved = 0;
/// ECX
///         EDX := Reserved = 0;
///     BREAK;
///     EAX = FH:
///         EAX := Intel Resource Director Technology Monitoring Enumeration Leaf; (* See Table 3-8. *)
///             := Intel Resource Director Technology Monitoring Enumeration Leaf;
/// EBX
///             := Intel Resource Director Technology Monitoring Enumeration Leaf;
/// ECX
///         EDX := Intel Resource Director Technology Monitoring Enumeration Leaf;
///     BREAK;
///     EAX = 10H:
///         EAX := Intel Resource Director Technology Allocation Enumeration Leaf; (* See Table 3-8. *)
///             := Intel Resource Director Technology Allocation Enumeration Leaf;
/// EBX
///             := Intel Resource Director Technology Allocation Enumeration Leaf;
/// ECX
///         EDX := Intel Resource Director Technology Allocation Enumeration Leaf;
///     BREAK;
///     EAX = 12H:
///         EAX := Intel SGX Enumeration Leaf; (* See Table 3-8. *)
///             := Intel SGX Enumeration Leaf;
/// EBX
///             := Intel SGX Enumeration Leaf;
/// ECX
///         EDX := Intel SGX Enumeration Leaf;
///     BREAK;
///     EAX = 14H:
///         EAX := Intel Processor Trace Enumeration Leaf; (* See Table 3-8. *)
///             := Intel Processor Trace Enumeration Leaf;
/// EBX
///             := Intel Processor Trace Enumeration Leaf;
/// ECX
///         EDX := Intel Processor Trace Enumeration Leaf;
///     BREAK;
///     EAX = 15H:
///         EAX := Time Stamp Counter and Nominal Core Crystal Clock Information Leaf; (* See Table 3-8. *)
///             := Time Stamp Counter and Nominal Core Crystal Clock Information Leaf;
/// EBX
///             := Time Stamp Counter and Nominal Core Crystal Clock Information Leaf;
/// ECX
///         EDX := Time Stamp Counter and Nominal Core Crystal Clock Information Leaf;
///     BREAK;
///     EAX = 16H:
///         EAX := Processor Frequency Information Enumeration Leaf; (* See Table 3-8. *)
///             := Processor Frequency Information Enumeration Leaf;
/// EBX
///             := Processor Frequency Information Enumeration Leaf;
/// ECX
///         EDX := Processor Frequency Information Enumeration Leaf;
///     BREAK;
///     EAX = 17H:
///         EAX := System-On-Chip Vendor Attribute Enumeration Leaf; (* See Table 3-8. *)
///             := System-On-Chip Vendor Attribute Enumeration Leaf;
/// EBX
///             := System-On-Chip Vendor Attribute Enumeration Leaf;
/// ECX
///         EDX := System-On-Chip Vendor Attribute Enumeration Leaf;
///     BREAK;
///     EAX = 18H:
///         EAX := Deterministic Address Translation Parameters Enumeration Leaf; (* See Table 3-8. *)
///             := Deterministic Address Translation Parameters Enumeration Leaf;
/// EBX
///             := Deterministic Address Translation Parameters Enumeration Leaf;
/// ECX
///         EDX := Deterministic Address Translation Parameters Enumeration Leaf;
///     BREAK;
///     EAX = 19H:
///         EAX := Key Locker Enumeration Leaf; (* See Table 3-8. *)
///             := Key Locker Enumeration Leaf;
/// EBX
///             := Key Locker Enumeration Leaf;
/// ECX
///         EDX := Key Locker Enumeration Leaf;
///     BREAK;
///     EAX = 1AH:
///         EAX := Native Model ID Enumeration Leaf; (* See Table 3-8. *)
///         EBX := Native Model ID Enumeration Leaf;
///             := Native Model ID Enumeration Leaf;
/// ECX
///         EDX := Native Model ID Enumeration Leaf;
///     BREAK;
///     EAX = 1BH:
///         EAX := PCONFIG Information Enumeration Leaf; (* See "INPUT EAX = 1BH: Returns PCONFIG Information" on page 3-252. *)
///         EBX := PCONFIG Information Enumeration Leaf;
///             := PCONFIG Information Enumeration Leaf;
/// ECX
///         EDX := PCONFIG Information Enumeration Leaf;
///     BREAK;
///     EAX = 1CH:
///         EAX := Last Branch Record Information Enumeration Leaf; (* See Table 3-8. *)
///         EBX := Last Branch Record Information Enumeration Leaf;
///             := Last Branch Record Information Enumeration Leaf;
/// ECX
///         EDX := Last Branch Record Information Enumeration Leaf;
///     BREAK;
///     EAX = 1DH:
///         EAX := Tile Information Enumeration Leaf; (* See Table 3-8. *)
///         EBX := Tile Information Enumeration Leaf;
///             := Tile Information Enumeration Leaf;
/// ECX
///         EDX := Tile Information Enumeration Leaf;
///     BREAK;
///     EAX = 1EH:
///         EAX := TMUL Information Enumeration Leaf; (* See Table 3-8. *)
///         EBX := TMUL Information Enumeration Leaf;
///             := TMUL Information Enumeration Leaf;
/// ECX
///         EDX := TMUL Information Enumeration Leaf;
///     BREAK;
///     EAX = 1FH:
///         EAX := V2 Extended Topology Enumeration Leaf; (* See Table 3-8. *)
///         EBX := V2 Extended Topology Enumeration Leaf;
///             := V2 Extended Topology Enumeration Leaf;
/// ECX
///         EDX := V2 Extended Topology Enumeration Leaf;
///     BREAK;
///     EAX = 20H:
///         EAX := Processor History Reset Sub-leaf; (* See Table 3-8. *)
///         EBX := Processor History Reset Sub-leaf;
///             := Processor History Reset Sub-leaf;
/// ECX
///         EDX := Processor History Reset Sub-leaf;
///     BREAK;
///     EAX = 80000000H:
///         EAX := Highest extended function input value understood by CPUID;
///         EBX := Reserved;
///         ECX := Reserved;
///         EDX := Reserved;
///     BREAK;
///     EAX = 80000001H:
///         EAX := Reserved;
///         EBX := Reserved;
///         ECX := Extended Feature Bits (* See Table 3-8.*);
///         EDX := Extended Feature Bits (* See Table 3-8. *);
///     BREAK;
///     EAX = 80000002H:
///         EAX := Processor Brand String;
///         EBX := Processor Brand String, continued;
///         ECX := Processor Brand String, continued;
///         EDX := Processor Brand String, continued;
///     BREAK;
///     EAX = 80000003H:
///         EAX := Processor Brand String, continued;
///         EBX := Processor Brand String, continued;
///         ECX := Processor Brand String, continued;
///         EDX := Processor Brand String, continued;
///     BREAK;
///     EAX = 80000004H:
///         EAX := Processor Brand String, continued;
///         EBX := Processor Brand String, continued;
///         ECX := Processor Brand String, continued;
///         EDX := Processor Brand String, continued;
///     BREAK;
///     EAX = 80000005H:
///         EAX := Reserved = 0;
///         EBX := Reserved = 0;
///         ECX := Reserved = 0;
///         EDX := Reserved = 0;
///     BREAK;
///     EAX = 80000006H:
///         EAX := Reserved = 0;
///         EBX := Reserved = 0;
///         ECX := Cache information;
///         EDX := Reserved = 0;
///     BREAK;
///     EAX = 80000007H:
///         EAX := Reserved = 0;
///         EBX := Reserved = 0;
///         ECX := Reserved = 0;
///         EDX := Reserved = Misc Feature Flags;
///     BREAK;
///     EAX = 80000008H:
///         EAX := Address Size Information;
///         EBX := Misc Feature Flags;
///         ECX := Reserved = 0;
///         EDX := Reserved = 0;
///     BREAK;
///     EAX >= 40000000H and EAX <= 4FFFFFFFH:
///     DEFAULT: (* EAX = Value outside of recognized range for CPUID. *)
///         (* If the highest basic information leaf data depend on ECX input value, ECX is honored.*)
///         EAX := Reserved; (* Information returned for highest basic information leaf. *)
///         EBX := Reserved; (* Information returned for highest basic information leaf. *)
///         ECX := Reserved; (* Information returned for highest basic information leaf. *)
///         EDX := Reserved; (* Information returned for highest basic information leaf. *)
///     BREAK;
/// ESAC;
/// ```
#[box_to_static_reference]
pub(super) fn cpuid() -> &'static [IrStatement] {
    [exception("CPUID")].into()
}

/// # Pseudocode
/// ```text
/// IF OperandSize = 16 (* CWD instruction *)
///     THEN
///         DX := SignExtend(AX);
///     ELSE IF OperandSize = 32 (* CDQ instruction *)
///         EDX := SignExtend(EAX); FI;
///     ELSE IF 64-Bit Mode and OperandSize = 64 (* CQO instruction*)
///         RDX := SignExtend(RAX); FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cqo() -> &'static [IrStatement] {
    let set_tmp = assign(u::sign_extend(rax.clone()), tmp128.clone(), size_relative(tmp128.clone()));
    let set_dx = assign(b::shr(tmp128.clone(), c(16)), rdx.clone(), size_relative(rdx.clone()));
    let set_ax = assign(tmp128.clone(), rax.clone(), size_relative(rax.clone()));
    let type1 = type_specified(rax.clone(), size_relative(rax.clone()), DataType::Int);
    let type2 = type_specified(rdx.clone(), size_relative(rdx.clone()), DataType::Int);
    [set_tmp, set_dx, set_ax, type1, type2].into()
}

/// # Pseudocode
/// ```text
/// Notes:
///     BIT_REFLECT64: DST[63-0] = SRC[0-63]
///     BIT_REFLECT32: DST[31-0] = SRC[0-31]
///     BIT_REFLECT16: DST[15-0] = SRC[0-15]
///     BIT_REFLECT8: DST[7-0] = SRC[0-7]
///     MOD2: Remainder from Polynomial division modulus 2
/// CRC32 instruction for 64-bit source operand and 64-bit destination operand:
///     TEMP1[63-0] := BIT_REFLECT64 (SRC[63-0])
///     TEMP2[31-0] := BIT_REFLECT32 (DEST[31-0])
///     TEMP3[95-0] := TEMP1[63-0] << 32
///     TEMP4[95-0] := TEMP2[31-0] << 64
///     TEMP5[95-0] := TEMP3[95-0] XOR TEMP4[95-0]
///     TEMP6[31-0] := TEMP5[95-0] MOD2 11EDC6F41H
///     DEST[31-0] := BIT_REFLECT (TEMP6[31-0])
///     DEST[63-32] := 00000000H
/// CRC32 instruction for 32-bit source operand and 32-bit destination operand:
///     TEMP1[31-0] := BIT_REFLECT32 (SRC[31-0])
///     TEMP2[31-0] := BIT_REFLECT32 (DEST[31-0])
///     TEMP3[63-0] := TEMP1[31-0] << 32
///     TEMP4[63-0] := TEMP2[31-0] << 32
///     TEMP5[63-0] := TEMP3[63-0] XOR TEMP4[63-0]
///     TEMP6[31-0] := TEMP5[63-0] MOD2 11EDC6F41H
///     DEST[31-0] := BIT_REFLECT (TEMP6[31-0])
/// CRC32 instruction for 16-bit source operand and 32-bit destination operand:
///     TEMP1[15-0] := BIT_REFLECT16 (SRC[15-0])
///     TEMP2[31-0] := BIT_REFLECT32 (DEST[31-0])
///     TEMP3[47-0] := TEMP1[15-0] << 32
///     TEMP4[47-0] := TEMP2[31-0] << 16
///     TEMP5[47-0] := TEMP3[47-0] XOR TEMP4[47-0]
///     TEMP6[31-0] := TEMP5[47-0] MOD2 11EDC6F41H
///     DEST[31-0] := BIT_REFLECT (TEMP6[31-0])
/// CRC32 instruction for 8-bit source operand and 64-bit destination operand:
///     TEMP1[7-0] := BIT_REFLECT8(SRC[7-0])
///     TEMP2[31-0] := BIT_REFLECT32 (DEST[31-0])
///     TEMP3[39-0] := TEMP1[7-0] << 32
///     TEMP4[39-0] := TEMP2[31-0] << 8
///     TEMP5[39-0] := TEMP3[39-0] XOR TEMP4[39-0]
///     TEMP6[31-0] := TEMP5[39-0] MOD2 11EDC6F41H
///     DEST[31-0] := BIT_REFLECT (TEMP6[31-0])
///     DEST[63-32] := 00000000H
/// CRC32 instruction for 8-bit source operand and 32-bit destination operand:
///     TEMP1[7-0] := BIT_REFLECT8(SRC[7-0])
///     TEMP2[31-0] := BIT_REFLECT32 (DEST[31-0])
///     TEMP3[39-0] := TEMP1[7-0] << 32
///     TEMP4[39-0] := TEMP2[31-0] << 8
///     TEMP5[39-0] := TEMP3[39-0] XOR TEMP4[39-0]
///     TEMP6[31-0] := TEMP5[39-0] MOD2 11EDC6F41H
///     DEST[31-0] := BIT_REFLECT (TEMP6[31-0])
/// ```
#[box_to_static_reference]
pub(super) fn crc32() -> &'static [IrStatement] {
    [exception("crc32")].into()
}

/// # Pseudocode
/// ```text
/// VCVTDQ2PD (EVEX Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 Convert_Integer_To_Double_Precision_Floating_Point(SRC[k+31:k])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTDQ2PD (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 Convert_Integer_To_Double_Precision_Floating_Point(SRC[31:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 Convert_Integer_To_Double_Precision_Floating_Point(SRC[k+31:k])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTDQ2PD (VEX.256 Encoded Version)
/// DEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[31:0])
/// DEST[127:64] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[63:32])
/// DEST[191:128] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[95:64])
/// DEST[255:192] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[127:96)
/// DEST[MAXVL-1:256] := 0
/// VCVTDQ2PD (VEX.128 Encoded Version)
/// DEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[31:0])
/// DEST[127:64] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[63:32])
/// DEST[MAXVL-1:128] := 0
/// CVTDQ2PD (128-bit Legacy SSE Version)
/// DEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[31:0])
/// DEST[127:64] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[63:32])
/// DEST[MAXVL-1:128] (unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn cvtdq2pd() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTDQ2PS (EVEX Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);  ; refer to Table 15-4 in the Intel® 64 and IA-32 Architectures
/// Software Developer's Manual, Volume 1
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);  ; refer to Table 15-4 in the Intel® 64 and IA-32 Architectures
/// Software Developer's Manual, Volume 1
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 Convert_Integer_To_Single_Precision_Floating_Point(SRC[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTDQ2PS (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 Convert_Integer_To_Single_Precision_Floating_Point(SRC[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 Convert_Integer_To_Single_Precision_Floating_Point(SRC[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTDQ2PS (VEX.256 Encoded Version)
/// DEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[31:0])
/// DEST[63:32] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[63:32])
/// DEST[95:64] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[95:64])
/// DEST[127:96] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[127:96)
/// DEST[159:128] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[159:128])
/// DEST[191:160] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[191:160])
/// DEST[223:192] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[223:192])
/// DEST[255:224] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[255:224)
/// DEST[MAXVL-1:256] := 0
/// VCVTDQ2PS (VEX.128 Encoded Version)
/// DEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[31:0])
/// DEST[63:32] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[63:32])
/// DEST[95:64] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[95:64])
/// DEST[127:96] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[127z:96)
/// DEST[MAXVL-1:128] := 0
/// CVTDQ2PS (128-bit Legacy SSE Version)
/// DEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[31:0])
/// DEST[63:32] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[63:32])
/// DEST[95:64] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[95:64])
/// DEST[127:96] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[127z:96)
/// DEST[MAXVL-1:128] (unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn cvtdq2ps() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPD2DQ (EVEX Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 Convert_Double_Precision_Floating_Point_To_Integer(SRC[k+63:k])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL/2] := 0
/// VCVTPD2DQ (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 Convert_Double_Precision_Floating_Point_To_Integer(SRC[63:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 Convert_Double_Precision_Floating_Point_To_Integer(SRC[k+63:k])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL/2] := 0
/// VCVTPD2DQ (VEX.256 Encoded Version)
/// DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[63:0])
/// DEST[63:32] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[127:64])
/// DEST[95:64] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[191:128])
/// DEST[127:96] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[255:192)
/// DEST[MAXVL-1:128] := 0
/// VCVTPD2DQ (VEX.128 Encoded Version)
/// DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[63:0])
/// DEST[63:32] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[127:64])
/// DEST[MAXVL-1:64] := 0
/// CVTPD2DQ (128-bit Legacy SSE Version)
/// DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[63:0])
/// DEST[63:32] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[127:64])
/// DEST[127:64] := 0
/// DEST[MAXVL-1:128] (unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn cvtpd2dq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer32(SRC[63:0]);
/// DEST[63:32] := Convert_Double_Precision_Floating_Point_To_Integer32(SRC[127:64]);
/// ```
#[box_to_static_reference]
pub(super) fn cvtpd2pi() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPD2PS (EVEX Encoded Version) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+31:i] := Convert_Double_Precision_Floating_Point_To_Single_Precision_Floating_Point(SRC[k+63:k])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL/2] := 0
/// VCVTPD2PS (EVEX Encoded Version) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=Convert_Double_Precision_Floating_Point_To_Single_Precision_Floating_Point(SRC[63:0])
///                     ELSE
///                         DEST[i+31:i] := Convert_Double_Precision_Floating_Point_To_Single_Precision_Floating_Point(SRC[k+63:k])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL/2] := 0
/// VCVTPD2PS (VEX.256 Encoded Version)
/// DEST[31:0] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[63:0])
/// DEST[63:32] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[127:64])
/// DEST[95:64] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[191:128])
/// DEST[127:96] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[255:192)
/// DEST[MAXVL-1:128] := 0
/// VCVTPD2PS (VEX.128 Encoded Version)
/// DEST[31:0] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[63:0])
/// DEST[63:32] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[127:64])
/// DEST[MAXVL-1:64] := 0
/// CVTPD2PS (128-bit Legacy SSE Version)
/// DEST[31:0] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[63:0])
/// DEST[63:32] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[127:64])
/// DEST[127:64] := 0
/// DEST[MAXVL-1:128] (unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn cvtpd2ps() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[31:0]);
/// DEST[127:64] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[63:32]);
/// ```
#[box_to_static_reference]
pub(super) fn cvtpi2pd() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[31:0]);
/// DEST[63:32] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[63:32]);
/// (* High quadword of destination unchanged *)
/// ```
#[box_to_static_reference]
pub(super) fn cvtpi2ps() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPS2DQ (Encoded Versions) When SRC Operand is a Register
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
///         THEN DEST[i+31:i] :=
///                 Convert_Single_Precision_Floating_Point_To_Integer(SRC[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTPS2DQ (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO 15
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 Convert_Single_Precision_Floating_Point_To_Integer(SRC[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTPS2DQ (VEX.256 Encoded Version)
/// DEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0])
/// DEST[63:32] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[63:32])
/// DEST[95:64] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[95:64])
/// DEST[127:96] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[127:96)
/// DEST[159:128] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[159:128])
/// DEST[191:160] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[191:160])
/// DEST[223:192] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[223:192])
/// DEST[255:224] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[255:224])
/// VCVTPS2DQ (VEX.128 Encoded Version)
/// DEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0])
/// DEST[63:32] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[63:32])
/// DEST[95:64] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[95:64])
/// DEST[127:96] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[127:96])
/// DEST[MAXVL-1:128] := 0
/// CVTPS2DQ (128-bit Legacy SSE Version)
/// DEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0])
/// DEST[63:32] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[63:32])
/// DEST[95:64] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[95:64])
/// DEST[127:96] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[127:96])
/// DEST[MAXVL-1:128] (unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn cvtps2dq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPS2PD (EVEX Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[k+31:k])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTPS2PD (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[31:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[k+31:k])
///                 FI;
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTPS2PD (VEX.256 Encoded Version)
/// DEST[63:0] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[31:0])
/// DEST[127:64] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[63:32])
/// DEST[191:128] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[95:64])
/// DEST[255:192] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[127:96)
/// DEST[MAXVL-1:256] := 0
/// VCVTPS2PD (VEX.128 Encoded Version)
/// DEST[63:0] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[31:0])
/// DEST[127:64] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[63:32])
/// DEST[MAXVL-1:128] := 0
/// CVTPS2PD (128-bit Legacy SSE Version)
/// DEST[63:0] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[31:0])
/// DEST[127:64] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[63:32])
/// DEST[MAXVL-1:128] (unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn cvtps2pd() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0]);
/// DEST[63:32] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[63:32]);
/// ```
#[box_to_static_reference]
pub(super) fn cvtps2pi() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTSD2SI (EVEX Encoded Version)
/// IF SRC *is register* AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF 64-Bit Mode and OperandSize = 64
///     THEN DEST[63:0] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[63:0]);
///     ELSEDEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[63:0]);
/// FI
/// (V)CVTSD2SI
/// IF 64-Bit Mode and OperandSize = 64
/// THEN
///     DEST[63:0] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[63:0]);
/// ELSE
///     DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[63:0]);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cvtsd2si() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTSD2SS (EVEX Encoded Version)
/// IF (SRC2 *is register*) AND (EVEX.b = 1)
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC2[63:0]);
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// VCVTSD2SS (VEX.128 Encoded Version)
/// DEST[31:0] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC2[63:0]);
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// CVTSD2SS (128-bit Legacy SSE Version)
/// DEST[31:0] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[63:0]);
/// (* DEST[MAXVL-1:32] Unmodified *)
/// ```
#[box_to_static_reference]
pub(super) fn cvtsd2ss() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTSI2SD (EVEX Encoded Version)
/// IF (SRC2 *is register*) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF 64-Bit Mode And OperandSize = 64
/// THEN
///     DEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC2[63:0]);
/// ELSE
///     DEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC2[31:0]);
/// FI;
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// VCVTSI2SD (VEX.128 Encoded Version)
/// IF 64-Bit Mode And OperandSize = 64
/// THEN
///     DEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC2[63:0]);
/// ELSE
///     DEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC2[31:0]);
/// FI;
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// CVTSI2SD
/// IF 64-Bit Mode And OperandSize = 64
/// THEN
///     DEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[63:0]);
/// ELSE
///     DEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[31:0]);
/// FI;
/// DEST[MAXVL-1:64] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn cvtsi2sd() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTSI2SS (EVEX Encoded Version)
/// IF (SRC2 *is register*) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF 64-Bit Mode And OperandSize = 64
/// THEN
///     DEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[63:0]);
/// ELSE
///     DEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[31:0]);
/// FI;
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// VCVTSI2SS (VEX.128 Encoded Version)
/// IF 64-Bit Mode And OperandSize = 64
/// THEN
///     DEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[63:0]);
/// ELSE
///     DEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[31:0]);
/// FI;
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// CVTSI2SS (128-bit Legacy SSE Version)
/// IF 64-Bit Mode And OperandSize = 64
/// THEN
///     DEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[63:0]);
/// ELSE
///     DEST[31:0] :=Convert_Integer_To_Single_Precision_Floating_Point(SRC[31:0]);
/// FI;
/// DEST[MAXVL-1:32] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn cvtsi2ss() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTSS2SD (EVEX Encoded Version)
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC2[31:0]);
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] = 0
///             FI;
/// FI;
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// VCVTSS2SD (VEX.128 Encoded Version)
/// DEST[63:0] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC2[31:0])
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// CVTSS2SD (128-bit Legacy SSE Version)
/// DEST[63:0] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[31:0]);
/// DEST[MAXVL-1:64] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn cvtss2sd() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTSS2SI (EVEX Encoded Version)
/// IF (SRC *is register*) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF 64-bit Mode and OperandSize = 64
/// THEN
///     DEST[63:0] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0]);
/// ELSE
///     DEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0]);
/// FI;
/// (V)CVTSS2SI (Legacy and VEX.128 Encoded Version)
/// IF 64-bit Mode and OperandSize = 64
/// THEN
///     DEST[63:0] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0]);
/// ELSE
///     DEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0]);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cvtss2si() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTPD2DQ (EVEX Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[k+63:k])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL/2] := 0
/// VCVTTPD2DQ (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[63:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[k+63:k])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL/2] := 0
/// VCVTTPD2DQ (VEX.256 Encoded Version)
/// DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[63:0])
/// DEST[63:32] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[127:64])
/// DEST[95:64] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[191:128])
/// DEST[127:96] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[255:192)
/// DEST[MAXVL-1:128] := 0
/// VCVTTPD2DQ (VEX.128 Encoded Version)
/// DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[63:0])
/// DEST[63:32] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[127:64])
/// DEST[MAXVL-1:64] := 0
/// CVTTPD2DQ (128-bit Legacy SSE Version)
/// DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[63:0])
/// DEST[63:32] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[127:64])
/// DEST[127:64] := 0
/// DEST[MAXVL-1:128] (unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn cvttpd2dq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer32_Truncate(SRC[63:0]);
/// DEST[63:32] := Convert_Double_Precision_Floating_Point_To_Integer32_Truncate(SRC[127:64]);
/// ```
#[box_to_static_reference]
pub(super) fn cvttpd2pi() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTPS2DQ (EVEX Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTTPS2DQ (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO 15
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTTPS2DQ (VEX.256 Encoded Version)
/// DEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[31:0])
/// DEST[63:32] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[63:32])
/// DEST[95:64] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[95:64])
/// DEST[127:96] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[127:96)
/// DEST[159:128] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[159:128])
/// DEST[191:160] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[191:160])
/// DEST[223:192] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[223:192])
/// DEST[255:224] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[255:224])
/// VCVTTPS2DQ (VEX.128 Encoded Version)
/// DEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[31:0])
/// DEST[63:32] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[63:32])
/// DEST[95:64] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[95:64])
/// DEST[127:96] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[127:96])
/// DEST[MAXVL-1:128] := 0
/// CVTTPS2DQ (128-bit Legacy SSE Version)
/// DEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[31:0])
/// DEST[63:32] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[63:32])
/// DEST[95:64] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[95:64])
/// DEST[127:96] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[127:96])
/// DEST[MAXVL-1:128] (unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn cvttps2dq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[31:0]);
/// DEST[63:32] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[63:32]);
/// ```
#[box_to_static_reference]
pub(super) fn cvttps2pi() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// (V)CVTTSD2SI (All Versions)
/// IF 64-Bit Mode and OperandSize = 64
/// THEN
///     DEST[63:0] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[63:0]);
/// ELSE
///     DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[63:0]);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cvttsd2si() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// (V)CVTTSS2SI (All Versions)
/// IF 64-Bit Mode and OperandSize = 64
/// THEN
///     DEST[63:0] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[31:0]);
/// ELSE
///     DEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[31:0]);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cvttss2si() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF OperandSize = 16 (* CWD instruction *)
///     THEN
///         DX := SignExtend(AX);
///     ELSE IF OperandSize = 32 (* CDQ instruction *)
///         EDX := SignExtend(EAX); FI;
///     ELSE IF 64-Bit Mode and OperandSize = 64 (* CQO instruction*)
///         RDX := SignExtend(RAX); FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cwd() -> &'static [IrStatement] {
    let set_tmp = assign(u::sign_extend(ax.clone()), tmp32.clone(), size_relative(tmp32.clone()));
    let set_dx = assign(b::shr(tmp32.clone(), c(16)), dx.clone(), size_relative(dx.clone()));
    let set_ax = assign(tmp32.clone(), ax.clone(), size_relative(ax.clone()));
    let type1 = type_specified(ax.clone(), size_relative(ax.clone()), DataType::Int);
    let type2 = type_specified(dx.clone(), size_relative(dx.clone()), DataType::Int);
    [set_tmp, set_dx, set_ax, type1, type2].into()
}

/// # Pseudocode
/// ```text
/// IF OperandSize = 16 (* Instruction = CBW *)
///     THEN
///         AX := SignExtend(AL);
///     ELSE IF (OperandSize = 32, Instruction = CWDE)
///         EAX := SignExtend(AX); FI;
///     ELSE (* 64-Bit Mode, OperandSize = 64, Instruction = CDQE*)
///         RAX := SignExtend(EAX);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn cwde() -> &'static [IrStatement] {
    let ext = assign(u::sign_extend(ax.clone()), eax.clone(), size_relative(eax.clone()));
    let type1 = type_specified(ax.clone(), size_relative(ax.clone()), DataType::Int);
    let type2 = type_specified(eax.clone(), size_relative(eax.clone()), DataType::Int);
    [ext, type1, type2].into()
}
