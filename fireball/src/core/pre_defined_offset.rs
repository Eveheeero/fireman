//! 프로그램에 내부에 미리 정의되어있는 '주소 별 이름'에 대한 정보를 담은 구조체인
//! "PreDefinedOffset"를 정의하는 모듈

use super::Address;

/// 파일 내부에, A 주소는 A'함수의 시작부분이다 등, 미리 지정되어있는 주소 정보가 저장되는 구조체
pub struct PreDefinedOffset {
    /// 파일에 대한 오프셋
    pub(crate) address: Address,
    /// 파일 내부에 지정되어있는 이름
    pub(crate) name: String,
}
