/// 파서 모듈에 대한 트레이트가 들어있는 모듈
mod fire;
pub use fire::Fire;

/// 파일 내부에 지정되어있는 데이터에 대한 구조체가 들어있는 모듈
mod pre_defined_offset;
pub(crate) use pre_defined_offset::PreDefinedOffset;

/// 주소에 대한 구조체가 들어있는 모듈
mod address;
pub(crate) use address::Address;
