use std::sync::Arc;

/// 코드 블럭과 다른 블럭과의 연관을 나타낸다.
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct Relation {}

lazy_static::lazy_static! {
    static ref RELATIONS: std::sync::RwLock<std::collections::HashSet<Arc<Relation>>> = Default::default();
}
