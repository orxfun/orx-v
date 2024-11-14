use super::jagged_row::FlatJaggedRowMut;
use crate::common_trait_helpers::debug::dbg_values_d2;
use crate::{Dim, NVec, D1, D2};
use core::fmt::Debug;
use core::marker::PhantomData;

/// A variable cardinality, jagged, `D2` vector represented by a tuple of a
/// flat `D1` storage and row end indices.
///
/// Flattening jagged vectors might be useful in improving cache locality.
///
/// # Examples
///
/// ```
/// use orx_v::*;
///
/// let row_end_indices = [1, 3, 3, 6];
/// let storage = [0, 1, 2, 3, 4, 5];
///
/// let jagged = storage.as_jagged(&row_end_indices);
/// assert_eq!(jagged.card([]), 4);
/// assert_eq!(jagged.card([0]), 1);
/// assert_eq!(jagged.card([1]), 2);
/// assert_eq!(jagged.card([2]), 0);
/// assert_eq!(jagged.card([3]), 3);
/// assert_eq!(
///     jagged.equality(&vec![vec![0], vec![1, 2], vec![], vec![3, 4, 5]]),
///     Equality::Equal,
/// );
/// ```
///
/// Note that row end indices can be any `V1<usize>`.
/// Alternatively, it might be created internally from row lengths.
///
/// ```
/// use orx_v::*;
///
/// let row_lengths = [1, 2, 0, 3];
/// let storage = [0, 1, 2, 3, 4, 5];
///
/// let jagged = storage.as_jagged_from_row_lengths(&row_lengths);
/// assert_eq!(jagged.card([]), 4);
/// assert_eq!(jagged.card([0]), 1);
/// assert_eq!(jagged.card([1]), 2);
/// assert_eq!(jagged.card([2]), 0);
/// assert_eq!(jagged.card([3]), 3);
/// assert_eq!(
///     jagged.equality(&vec![vec![0], vec![1, 2], vec![], vec![3, 4, 5]]),
///     Equality::Equal,
/// );
/// ```
pub struct FlatJagged<V, I, T>
where
    V: NVec<D1, T>,
    I: NVec<D1, usize>,
{
    pub(super) flat_vec: V,
    row_end_indices: I,
    phantom: PhantomData<T>,
}

impl<V, I, T> FlatJagged<V, I, T>
where
    V: NVec<D1, T>,
    I: NVec<D1, usize>,
{
    pub(super) fn new(flat_vec: V, row_end_indices: I) -> Self {
        Self {
            flat_vec,
            row_end_indices,
            phantom: PhantomData,
        }
    }
}

impl<V, I, T> Debug for FlatJagged<V, I, T>
where
    T: Debug,
    V: NVec<D1, T>,
    I: NVec<D1, usize>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{ kind: FlatJagged, dim: D2, values: ",)?;
        dbg_values_d2(f, self)?;
        write!(f, " }}")
    }
}

impl<V, I, T> FlatJagged<V, I, T>
where
    V: NVec<D1, T>,
    I: NVec<D1, usize>,
{
    /// Transforms the flat-jagged vec into two of its underlying `V1`s:
    /// * `flat_vec` which is the flattened storage of the jagged vec, and
    /// * `row_end_indices` which is the scan of row lengths.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let row_lengths = [0, 3, 2, 1];
    /// let storage = [0, 1, 2, 3, 4, 5];
    ///
    /// let jagged = storage.as_jagged_from_row_lengths(&row_lengths);
    /// assert_eq!(jagged.card([]), 4);
    /// assert_eq!(jagged.card([0]), 0);
    /// assert_eq!(jagged.card([1]), 3);
    /// assert_eq!(jagged.card([2]), 2);
    /// assert_eq!(jagged.card([3]), 1);
    /// assert_eq!(
    ///     jagged.equality(&vec![vec![], vec![0, 1, 2], vec![3, 4], vec![5]]),
    ///     Equality::Equal,
    /// );
    ///
    /// let (v1, end_indices) = jagged.into_inner();
    /// assert_eq!(
    ///     v1.equality(&[0, 1, 2, 3, 4, 5]),
    ///     Equality::Equal
    /// );
    /// assert_eq!(
    ///     end_indices.equality(&[0, 3, 5, 6]),
    ///     Equality::Equal
    /// );
    /// ```
    pub fn into_inner(self) -> (V, I) {
        (self.flat_vec, self.row_end_indices)
    }

    // helpers
    #[inline(always)]
    pub(super) fn num_rows(&self) -> usize {
        self.row_end_indices.num_children()
    }

    pub(super) fn to_d1_idx(&self, idx: <D2 as Dim>::Idx) -> usize {
        let [i, j] = idx;
        let begin = match i {
            0 => 0,
            x if x >= self.num_rows() => panic_oob_idx_i(idx, self.num_rows()),
            _ => self.row_end_indices.at(i - 1),
        };

        let end = self.row_end_indices.at(i);

        let flat_idx = begin + j;

        match flat_idx < end {
            true => flat_idx,
            false => panic_oob_idx_j(idx, i, end),
        }
    }

    pub(super) fn row_range(&self, i: usize) -> (usize, usize) {
        let begin = match i {
            0 => 0,
            x if x >= self.num_rows() => panic_oob_i(i, self.num_rows()),
            _ => self.row_end_indices.at(i - 1),
        };

        let end = self.row_end_indices.at(i);
        assert!(begin<=end, "Invalid row end indices of the FlatJagged; elements of the row end indices must be non-decreasing");
        (begin, end)
    }

    pub(super) fn row_mut(&mut self, i: usize) -> FlatJaggedRowMut<V, I, T> {
        match i < self.num_rows() {
            true => FlatJaggedRowMut { jagged: self, i },
            false => panic_oob_i(i, self.num_rows()),
        }
    }

    pub(super) fn is_rectangular(&self) -> bool {
        match self.row_end_indices.card([]) {
            0 => true,
            n => {
                let m = self.row_end_indices.at([0]);
                let mut end = m;
                for i in 1..n {
                    end += m;
                    if self.row_end_indices.at([i]) != end {
                        return false;
                    }
                }
                true
            }
        }
    }
}

// errors

#[allow(clippy::panic)]
fn panic_oob_i(i: usize, num_rows: usize) -> ! {
    panic!(
        "FlatJagged row-idx {} is out of bounds: jagged array has {} rows",
        i, num_rows
    );
}

#[allow(clippy::panic)]
fn panic_oob_idx_i(idx: <D2 as Dim>::Idx, num_rows: usize) -> ! {
    panic!(
        "FlatJagged idx {:?} is out of bounds: jagged array has {} rows",
        idx, num_rows
    );
}

#[allow(clippy::panic)]
fn panic_oob_idx_j(idx: <D2 as Dim>::Idx, i: usize, end: usize) -> ! {
    panic!(
        "FlatJagged idx {:?} is out of bounds: row {} has {} columns",
        idx, i, end
    );
}
