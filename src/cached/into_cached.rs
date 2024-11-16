use super::{cache::DefaultCache, Cache, CachedVec};
use crate::{Dim, NVec};
use core::hash::Hash;

/// Converts an `NVec<D, T>` into a cached vector which maintains an internal cache.
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
/// Note that a cached vector itself is an `NVec`. Therefore, it abstracts away the internal
/// cache management and allows us to treat it as any other vector.
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
pub trait IntoCached<D, T>: NVec<D, T>
where
    D: Dim,
    D::Idx: Ord + Hash,
    T: Copy,
{
    /// Converts an `NVec<D, T>` into a cached vector which maintains an internal cache.
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
    /// Note that a cached vector itself is an `NVec`. Therefore, it abstracts away the internal
    /// cache management and allows us to treat it as any other vector.
    ///
    /// In such scenarios, [`IntoCached`] trait makes it very convenient to convert a functional
    /// vector into a cached functional vector.
    ///
    /// # Safety
    ///
    /// The cache implementation that `CachedVec` uses by default is
    /// * `HashMap` when std feature is enabled,
    /// * `BTReeMap` in a no-std program.
    ///
    /// The cached vector adds interior mutability to these structures which is currently not
    /// thread-safe.
    /// Practically, this means the following:
    /// * We can use this vector safely by a single thread.
    /// * Using this vector concurrently by multiple threads leads to data race.
    ///   For instance, if the cached vector is an input of a parallelized algorithm,
    ///   we need to provide a different copy of the vector to each thread.
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
    fn into_cached(self) -> CachedVec<D, T, Self, DefaultCache<D, T>> {
        CachedVec::new(self, DefaultCache::<D, T>::default())
    }

    /// Converts an `NVec<D, T>` into a cached vector which maintains an internal cache.
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
    /// Note that a cached vector itself is an `NVec`. Therefore, it abstracts away the internal
    /// cache management and allows us to treat it as any other vector.
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
    /// let cache = DefaultCache::<D2, _>::from_iter([([4, 0], 4)]);
    /// let v2 = V.d2().fun(|[i, j]| api_call_to_get_distance(i, j)).into_cached_with(cache);
    /// assert_eq!(v2.cache_len(), 1);
    ///
    /// // does not make an api call since the element exists in the cache
    /// assert_eq!(v2.at([4, 0]), 4);
    /// assert_eq!(v2.cache_len(), 1);
    ///
    /// // makes the api call; caches and returns the value
    /// assert_eq!(v2.at([0, 3]), 3);
    /// assert_eq!(v2.cache_len(), 2);
    ///
    /// // does not repeat the api call; returns the value from the cache
    /// assert_eq!(v2.at([0, 3]), 3);
    ///
    /// // we can destruct the cached vec into vec & cache
    /// let (funvec, cache) = v2.into_inner();
    /// assert_eq!(cache.len(), 2);
    /// ```
    fn into_cached_with<C: Cache<D::Idx, T>>(self, cache: C) -> CachedVec<D, T, Self, C> {
        CachedVec::new(self, cache)
    }
}
