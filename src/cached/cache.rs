#![allow(type_alias_bounds)]
use crate::Dim;

/// A simple map of indices to values.
///
/// Note that HashMap (with std) or BTreeMap (when no-std) satisfy the requirements and
/// implement the `Cache` trait, and hence, can be used as the caching storage in cached
/// vectors.
///
/// Alternatively, more advanced caches can be provided depending on the use case, such
/// as least recently used (LRU) caches.
pub trait Cache<Idx, T>: Default {
    /// Number of (idx, value) pairs in the cache.
    fn len(&self) -> usize;

    /// Returns true if the cache is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// If the cache contains an element with the given `idx`, the method returns a
    /// mutable reference to the value.
    ///
    /// Otherwise,
    /// * calls the `value()` method to create the value,
    /// * inserts `(idx, value())` to the cache, and
    /// * returns a mutable reference to the newly inserted value.
    fn entry_or_insert_with<F>(&mut self, idx: Idx, value: F) -> &mut T
    where
        F: FnOnce(Idx) -> T;

    /// Clears the cache.
    fn clear(&mut self);
}

/// Default type used as the cache of cached vectors.
#[cfg(not(any(test, feature = "std")))]
pub type DefaultCache<D: Dim, T> = alloc::collections::btree_map::BTreeMap<D::Idx, T>;

/// Default type used as the cache of cached vectors.
#[cfg(any(test, feature = "std"))]
pub type DefaultCache<D: Dim, T> = std::collections::hash_map::HashMap<D::Idx, T>;
