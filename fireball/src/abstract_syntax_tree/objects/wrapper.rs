use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct Wrapped<T> {
    pub item: T,
    pub comment: Option<String>,
}

impl<T> AsRef<T> for Wrapped<T> {
    fn as_ref(&self) -> &T {
        &self.item
    }
}
impl<T> Deref for Wrapped<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}
