use std::sync::Arc;

use super::PE;
use crate::{
    core::{Address, Block, InstructionHistory, Relation, RelationType},
    prelude::BlockParsingError,
};

impl PE {
    /// ### Todo
    /// - jmp, je, jle외에도 모든 형태의 분기문에 대한 처리 필요
    /// - 점프한 주소가 범위를 벗어났을때 중단하는 처리 필요
    pub(super) fn _parse_block(
        &self,
        address: Address,
        history: &mut InstructionHistory,
    ) -> Result<Arc<Block>, BlockParsingError> {
        log::trace!("블럭 파싱 시작");
        log::trace!("블럭 시작 주소: {}", address.get_virtual_address());

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
                .ok()
                .ok_or(BlockParsingError::TriedToParseOutsideOfSection)?;
            let inst = &insts.get(0).ok_or(BlockParsingError::NoInstruction)?;
            history.data.push(inst.into());
            log::trace!("{}", inst);
            match inst.mnemonic().unwrap() {
                "call" => {
                    log::trace!("call 인스트럭션 발견");
                    let target = insn_to_opu64(now_address.clone(), &inst, history)?;
                    let target_address = Address::from_virtual_address(&self.sections, target);
                    connected_to = Some(Relation::new(
                        now_address.clone(),
                        target_address,
                        RelationType::Call,
                    ));
                    block_end = now_address;
                    break;
                }

                "jmp" => {
                    log::trace!("jmp 인스트럭션 발견");
                    let target = insn_to_opu64(now_address.clone(), &inst, history)?;
                    let target_address = Address::from_virtual_address(&self.sections, target);
                    connected_to = Some(Relation::new(
                        now_address.clone(),
                        target_address,
                        RelationType::Jump,
                    ));
                    block_end = now_address;
                    break;
                }

                "jnc" | "jnz" | "je" | "js" | "jnb" | "ja" | "jg" | "jnle" | "jpojs" | "jnae"
                | "jl" | "jna" | "jb" | "jne" | "jle" | "jrcxz" | "jns" | "jc" | "jo" | "jnge"
                | "jnbe" | "jecxz" | "jpo" | "jz" | "jae" | "jpe" | "jnl" | "jp" | "jge"
                | "jbe" | "jcxz" | "jno" | "jnp" | "jng" => {
                    log::trace!("jcc 인스트럭션 발견");
                    let target = insn_to_opu64(now_address.clone(), &inst, history)?;
                    let target_address = Address::from_virtual_address(&self.sections, target);
                    connected_to = Some(Relation::new(
                        now_address.clone(),
                        target_address,
                        RelationType::Jcc,
                    ));
                    block_end = now_address;
                    break;
                }

                "ret" => {
                    log::trace!("ret 인스트럭션 발견");
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

        Ok(block)
    }
}

/// 인스트럭션을 입력값으로, 여러 형태의 대상 주소를 파싱해 u64형태로 반환한다.
///
/// ### Arguments
/// - `now_address: Address` - 현재 진행 주소
/// - `inst: &capstone::Insn` - 파싱 대상 인스트럭션
/// - `history: &mut InstructionHistory` - 인스트럭션 히스토리
///
/// ### Results
/// - `Result<u64, &static str>` - 파싱에 성공할 경우 대상 주소를, 실패했을 경우(구현되지 않아 실패하는 등...) Err를 반환한다.
///
/// ### Todo
/// - dword ptr [eip + 0x??] 패던 외에도, eax나 다른 레지스터를 기반으로 점프하는 명령어에 대한 처리 필요
fn insn_to_opu64(
    now_address: Address,
    inst: &capstone::Insn,
    history: &mut InstructionHistory,
) -> Result<u64, &'static str> {
    let op = inst.op_str().unwrap();

    /* 대상 주소 파싱 */
    for (idx, pattern) in crate::arch::x86_64::op_patterns::PATTERNS
        .iter()
        .enumerate()
    {
        if let Some(captures) = pattern.captures(op) {
            return crate::arch::x86_64::op_parse::FUNCTIONS[idx](
                now_address,
                inst,
                history,
                captures,
            );
        }
    }
    panic!("패턴이 없는 인스트럭션을 파싱하려고 했습니다: {}", inst);
}
