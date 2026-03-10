use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// XABORT
/// IF RTM_ACTIVE = 0
///     THEN
///         Treat as NOP;
///     ELSE
///         GOTO RTM_ABORT_PROCESSING;
/// FI;
/// (* For any RTM abort condition encountered during RTM execution *)
/// RTM_ABORT_PROCESSING:
///     Restore architectural register state;
///     Discard memory updates performed in transaction;
///     Update EAX with status and XABORT argument;
///     RTM_NEST_COUNT:= 0;
///     RTM_ACTIVE:= 0;
///     SUSLDTRK_ACTIVE := 0;
///     IF 64-bit Mode
///         THEN
///             RIP:= fallbackRIP;
///         ELSE
///             EIP := fallbackEIP;
///     FI;
/// END
/// ```
#[box_to_static_reference]
pub(super) fn xabort() -> &'static [IrStatement] {
    [exception("xabort")].into()
}

/// # Pseudocode
/// ```text
/// XACQUIRE
/// IF XACQUIRE-enabled instruction
///     THEN
///         IF (HLE_NEST_COUNT < MAX_HLE_NEST_COUNT) THEN
///             HLE_NEST_COUNT++
///             IF (HLE_NEST_COUNT = 1) THEN
///                 HLE_ACTIVE := 1
///                 IF 64-bit mode
///                     THEN
///                         restartRIP := instruction pointer of the XACQUIRE-enabled instruction
///                     ELSE
///                         restartEIP := instruction pointer of the XACQUIRE-enabled instruction
///                 FI;
///                 Enter HLE Execution (* record register state, start tracking memory state *)
///             FI; (* HLE_NEST_COUNT = 1*)
///             IF ElisionBufferAvailable
///                 THEN
///                     Allocate elision buffer
///                     Record address and data for forwarding and commit checking
///                     Perform elision
///                 ELSE
///                     Perform lock acquire operation transactionally but without elision
///             FI;
///         ELSE (* HLE_NEST_COUNT = MAX_HLE_NEST_COUNT*)
///                 GOTO HLE_ABORT_PROCESSING
///         FI;
///     ELSE
///         Treat instruction as non-XACQUIRE F2H prefixed legacy instruction
/// FI;
/// XRELEASE
/// IF XRELEASE-enabled instruction
///     THEN
///         IF (HLE_NEST_COUNT > 0)
///             THEN
///                 HLE_NEST_COUNT--
///                 IF lock address matches in elision buffer THEN
///                     IF lock satisfies address and value requirements THEN
///                         Deallocate elision buffer
///                     ELSE
///                         GOTO HLE_ABORT_PROCESSING
///                     FI;
///                 FI;
///                 IF (HLE_NEST_COUNT = 0)
///                     THEN
///                         IF NoAllocatedElisionBuffer
///                             THEN
///                                 Try to commit transactional execution
///                                 IF fail to commit transactional execution
///                                     THEN
///                                         GOTO HLE_ABORT_PROCESSING;
///                                     ELSE (* commit success *)
///                                         HLE_ACTIVE := 0
///                                 FI;
///                             ELSE
///                                 GOTO HLE_ABORT_PROCESSING
///                         FI;
///                 FI;
///         FI; (* HLE_NEST_COUNT > 0 *)
///     ELSE
///         Treat instruction as non-XRELEASE F3H prefixed legacy instruction
/// FI;
/// (* For any HLE abort condition encountered during HLE execution *)
/// HLE_ABORT_PROCESSING:
/// HLE_ACTIVE := 0
///     HLE_NEST_COUNT := 0
///     Restore architectural register state
///     Discard memory updates performed in transaction
///     Free any allocated lock elision buffers
///     IF 64-bit mode
///         THEN
///             RIP := restartRIP
///         ELSE
///             EIP := restartEIP
///     FI;
///     Execute and retire instruction at RIP (or EIP) and ignore any HLE hint
/// END
/// ```
#[box_to_static_reference]
pub(super) fn xacquire() -> &'static [IrStatement] {
    [exception("xacquire")].into()
}

/// # Pseudocode
/// ```text
/// TEMP := SRC + DEST;
/// SRC := DEST;
/// DEST := TEMP;
/// ```
#[box_to_static_reference]
pub(super) fn xadd() -> &'static [IrStatement] {
    let sum = b::add(o1(), o2());
    let save_o1 = assign(o1(), o2(), o2_size());
    let set_o1 = assign(sum.clone(), o1(), o1_size());
    let calc_flags = calc_flags_automatically(sum, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [save_o1, set_o1, calc_flags].into()
}

/// # Pseudocode
/// ```text
/// XBEGIN
/// IF RTM_NEST_COUNT < MAX_RTM_NEST_COUNT AND SUSLDTRK_ACTIVE = 0
///     THEN
///         RTM_NEST_COUNT++
///         IF RTM_NEST_COUNT = 1 THEN
///             IF 64-bit Mode
///                 THEN
///                     IF OperandSize = 16
///                         THEN fallbackRIP := RIP + SignExtend64(rel16);
///                         ELSE fallbackRIP := RIP + SignExtend64(rel32);
///                     FI;
///                     IF fallbackRIP is not canonical
///                         THEN #GP(0);
///                     FI;
///                 ELSE
///                     IF OperandSize = 16
///                         THEN fallbackEIP := EIP + SignExtend32(rel16);
///                         ELSE fallbackEIP := EIP + rel32;
///                     FI;
///                     IF fallbackEIP outside code segment limit
///                         THEN #GP(0);
///                     FI;
///             RTM_ACTIVE := 1
///             Enter RTM Execution (* record register state, start tracking memory state*)
///         FI; (* RTM_NEST_COUNT = 1 *)
///     ELSE (* RTM_NEST_COUNT = MAX_RTM_NEST_COUNT OR SUSLDTRK_ACTIVE = 1 *)
///         GOTO RTM_ABORT_PROCESSING
/// FI;
/// (* For any RTM abort condition encountered during RTM execution *)
/// RTM_ABORT_PROCESSING:
///     Restore architectural register state
///     Discard memory updates performed in transaction
///     Update EAX with status
///     RTM_NEST_COUNT := 0
///     RTM_ACTIVE := 0
///     SUSLDTRK_ACTIVE := 0
///     IF 64-bit mode
///         THEN
///             RIP := fallbackRIP
///         ELSE
///             EIP := fallbackEIP
///     FI;
/// END
/// ```
#[box_to_static_reference]
pub(super) fn xbegin() -> &'static [IrStatement] {
    [exception("xbegin")].into()
}

/// # Pseudocode
/// ```text
/// TEMP := DEST;
/// DEST := SRC;
/// SRC := TEMP;
/// ```
#[box_to_static_reference]
pub(super) fn xchg() -> &'static [IrStatement] {
    let set_tmp = assign(o1(), tmp64.clone(), o1_size());
    let set_o1 = assign(o2(), o1(), o1_size());
    let set_o2 = assign(tmp64.clone(), o2(), o2_size());
    [set_tmp, set_o1, set_o2].into()
}

/// # Pseudocode
/// ```text
/// XEND
/// IF (RTM_ACTIVE = 0) THEN
///     SIGNAL #GP
/// ELSE
///     IF SUSLDTRK_ACTIVE = 1
///         THEN GOTO RTM_ABORT_PROCESSING;
///     FI;
///     RTM_NEST_COUNT--
///     IF (RTM_NEST_COUNT = 0) THEN
///         Try to commit transaction
///         IF fail to commit transactional execution
///             THEN
///                 GOTO RTM_ABORT_PROCESSING;
///             ELSE (* commit success *)
///                 RTM_ACTIVE := 0
///         FI;
///     FI;
/// FI;
/// (* For any RTM abort condition encountered during RTM execution *)
/// RTM_ABORT_PROCESSING:
///     Restore architectural register state
///     Discard memory updates performed in transaction
///     Update EAX with status
///     RTM_NEST_COUNT := 0
///     RTM_ACTIVE := 0
///     SUSLDTRK_ACTIVE := 0
///     IF 64-bit Mode
///         THEN
///             RIP := fallbackRIP
///         ELSE
///             EIP := fallbackEIP
///     FI;
/// END
/// ```
#[box_to_static_reference]
pub(super) fn xend() -> &'static [IrStatement] {
    [exception("xend")].into()
}

/// # Pseudocode
/// ```text
/// EDX:EAX := XCR[ECX];
/// ```
#[box_to_static_reference]
pub(super) fn xgetbv() -> &'static [IrStatement] {
    [exception("xgetbv")].into()
}

/// # Pseudocode
/// ```text
/// IF AddressSize = 16
///     THEN
///         AL := (DS:BX + ZeroExtend(AL));
///     ELSE IF (AddressSize = 32)
///         AL := (DS:EBX + ZeroExtend(AL)); FI;
///     ELSE (AddressSize = 64)
///         AL := (RBX + ZeroExtend(AL));
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn xlat() -> &'static [IrStatement] {
    let cond = condition(b::equal(architecture_bit_size(), c(16), o1_size()), [stmt_0, stmt_1, stmt_2], []);
    [cond].into()
}

/// # Pseudocode
/// ```text
/// IF AddressSize = 16
///     THEN
///         AL := (DS:BX + ZeroExtend(AL));
///     ELSE IF (AddressSize = 32)
///         AL := (DS:EBX + ZeroExtend(AL)); FI;
///     ELSE (AddressSize = 64)
///         AL := (RBX + ZeroExtend(AL));
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn xlatb() -> &'static [IrStatement] {
    let cond = condition(b::equal(architecture_bit_size(), c(16), o1_size()), [stmt_0, stmt_1, stmt_2], []);
    [cond].into()
}

/// # Pseudocode
/// ```text
/// DEST := DEST XOR SRC;
/// ```
#[box_to_static_reference]
pub(super) fn xor() -> &'static [IrStatement] {
    let cond = b::equal(o1(), o2(), o1_size());
    let true_b = [
        assign(c(0), o1(), o1_size()),
        assign(c(1), zf.clone(), size_relative(zf.clone())),
        assign(c(0), sf.clone(), size_relative(sf.clone())),
        assign(c(0), pf.clone(), size_relative(pf.clone())),
    ];
    let false_b = b::xor(o1(), o2());
    let false_b = [
        assign(false_b.clone(), o1(), o1_size()),
        assign(c(0), zf.clone(), size_relative(zf.clone())),
        calc_flags_automatically(false_b, o1_size(), &[&sf, &pf]),
    ];
    let xor = condition(cond, true_b, false_b);
    let set_of = assign(c(0), of.clone(), size_relative(of.clone()));
    let set_cf = assign(c(0), cf.clone(), size_relative(cf.clone()));
    extend_undefined_flags(&[xor, set_of, set_cf], &[&af])
}

/// # Pseudocode
/// ```text
/// VXORPD (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b == 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+63:i] := SRC1[i+63:i] BITWISE XOR SRC2[63:0];
///                 ELSE DEST[i+63:i] := SRC1[i+63:i] BITWISE XOR SRC2[i+63:i];
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VXORPD (VEX.256 Encoded Version)
/// DEST[63:0] := SRC1[63:0] BITWISE XOR SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] BITWISE XOR SRC2[127:64]
/// DEST[191:128] := SRC1[191:128] BITWISE XOR SRC2[191:128]
/// DEST[255:192] := SRC1[255:192] BITWISE XOR SRC2[255:192]
/// DEST[MAXVL-1:256] := 0
/// VXORPD (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[63:0] BITWISE XOR SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] BITWISE XOR SRC2[127:64]
/// DEST[MAXVL-1:128] := 0
/// XORPD (128-bit Legacy SSE Version)
/// DEST[63:0] := DEST[63:0] BITWISE XOR SRC[63:0]
/// DEST[127:64] := DEST[127:64] BITWISE XOR SRC[127:64]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn xorpd() -> &'static [IrStatement] {
    let assignment = assign(b::xor(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VXORPS (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b == 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+31:i] := SRC1[i+31:i] BITWISE XOR SRC2[31:0];
///                 ELSE DEST[i+31:i] := SRC1[i+31:i] BITWISE XOR SRC2[i+31:i];
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VXORPS (VEX.256 Encoded Version)
/// DEST[31:0] := SRC1[31:0] BITWISE XOR SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] BITWISE XOR SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] BITWISE XOR SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] BITWISE XOR SRC2[127:96]
/// DEST[159:128] := SRC1[159:128] BITWISE XOR SRC2[159:128]
/// DEST[191:160] := SRC1[191:160] BITWISE XOR SRC2[191:160]
/// DEST[223:192] := SRC1[223:192] BITWISE XOR SRC2[223:192]
/// DEST[255:224] := SRC1[255:224] BITWISE XOR SRC2[255:224].
/// DEST[MAXVL-1:256] := 0
/// VXORPS (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0] BITWISE XOR SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] BITWISE XOR SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] BITWISE XOR SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] BITWISE XOR SRC2[127:96]
/// DEST[MAXVL-1:128] := 0
/// XORPS (128-bit Legacy SSE Version)
/// DEST[31:0] := SRC1[31:0] BITWISE XOR SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] BITWISE XOR SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] BITWISE XOR SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] BITWISE XOR SRC2[127:96]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn xorps() -> &'static [IrStatement] {
    let assignment = assign(b::xor(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// XACQUIRE
/// IF XACQUIRE-enabled instruction
///     THEN
///         IF (HLE_NEST_COUNT < MAX_HLE_NEST_COUNT) THEN
///             HLE_NEST_COUNT++
///             IF (HLE_NEST_COUNT = 1) THEN
///                 HLE_ACTIVE := 1
///                 IF 64-bit mode
///                     THEN
///                         restartRIP := instruction pointer of the XACQUIRE-enabled instruction
///                     ELSE
///                         restartEIP := instruction pointer of the XACQUIRE-enabled instruction
///                 FI;
///                 Enter HLE Execution (* record register state, start tracking memory state *)
///             FI; (* HLE_NEST_COUNT = 1*)
///             IF ElisionBufferAvailable
///                 THEN
///                     Allocate elision buffer
///                     Record address and data for forwarding and commit checking
///                     Perform elision
///                 ELSE
///                     Perform lock acquire operation transactionally but without elision
///             FI;
///         ELSE (* HLE_NEST_COUNT = MAX_HLE_NEST_COUNT*)
///                 GOTO HLE_ABORT_PROCESSING
///         FI;
///     ELSE
///         Treat instruction as non-XACQUIRE F2H prefixed legacy instruction
/// FI;
/// XRELEASE
/// IF XRELEASE-enabled instruction
///     THEN
///         IF (HLE_NEST_COUNT > 0)
///             THEN
///                 HLE_NEST_COUNT--
///                 IF lock address matches in elision buffer THEN
///                     IF lock satisfies address and value requirements THEN
///                         Deallocate elision buffer
///                     ELSE
///                         GOTO HLE_ABORT_PROCESSING
///                     FI;
///                 FI;
///                 IF (HLE_NEST_COUNT = 0)
///                     THEN
///                         IF NoAllocatedElisionBuffer
///                             THEN
///                                 Try to commit transactional execution
///                                 IF fail to commit transactional execution
///                                     THEN
///                                         GOTO HLE_ABORT_PROCESSING;
///                                     ELSE (* commit success *)
///                                         HLE_ACTIVE := 0
///                                 FI;
///                             ELSE
///                                 GOTO HLE_ABORT_PROCESSING
///                         FI;
///                 FI;
///         FI; (* HLE_NEST_COUNT > 0 *)
///     ELSE
///         Treat instruction as non-XRELEASE F3H prefixed legacy instruction
/// FI;
/// (* For any HLE abort condition encountered during HLE execution *)
/// HLE_ABORT_PROCESSING:
/// HLE_ACTIVE := 0
///     HLE_NEST_COUNT := 0
///     Restore architectural register state
///     Discard memory updates performed in transaction
///     Free any allocated lock elision buffers
///     IF 64-bit mode
///         THEN
///             RIP := restartRIP
///         ELSE
///             EIP := restartEIP
///     FI;
///     Execute and retire instruction at RIP (or EIP) and ignore any HLE hint
/// END
/// ```
#[box_to_static_reference]
pub(super) fn xrelease() -> &'static [IrStatement] {
    [exception("xrelease")].into()
}

/// # Pseudocode
/// ```text
/// XRESLDTRK
/// IF RTM_ACTIVE = 1:
///     IF SUSLDTRK_ACTIVE = 1:
///         SUSLDTRK_ACTIVE := 0
///     ELSE:
///         RTM_ABORT
/// ELSE:
///     NOP
/// ```
#[box_to_static_reference]
pub(super) fn xresldtrk() -> &'static [IrStatement] {
    [exception("xresldtrk")].into()
}

/// # Pseudocode
/// ```text
/// RFBM := XCR0 AND EDX:EAX;/* bitwise logical AND */
/// COMPMASK := XCOMP_BV field from XSAVE header;
/// RSTORMASK := XSTATE_BV field from XSAVE header;
/// IF COMPMASK[63] = 0
///     THEN
///         /* Standard form of XRSTOR */
///         TO_BE_RESTORED := RFBM AND RSTORMASK;
///         TO_BE_INITIALIZED := RFBM AND NOT RSTORMASK;
///         IF TO_BE_RESTORED[0] = 1
///             THEN
///                 XINUSE[0] := 1;
///                 load x87 state from legacy region of XSAVE area;
///         ELSIF TO_BE_INITIALIZED[0] = 1
///             THEN
///                 XINUSE[0] := 0;
///                 initialize x87 state;
///         FI;
///         IF RFBM[1] = 1 OR RFBM[2]= 1
///             THEN load MXCSR from legacy region of XSAVE area;
///         FI;
///         IF TO_BE_RESTORED[1] = 1
///             THEN
///                 XINUSE[1] := 1;
///                 load XMM registers from legacy region of XSAVE area; // this step does not load MXCSR
///         ELSIF TO_BE_INITIALIZED[1] = 1
///             THEN
///                 XINUSE[1] := 0;
///                 set all XMM registers to 0; // this step does not initialize MXCSR
///         FI;
///         FOR i := 2 TO 62
///             IF TO_BE_RESTORED[i] = 1
///                 THEN
///                     XINUSE[i] := 1;
///                     load XSAVE state component i at offset n from base of XSAVE area;
///                         // n enumerated by CPUID(EAX=0DH,ECX=i):EBX)
///             ELSIF TO_BE_INITIALIZED[i] = 1
///                 THEN
///                     XINUSE[i] := 0;
///                     initialize XSAVE state component i;
///             FI;
///         ENDFOR;
///     ELSE
///         /* Compacted form of XRSTOR */
///         IF CPUID.(EAX=0DH,ECX=1):EAX.XSAVEC[bit1]= 0
///             THEN/* compacted form not supported */
///                 #GP(0);
///         FORMAT = COMPMASK AND 7FFFFFFF_FFFFFFFFH;
///         RESTORE_FEATURES = FORMAT AND RFBM;
///         TO_BE_RESTORED := RESTORE_FEATURES AND RSTORMASK;
///         FORCE_INIT := RFBM AND NOT FORMAT;
///         TO_BE_INITIALIZED = (RFBM AND NOT RSTORMASK) OR FORCE_INIT;
///         IF TO_BE_RESTORED[0] = 1
///             THEN
///                 XINUSE[0] := 1;
///                 load x87 state from legacy region of XSAVE area;
///         ELSIF TO_BE_INITIALIZED[0] = 1
///             THEN
///                 XINUSE[0] := 0;
///                 initialize x87 state;
///         FI;
///         IF TO_BE_RESTORED[1] = 1
///             THEN
///                 XINUSE[1] := 1;
///                 load SSE state from legacy region of XSAVE area; // this step loads the XMM registers and MXCSR
///         ELSIF TO_BE_INITIALIZED[1] = 1
///             THEN
///                 set all XMM registers to 0;
///                 XINUSE[1] := 0;
///                 MXCSR := 1F80H;
///         FI;
///         NEXT_FEATURE_OFFSET = 576;
///                                 // Legacy area and XSAVE header consume 576 bytes
///         FOR i := 2 TO 62
///             IF FORMAT[i] = 1
///                 THEN
///                     IF TO_BE_RESTORED[i] = 1
///                         THEN
///                             XINUSE[i] := 1;
///                             load XSAVE state component i at offset NEXT_FEATURE_OFFSET from base of XSAVE area;
///                     FI;
///                     NEXT_FEATURE_OFFSET = NEXT_FEATURE_OFFSET + n (n enumerated by CPUID(EAX=0DH,ECX=i):EAX);
///             FI;
///             IF TO_BE_INITIALIZED[i] = 1
///                 THEN
///                     XINUSE[i] := 0;
///                     initialize XSAVE state component i;
///             FI;
///         ENDFOR;
/// FI;
/// XMODIFIED := NOT RFBM;
/// IF in VMX non-root operation
///     THEN VMXNR := 1;
///     ELSE VMXNR := 0;
/// FI;
/// LAXA := linear address of XSAVE area;
/// XRSTOR_INFO := CPL,VMXNR,LAXA,COMPMASK;
/// ```
#[box_to_static_reference]
pub(super) fn xrstor() -> &'static [IrStatement] {
    [exception("xrstor")].into()
}

/// # Pseudocode
/// ```text
/// RFBM := (XCR0 OR IA32_XSS) AND EDX:EAX;
///                             /* bitwise logical OR and AND */
/// COMPMASK := XCOMP_BV field from XSAVE header;
/// RSTORMASK := XSTATE_BV field from XSAVE header;
/// FORMAT = COMPMASK AND 7FFFFFFF_FFFFFFFFH;
/// RESTORE_FEATURES = FORMAT AND RFBM;
/// TO_BE_RESTORED := RESTORE_FEATURES AND RSTORMASK;
/// FORCE_INIT := RFBM AND NOT FORMAT;
/// TO_BE_INITIALIZED = (RFBM AND NOT RSTORMASK) OR FORCE_INIT;
/// IF TO_BE_RESTORED[0] = 1
///     THEN
///         XINUSE[0] := 1;
///         load x87 state from legacy region of XSAVE area;
/// ELSIF TO_BE_INITIALIZED[0] = 1
///     THEN
///         XINUSE[0] := 0;
///         initialize x87 state;
/// FI;
/// IF TO_BE_RESTORED[1] = 1
///     THEN
///         XINUSE[1] := 1;
///         load SSE state from legacy region of XSAVE area; // this step loads the XMM registers and MXCSR
/// ELSIF TO_BE_INITIALIZED[1] = 1
///     THEN
///         set all XMM registers to 0;
///         XINUSE[1] := 0;
///         MXCSR := 1F80H;
/// FI;
/// NEXT_FEATURE_OFFSET = 576;
///                         // Legacy area and XSAVE header consume 576 bytes
/// FOR i := 2 TO 62
/// 
///     IF FORMAT[i] = 1
///         THEN
///             IF TO_BE_RESTORED[i] = 1
///                 THEN
///                     XINUSE[i] := 1;
///                     load XSAVE state component i at offset NEXT_FEATURE_OFFSET from base of XSAVE area;
///             FI;
///             NEXT_FEATURE_OFFSET = NEXT_FEATURE_OFFSET + n (n enumerated by CPUID(EAX=0DH,ECX=i):EAX);
///     FI;
///     IF TO_BE_INITIALIZED[i] = 1
///         THEN
///             XINUSE[i] := 0;
///             initialize XSAVE state component i;
///     FI;
/// ENDFOR;
/// XMODIFIED := NOT RFBM;
/// IF in VMX non-root operation
///     THEN VMXNR := 1;
///     ELSE VMXNR := 0;
/// FI;
/// LAXA := linear address of XSAVE area;
/// XRSTOR_INFO := CPL,VMXNR,LAXA,COMPMASK;
/// ```
#[box_to_static_reference]
pub(super) fn xrstors() -> &'static [IrStatement] {
    [exception("xrstors")].into()
}

/// # Pseudocode
/// ```text
/// RFBM := XCR0 AND EDX:EAX;/* bitwise logical AND */
/// OLD_BV := XSTATE_BV field from XSAVE header;
/// IF RFBM[0]= 1
///     THEN store x87 state into legacy region of XSAVE area;
/// FI;
/// IF RFBM[1]= 1
///     THEN store XMM registers into legacy region of XSAVE area; // this step does not save MXCSR or MXCSR_MASK
/// FI;
/// IF RFBM[1]= 1 OR RFBM[2]= 1
///     THEN store MXCSR and MXCSR_MASK into legacy region of XSAVE area;
/// FI;
/// FOR i := 2 TO 62
///     IF RFBM[i] = 1
///         THEN save XSAVE state component i at offset n from base of XSAVE area (n enumerated by CPUID(EAX=0DH,ECX=i):EBX);
///     FI;
/// ENDFOR;
/// XSTATE_BV field in XSAVE header := (OLD_BV AND NOT RFBM) OR (XINUSE AND RFBM);
/// ```
#[box_to_static_reference]
pub(super) fn xsave() -> &'static [IrStatement] {
    [exception("xsave")].into()
}

/// # Pseudocode
/// ```text
/// RFBM := XCR0 AND EDX:EAX;
///                     /* bitwise logical AND */
/// TO_BE_SAVED := RFBM AND XINUSE;/* bitwise logical AND */
/// If MXCSR  ≠1F80H AND RFBM[1]
///     TO_BE_SAVED[1] = 1;
/// FI;
/// IF TO_BE_SAVED[0]= 1
///     THEN store x87 state into legacy region of XSAVE area;
/// FI;
/// IF TO_BE_SAVED[1]= 1
///     THEN store SSE state into legacy region of XSAVE area; // this step saves the XMM registers, MXCSR, and MXCSR_MASK
/// FI;
/// NEXT_FEATURE_OFFSET = 576;
///                     // Legacy area and XSAVE header consume 576 bytes
/// FOR i := 2 TO 62
///     IF RFBM[i] = 1
///         THEN
///             IF TO_BE_SAVED[i]
///                 THEN save XSAVE state component i at offset NEXT_FEATURE_OFFSET from base of XSAVE area;
///             FI;
///             NEXT_FEATURE_OFFSET = NEXT_FEATURE_OFFSET + n (n enumerated by CPUID(EAX=0DH,ECX=i):EAX);
///     FI;
/// ENDFOR;
/// XSTATE_BV field in XSAVE header := TO_BE_SAVED;
/// XCOMP_BV field in XSAVE header := RFBM OR 80000000_00000000H;
/// ```
#[box_to_static_reference]
pub(super) fn xsavec() -> &'static [IrStatement] {
    [exception("xsavec")].into()
}

/// # Pseudocode
/// ```text
/// RFBM := XCR0 AND EDX:EAX;/* bitwise logical AND */
/// OLD_BV := XSTATE_BV field from XSAVE header;
/// TO_BE_SAVED := RFBM AND XINUSE;
/// IF in VMX non-root operation
///     THEN VMXNR := 1;
///     ELSE VMXNR := 0;
/// FI;
/// LAXA := linear address of XSAVE area;
/// IF XRSTOR_INFO= CPL,VMXNR,LAXA,00000000_00000000H
///     THEN TO_BE_SAVED := TO_BE_SAVED AND XMODIFIED;
/// FI;
/// IF TO_BE_SAVED[0]= 1
///     THEN store x87 state into legacy region of XSAVE area;
/// FI;
/// IF TO_BE_SAVED[1]
///     THEN store XMM registers into legacy region of XSAVE area; // this step does not save MXCSR or MXCSR_MASK
/// FI;
/// IF RFBM[1]= 1 or RFBM[2]= 1
///     THEN store MXCSR and MXCSR_MASK into legacy region of XSAVE area;
/// FI;
/// FOR i := 2 TO 62
///     IF TO_BE_SAVED[i] = 1
///         THEN save XSAVE state component i at offset n from base of XSAVE area (n enumerated by CPUID(EAX=0DH,ECX=i):EBX);
///     FI;
/// ENDFOR;
/// XSTATE_BV field in XSAVE header := (OLD_BV AND NOT RFBM) OR (XINUSE AND RFBM);
/// ```
#[box_to_static_reference]
pub(super) fn xsaveopt() -> &'static [IrStatement] {
    [exception("xsaveopt")].into()
}

/// # Pseudocode
/// ```text
/// RFBM := (XCR0 OR IA32_XSS) AND EDX:EAX;
///                                     /* bitwise logical OR and AND */
/// IF in VMX non-root operation
///     THEN VMXNR := 1;
///     ELSE VMXNR := 0;
/// FI;
/// LAXA := linear address of XSAVE area;
/// COMPMASK := RFBM OR 80000000_00000000H;
/// TO_BE_SAVED := RFBM AND XINUSE;
/// IF XRSTOR_INFO= CPL,VMXNR,LAXA,COMPMASK
///     THEN TO_BE_SAVED := TO_BE_SAVED AND XMODIFIED;
/// FI;
/// IF MXCSR  ≠1F80H AND RFBM[1]
///     THEN TO_BE_SAVED[1] = 1;
/// FI;
/// IF TO_BE_SAVED[0]= 1
///     THEN store x87 state into legacy region of XSAVE area;
/// FI;
/// IF TO_BE_SAVED[1]= 1
///     THEN store SSE state into legacy region of XSAVE area; // this step saves the XMM registers, MXCSR, and MXCSR_MASK
/// FI;
/// NEXT_FEATURE_OFFSET = 576;
///                                 // Legacy area and XSAVE header consume 576 bytes
/// FOR i := 2 TO 62
///     IF RFBM[i] = 1
///         THEN
///             IF TO_BE_SAVED[i]
///                 THEN
///                     save XSAVE state component i at offset NEXT_FEATURE_OFFSET from base of XSAVE area;
///                     IF i = 8
///                             // state component 8 is for PT state
///                         THEN IA32_RTIT_CTL.TraceEn[bit0] := 0;
///                     FI;
///             FI;
///             NEXT_FEATURE_OFFSET = NEXT_FEATURE_OFFSET + n (n enumerated by CPUID(EAX=0DH,ECX=i):EAX);
///     FI;
/// ENDFOR;
/// NEW_HEADER := RFBM AND XINUSE;
/// IF MXCSR  ≠1F80H AND RFBM[1]
///     THEN NEW_HEADER[1] = 1;
/// FI;
/// XSTATE_BV field in XSAVE header := NEW_HEADER;
/// XCOMP_BV field in XSAVE header := COMPMASK;
/// ```
#[box_to_static_reference]
pub(super) fn xsaves() -> &'static [IrStatement] {
    [exception("xsaves")].into()
}

/// # Pseudocode
/// ```text
/// XCR[ECX] := EDX:EAX;
/// ```
#[box_to_static_reference]
pub(super) fn xsetbv() -> &'static [IrStatement] {
    let stmt_0 = assign(edx.clone(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// XSUSLDTRK
/// IF RTM_ACTIVE = 1:
///     IF SUSLDTRK_ACTIVE = 0:
///         SUSLDTRK_ACTIVE := 1
///     ELSE:
///         RTM_ABORT
/// ELSE:
///     NOP
/// ```
#[box_to_static_reference]
pub(super) fn xsusldtrk() -> &'static [IrStatement] {
    [exception("xsusldtrk")].into()
}

/// # Pseudocode
/// ```text
/// XTEST
/// IF (RTM_ACTIVE = 1 OR HLE_ACTIVE = 1)
///     THEN
///         ZF := 0
///     ELSE
///         ZF := 1
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn xtest() -> &'static [IrStatement] {
    [exception("xtest")].into()
}
