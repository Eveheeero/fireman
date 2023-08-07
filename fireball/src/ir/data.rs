use bitvec::vec::BitVec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IRData {
    /// mov eax, 0x1234의 0x1234
    Constant(BitVec<u8>),
    // mov eax, ebx의 ebx
    Register(crate::ir::Register),
    /// mov eax, dword ptr [eax]의 dword ptr [eax]
    Dereference(Box<IRData>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessType {
    Read,
    Write,
}
