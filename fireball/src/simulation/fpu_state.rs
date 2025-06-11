//! x87 FPU state emulation
//!
//! This module provides emulation for the x87 FPU stack-based architecture.
//! The FPU has 8 registers (ST0-ST7) organized as a stack with ST0 at the top.

use crate::simulation::SimulationResult;

/// FPU Control Word fields
#[derive(Debug, Clone, Copy)]
pub struct FpuControlWord {
    /// Precision control (00=single, 10=double, 11=extended)
    pub precision: u8,
    /// Rounding control (00=nearest, 01=down, 10=up, 11=truncate)
    pub rounding: u8,
    /// Exception masks
    pub exception_masks: u8,
}

impl Default for FpuControlWord {
    fn default() -> Self {
        Self {
            precision: 0b11,       // Extended precision
            rounding: 0b00,        // Round to nearest
            exception_masks: 0x3f, // All exceptions masked
        }
    }
}

/// FPU Status Word fields
#[derive(Debug, Clone, Copy, Default)]
pub struct FpuStatusWord {
    /// Stack top pointer (0-7)
    pub top: u8,
    /// Condition code flags (C0-C3)
    pub condition_codes: u8,
    /// Exception flags
    pub exception_flags: u8,
    /// Stack fault
    pub stack_fault: bool,
    /// Error summary
    pub error_summary: bool,
    /// Busy flag
    pub busy: bool,
}

/// x87 FPU state
#[derive(Debug, Clone)]
pub struct FpuState {
    /// FPU register stack (8 x 80-bit registers)
    /// Stored as f64 for simplicity, but real x87 uses 80-bit extended precision
    stack: [f64; 8],
    /// Tag word - 2 bits per register (00=valid, 01=zero, 10=special, 11=empty)
    tags: [u8; 8],
    /// Control word
    control: FpuControlWord,
    /// Status word
    status: FpuStatusWord,
    /// Last instruction pointer
    last_ip: u64,
    /// Last data pointer
    last_dp: u64,
    /// Last opcode
    last_opcode: u16,
}

impl FpuState {
    /// Create a new FPU state with all registers empty
    pub fn new() -> Self {
        Self {
            stack: [0.0; 8],
            tags: [0b11; 8], // All empty
            control: FpuControlWord::default(),
            status: FpuStatusWord::default(),
            last_ip: 0,
            last_dp: 0,
            last_opcode: 0,
        }
    }

    /// Get the physical index for a stack register (ST(i))
    fn get_physical_index(&self, st_index: u8) -> u8 {
        (self.status.top + st_index) & 0x7
    }

    /// Check if a stack register is empty
    pub fn is_empty(&self, st_index: u8) -> bool {
        let phys_index = self.get_physical_index(st_index) as usize;
        self.tags[phys_index] == 0b11
    }

    /// Push a value onto the FPU stack
    pub fn push(&mut self, value: f64) -> SimulationResult<()> {
        // Decrement TOP
        self.status.top = (self.status.top.wrapping_sub(1)) & 0x7;

        let phys_index = self.get_physical_index(0) as usize;

        // Check for stack overflow
        if self.tags[phys_index] != 0b11 {
            self.status.stack_fault = true;
            self.status.error_summary = true;
            return Err(crate::simulation::SimulationError::FpuStackOverflow);
        }

        // Store value and mark as valid
        self.stack[phys_index] = value;
        self.tags[phys_index] = if value == 0.0 { 0b01 } else { 0b00 };

        Ok(())
    }

    /// Pop a value from the FPU stack
    pub fn pop(&mut self) -> SimulationResult<f64> {
        let phys_index = self.get_physical_index(0) as usize;

        // Check for stack underflow
        if self.tags[phys_index] == 0b11 {
            self.status.stack_fault = true;
            self.status.error_summary = true;
            return Err(crate::simulation::SimulationError::FpuStackUnderflow);
        }

        let value = self.stack[phys_index];
        self.tags[phys_index] = 0b11; // Mark as empty

        // Increment TOP
        self.status.top = (self.status.top + 1) & 0x7;

        Ok(value)
    }

    /// Get value from stack register ST(i) without popping
    pub fn get(&self, st_index: u8) -> SimulationResult<f64> {
        if st_index > 7 {
            return Err(crate::simulation::SimulationError::InvalidFpuRegister(
                st_index,
            ));
        }

        let phys_index = self.get_physical_index(st_index) as usize;

        if self.tags[phys_index] == 0b11 {
            // Note: In real hardware, this would set stack_fault
            return Err(crate::simulation::SimulationError::FpuStackUnderflow);
        }

        Ok(self.stack[phys_index])
    }

    /// Set value in stack register ST(i) without pushing
    pub fn set(&mut self, st_index: u8, value: f64) -> SimulationResult<()> {
        if st_index > 7 {
            return Err(crate::simulation::SimulationError::InvalidFpuRegister(
                st_index,
            ));
        }

        let phys_index = self.get_physical_index(st_index) as usize;

        self.stack[phys_index] = value;
        self.tags[phys_index] = if value == 0.0 { 0b01 } else { 0b00 };

        Ok(())
    }

    /// Exchange ST(0) with ST(i)
    pub fn exchange(&mut self, st_index: u8) -> SimulationResult<()> {
        if st_index > 7 {
            return Err(crate::simulation::SimulationError::InvalidFpuRegister(
                st_index,
            ));
        }

        let phys_0 = self.get_physical_index(0) as usize;
        let phys_i = self.get_physical_index(st_index) as usize;

        // Exchange values
        self.stack.swap(phys_0, phys_i);
        self.tags.swap(phys_0, phys_i);

        Ok(())
    }

    /// Update condition codes after comparison
    pub fn set_condition_codes(&mut self, result: std::cmp::Ordering) {
        use std::cmp::Ordering;

        // C3 C2 C0 = result
        // Less:    0  0  1
        // Equal:   1  0  0
        // Greater: 0  0  0
        // Unord:   1  1  1
        match result {
            Ordering::Less => {
                self.status.condition_codes = 0b0001; // C0=1
            }
            Ordering::Equal => {
                self.status.condition_codes = 0b1000; // C3=1
            }
            Ordering::Greater => {
                self.status.condition_codes = 0b0000; // All clear
            }
        }
    }

    /// Get the current stack top pointer
    pub fn get_top(&self) -> u8 {
        self.status.top
    }

    /// Reset FPU state (FINIT)
    pub fn reset(&mut self) {
        self.stack = [0.0; 8];
        self.tags = [0b11; 8]; // All empty
        self.control = FpuControlWord::default();
        self.status = FpuStatusWord::default();
        self.last_ip = 0;
        self.last_dp = 0;
        self.last_opcode = 0;
    }

    /// Get status word as u16
    pub fn get_status_word(&self) -> u16 {
        let mut sw = 0u16;
        sw |= (self.status.top as u16) << 11;
        sw |= (self.status.condition_codes as u16 & 0xF) << 8;
        sw |= self.status.exception_flags as u16;
        if self.status.stack_fault {
            sw |= 1 << 6;
        }
        if self.status.error_summary {
            sw |= 1 << 7;
        }
        if self.status.busy {
            sw |= 1 << 15;
        }
        sw
    }

    /// Set control word from u16
    pub fn set_control_word(&mut self, cw: u16) {
        self.control.precision = ((cw >> 8) & 0x3) as u8;
        self.control.rounding = ((cw >> 10) & 0x3) as u8;
        self.control.exception_masks = (cw & 0x3F) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fpu_push_pop() {
        let mut fpu = FpuState::new();

        // Push values
        fpu.push(std::f64::consts::PI).unwrap();
        fpu.push(2.71).unwrap();

        // Pop values (LIFO order)
        assert_eq!(fpu.pop().unwrap(), 2.71);
        assert_eq!(fpu.pop().unwrap(), std::f64::consts::PI);

        // Stack should be empty now
        assert!(fpu.pop().is_err());
    }

    #[test]
    fn test_fpu_exchange() {
        let mut fpu = FpuState::new();

        fpu.push(1.0).unwrap();
        fpu.push(2.0).unwrap();
        fpu.push(3.0).unwrap();

        // ST(0) = 3.0, ST(1) = 2.0, ST(2) = 1.0
        assert_eq!(fpu.get(0).unwrap(), 3.0);
        assert_eq!(fpu.get(1).unwrap(), 2.0);
        assert_eq!(fpu.get(2).unwrap(), 1.0);

        // Exchange ST(0) with ST(2)
        fpu.exchange(2).unwrap();

        assert_eq!(fpu.get(0).unwrap(), 1.0);
        assert_eq!(fpu.get(1).unwrap(), 2.0);
        assert_eq!(fpu.get(2).unwrap(), 3.0);
    }

    #[test]
    fn test_fpu_condition_codes() {
        let mut fpu = FpuState::new();

        fpu.set_condition_codes(std::cmp::Ordering::Less);
        assert_eq!(fpu.status.condition_codes & 0x1, 1); // C0 = 1

        fpu.set_condition_codes(std::cmp::Ordering::Equal);
        assert_eq!(fpu.status.condition_codes & 0x8, 8); // C3 = 1

        fpu.set_condition_codes(std::cmp::Ordering::Greater);
        assert_eq!(fpu.status.condition_codes, 0); // All clear
    }
}
