use alloc::collections::btree_map::BTreeMap;

pub trait KvMap<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
}

impl<K: Ord, V> KvMap<K, V> for BTreeMap<K, V> {
    #[inline]
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }
}

// TODO: impl for hashmap
