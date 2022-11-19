use std::sync::Arc;

use super::PE;
use crate::core::{Address, Block, Relation};

impl PE {
    pub(super) fn _parse_block(&self, address: Address) -> Arc<Block> {
        /* 기본정보 파싱 및 변수 선언 */
        // 블록이 들어갈 섹션
        let section = self
            .sections
            .from_virtual_address(address.get_virtual_address())
            .unwrap();
        // 블록의 시작주소
        let block_start = address.clone();
        // 블록의 끝 주소
        let block_end: Address;
        // 블록이 가지고 있는, 다른 블록과의 관계
        let connected_to: Option<Arc<Relation>>;

        /* 한 줄씩 인스트럭션을 불러오면서, 다른 구역으로 이동하는 명령이 있는지 확인 */
        let mut now_address = address;
        loop {
            let insts = self
                .parse_assem_count(now_address.clone(), 1)
                .expect("Disassemble Error!");
            let inst = &insts[0];
            match inst.mnemonic().unwrap() {
                "call" => {
                    let target = inst.op_str().unwrap();
                    let target_address = Address::from_virtual_address(
                        &self.sections,
                        target.parse::<u64>().unwrap(),
                    )
                    .unwrap();
                    connected_to = Some(Relation::new(now_address.clone(), target_address));
                    block_end = now_address;
                    break;
                }
                "jmp" => {
                    let target = inst.op_str().unwrap();
                    let target_address = Address::from_virtual_address(
                        &self.sections,
                        target.parse::<u64>().unwrap(),
                    )
                    .unwrap();
                    connected_to = Some(Relation::new(now_address.clone(), target_address));
                    block_end = now_address;
                    break;
                }
                "ret" => {
                    connected_to = None;
                    block_end = now_address;
                    break;
                }
                _ => {
                    now_address = now_address + inst.len() as u64;
                }
            }
        }

        /* 블록 생성 및 반환 */
        // 블록 객체 생성
        let block = self
            .blocks
            .generate_block(section, block_start, Some(block_end), None);
        // 블록과 연결된 좌표 등록
        if let Some(connected_to) = connected_to {
            block.add_connected_to(connected_to.clone());
        }

        return block;
    }
}
