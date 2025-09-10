use crate::ir::Architecture;
use std::hash::Hasher;
use std::ops::Range;
use std::sync::LazyLock;

#[derive(Debug, Clone, Eq, Copy)]
pub struct Register {
    architecture: Architecture,
    name: &'static str,
    bit_start: usize,
    bit_end: usize,
}

impl Register {
    #[inline]
    pub(crate) const fn new(
        architecture: Architecture,
        name: &'static str,
        range: Range<usize>,
    ) -> Self {
        Register {
            architecture,
            name,
            bit_start: range.start,
            bit_end: range.end,
        }
    }
    #[inline]
    pub(crate) fn architecture(&self) -> Architecture {
        self.architecture
    }
    #[inline]
    pub(crate) fn bit_range(&self) -> Range<usize> {
        self.bit_start..self.bit_end
    }
    #[inline]
    pub fn bit_len(&self) -> usize {
        self.bit_end - self.bit_start
    }
    #[inline]
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn is_stack_related(&self) -> bool {
        use crate::ir::{Architecture, VirtualMachine, x86_64::X64Range};
        match self.architecture() {
            Architecture::X64 => {
                static SP_BIT_START: LazyLock<usize> =
                    LazyLock::new(|| <VirtualMachine as X64Range>::sp().bit_range().start);
                static BP_BIT_START: LazyLock<usize> =
                    LazyLock::new(|| <VirtualMachine as X64Range>::bp().bit_range().start);
                let register_start = self.bit_range().start;
                *SP_BIT_START == register_start || *BP_BIT_START == register_start
            }
        }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl PartialEq for Register {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.name, other.name)
    }
}
impl std::hash::Hash for Register {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.architecture.hash(state);
        self.name.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::{VirtualMachine, x86_64::X64Range};

    #[test]
    fn test_stack_related() {
        assert!(<VirtualMachine as X64Range>::spl().is_stack_related());
        assert!(<VirtualMachine as X64Range>::sp().is_stack_related());
        assert!(<VirtualMachine as X64Range>::esp().is_stack_related());
        assert!(<VirtualMachine as X64Range>::rsp().is_stack_related());
        assert!(<VirtualMachine as X64Range>::bpl().is_stack_related());
        assert!(<VirtualMachine as X64Range>::bp().is_stack_related());
        assert!(<VirtualMachine as X64Range>::ebp().is_stack_related());
        assert!(<VirtualMachine as X64Range>::rbp().is_stack_related());
        assert!(!<VirtualMachine as X64Range>::al().is_stack_related());
        assert!(!<VirtualMachine as X64Range>::fpu_pe().is_stack_related());
        assert!(!<VirtualMachine as X64Range>::xmm4().is_stack_related());
    }
    #[test]
    fn register_equals() {
        let one = <VirtualMachine as X64Range>::fpu_pe();
        let two = <VirtualMachine as X64Range>::fpu_pe();
        assert_eq!(one, two, "eq doesn't work");
    }
}
