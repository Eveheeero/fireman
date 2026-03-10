#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ArmRegister {
    X(u8),
    W(u8),
    V(u8),
    Q(u8),
    D(u8),
    S(u8),
    H(u8),
    B(u8),
    Z(u8),
    P(u8),
    Sp,
    Wsp,
    Xzr,
    Wzr,
    Fp,
    Lr,
    Pc,
    Ffr,
}
