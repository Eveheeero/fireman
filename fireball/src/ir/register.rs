use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Register {
    bit_start: usize,
    bit_end: usize,
}

impl Register {
    #[inline]
    pub(crate) const fn new(range: Range<usize>) -> Self {
        Register {
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
}
