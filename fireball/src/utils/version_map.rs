use hashbrown::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct VersionMap<Version: Ord + Hash, V> {
    inner: HashMap<Version, V>,
}

impl<Version: Ord + Hash, V> VersionMap<Version, V> {
    #[inline]
    pub fn new(version: Version, value: V) -> Self {
        Self {
            inner: HashMap::from([(version, value)]),
        }
    }
    #[inline]
    pub fn raw(&self) -> &HashMap<Version, V> {
        &self.inner
    }
    #[inline]
    pub fn raw_mut(&mut self) -> &mut HashMap<Version, V> {
        &mut self.inner
    }
    #[inline]
    pub fn get(&self, version: &Version) -> Option<&V> {
        self.inner.get(version)
    }
    #[inline]
    pub fn get_mut(&mut self, version: &Version) -> Option<&mut V> {
        self.inner.get_mut(version)
    }
    #[inline]
    pub fn get_last_version(&self) -> &V {
        let key = self.last_version();
        self.inner.get(key).unwrap()
    }
    #[inline]
    pub fn last_version(&self) -> &Version {
        self.inner.keys().max().unwrap()
    }
    #[inline]
    pub fn versions(&self) -> impl Iterator<Item = &Version> {
        self.inner.keys()
    }
    #[inline]
    pub fn insert(&mut self, version: Version, value: V) -> Result<(), ()> {
        let v = self.inner.get(&version);
        if v.is_some() {
            return Err(());
        }
        self.inner.insert(version, value);
        Ok(())
    }
}
