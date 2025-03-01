use super::PE;
use crate::{core::Address, prelude::*};

impl PE {
    pub(super) fn _decom_function(&self, address: Address) -> Result<(), DecompileError> {
        // 블럭 생성
        let block = self.generate_block_from_address(&address);
        // 해당 블럭의 인스트럭션 파싱
        let end_address = block.get_end_address();
        let block_size = if let Some(end_address) = end_address {
            end_address - &address
        } else {
            warn!(?address, "디컴파일 대상의 종료 위치를 찾을 수 없음");
            1
        };
        let _instructions = self.parse_assem_range(&address, block_size);
        // 인스트럭션으로 ir생성
        // let ir = arch::x86_64::create_ir_stateme();
        // 접근 메모리 영역 파악
        // 접근 사이즈 및 사용 인스트럭션에 따른 타입 지정
        // native api 호출 인자에 따른 타입 재 지정
        // 함수 코드 생성
        // 해당 함수 관련 분석 내용 저장
        // 해당 블록에 접근하고 있는 블록 디컴파일 재시도

        todo!();
    }
}
