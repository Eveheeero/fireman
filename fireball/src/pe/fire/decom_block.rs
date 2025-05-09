use super::PE;
use crate::{
    core::Address,
    ir::{Ir, IrBlock},
    prelude::*,
};

impl PE {
    pub(super) fn _decom_block(&self, address: &Address) -> Result<(), DecompileError> {
        debug!(?address, "블럭 디컴파일 시작");

        // 블럭 생성
        let block = self.generate_block_from_address(address);
        // 해당 블럭의 인스트럭션 파싱
        let end_address = block.get_end_address();
        let block_size = if let Some(end_address) = end_address {
            end_address - address
        } else {
            warn!(?address, "디컴파일 대상의 종료 위치를 찾을 수 없음");
            1
        };

        /* 인스트럭션 변환 */
        let instructions = self.parse_assem_range(address, block_size)?;
        let mut ir_block = Vec::new();
        let mut instruction_address = address.clone();
        for instruction in instructions {
            let instruction_size = instruction
                .inner
                .bytes
                .as_ref()
                .expect("어셈블리 파싱 결과값에는 바이트 값이 존재함")
                .len();

            /* ir생성 */
            let statements =
                crate::arch::x86_64::instruction_analyze::create_ir_statement(&instruction);
            if statements.is_none() {
                warn!(?address, "인스트럭션 변환 실패");
            };
            let ir = Ir {
                address: instruction_address.clone(),
                instruction: instruction.into(),
                statements,
            };
            ir_block.push(ir);

            /* 후처리 */
            // 인스트럭션 주소 이동
            instruction_address += instruction_size as u64;
        }
        debug!(
            "블럭 내부 인스트럭션 IR 변환 완료, 총 {}개",
            ir_block
                .iter()
                .filter(|x| x.statements.is_some())
                .map(|x| x.statements.as_ref().unwrap().len())
                .sum::<usize>()
        );
        let mut ir_block = IrBlock::new(ir_block);

        /* 분석 */
        // 데이터 엑세스 분석
        ir_block.analyze_data_access();
        // 접근 메모리 영역 파악 및 사용 인스트럭션에 따른 타입 지정
        ir_block.analyze_datatypes();
        // 블럭 내부 변수 설정
        ir_block.analyze_variables().unwrap();
        // native api 호출 인자에 따른 타입 재 지정
        // TODO
        // 해당 블럭 내부 사용 인자 파악
        // TODO 사용되는 인자가 많을 경우 다른 함수의 내부인것으로 판단
        // 함수 코드 생성
        // TODO 이후 생성된 코드 ir_block에 저장
        // 분석 결과 확인
        let validate_result = ir_block.validate();
        if let Err(e) = validate_result {
            error!(?e, "블럭 분석 결과 오류");
        }
        // 분석 내용 블럭에 저장
        block.set_ir(ir_block);
        // 해당 블록에 접근하고 있는 블록 디컴파일 재시도 -> 재귀는 1번까지 허용
        // TODO + 이미 분석된 블럭을 재분석하려고 시도할 시 일부만 분석되도록? 아니면 초기화?

        Ok(())
    }
}
