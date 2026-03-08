//! Win64 `.pdata` / `.xdata` unwind metadata parsing for PE binaries.
//!
//! This module performs a conservative best-effort parse of PE exception data
//! exposed by `goblin` and extracts function ranges plus a small summary of the
//! associated unwind information. It currently targets Win64 `RUNTIME_FUNCTION`
//! records and does not reconstruct EH scopes.

use crate::{
    core::{Address, PreDefinedOffset, PreDefinedOffsets, Sections},
    prelude::*,
};
use goblin::pe::exception::{UnwindHandler, UnwindOperation};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct CfiInfo {
    pub functions: Vec<UnwindFunctionInfo>,
}

#[derive(Debug, Clone)]
pub struct UnwindFunctionInfo {
    pub begin_rva: u32,
    pub end_rva: u32,
    pub unwind_info_rva: u32,
    pub size_of_prolog: u8,
    pub frame_register: String,
    pub frame_register_offset: u32,
    pub stack_allocation: u32,
    pub saved_registers: Vec<String>,
    pub saved_xmm_registers: Vec<String>,
    pub has_exception_handler: bool,
    pub has_termination_handler: bool,
    pub has_chained_info: bool,
    pub unwind_code_count: usize,
}

pub fn try_load_cfi(pe: &goblin::pe::PE) -> Option<CfiInfo> {
    let exception_data = pe.exception_data.as_ref()?;

    let mut functions = Vec::new();
    for function_result in exception_data.functions() {
        let function = match function_result {
            Ok(function) => function,
            Err(error) => {
                debug!("Stopped Win64 unwind function iteration early: {error}");
                break;
            }
        };

        if function.begin_address == 0 || function.begin_address >= function.end_address {
            continue;
        }

        let unwind_info = match exception_data.get_unwind_info(function, &pe.sections) {
            Ok(unwind_info) => unwind_info,
            Err(error) => {
                debug!(
                    "Skipping malformed unwind info for runtime function {:#x}-{:#x}: {error}",
                    function.begin_address, function.end_address
                );
                continue;
            }
        };

        let mut stack_allocation = 0u32;
        let mut saved_registers = Vec::new();
        let mut saved_xmm_registers = Vec::new();
        let mut unwind_code_count = 0usize;

        for unwind_code_result in unwind_info.unwind_codes() {
            let unwind_code = match unwind_code_result {
                Ok(unwind_code) => unwind_code,
                Err(error) => {
                    debug!(
                        "Stopped unwind-code iteration early for runtime function {:#x}-{:#x}: {error}",
                        function.begin_address, function.end_address
                    );
                    break;
                }
            };

            unwind_code_count += 1;
            match unwind_code.operation {
                UnwindOperation::PushNonVolatile(register)
                | UnwindOperation::SaveNonVolatile(register, _) => {
                    push_unique(
                        &mut saved_registers,
                        normalize_register_name(register.name()),
                    );
                }
                UnwindOperation::Alloc(size) => {
                    stack_allocation = stack_allocation.saturating_add(size);
                }
                UnwindOperation::SaveXMM(register, _)
                | UnwindOperation::SaveXMM128(register, _) => {
                    push_unique(
                        &mut saved_xmm_registers,
                        normalize_register_name(register.name()),
                    );
                }
                UnwindOperation::SetFPRegister
                | UnwindOperation::Epilog { .. }
                | UnwindOperation::PushMachineFrame(_)
                | UnwindOperation::Noop => {}
                _ => {}
            }
        }

        let (has_exception_handler, has_termination_handler) = match unwind_info.handler.as_ref() {
            Some(UnwindHandler::ExceptionHandler(_, _)) => (true, false),
            Some(UnwindHandler::TerminationHandler(_, _)) => (false, true),
            None => (false, false),
        };

        functions.push(UnwindFunctionInfo {
            begin_rva: function.begin_address,
            end_rva: function.end_address,
            unwind_info_rva: function.unwind_info_address,
            size_of_prolog: unwind_info.size_of_prolog,
            frame_register: normalize_register_name(unwind_info.frame_register.name()),
            frame_register_offset: unwind_info.frame_register_offset,
            stack_allocation,
            saved_registers,
            saved_xmm_registers,
            has_exception_handler,
            has_termination_handler,
            has_chained_info: unwind_info.chained_info.is_some(),
            unwind_code_count,
        });
    }

    debug!(
        "Recovered {} Win64 unwind records from .pdata/.xdata",
        functions.len()
    );

    Some(CfiInfo { functions })
}

pub fn merge_cfi_symbols(cfi_info: &CfiInfo, defined: &PreDefinedOffsets, sections: &Sections) {
    let mut known_addresses = collect_known_addresses(defined);
    let mut merged = 0u64;

    for function in &cfi_info.functions {
        let address = Address::from_virtual_address(sections, function.begin_rva as u64);
        if !known_addresses.insert(address.clone()) {
            continue;
        }

        defined.insert(PreDefinedOffset {
            name: format!("func_{:x}", function.begin_rva),
            address,
        });
        merged += 1;
    }

    debug!(
        "Merged {} Win64 unwind function symbols into predefined offsets",
        merged
    );
}

fn collect_known_addresses(defined: &PreDefinedOffsets) -> HashSet<Address> {
    defined
        .get_reader()
        .iter()
        .map(|offset| offset.address.clone())
        .collect()
}

fn normalize_register_name(name: &str) -> String {
    name.trim_start_matches('$').to_string()
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.iter().any(|existing| existing == &value) {
        values.push(value);
    }
}
