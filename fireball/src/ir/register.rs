use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, PartialOrd, Ord)]
pub struct Register {
    name: &'static str,
    bit_start: usize,
    bit_end: usize,
}

impl Register {
    #[inline]
    pub(crate) const fn new(name: &'static str, range: Range<usize>) -> Self {
        Register {
            name,
            bit_start: range.start,
            bit_end: range.end,
        }
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
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
