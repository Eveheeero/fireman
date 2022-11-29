use std::sync::Arc;

use super::PE;
use crate::core::{Address, Block, Relation, RelationType};

impl PE {
    /// ### Todo
    /// - jmp, je, jle외에도 모든 형태의 분기문에 대한 처리 필요
    /// - 점프한 주소가 범위를 벗어났을때 중단하는 처리 필요
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
            println!("{}", inst);
            match inst.mnemonic().unwrap() {
                "call" => {
                    let target = insn_to_opu64(now_address.clone(), &inst);
                    let target_address =
                        Address::from_virtual_address(&self.sections, target).unwrap();
                    connected_to = Some(Relation::new(
                        now_address.clone(),
                        target_address,
                        RelationType::Call,
                    ));
                    block_end = now_address;
                    break;
                }

                "jmp" | "jnc" | "jnz" | "je" | "js" | "jnb" | "ja" | "jg" | "jnle" | "jpojs"
                | "jnae" | "jl" | "jna" | "jb" | "jne" | "jle" | "jrcxz" | "jns" | "jc" | "jo"
                | "jnge" | "jnbe" | "jecxz" | "jpo" | "jz" | "jae" | "jpe" | "jnl" | "jp"
                | "jge" | "jbe" | "jcxz" | "jno" | "jnp" | "jng" => {
                    let target = insn_to_opu64(now_address.clone(), &inst);
                    let target_address =
                        Address::from_virtual_address(&self.sections, target).unwrap();
                    connected_to = Some(Relation::new(
                        now_address.clone(),
                        target_address,
                        RelationType::Jump,
                    ));
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

/// 인스트럭션을 입력값으로, 여러 형태의 대상 주소를 파싱해 u64형태로 반환한다.
///
/// ### Todo
/// - dword ptr [eip + 0x??] 패던 외에도, eax나 다른 레지스터를 기반으로 점프하는 명령어에 대한 처리 필요
fn insn_to_opu64(now_address: Address, inst: &capstone::Insn) -> u64 {
    let op = inst.op_str().unwrap();
    if op.starts_with("0x") {
        // 형태가 0x1234인 경우
        u64::from_str_radix(op.trim_start_matches("0x"), 16).unwrap()
    } else if op.starts_with("dword ptr [") {
        // 형태가 qword ptr [eip + 0x1234]인 경우
        if op.contains("eip") {
            now_address.get_virtual_address()
                + u64::from_str_radix(
                    op.trim_start_matches("dword ptr [eip + 0x")
                        .trim_end_matches("]"),
                    16,
                )
                .unwrap()
        } else {
            todo!("eip외의 레지스터를 연산한 후, 해당 주소값을 구하는 방법 고안 필요")
        }
    } else if op.starts_with("qword ptr [") {
        // 형태가 qword ptr [rip + 0x1234]인 경우
        if op.contains("rip") {
            now_address.get_virtual_address()
                + u64::from_str_radix(
                    op.trim_start_matches("qword ptr [rip + 0x")
                        .trim_end_matches("]"),
                    16,
                )
                .unwrap()
        } else {
            todo!("rip외의 레지스터를 연산한 후, 해당 주소값을 구하는 방법 고안 필요")
        }
    } else {
        unimplemented!()
    }
    // 16비트 전용 jmp문?
}
