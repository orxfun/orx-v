use super::FlatJagged;
use crate::{NVec, D1};

/// A mutable row of a [`FlatJagged`] vector, which is naturally a `D1` vector.
///
/// # Examples
///
/// ```
/// use orx_v::*;
///
/// let storage = vec![0, 2, 4, 6];
/// let row_lengths = [3, 0, 1];
/// let mut jagged = storage.into_jagged_from_row_lengths(&row_lengths);
///
/// assert_eq!(
///     jagged.equality(&vec![vec![0, 2, 4], vec![], vec![6]]),
///     Equality::Equal,
/// );
///
/// {
///     let mut row = jagged.child_mut(0);
///     assert_eq!(
///         row.equality(&[0, 2, 4]),
///         Equality::Equal,
///     );
///
///     row.set(0, 42);
///     *row.at_mut(1) += 10;
///     assert_eq!(
///         row.equality(&[42, 12, 4]),
///         Equality::Equal,
///     );
/// }
///
/// assert_eq!(
///     jagged.equality(&vec![vec![42, 12, 4], vec![], vec![6]]),
///     Equality::Equal,
/// );
/// ```
pub struct FlatJaggedRowMut<'a, V, I, T>
where
    V: NVec<D1, T>,
    I: NVec<D1, usize>,
{
    pub(super) jagged: &'a mut FlatJagged<V, I, T>,
    pub(super) i: usize,
}
