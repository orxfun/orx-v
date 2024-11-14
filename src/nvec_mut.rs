use crate::{
    dim::{Dim, IntoIdx},
    nvec::NVec,
};

/// A `D` dimensional mutable vector.
///
/// [`V1Mut`], [`V2Mut`], etc. are type aliases for `NVecMut<D1, T>`, `NVecMut<D2, T>`, and so on.
///
/// [`V1Mut`]: crate::V1Mut
/// [`V2Mut`]: crate::V2Mut
pub trait NVecMut<D: Dim, T>: NVec<D, T> {
    // required

    /// Returns a mutable reference to the element at the `idx`-th
    /// position of the vector.
    ///
    /// Note that the dimensions of the vector and the index are equal;
    /// and hence, the result is the scalar.
    ///
    /// # Panics
    ///
    /// Panics if the `idx` is not `in_bounds`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_v::*;
    ///
    /// let mut vec = vec![
    ///     vec![0, 1, 2],
    ///     vec![3],
    ///     vec![4, 5],
    /// ];
    ///
    /// *vec.at_mut([0, 1]) = 42;
    /// *vec.at_mut([2, 0]) = 7;
    ///
    /// assert_eq!(
    ///     vec.equality(&[vec![0, 42, 2], vec![3], vec![7, 5]]),
    ///     Equality::Equal
    /// );
    /// ```
    fn at_mut<Idx: IntoIdx<D>>(&mut self, idx: Idx) -> &mut T;

    /// Sets `value`of the element at the `idx`-th position of the vector.
    ///
    /// Note that the dimensions of the vector and the index are equal;
    /// and hence, the method sets value of the scalar.
    ///
    /// # Panics
    ///
    /// Panics if the `idx` is not `in_bounds`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_v::*;
    ///
    /// let mut vec = vec![
    ///     vec![0, 1, 2],
    ///     vec![3],
    ///     vec![4, 5],
    /// ];
    ///
    /// vec.set([0, 1], 42);
    /// vec.set([2, 0], 7);
    ///
    /// assert_eq!(
    ///     vec.equality(&[vec![0, 42, 2], vec![3], vec![7, 5]]),
    ///     Equality::Equal
    /// );
    /// ```
    fn set<Idx: IntoIdx<D>>(&mut self, idx: Idx, value: T) {
        *self.at_mut(idx) = value;
    }

    /// Returns a mutable reference to the `i`-th child of the vector.
    ///
    /// Note that child has a dimension that is one less than the dimension
    /// of this vector.
    ///
    /// # Panics
    ///
    /// Panics if `i` is out of bounds; i.e., `i >= vec.num_children()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_v::*;
    ///
    /// // D2
    /// let mut vec = vec![
    ///     vec![0, 1, 2],
    ///     vec![3],
    ///     vec![4, 5],
    /// ];
    ///
    /// {
    ///     // child is a D1 vec
    ///     let mut child = vec.child_mut(2);
    ///
    ///     *child.at_mut([0]) = 42;
    ///     child.set([1], 7);
    /// }
    ///
    /// assert_eq!(
    ///     vec.equality(&[vec![0, 1, 2], vec![3], vec![42, 7]]),
    ///     Equality::Equal
    /// );
    /// ```
    fn child_mut(&mut self, i: D::ChildIdx) -> impl NVecMut<D::PrevDim, T>;

    /// Applies the mutating function `f` over all scalar elements of the vector.
    ///
    /// # Example
    /// ```
    /// use orx_v::*;
    ///
    /// let mut v1 = [3, 6, 12];
    /// v1.mut_all(|x| *x *= 10);
    /// assert_eq!(v1.equality(&[30, 60, 120]), Equality::Equal);
    ///
    /// let mut v2 = vec![
    ///     vec![1, 2],
    ///     vec![3, 4],
    ///     vec![5, 6],
    /// ];
    /// v2.mut_all(|x| *x *= 10);
    /// assert_eq!(
    ///     v2.equality(&[[10, 20], [30, 40], [50, 60]]),
    ///     Equality::Equal,
    /// );
    /// ```
    fn mut_all<F>(&mut self, f: F)
    where
        F: FnMut(&mut T);

    /// Sets all elements of the vector to the given `value`.
    /// This method is often used at initialization stage of algorithms.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let mut v1 = vec![1, 2, 3, 4, 5];
    /// v1.reset_all(0);
    /// assert_eq!(
    ///     v1.equality(&[0, 0, 0, 0, 0]),
    ///     Equality::Equal,
    /// );
    ///
    /// let mut v2 =[
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// ];
    /// v2.reset_all(42);
    /// assert_eq!(
    ///     v2.equality(&[[42, 42, 42], [42, 42, 42]]),
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
    /// position of the vector if the index is `in_bounds`;
    /// returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_v::*;
    ///
    /// let mut vec = vec![
    ///     vec![0, 1, 2],
    ///     vec![3],
    ///     vec![4, 5],
    /// ];
    ///
    /// *vec.try_at_mut([0, 1]).unwrap() = 42;
    /// assert_eq!(vec.at([0, 1]), 42);
    ///
    /// // vec.at_mut([1, 1]); // panics!
    /// assert_eq!(vec.try_at_mut([1, 1]), None);
    /// ```
    fn try_at_mut(&mut self, idx: impl IntoIdx<D>) -> Option<&mut T> {
        match D::in_bounds(idx, self) {
            true => Some(self.at_mut(idx)),
            false => None,
        }
    }
}

// &mut V auto impl

impl<T, D: Dim, V: NVecMut<D, T>> NVecMut<D, T> for &mut V {
    fn at_mut<Idx: IntoIdx<D>>(&mut self, idx: Idx) -> &mut T {
        <V as NVecMut<D, T>>::at_mut(self, idx)
    }

    fn set<Idx: IntoIdx<D>>(&mut self, idx: Idx, value: T) {
        <V as NVecMut<D, T>>::set(self, idx, value);
    }

    fn child_mut(&mut self, i: <D as Dim>::ChildIdx) -> impl NVecMut<<D as Dim>::PrevDim, T> {
        <V as NVecMut<D, T>>::child_mut(self, i)
    }

    fn mut_all<F>(&mut self, f: F)
    where
        F: FnMut(&mut T),
    {
        <V as NVecMut<D, T>>::mut_all(self, f);
    }

    fn reset_all(&mut self, value: T)
    where
        T: PartialEq + Copy,
    {
        <V as NVecMut<D, T>>::reset_all(self, value);
    }
}
