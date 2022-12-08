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
    let virtual_address = u64::from_str_radix(&captures["address"], 16).unwrap();
    Ok(virtual_address)
}

fn function1(
    now_address: Address,
    _inst: &capstone::Insn,
    _history: &mut InstructionHistory,
    captures: Captures,
) -> Result<u64, &'static str> {
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
    Ok(virtual_address)
}

fn function2(
    now_address: Address,
    inst: &capstone::Insn,
    history: &mut InstructionHistory,
    _captures: Captures,
) -> Result<u64, &'static str> {
    use super::op_patterns::OTHERS;

    let target_register = inst.op_str().unwrap();
    for history in history.data.iter().rev().skip(1) {
        if history.op.contains(target_register) {
            // OP (eax)등에 대한 연산이 들어갔으면
            match history.mnemonic.as_str() {
                "mov" => {
                    let captures = OTHERS[2].captures(&history.op).unwrap();
                    if captures["to"].to_string() == target_register {
                        // mov eax, 0x1234 등의 형태인 경우
                        if OTHERS[1].is_match(&captures["base"]) {
                            let address_at_there =
                                u64::from_str_radix(&captures["relative_address"], 16).unwrap();
                            return Ok(address_at_there);
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                "lea" => {
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
    Ok(0)
}
