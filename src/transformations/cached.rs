use crate::{Dim, IntoIndex, NVec, NVecMut};
use core::{cell::RefCell, marker::PhantomData};

#[cfg(not(feature = "std"))]
type Map<K, T> = alloc::collections::btree_map::BTreeMap<K, T>;

#[cfg(feature = "std")]
type Map<K, T> = std::collections::hash_map::HashMap<K, T>;

pub struct Cached<N, T, V>
where
    N: Dim,
    V: NVec<N, T>,
{
    inner: V,
    cache: RefCell<Map<N::Idx, T>>,
    phantom: PhantomData<N>,
}

impl<N, T, V> Cached<N, T, V>
where
    N: Dim,
    V: NVec<N, T>,
    T: Copy,
{
    pub fn into_inner_cache(self) -> Map<N::Idx, T> {
        self.cache.into_inner()
    }
}

// nvecs

impl<N, T, V> NVec<N, T> for Cached<N, T, V>
where
    N: Dim,
    V: NVec<N, T>,
    T: Copy,
{
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
        let index = index.into_index();
        let cached = self.cache.borrow().get(&index).copied();
        match cached {
            // already available, fetch and return
            Some(w) => w,

            // compute, cache and return
            None => {
                let value = self.inner.at(index);
                self.cache.borrow_mut().insert(index, value);
                value
            }
        }
    }

    fn try_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T> {
        Some(self.at(index))
    }
}

impl<N, T, V> NVecMut<N, T> for Cached<N, T, V>
where
    N: Dim,
    V: NVec<N, T> + NVecMut<N, T>,
    T: Copy,
{
    fn set<Idx: IntoIndex<N>>(&mut self, index: Idx, value: T) {
        let index = index.into_index();
        let mut binding = self.cache.borrow_mut();
        let cached = binding.get_mut(&index);
        match cached {
            // already available, update
            Some(w) => *w = value,

            // not available, mutate inner
            None => self.inner.set(index, value),
        }
    }
}

// into

pub trait IntoCached<N, T>
where
    N: Dim,
    T: Copy,
    Self: Sized + NVec<N, T>,
{
    fn cached(self) -> Cached<N, T, Self>;
}

impl<N, T, V> IntoCached<N, T> for V
where
    N: Dim,
    T: Copy,
    V: NVec<N, T>,
{
    fn cached(self) -> Cached<N, T, V> {
        Cached {
            inner: self,
            cache: Default::default(),
            phantom: Default::default(),
        }
    }
}
