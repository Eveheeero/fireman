use std::{ops::Deref, sync::Arc};

/// Arc or static
#[derive(Debug)]
pub enum Aos<T: ?Sized + 'static> {
    Arc(Arc<T>),
    StaticRef(&'static T),
    StaticOwned(*const T),
}

unsafe impl<T> Send for Aos<T> {}
unsafe impl<T> Sync for Aos<T> {}

impl<T> Deref for Aos<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Aos::Arc(inner) => inner,
            Aos::StaticRef(inner) => inner,
            Aos::StaticOwned(inner) => unsafe { inner.as_ref().unwrap() },
        }
    }
}
impl<T> AsRef<T> for Aos<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T> Clone for Aos<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Arc(arg0) => Self::Arc(arg0.clone()),
            Self::StaticRef(arg0) => Self::StaticRef(arg0),
            Self::StaticOwned(arg0) => Self::StaticRef(unsafe { arg0.as_ref().unwrap() }),
        }
    }
}

impl<T> Aos<T> {
    pub fn new_static(t: T) -> Self {
        Self::StaticOwned(Box::into_raw(Box::new(t)))
    }
    pub fn new(t: T) -> Self {
        Self::Arc(Arc::new(t))
    }
    pub fn as_ptr(this: &Self) -> *const T {
        match this {
            Self::Arc(inner) => Arc::as_ptr(inner),
            Self::StaticRef(inner) => *inner as *const T,
            Self::StaticOwned(inner) => *inner,
        }
    }
}

impl<T> From<T> for Aos<T> {
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<T: Eq> Eq for Aos<T> {}
impl<T: PartialEq + Eq> PartialEq for Aos<T> {
    fn eq(&self, other: &Aos<T>) -> bool {
        self.as_ref() == other.as_ref()
    }
}
impl<T: Ord> Ord for Aos<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}
impl<T: PartialOrd + Eq> PartialOrd for Aos<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}
impl<T: std::hash::Hash> std::hash::Hash for Aos<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let inner = self.as_ref();
        core::mem::discriminant(inner).hash(state);
    }
}
