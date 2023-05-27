//! 컴퓨터의 동작을 추상화한 IR을 정의합니다.

use crate::prelude::BitBox;

/// 컴퓨터가 동작하는 행동을 재현하기 위한 구조체
///
/// ### Todo
///
/// - 레지스터 데이터 외에도, 메모리 변환, 파일 등을 다뤄야 합니다.
pub struct Ir {
    /// 0~64비트의 값은 rax, 64~128비트의 값은 rbx 를 가지고 있는 등으로, CPU의 데이터를 가지고 있습니다.
    register: BitBox,
}

/// IR 데이터의 기본적인 행동 인터페이스 (파일 변환 등..., 하지만 구현되지 않았습니다.)
impl Ir {}

/// IR의 레지스터 데이터를 원본 형태로 가져올 수 있는 인터페이스입니다.
pub trait IRRaw {
    /// 가공되지 앟은 레지스터 데이터를 가져옵니다.
    fn get_register(&self) -> &BitBox;
}

pub mod arm;
pub mod x86_64;
