use crate::{
    core::{Address, InstructionHistory},
    prelude::BlockParsingError,
};

use regex::Captures;

/// jcc나 call등의 명령에 대한 주소 패턴을 파싱한다.
/// 해당 함수들은 op_patterns의 JMP_TARGET_INST_PATTERNS와 같이 사용된다.
pub const JMP_TARGET_INST_PARSE: &[&dyn Fn(
    Address,
    &capstone::Insn,
    &mut InstructionHistory,
    Captures,
) -> Result<u64, BlockParsingError>] = &[&jmp_inst_parse_0, &jmp_inst_parse_1, &jmp_inst_parse_2];

/// jmp 0xabcdef 패턴에 대한 점프 주소 파싱
fn jmp_inst_parse_0(
    _now_address: Address,
    _inst: &capstone::Insn,
    _history: &mut InstructionHistory,
    captures: Captures,
) -> Result<u64, BlockParsingError> {
    log::trace!("다음과 같은 패턴으로 분기 대상 주소 파싱 시작 : 0x????????");
    log::debug!("정규식 매칭 결과 : {:?}", captures);

    let virtual_address = u64::from_str_radix(&captures["address"], 16).unwrap();

    log::debug!("파싱된 분기 대상 주소 : 0x{:x}", virtual_address);
    Ok(virtual_address)
}

/// jmp dword ptr [eip + 0xabcdef] 패턴에 대한 점프 주소 파싱
fn jmp_inst_parse_1(
    now_address: Address,
    _inst: &capstone::Insn,
    _history: &mut InstructionHistory,
    captures: Captures,
) -> Result<u64, BlockParsingError> {
    log::trace!("다음과 같은 패턴으로 분기 대상 주소 파싱 시작 : ?word ptr [?ip ? 0x????????]");
    log::debug!("정규식 매칭 결과 : {:?}", captures);

    let virtual_address;
    match &captures["operator"] {
        "+" => {
            virtual_address = now_address.get_virtual_address()
                + u64::from_str_radix(&captures["relative_address"], 16).unwrap()
        }
        "-" => {
            virtual_address = now_address.get_virtual_address()
                - u64::from_str_radix(&captures["relative_address"], 16).unwrap()
        }
        _ => unreachable!("Invalid operator"),
    };

    log::debug!("파싱된 분기 대상 주소 : 0x{:x}", virtual_address);
    Ok(virtual_address)
}

/// jmp eax 패턴에 대한 점프 주소 파싱
fn jmp_inst_parse_2(
    _now_address: Address,
    inst: &capstone::Insn,
    history: &mut InstructionHistory,
    captures: Captures,
) -> Result<u64, BlockParsingError> {
    log::trace!("다음과 같은 패턴으로 분기 대상 주소 파싱 시작 : ???");
    log::debug!("정규식 매칭 결과 : {:?}", captures);

    let virtual_address = find_register_from_history(inst.op_str().unwrap().into(), history, 0)?;

    log::debug!("파싱된 분기 대상 주소 : 0x{:x}", virtual_address);
    Ok(virtual_address)
}

/// 인스트럭션의 기록을 기반으로 특정 레지스터의 값을 추측합니다.
///
/// ### Arguments
/// - `target: &str` - 탐색 대상 레지스터
/// - `history_o: &InstructionHistory` - 인스트럭션 기록
/// - `index: usize` - index번째 + 1의 명령어부터 탐색을 시작합니다.
///
/// ### Returns
/// `Result<u64, BlockParsingError>` - 탐색 성공 시 레지스터의 값(모든것을 계산한 상대주소), 실패 시 에러
fn find_register_from_history(
    target: super::register::Register,
    history_o: &InstructionHistory,
    index: usize,
) -> Result<u64, BlockParsingError> {
    use super::op_patterns::NOT_JMP_TARGET_INST_PATTERNS;
    use super::register::*;

    let mut result = Err(BlockParsingError::Unknown);

    log::debug!("탐색 대상 레지스터 : {}", target);
    match target {
        Register::Ip(_) => {
            /* 탐색 대상 레지스터가 ip이면 */
            log::debug!("탐색 대상 레지스터가 ip이므로, 이전 명령어의 주소를 반환");
            return Ok(history_o
                .data
                .iter()
                .rev()
                .skip(index)
                .next()
                .unwrap()
                .address);
        }
        Register::Sp(_) => {
            /* 탐색 대상 레지스터가 ?sp를 기반으로 두면 */
            // Todo 추후 개발 필요
            log::info!("스택 레지스터를 기반으로 연산하는 로직은 현재 개발되어있지 않으며, 추후 개발할 예정입니다.");
            return Err(BlockParsingError::CantCalcRegister);
        }
        Register::Bp(_) => {
            /* 탐색 대상 레지스터가 ?bp를 기반으로 두면 */
            // Todo 추후 개발 필요
            log::info!("스택 레지스터를 기반으로 연산하는 로직은 현재 개발되어있지 않으며, 추후 개발할 예정입니다.");
            return Err(BlockParsingError::CantCalcRegister);
        }
        _ => {}
    }
    for (index, history) in history_o.data.iter().rev().enumerate().skip(index + 1) {
        log::debug!("{} {} : {}", history.mnemonic, history.op, history.len);

        if history.op.contains(target.str()) {
            // 탐색 대상에 대한 연산이 들어갔으면
            match history.mnemonic.as_str() {
                "mov" => {
                    log::trace!("mov 대상 패턴 파싱 시작");

                    log::debug!("패턴 매칭 : ???, ?word ptr [??? ? 0x????????]");
                    if let Some(captures) = NOT_JMP_TARGET_INST_PATTERNS[1].captures(&history.op) {
                        if &captures["to"] == target.str() {
                            /* 타겟 레지스터에 영향을 주는 경우 */
                            let base = find_register_from_history(
                                captures["base"].into(),
                                history_o,
                                index,
                            )?;
                            let relative_address =
                                u64::from_str_radix(&captures["relative_address"], 16).unwrap();

                            match &captures["operator"] {
                                "+" => result = Ok(base + relative_address),
                                "-" => result = Ok(base - relative_address),
                                _ => unreachable!("Invalid operator"),
                            }
                            break;
                        } else {
                            /* 타겟 레지스터에 영향을 주지 않는 경우 */
                            continue;
                        }
                    }

                    log::debug!("패턴 매칭 : ???, ?word ptr [??? ? ???*?]");
                    if let Some(captures) = NOT_JMP_TARGET_INST_PATTERNS[2].captures(&history.op) {
                        if &captures["to"] == target.str() {
                            /* 타겟 레지스터에 영향을 주는 경우 */
                            let base = find_register_from_history(
                                captures["base"].into(),
                                history_o,
                                index,
                            )?;
                            let other = find_register_from_history(
                                captures["other"].into(),
                                history_o,
                                index,
                            )?;
                            let mul = u64::from_str_radix(captures["mul"].into(), 10).unwrap();

                            match &captures["operator"] {
                                "+" => result = Ok(base + other * mul),
                                "-" => result = Ok(base - other * mul),
                                _ => unreachable!("Invalid operator"),
                            }
                            break;
                        } else {
                            /* 타겟 레지스터에 영향을 주지 않는 경우 */
                            continue;
                        }
                    }

                    log::debug!("패턴 매칭 : ?word ptr [???], ???");
                    if NOT_JMP_TARGET_INST_PATTERNS[3].is_match(&history.op) {
                        // 해당 패턴은 타겟 레지스터에 영향을 줄 수 없다.
                        continue;
                    }

                    log::debug!("패턴 매칭 : ???, ?word ptr [??? ? 4]");
                    if let Some(captures) = NOT_JMP_TARGET_INST_PATTERNS[4].captures(&history.op) {
                        if &captures["to"] == target.str() {
                            /* 타겟 레지스터에 영향을 주는 경우 */
                            let base = find_register_from_history(
                                captures["base"].into(),
                                history_o,
                                index,
                            )?;
                            let other = u64::from(find_register_from_history(
                                captures["other"].into(),
                                history_o,
                                index,
                            )?);

                            match &captures["operator"] {
                                "+" => result = Ok(base + other),
                                "-" => result = Ok(base - other),
                                _ => unreachable!("Invalid operator"),
                            }
                            break;
                        } else {
                            /* 타겟 레지스터에 영향을 주지 않는 경우 */
                            continue;
                        }
                    }

                    log::debug!("패턴 매칭 : ???, ?word ptr [??? ? ???]");
                    if let Some(captures) = NOT_JMP_TARGET_INST_PATTERNS[7].captures(&history.op) {
                        if &captures["to"] == target.str() {
                            /* 타겟 레지스터에 영향을 주는 경우 */
                            let base = find_register_from_history(
                                captures["base"].into(),
                                history_o,
                                index,
                            )?;
                            let other = find_register_from_history(
                                captures["other"].into(),
                                history_o,
                                index,
                            )?;

                            match &captures["operator"] {
                                "+" => result = Ok(base + other),
                                "-" => result = Ok(base - other),
                                _ => unreachable!("Invalid operator"),
                            }
                            break;
                        } else {
                            /* 타겟 레지스터에 영향을 주지 않는 경우 */
                            continue;
                        }
                    }

                    log::debug!("패턴 매칭 : ???, ?word ptr [???]");
                    if let Some(captures) = NOT_JMP_TARGET_INST_PATTERNS[8].captures(&history.op) {
                        if &captures["to"] == target.str() {
                            /* 타겟 레지스터에 영향을 주는 경우 */
                            let base = find_register_from_history(
                                captures["base"].into(),
                                history_o,
                                index,
                            )?;

                            result = Ok(base);
                            break;
                        } else {
                            /* 타겟 레지스터에 영향을 주지 않는 경우 */
                            continue;
                        }
                    }

                    log::warn!("mov 대상 OP 파싱 실패 : {}", history.op);
                    unimplemented!()
                }
                "lea" => {
                    log::trace!("lea 대상 패턴 파싱 시작");

                    log::trace!("패턴 매칭 : ???, [??? ? 0x????????]");
                    if let Some(captures) = NOT_JMP_TARGET_INST_PATTERNS[0].captures(&history.op) {
                        if &captures["to"] == target.str() {
                            /* 타겟 레지스터에 영향을 주는 경우 */
                            let base = find_register_from_history(
                                captures["base"].into(),
                                history_o,
                                index,
                            )?;
                            let relative_address =
                                u64::from_str_radix(&captures["relative_address"], 16).unwrap();

                            match &captures["operator"] {
                                "+" => result = Ok(base + relative_address),
                                "-" => result = Ok(base - relative_address),
                                _ => unreachable!("Invalid operator"),
                            }
                            break;
                        } else {
                            /* 타겟 레지스터에 영향을 주지 않는 경우 */
                            continue;
                        }
                    }

                    log::warn!("lea 대상 OP 파싱 실패 : {}", history.op);
                    unimplemented!()
                }
                "add" => {
                    log::trace!("add 대상 패턴 파싱 시작");

                    log::debug!("패턴 매칭 :???, 1234");
                    if let Some(captures) = NOT_JMP_TARGET_INST_PATTERNS[5].captures(&history.op) {
                        if &captures["to"] == target.str() {
                            /* 타겟 레지스터에 영향을 주는 경우 */
                            let to = find_register_from_history(
                                captures["to"].into(),
                                history_o,
                                index,
                            )?;
                            let other = u64::from_str_radix(&captures["other"], 10).unwrap();

                            result = Ok(to + other);
                            break;
                        }
                    }

                    log::debug!("패턴 매칭 :???, ???");
                    if let Some(captures) = NOT_JMP_TARGET_INST_PATTERNS[6].captures(&history.op) {
                        if &captures["to"] == target.str() {
                            /* 타겟 레지스터에 영향을 주는 경우 */
                            let to = find_register_from_history(
                                captures["to"].into(),
                                history_o,
                                index,
                            )?;
                            let other = find_register_from_history(
                                captures["other"].into(),
                                history_o,
                                index,
                            )?;

                            result = Ok(to + other);
                            break;
                        }
                    }

                    log::warn!("add 대상 OP 파싱 실패 : {}", history.op);
                    unimplemented!()
                }
                "sub" => todo!(),
                _ => continue,
            }
        }
    }

    log::debug!("파싱된 값 : {:#?}", result);
    result
}
