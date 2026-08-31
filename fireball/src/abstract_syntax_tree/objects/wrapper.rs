use std::{
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};

#[derive(Debug, Clone)]
pub struct Wrapped<T> {
    pub id: AstNodeId,
    pub item: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstNodeId(pub(crate) usize);

impl AstNodeId {
    pub fn new() -> Self {
        static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        AstNodeId(id)
    }
}

impl<T: PartialEq> PartialEq for Wrapped<T> {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
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
