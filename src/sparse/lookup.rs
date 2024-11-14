#![allow(type_alias_bounds)]
use crate::Dim;

/// A map of indices to values.
///
/// Note that HashMap (with std) or BTreeMap (when no-std) satisfy the requirements and
/// implement the `Lookup` trait, and hence, can be used as the storage in sparse vectors.
///
/// Alternatively, more advanced lookup structures can be provided depending on the use case.
pub trait Lookup<Idx, T>: Default {
    /// Number of elements stored in the lookup.
    fn len(&self) -> usize;

    /// Returns true if the lookup is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns whether or not the `idx` is present as a key in the lookup.
    fn contains_key(&self, idx: &Idx) -> bool;

    /// Returns a reference to the value with the given `idx` if it exists in the lookup;
    /// returns None otherwise.
    fn get(&self, idx: &Idx) -> Option<&T>;

    /// Inserts the `value` with key `idx`.
    /// If the key already existed, its value is updated.
    fn insert(&mut self, idx: Idx, value: T);

    /// If the lookup contains an element with the given `idx`, the method returns a
    /// mutable reference to the value.
    ///
    /// Otherwise,
    /// * inserts `(idx, value)` to the lookup, and
    /// * returns a mutable reference to the newly inserted value.
    fn entry_or_insert(&mut self, idx: Idx, value: T) -> &mut T;

    /// Returns an iterator yielding mutable references to the values in the lookup.
    fn values_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        T: 'a;

    /// Returns an iterator yielding indices and mutable references to the values in the lookup.
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (&'a Idx, &'a mut T)>
    where
        T: 'a,
        Idx: 'a;

    /// Clears the lookup.
    fn clear(&mut self);
}

/// Default type used as the lookup of sparse vectors.
#[cfg(not(any(test, feature = "std")))]
pub type DefaultLookup<D: Dim, T> = alloc::collections::btree_map::BTreeMap<D::Idx, T>;

/// Default type used as the lookup of sparse vectors.
#[cfg(any(test, feature = "std"))]
pub type DefaultLookup<D: Dim, T> = std::collections::hash_map::HashMap<D::Idx, T>;
