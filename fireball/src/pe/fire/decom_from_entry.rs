use super::PE;
use crate::{
    core::{Address, Fire},
    prelude::DecompileError,
};

impl PE {
    pub(super) fn _decom_from_entry(&self) -> Result<(), DecompileError> {
        let gl = goblin::pe::PE::parse(&self.binary)?;

        // 프로그램의 엔트리포인트
        let entry = Address::from_virtual_address(&self.sections, gl.entry as u64).unwrap();
        // 어떤 주소에서 다른 불록으로 이동했는지에 대한 스택
        let mut stack = Vec::new();

        let mut now = entry;
        loop {
            // 블록 파싱
            let block = if let Ok(block) = self.parse_block(now) {
                block
            } else {
                // 블록 파싱에 실패했을 경우, 해당 블록에 진입하기 이전의 주소에서 다시 시작한다.
                now = match stack.pop() {
                    Some(address) => address,
                    None => break,
                };
                // 시작하기 이전 주소의 다음라인을 가져온다.
                let insts = self.parse_assem_count(now.clone(), 1).unwrap();
                now += insts[0].len() as u64;
                continue;
            };
            let connected_to = match block.get_connected_to().first() {
                Some(connected_to) => connected_to.clone(),
                None => break,
            };

            // 다음에 시작할 주소와, 어떤 주소에서 해당 주소로 이동했는지를 저장한다.
            now = connected_to.to().clone();
            stack.push(connected_to.from().clone());
        }

        Ok(())
    }
}
