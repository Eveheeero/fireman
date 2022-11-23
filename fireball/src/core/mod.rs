#![allow(unused_imports)]
#![allow(dead_code)]

/// 디컴파일 공통 기능을 담당하는 "Fire"구조체에 대해 정의하는 모듈
mod fire;
pub use fire::Fire;

/// 프로그램에 대한 "PreDefinedOffset"를 모아두는 구조체를 정의하는 모듈
mod pre_defined_offsets;
pub use pre_defined_offsets::PreDefinedOffsets;

/// 프로그램에 내부에 미리 정의되어있는 '주소 별 이름'에 대한 정보를 담은 구조체인
/// "PreDefinedOffset"를 정의하는 모듈
mod pre_defined_offset;
pub use pre_defined_offset::PreDefinedOffset;

/// 프로그램 내부의 가상주소 혹은 파일 오프셋에 대한 정보를 담고있는 구조체인
/// "Address"를 정의하는 모듈
///
/// # Note
/// - 주소에 대한 정보는 저장해 둘 필요가 없기 때문에 Addresses는 존재하지 않습니다.
mod address;
pub use address::Address;

/// Module that contains structure that contains all section information
///
///
/// 프로그램의 모든 섹션 정보를 가지고 있는 구조체를 정의하는 모듈
mod sections;
pub use sections::Sections;

/// Module that contains structure that contains section's information
///
///
/// 섹션에 대한 정보를 담은 구조체가 들어있는 모듈
mod section;
pub use section::Section;

/// 프로그램을 분석한 결과로 나온 "Block"를 모아두는 구조체를 정의하는 모듈
mod blocks;
pub use blocks::Blocks;

/// 프로그램을 분석하여 각 섹션별로 나온 "Block"을 정의하는 모듈
mod block;
pub use block::Block;

/// "Relation"을 모아두는 구조체를 정의하는 모듈
mod relations;
pub use relations::Relations;

/// 분석중 나온 분기에 대한 연관관계를 정의하는 모듈
mod relation;
pub use relation::Relation;
