use super::Matrix;
use crate::{IntoIdx, D2};

/// A mutable matrix view over a `D2` vector with rectangular cardinality,
/// or over a flattened representation by a `D1` vector.
///
/// A mutable matrix view can be created by:
/// * calling [`as_matrix_mut`] or [`as_matrix_col_major_mut`] methods on a `D2`
///   vector implementing `NVecMut<D2, _>`, or equivalently, `V2Mut<_>`; or by:
/// * calling [`as_matrix_mut`] or [`as_matrix_col_major_mut`] methods on a `D1`
///   vector implementing `NVecMut<D1, _>`, or equivalently, `V1Mut<_>`.
///
/// Alternatively an owned mutable matrix can be crated by
/// * calling [`into_matrix`] method on a `D2` mutable vector; or by:
/// * calling [`v1_into_matrix`] method on a `D1` mutable vector.
///
/// [`into_matrix`]: crate::V2AsMatrix::into_matrix
/// [`as_matrix_mut`]: crate::V2AsMatrix::as_matrix_mut
/// [`as_matrix_col_major_mut`]: crate::V2AsMatrix::as_matrix_col_major_mut
/// [`v1_into_matrix`]: crate::V1AsMatrix::v1_into_matrix
/// [`as_matrix_mut`]: crate::V1AsMatrix::as_matrix_mut
/// [`as_matrix_col_major_mut`]: crate::V1AsMatrix::as_matrix_col_major_mut
pub trait MatrixMut<T>: Matrix<T> {
    /// Returns a mutable reference to the element at the given `idx` of the matrix.
    ///
    /// # Panics
    ///
    /// Panics if the `idx` is not `in_bounds`.
    fn at_mut<Idx: IntoIdx<D2>>(&mut self, idx: Idx) -> &mut T;

    /// Sets the element at the given `idx` of the matrix to the `value`.
    ///
    /// # Panics
    ///
    /// Panics if the `idx` is not `in_bounds`.
    fn set<Idx: IntoIdx<D2>>(&mut self, idx: Idx, value: T) {
        *self.at_mut(idx) = value;
    }

    /// Applies the mutating function `f` over all elements of the matrix.
    fn mut_all<F>(&mut self, f: F)
    where
        F: FnMut(&mut T);

    /// Sets all elements of the matrix to the given `value`.
    /// This method is often used at initialization stage of algorithms.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let mut v2 =[
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// ];
    /// let mut mat = v2.as_matrix_mut();
    /// mat.reset_all(42);
    /// assert_eq!(
    ///     mat.equality(&[[42, 42, 42], [42, 42, 42]].as_matrix()),
    ///     Equality::<D2>::Equal,
    /// );
    /// ```
    ///
    /// Or more practically, consider the Dijkstra's shortest path algorithm as implemented in the
    /// [Algorithms](https://github.com/TianyiShi2001/Algorithms/blob/main/src/graph/shortest_path/dijkstra.rs)
    /// repository.
    ///
    /// In addition to a priority queue, the implementation uses a distances vector throughout the
    /// search. One way to avoid re-allocation of such internal data, we often cache and reuse them.
    /// In such a case, we need to set all elements of this vector to infinity on initialization,
    /// where `reset_all` method is useful.
    ///
    /// ```ignore
    /// use orx_v::*;
    ///
    /// impl WeightedAdjacencyList {
    ///     fn dijkstra(&mut self, start: usize, end: usize) -> Option<(f64, Vec<usize>)> {
    ///         // initialization
    ///         self.distances.reset_all(f64::INFINITY);
    ///         ...
    ///
    ///         // search
    ///         while let Some((node, cur_dist)) = pq.pop() {
    ///             ...
    ///         }
    ///         ...
    ///     }
    /// }
    /// ```
    fn reset_all(&mut self, value: T)
    where
        T: PartialEq + Copy;

    // provided

    /// Returns a mutable reference to the element at the `idx`-th
    /// position of the matrix if the index is `in_bounds`;
    /// returns None otherwise.
    fn try_at_mut(&mut self, idx: impl IntoIdx<D2>) -> Option<&mut T> {
        let [i, j] = idx.into_idx();
        match i < self.num_rows() && j < self.num_cols() {
            true => Some(self.at_mut(idx)),
            false => None,
        }
    }
}

// &mut V auto impl

impl<T, M: MatrixMut<T>> MatrixMut<T> for &mut M {
    fn at_mut<Idx: IntoIdx<D2>>(&mut self, idx: Idx) -> &mut T {
        <M as MatrixMut<T>>::at_mut(self, idx)
    }

    fn mut_all<F>(&mut self, f: F)
    where
        F: FnMut(&mut T),
    {
        <M as MatrixMut<T>>::mut_all(self, f);
    }

    fn reset_all(&mut self, value: T)
    where
        T: PartialEq + Copy,
    {
        <M as MatrixMut<T>>::reset_all(self, value);
    }
}
