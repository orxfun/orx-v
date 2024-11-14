use crate::Lookup;
use alloc::collections::btree_map::BTreeMap;

impl<Idx: Ord, T> Lookup<Idx, T> for BTreeMap<Idx, T> {
    #[inline(always)]
    fn len(&self) -> usize {
        BTreeMap::len(self)
    }

    #[inline(always)]
    fn contains_key(&self, idx: &Idx) -> bool {
        BTreeMap::contains_key(self, idx)
    }

    #[inline(always)]
    fn get(&self, idx: &Idx) -> Option<&T> {
        BTreeMap::get(self, idx)
    }

    #[inline(always)]
    fn insert(&mut self, idx: Idx, value: T) {
        BTreeMap::insert(self, idx, value);
    }

    #[inline(always)]
    fn entry_or_insert(&mut self, idx: Idx, value: T) -> &mut T {
        BTreeMap::entry(self, idx).or_insert(value)
    }

    fn values_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        T: 'a,
    {
        self.values_mut()
    }

    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (&'a Idx, &'a mut T)>
    where
        T: 'a,
        Idx: 'a,
    {
        self.iter_mut()
    }

    fn clear(&mut self) {
        self.clear();
    }
}
