use crate::Cache;
use core::hash::Hash;
use std::collections::HashMap;

impl<Idx: Eq + Hash + Copy, T> Cache<Idx, T> for HashMap<Idx, T> {
    #[inline(always)]
    fn len(&self) -> usize {
        HashMap::len(self)
    }

    fn clear(&mut self) {
        HashMap::clear(self)
    }

    #[inline(always)]
    fn entry_or_insert_with<F>(&mut self, idx: Idx, value: F) -> &mut T
    where
        F: FnOnce(Idx) -> T,
    {
        HashMap::entry(self, idx).or_insert_with(|| value(idx))
    }
}
