#![allow(unused_imports)]
#![allow(dead_code)]

/// 파서 모듈에 대한 트레이트가 들어있는 모듈
mod fire;
pub use fire::Fire;

/// 파일 내부에 지정되어있는 데이터에 대한 구조체가 들어있는 모듈
mod pre_defined_offset;
pub(crate) use pre_defined_offset::PreDefinedOffset;

/// 주소에 대한 구조체가 들어있는 모듈
mod address;
pub(crate) use address::Address;

/// 섹션에 대한 구조체가 들어있는 모듈
///
/// ## TODO
/// 현재는 섹션 정보를 전역변수로 지정하도록 설정되어 있습니다.
/// 한 프로그램은 하나의 파일만을 로드해 분석한다는 전제 하에 전역 변수로 지정을 하였지만,
/// PE 구조체의 디컴파일 기능이 완성된 후, Sections라는 섹션 정보 모음 구조체가 생성되어
/// PE 구조체의 내부에 해당 정보가 들어가야 합니다.
mod section;
pub(crate) use section::Section;

/// 파싱하는 코드 블럭이 들어있는 모듈
mod block;
pub(crate) use block::Block;

/// 코드 블럭에 대한 연관 블럭이 들어있는 모듈
mod relation;
pub(crate) use relation::Relation;
