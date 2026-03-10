use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// CheckForPendingUnmaskedFloatingPointExceptions;
/// ```
#[box_to_static_reference]
pub(super) fn wait() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// WriteBack(InternalCaches);
/// Flush(InternalCaches);
/// SignalWriteBack(ExternalCaches);
/// SignalFlush(ExternalCaches);
/// Continue; (* Continue execution *)
/// ```
#[box_to_static_reference]
pub(super) fn wbinvd() -> &'static [IrStatement] {
    [exception("WBINVD")].into()
}

/// # Pseudocode
/// ```text
/// WriteBack(InternalCaches);
/// Continue; (* Continue execution *)
/// ```
#[box_to_static_reference]
pub(super) fn wbnoinvd() -> &'static [IrStatement] {
    [exception("wbnoinvd")].into()
}

/// # Pseudocode
/// ```text
/// FS/GS segment base address := SRC;
/// ```
#[box_to_static_reference]
pub(super) fn wrfsbase() -> &'static [IrStatement] {
    [exception("wrfsbase")].into()
}

/// # Pseudocode
/// ```text
/// FS/GS segment base address := SRC;
/// ```
#[box_to_static_reference]
pub(super) fn wrgsbase() -> &'static [IrStatement] {
    [exception("wrgsbase")].into()
}

/// # Pseudocode
/// ```text
/// MSR[ECX] := EDX:EAX;
/// ```
#[box_to_static_reference]
pub(super) fn wrmsr() -> &'static [IrStatement] {
    [exception("WRMSR")].into()
}

/// # Pseudocode
/// ```text
/// IF (ECX = 0 AND EDX = 0)
///     THEN PKRU := EAX;
///     ELSE #GP(0);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn wrpkru() -> &'static [IrStatement] {
    [exception("wrpkru")].into()
}

/// # Pseudocode
/// ```text
/// IF CPL = 3
///     IF (CR4.CET & IA32_U_CET.SH_STK_EN) = 0
///         THEN #UD; FI;
///     IF (IA32_U_CET.WR_SHSTK_EN) = 0
///         THEN #UD; FI;
/// ELSE
///     IF (CR4.CET & IA32_S_CET.SH_STK_EN) = 0
///         THEN #UD; FI;
///     IF (IA32_S_CET.WR_SHSTK_EN) = 0
///         THEN #UD; FI;
/// FI;
/// DEST_LA = Linear_Address(mem operand)
/// IF (operand size is 64 bit)
///     THEN
///         (* Destination not 8B aligned *)
///         IF DEST_LA[2:0]
///             THEN GP(0); FI;
///         Shadow_stack_store 8 bytes of SRC to DEST_LA;
///     ELSE
///         (* Destination not 4B aligned *)
///         IF DEST_LA[1:0]
///             THEN GP(0); FI;
///         Shadow_stack_store 4 bytes of SRC[31:0] to DEST_LA;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn wrssd() -> &'static [IrStatement] {
    [exception("wrssd")].into()
}

/// # Pseudocode
/// ```text
/// IF CPL = 3
///     IF (CR4.CET & IA32_U_CET.SH_STK_EN) = 0
///         THEN #UD; FI;
///     IF (IA32_U_CET.WR_SHSTK_EN) = 0
///         THEN #UD; FI;
/// ELSE
///     IF (CR4.CET & IA32_S_CET.SH_STK_EN) = 0
///         THEN #UD; FI;
///     IF (IA32_S_CET.WR_SHSTK_EN) = 0
///         THEN #UD; FI;
/// FI;
/// DEST_LA = Linear_Address(mem operand)
/// IF (operand size is 64 bit)
///     THEN
///         (* Destination not 8B aligned *)
///         IF DEST_LA[2:0]
///             THEN GP(0); FI;
///         Shadow_stack_store 8 bytes of SRC to DEST_LA;
///     ELSE
///         (* Destination not 4B aligned *)
///         IF DEST_LA[1:0]
///             THEN GP(0); FI;
///         Shadow_stack_store 4 bytes of SRC[31:0] to DEST_LA;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn wrssq() -> &'static [IrStatement] {
    [exception("wrssq")].into()
}

/// # Pseudocode
/// ```text
/// IF CR4.CET = 0
///     THEN #UD; FI;
/// IF CPL > 0
///     THEN #GP(0); FI;
/// DEST_LA = Linear_Address(mem operand)
/// IF (operand size is 64 bit)
///     THEN
///         (* Destination not 8B aligned *)
///         IF DEST_LA[2:0]
///             THEN GP(0); FI;
///         Shadow_stack_store 8 bytes of SRC to DEST_LA as user-mode access;
///     ELSE
///         (* Destination not 4B aligned *)
///         IF DEST_LA[1:0]
///             THEN GP(0); FI;
///         Shadow_stack_store 4 bytes of SRC[31:0] to DEST_LA as user-mode access;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn wrussd() -> &'static [IrStatement] {
    [exception("wrussd")].into()
}

/// # Pseudocode
/// ```text
/// IF CR4.CET = 0
///     THEN #UD; FI;
/// IF CPL > 0
///     THEN #GP(0); FI;
/// DEST_LA = Linear_Address(mem operand)
/// IF (operand size is 64 bit)
///     THEN
///         (* Destination not 8B aligned *)
///         IF DEST_LA[2:0]
///             THEN GP(0); FI;
///         Shadow_stack_store 8 bytes of SRC to DEST_LA as user-mode access;
///     ELSE
///         (* Destination not 4B aligned *)
///         IF DEST_LA[1:0]
///             THEN GP(0); FI;
///         Shadow_stack_store 4 bytes of SRC[31:0] to DEST_LA as user-mode access;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn wrussq() -> &'static [IrStatement] {
    [exception("wrussq")].into()
}
