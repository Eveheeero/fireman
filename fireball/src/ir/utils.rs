use hashbrown::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IrStatementDescriptor {
    ir_index: u32,
    statement_index: u8,
}

impl IrStatementDescriptor {
    #[inline]
    pub fn new(ir_index: u32, statement_index: u8) -> Self {
        Self {
            ir_index,
            statement_index,
        }
    }
    #[inline]
    pub fn ir_index(&self) -> u32 {
        self.ir_index
    }
    #[inline]
    pub fn statement_index(&self) -> u8 {
        self.statement_index
    }
    #[inline]
    pub fn from_u64(value: u64) -> Self {
        Self {
            ir_index: (value >> 32) as u32,
            statement_index: (value & 0xffffffff) as u8,
        }
    }
    #[inline]
    pub fn to_u64(&self) -> u64 {
        ((self.ir_index as u64) << 32) | (self.statement_index as u64)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrStatementDescriptorMap<T> {
    inner: HashMap<u64, T>,
}

impl<T> IrStatementDescriptorMap<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    #[inline]
    pub fn get(&self, key: IrStatementDescriptor) -> Option<&T> {
        self.inner.get(&key.to_u64())
    }
    #[inline]
    pub fn get_mut(&mut self, key: IrStatementDescriptor) -> Option<&mut T> {
        self.inner.get_mut(&key.to_u64())
    }
    #[inline]
    pub fn insert(&mut self, key: IrStatementDescriptor, value: T) -> Option<T> {
        self.inner.insert(key.to_u64(), value)
    }
    #[inline]
    pub fn insert_checked(&mut self, key: IrStatementDescriptor, value: T) -> bool {
        self.inner.try_insert(key.to_u64(), value).is_ok()
    }
    #[inline]
    pub fn remove(&mut self, key: IrStatementDescriptor) -> Option<T> {
        self.inner.remove(&key.to_u64())
    }
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear()
    }
    #[inline]
    pub fn contains_key(&self, key: IrStatementDescriptor) -> bool {
        self.inner.contains_key(&key.to_u64())
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    #[inline]
    pub fn values(&self) -> hashbrown::hash_map::Values<u64, T> {
        self.inner.values()
    }
    #[inline]
    pub fn keys(&self) -> Vec<IrStatementDescriptor> {
        self.inner
            .keys()
            .map(|x| IrStatementDescriptor::from_u64(*x))
            .collect()
    }
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (IrStatementDescriptor, &T)> {
        self.inner
            .iter()
            .map(|(key, value)| (IrStatementDescriptor::from_u64(*key), value))
    }
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (IrStatementDescriptor, &mut T)> {
        self.inner
            .iter_mut()
            .map(|(key, value)| (IrStatementDescriptor::from_u64(*key), value))
    }
}
