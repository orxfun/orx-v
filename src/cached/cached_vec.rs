use super::cache::{Cache, DefaultCache};
use crate::common_trait_helpers::debug::*;
use crate::{NVec, dim::*};
use core::fmt::Debug;
use core::{cell::UnsafeCell, marker::PhantomData};

/// Wraps an `NVec<D, T>` into a cached vector which maintains an internal cache.
///
/// Each time an element is requested, the vector first checks the cache:
/// * if the value is readily available in the cache, the vector returns it,    
/// * otherwise, it computes its value, caches it for future use and returns it.
///
/// The cache is often a simple lookup or map, such as the HashMap. However, it might
/// be more advanced such as the least frequently used cache. Any internal structure
/// implementing the [`Cache`] can be used.
///
/// The aim of the cached vector is to improve the execution time where computing an
/// element is expensive. Consider for instance element (i, j) corresponds to the
/// duration between two addresses i and j on a street network which requires running
/// an algorithm to compute. Further assume that:
/// * pre-computing all elements is not justified as we will access only a small portion
///   of the entire matrix, and
/// * we do not know ahead of time which indices that we will access.
///
/// In such scenarios, [`IntoCached`] trait makes it very convenient to convert a functional
/// vector into a cached functional vector.
///
/// # Examples
///
/// ```
/// use orx_v::*;
///
/// // assume an expensive api call to compute distance
/// fn api_call_to_get_distance(from: usize, to: usize) -> u64 {
///     match from > to {
///         true => (from - to) as u64,
///         false => (to - from) as u64,
///     }
/// }
///
/// let v2 = V.d2().fun(|[i, j]| api_call_to_get_distance(i, j)).into_cached();
/// assert_eq!(v2.cache_len(), 0);
///
/// // makes the api call; caches and returns the value
/// assert_eq!(v2.at([0, 3]), 3);
/// assert_eq!(v2.cache_len(), 1);
///
/// // does not repeat the api call; returns the value from the cache
/// assert_eq!(v2.at([0, 3]), 3);
/// ```
///
/// [`IntoCached`]: crate::IntoCached
pub struct CachedVec<D, T, V, C = DefaultCache<D, T>>
where
    D: Dim,
    V: NVec<D, T>,
    C: Cache<D::Idx, T>,
    T: Copy,
{
    pub(super) vec: V,
    pub(super) cache: UnsafeCell<C>,
    phantom: PhantomData<(D, T)>,
}

impl<D, T, V, C> CachedVec<D, T, V, C>
where
    D: Dim,
    V: NVec<D, T>,
    C: Cache<D::Idx, T>,
    T: Copy,
{
    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    pub(super) unsafe fn entry_or_insert_with(&self, idx: impl IntoIdx<D>) -> &mut T {
        let cache = unsafe { &mut *self.cache.get() };
        cache.entry_or_insert_with(idx.into_idx(), |idx| self.vec.at(idx))
    }

    pub(crate) fn new(vec: V, cache: C) -> Self {
        Self {
            vec,
            cache: cache.into(),
            phantom: PhantomData,
        }
    }

    /// Destructs the cached vec and returns the tuple of the underlying `NVec<D, T>`
    /// and cache.
    ///
    /// Note that a new cached vector can be constructed by re-using the cache by
    /// calling the [`into_cached_with`] method on the vec.
    ///
    /// [`into_cached_with`]: `crate::IntoCached::into_cached_with`
    pub fn into_inner(self) -> (V, C) {
        (self.vec, self.cache.into_inner())
    }

    /// Clears the internal cache of the cached vector; i.e., forgets all cached
    /// elements.
    pub fn clean_cache(&mut self) {
        let cache = unsafe { &mut *self.cache.get() };
        cache.clear();
    }

    /// Returns the number of elements which are currently available in the cache.
    pub fn cache_len(&self) -> usize {
        let cache = unsafe { &*self.cache.get() };
        cache.len()
    }
}

macro_rules! impl_debug {
    ($dim:ty, $dbg_fn:ident) => {
        impl<T, V, C> Debug for CachedVec<$dim, T, V, C>
        where
            V: NVec<$dim, T>,
            C: Cache<<$dim as Dim>::Idx, T>,
            T: Copy + Debug,
            Self: NVec<$dim, T>,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(
                    f,
                    "{{ kind: CachedVec, dim: D{}, is_bounded: {}, cache_len: {}, values: ",
                    <$dim as Dim>::dimension(),
                    self.is_bounded(),
                    self.cache_len(),
                )?;
                $dbg_fn(f, self)?;
                write!(f, " }}")
            }
        }
    };
}

impl_debug!(D1, dbg_values_d1);
impl_debug!(D2, dbg_values_d2);
impl_debug!(D3, dbg_values_d3);
impl_debug!(D4, dbg_values_d4);
