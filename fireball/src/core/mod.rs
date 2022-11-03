#![allow(unused_imports)]
#![allow(dead_code)]

/// 파서 모듈에 대한 트레이트가 들어있는 모듈
mod fire;
pub use fire::Fire;

/// 파일 내부에 미리 지정되어있는 데이터에 대한 구조체를 저장하는 구조체가 담긴 모듈
mod pre_defined_offsets;
pub use pre_defined_offsets::PreDefinedOffsets;

/// 파일 내부에 지정되어있는 데이터에 대한 구조체가 들어있는 모듈
mod pre_defined_offset;
pub use pre_defined_offset::PreDefinedOffset;

/// 주소에 대한 구조체가 들어있는 모듈
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

mod blocks;
pub use blocks::Blocks;

/// 파싱하는 코드 블럭이 들어있는 모듈
mod block;
pub use block::Block;

/// 코드 블럭에 대한 연관 블럭이 들어있는 모듈
mod relations;
pub use relations::Relations;

/// 코드 블럭에 대한 연관 블럭이 들어있는 모듈
mod relation;
pub use relation::Relation;
