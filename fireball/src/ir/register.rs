use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Register {
    inner: Range<usize>,
}

impl Register {
    pub(crate) fn inner(&self) -> Range<usize> {
        self.inner.clone()
    }
    pub fn bit_len(&self) -> usize {
        self.inner.end - self.inner.start
    }
}

impl From<Range<usize>> for Register {
    fn from(range: Range<usize>) -> Self {
        Register { inner: range }
    }
}
