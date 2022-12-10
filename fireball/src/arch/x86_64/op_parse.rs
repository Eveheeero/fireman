use crate::{
    core::{Address, InstructionHistory},
    prelude::BlockParsingError,
};

use regex::Captures;

pub const FUNCTIONS: &[&dyn Fn(
    Address,
    &capstone::Insn,
    &mut InstructionHistory,
    Captures,
) -> Result<u64, BlockParsingError>] = &[&function0, &function1, &function2];

fn function0(
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

fn function1(
    now_address: Address,
    _inst: &capstone::Insn,
    _history: &mut InstructionHistory,
    captures: Captures,
) -> Result<u64, BlockParsingError> {
    log::trace!("다음과 같은 패턴으로 분기 대상 주소 파싱 시작 : ?word ptr [??? ? 0x????????]");
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

fn function2(
    _now_address: Address,
    inst: &capstone::Insn,
    history: &mut InstructionHistory,
    captures: Captures,
) -> Result<u64, BlockParsingError> {
    log::trace!("다음과 같은 패턴으로 분기 대상 주소 파싱 시작 : ???");
    log::debug!("정규식 매칭 결과 : {:?}", captures);

    let virtual_address = find_register_from_history(inst.op_str().unwrap(), history, 0)?;

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
    target: &str,
    history_o: &InstructionHistory,
    index: usize,
) -> Result<u64, BlockParsingError> {
    use super::op_patterns::OTHERS;
    let mut result = Err(BlockParsingError::Unknown);

    log::debug!("탐색 대상 레지스터 : {}", target);
    match () {
        () if OTHERS[1].is_match(target) => {
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
        () if OTHERS[5].is_match(target) => {
            /* 탐색 대상 레지스터가 ?sp를 기반으로 두면 */
            // Todo 추후 개발 필요
            log::info!("스택 레지스터를 기반으로 연산하는 로직은 현재 개발되어있지 않으며, 추후 개발할 예정입니다.");
            return Err(BlockParsingError::CantCalcRegister);
        }
        () if OTHERS[6].is_match(target) => {
            /* 탐색 대상 레지스터가 ?bp를 기반으로 두면 */
            // Todo 추후 개발 필요
            log::info!("스택 레지스터를 기반으로 연산하는 로직은 현재 개발되어있지 않으며, 추후 개발할 예정입니다.");
            return Err(BlockParsingError::CantCalcRegister);
        }
        _ => {}
    }
    for (index, history) in history_o.data.iter().rev().enumerate().skip(index + 1) {
        log::debug!("{} {} : {}", history.mnemonic, history.op, history.len);

        if history.op.contains(target) {
            // 탐색 대상에 대한 연산이 들어갔으면
            match history.mnemonic.as_str() {
                "mov" => {
                    log::trace!("mov 대상 패턴 파싱 시작");

                    log::debug!("패턴 매칭 : ???, ?word ptr [??? ? 0x????????]");
                    if let Some(captures) = OTHERS[2].captures(&history.op) {
                        if captures["to"].to_string() == target {
                            /* 타겟 레지스터에 영향을 주는 경우 */
                            let base =
                                find_register_from_history(&captures["base"], history_o, index)?;
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
                    if let Some(captures) = OTHERS[3].captures(&history.op) {
                        if captures["to"].to_string() == target {
                            /* 타겟 레지스터에 영향을 주는 경우 */
                            let base =
                                find_register_from_history(&captures["base"], history_o, index)?;
                            let other =
                                find_register_from_history(&captures["other"], history_o, index)?;
                            let mul = u64::from_str_radix(&captures["mul"], 10).unwrap();

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
                    if OTHERS[4].is_match(&history.op) {
                        // 해당 패턴은 타겟 레지스터에 영향을 줄 수 없다.
                        continue;
                    }

                    log::debug!("패턴 매칭 : ???, ?word ptr [??? ? 4]");
                    if let Some(captures) = OTHERS[7].captures(&history.op) {
                        if captures["to"].to_string() == target {
                            /* 타겟 레지스터에 영향을 주는 경우 */
                            let base =
                                find_register_from_history(&captures["base"], history_o, index)?;
                            let other = u64::from(find_register_from_history(
                                &captures["other"],
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

                    log::warn!("mov 대상 OP 파싱 실패 : {}", history.op);
                    unimplemented!()
                }
                "lea" => {
                    log::trace!("lea 대상 패턴 파싱 시작");

                    log::trace!("패턴 매칭 : ???, [??? ? 0x????????]");
                    if let Some(captures) = OTHERS[0].captures(&history.op) {
                        if captures["to"].to_string() == target {
                            /* 타겟 레지스터에 영향을 주는 경우 */
                            let base =
                                find_register_from_history(&captures["base"], history_o, index)?;
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
                "add" => todo!(),
                "sub" => todo!(),
                _ => continue,
            }
        }
    }

    log::debug!("파싱된 값 : {:#?}", result);
    result
}
