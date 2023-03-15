use crate::core::{Address, InstructionHistory};

use regex::Captures;

pub const FUNCTIONS: &[&dyn Fn(
    Address,
    &capstone::Insn,
    &mut InstructionHistory,
    Captures,
) -> Result<u64, &'static str>] = &[&function0, &function1, &function2];

fn function0(
    _now_address: Address,
    _inst: &capstone::Insn,
    _history: &mut InstructionHistory,
    captures: Captures,
) -> Result<u64, &'static str> {
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
) -> Result<u64, &'static str> {
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
        _ => return Err("Invalid operator"),
    };
    log::debug!("파싱된 분기 대상 주소 : 0x{:x}", virtual_address);
    Ok(virtual_address)
}

fn function2(
    now_address: Address,
    inst: &capstone::Insn,
    history: &mut InstructionHistory,
    captures: Captures,
) -> Result<u64, &'static str> {
    log::trace!("다음과 같은 패턴으로 분기 대상 주소 파싱 시작 : ???");
    log::debug!("정규식 매칭 결과 : {:?}", captures);
    use super::op_patterns::OTHERS;

    let target_register = inst.op_str().unwrap();
    log::debug!("탐색 대상 레지스터 : {}", target_register);
    for history in history.data.iter().rev().skip(1) {
        log::debug!(
            "이전 명령어 : op - {}, mnemonic - {}, len - {}",
            history.op,
            history.mnemonic,
            history.len
        );
        if history.op.contains(target_register) {
            // OP (eax)등에 대한 연산이 들어갔으면
            match history.mnemonic.as_str() {
                "mov" => {
                    log::trace!("mov 대상 패턴 파싱 시작 : ???, ?word ptr [??? ? 0x????????]");
                    let captures = OTHERS[2].captures(&history.op).unwrap();
                    if captures["to"].to_string() == target_register {
                        // mov eax, 0x1234 등의 형태인 경우
                        if OTHERS[1].is_match(&captures["base"]) {
                            let address_at_there =
                                u64::from_str_radix(&captures["relative_address"], 16).unwrap();
                            log::debug!("파싱된 분기 대상 주소 : 0x{:x}", address_at_there);
                            return Ok(address_at_there);
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                "lea" => {
                    log::trace!("lea 대상 패턴 파싱 시작 : ???, [??? ? 0x????????]");
                    let captures = OTHERS[0].captures(&history.op).unwrap();
                    if captures["to"].to_string() == target_register {
                        // lea eax, [rip + 0x1234] 등의 형태인 경우
                        let address_at_there = match &captures["operator"] {
                            "+" if OTHERS[1].is_match(&captures["base"]) => {
                                now_address.get_virtual_address() - history.address
                                    + u64::from_str_radix(&captures["relative_address"], 16)
                                        .unwrap()
                            }
                            "-" if OTHERS[1].is_match(&captures["base"]) => {
                                now_address.get_virtual_address()
                                    - history.address
                                    - u64::from_str_radix(&captures["relative_address"], 16)
                                        .unwrap()
                            }
                            _ => return Err("Invalid operator"),
                        };
                        log::debug!("파싱된 분기 대상 주소 : 0x{:x}", address_at_there);
                        return Ok(address_at_there);
                    } else {
                        continue;
                    }
                }
                "add" => todo!(),
                "sub" => todo!(),
                _ => continue,
            }
        }
    }
    Err("Not Found")
}
