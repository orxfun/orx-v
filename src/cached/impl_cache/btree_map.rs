use crate::Cache;
use alloc::collections::btree_map::BTreeMap;

impl<Idx: Ord + Copy, T> Cache<Idx, T> for BTreeMap<Idx, T> {
    #[inline(always)]
    fn len(&self) -> usize {
        BTreeMap::len(self)
    }

    fn clear(&mut self) {
        BTreeMap::clear(self)
    }

    #[inline(always)]
    fn entry_or_insert_with<F>(&mut self, idx: Idx, value: F) -> &mut T
    where
        F: FnOnce(Idx) -> T,
    {
        BTreeMap::entry(self, idx).or_insert_with(|| value(idx))
    }
}
