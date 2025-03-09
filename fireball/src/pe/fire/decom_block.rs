use super::PE;
use crate::{
    core::Address,
    ir::{Ir, IrBlock},
    prelude::*,
};

impl PE {
    pub(super) fn _decom_block(&self, address: Address) -> Result<(), DecompileError> {
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

        /* 인스트럭션 변환 */
        let instructions = self.parse_assem_range(&address, block_size)?;
        let mut ir_block = Vec::new();
        let mut instruction_address = address;
        for instruction in instructions {
            // 인스트럭션으로 ir생성
            let statements =
                crate::arch::x86_64::instruction_analyze::create_ir_statement(&instruction);
            let ir = Ir {
                address: instruction_address.clone(),
                statements,
            };
            ir_block.push(ir);

            /* 후처리 */
            // 인스트럭션 주소 이동
            let instruction_size = instruction
                .inner
                .bytes
                .expect("어셈블리 파싱 결과값에는 바이트 값이 존재함")
                .len();
            instruction_address += instruction_size as u64;
        }
        let _ir_block = IrBlock::new(ir_block);

        /* 분석 */
        // 접근 메모리 영역 파악
        // 접근 사이즈 및 사용 인스트럭션에 따른 타입 지정
        // native api 호출 인자에 따른 타입 재 지정
        // 함수 코드 생성
        // 해당 함수 관련 분석 내용 저장 -> 블럭에 저장
        // 해당 블록에 접근하고 있는 블록 디컴파일 재시도 -> 재귀는 1번까지 허용

        todo!();
    }
}
