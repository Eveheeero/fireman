use super::PE;
use crate::{
    core::{Address, Fire, InstructionHistory, RelationType},
    prelude::{trace, DecompileError},
};

impl PE {
    pub(super) fn _decom_from_entry(&self) -> Result<(), DecompileError> {
        let gl = goblin::pe::PE::parse(&self.binary)?;

        // 프로그램의 엔트리포인트
        let entry = Address::from_virtual_address(&self.sections, gl.entry as u64);
        // 어떤 주소에서 다른 불록으로 이동했는지에 대한 스택
        let mut stack = Vec::new();
        // 인스트럭션 기록
        let mut history = InstructionHistory::default();

        let mut now = entry;
        loop {
            trace!("블록 파싱 시작");
            trace!("블럭의 깊이 : {}", stack.len());
            trace!("블록 파싱 시작 주소 : {:#x}", now.get_virtual_address());

            if self.blocks.find_from_start_address(now.clone()).is_some() {
                // 이미 파싱된 블록이라면, 다음 블록으로 넘어간다.
                match self.rewind_stack(&mut stack) {
                    Ok(address) => {
                        now = address;
                        continue;
                    }
                    Err(_) => break,
                };
            }

            // 블록 파싱
            let block = if let Ok(block) = self.parse_block(now, &mut history) {
                block
            } else {
                // 블록 파싱에 실패했을 경우, 해당 블록에 진입하기 이전의 주소에서 다시 시작한다.
                match self.rewind_stack(&mut stack) {
                    Ok(address) => {
                        now = address;
                        continue;
                    }
                    Err(_) => break,
                };
            };
            let connected_to = match block.get_connected_to().first() {
                Some(connected_to) => connected_to.clone(),
                None => {
                    match self.rewind_stack(&mut stack) {
                        Ok(address) => {
                            now = address;
                            continue;
                        }
                        Err(_) => break,
                    };
                }
            };

            // 다음에 시작할 주소와, 어떤 주소에서 해당 주소로 이동했는지를 저장한다.
            now = connected_to.to().clone();
            if connected_to.relation_type() != &RelationType::Jump {
                // 블록 연결 타입이 JMP가 아닐경우 스택에 저장한다.
                stack.push(connected_to.from().clone());
            }
        }

        Ok(())
    }

    /// 스택프레임을 하나 해제한다.
    fn rewind_stack(&self, stack: &mut Vec<Address>) -> Result<Address, ()> {
        let mut now = match stack.pop() {
            Some(address) => address,
            None => return Err(()),
        };

        // 시작하기 이전 주소의 다음라인을 가져온다.
        let insts = self
            .parse_assem_count(now.clone(), 1)
            .expect("어셈블리를 파싱하는데에 실패했습니다."); // 해당 라인에서 어셈블리 파싱은 반드시 실행해야 한다. 스택을 푼 후의, 다음 파싱할 주소가 파일 오프셋의 바깥일 리 없기 때문
        now += insts[0].len() as u64;
        Ok(now)
    }
}
