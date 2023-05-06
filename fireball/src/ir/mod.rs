use crate::prelude::{BitBox, BitSlice};

pub struct Ir {
    register: BitBox,
}

impl Ir {}

pub trait IRRaw {
    fn get_register(&self) -> &BitBox;
}

pub trait X64 {
    fn new() -> Self;
    fn eax(&self) -> &BitSlice;
}
pub trait ARM {
    fn new() -> Self;
}

impl X64 for Ir {
    fn new() -> Self {
        let mut register = bitvec::prelude::BitVec::new();
        // TODO X64컴퓨터에 맞는 모든 레지스터 사이즈를 구해 넣어야 한다.
        register.resize(100, false);
        Self {
            register: register.into_boxed_bitslice(),
        }
    }

    fn eax(&self) -> &BitSlice {
        // TODO X64컴퓨터의 eax레지스터는 ~부터 ~까지의 공간을 차지한다 로 구현해야 한다.
        &self.register[0..16]
    }
}
