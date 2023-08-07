use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Register {
    inner: Range<usize>,
}

impl Register {
    pub(crate) fn inner(&self) -> Range<usize> {
        self.inner.clone()
    }
}

impl From<Range<usize>> for Register {
    fn from(range: Range<usize>) -> Self {
        Register { inner: range }
    }
}
