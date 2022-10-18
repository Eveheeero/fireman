/// 섹션에 대한 정보가 들어있는 구조체
#[derive(Debug, Clone)]
pub(crate) struct Section {
    pub(crate) name: String,
    pub(crate) base_addr: usize,
}

lazy_static::lazy_static! {
  /// 섹션 정보의 집합
  pub(crate) static ref SECTIONS: std::collections::btree_set::BTreeSet<Section> = Default::default();
}
