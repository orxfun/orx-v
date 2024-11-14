use crate::{dim::*, nvec_core::NVecCore, NVecCoreSealed};

/// A `D` dimensional vector.
///
/// The objective of this trait to enable polymorphism over containers that are or
/// that are capable of behaving as contagious data structures with efficient random
/// access by indices of its elements.
///
/// [`V1`], [`V2`], etc. are type aliases for `NVec<D1, T>`, `NVec<D2, T>`, and so on.
///
/// # Motivation
///
/// The `NVec` trait mainly aims to support algorithms.
/// The goal is to **implement the algorithm once** without considering the underlying
/// type of the vectors, and it works for **all inputs** that behave like a multi-dimensional
/// vector without any loss of performance.
///
/// The most straightforward example is the standard `Vec<T>` or `&[T]`; they both
/// implement `V1<T>`.
/// And consequently, `Vec<Vec<T>>` implements `V2<T>`.
/// Actually, inner type can vary since `Vec<X>` implements `V2<T>` provided that `X`
/// implements `V1<T>`.
/// This might be the first example of polymorphism.
/// This trait aims at enabling any type that makes sense to implement the vector trait
/// with the corresponding dimension.
///
/// To elaborate the idea of polymorphism, consider an algorithm that requires a distance
/// matrix as its input such that the (i,j)-th element of the matrix represents the
/// distance from location i to location j. Our goal is to implement this algorithm once,
/// which works with different distance matrix representations without any loss of
/// performance.
///
/// Say our distance unit is `u32`, so our input distance matrix is `impl V2<u32>`.
///
/// ```ignore
/// fn algorithm(distance_matrix: impl V2<u32>) { ... }
/// ```
///
/// We want to be able to call `algorithm` with any input that makes sense as a distance
/// matrix.
///
/// Let's think about what can implement `V2<u32>`?
///
/// ## Dense
///
/// We can store all distances in a 2-dimensional structure, `Vec<Vec<u32>>` for instance.
/// As mentioned above, `Vec<Vec<u32>>` implements `V2<u32>`, so this works.
///
/// ```ignore
/// let dense_matrix = vec![vec![0, 2, 3], vec![3, 0, 7], vec![2, 2, 0]];
/// algorithm(&dense_matrix);
/// ```
///
/// ## Flattened Dense
///
/// Sometimes it is advantageous to flatten matrices and treat the 1-dimensional structure
/// as a 2-dimensional structure.
///
/// We can also treat a `V1<T>` as a jagged `V2<T>` by providing the additional information
/// about the column lengths which can be any `V1<usize>`.
///
/// ```ignore
/// let flat_storage = vec![0, 2, 3, 3, 0, 7, 2, 2, 0];
/// let row_end_indices = V.d1().fun(|[i]| 3 * (i + 1)).bounded(3);
/// let flat_dense_matrix = flat_storage.as_jagged(&row_end_indices);
/// algorithm(&flat_dense_matrix);
/// ```
///
/// ## Constant
///
/// Sometimes we have scenarios where all entries of the matrix are identical to some value.
/// This is often very handy in input polymorphism.
/// * Consider the shortest distance problem.
///   When, all distances are equal to 1 (`d[i, j] = 1`, for all i, j),
///   solution of the same problem represents the
///   minimum number of arcs to reach the destination.
/// * Consider the minimum cost flow problem.
///   When all edges have infinite capacity (`cap[i, j] = INF`, for all i, j),
///   and when we send one unit of flow from s to t,
///   the solution represents the shortest path from s to t.
///
/// In order to use the `algorithm`, should we create an n-by-n storage of the same values?
/// Not necessarily.
///
/// ```ignore
/// let all_ones = V.d2().constant(1).with_rectangular_bounds([3, 3]);
/// algorithm(&all_ones);
/// ```
///
/// ## Sparse
///
/// Quite often, we run into a situation where our vector is sparse and we are worried about
/// the wasted memory. Consider, for instance, that only certain locations are reachable from
/// one location, and hence, only those pairs have distance values. We assume that the distances
/// between all other locations are `u32::MAX`.
///
/// Should we fill an n-by-n storage where almost all elements are equal to `u32::MAX`?
/// Not necessarily.
///
/// ```ignore
/// let mut sparse_matrix = V.d2().sparse(u32::MAX);
/// *sparse_matrix.at_mut([0, 1]) = 10;
/// *sparse_matrix.at_mut([1, 2]) = 7;
/// *sparse_matrix.at_mut([1, 3]) = 5;
/// *sparse_matrix.at_mut([3, 8]) = 60;
/// assert_eq!(sparse_matrix.lookup_len(), 4);
/// algorithm(&sparse_matrix);
/// ```
///
/// ## Functional
///
/// Sometimes, we do not prefer to store any distances at all. The reasons might vary.
/// For instance,
/// * computing the element might be so cheap that it doesn't make sense to store and look up,
/// * the size of the matrix is critically large given the hardware, we prefer to pay the
///   computational price of re-calculating the element every time it is requested.
///
/// In these cases, we can simply provide the function that computes the element.
///
/// ```ignore
/// struct Location(u32, u32);
/// fn euclidean(a: &Location, b: &Location) -> u32 {
///     (((a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1)) as f64).sqrt() as u32
/// }
/// let locations = vec![Location(0, 3), Location(3, 2), Location(4, 1)];
/// let euclidean_matrix = V.d2().fun(|[i, j]| euclidean(&locations[i], &locations[j]));
/// algorithm(&euclidean_matrix);
/// ```
///
/// ## Cached
///
/// Sometimes, we do not prefer to store distances but due to a different reason.
/// * The matrix might be huge, while the algorithm accesses only a small subset of elements.
/// * But we don't necessarily know ahead of time which elements will be accessed.
/// * Instead of pre-computing the entire matrix, we can compute elements on demand.
/// * However, unlike the Euclidean example above, the computation of the element might be
///   relatively expensive.
/// * In order to avoid repeating the computation of the element on repeated accesses,
///   we cache the computed elements.
///
/// We can achieve this by simply calling `into_cached` on a functional vector.
///
/// ```ignore
/// struct Address(/* coordinates */);
/// fn distance_api(a: &Address, b: &Address) -> u32 {
///     todo!("make an api call to get the shortest distance wrt routing on the road network")
/// }
/// let addresses = vec![Address(), Address(), Address()];
/// let cached_distances = V
///     .d2()
///     .fun(|[i, j]| distance_api(&addresses[i], &addresses[j]))
///     .into_cached();
///
/// assert_eq!(cached_distances.cache_len(), 0); // cache is initially empty
/// algorithm(&cached_distances);
/// ```
///
/// [`V1`]: crate::V1
/// [`V2`]: crate::V2
pub trait NVec<D: Dim, T>: NVecCore<D, T> {
    // required

    /// Returns the element at the `idx`-th position of the vector.
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
    /// let vec = vec![
    ///     vec![0, 1, 2],
    ///     vec![3],
    ///     vec![4, 5],
    /// ];
    ///
    /// assert_eq!(vec.at([0, 1]), 1);
    /// assert_eq!(vec.at([1, 0]), 3);
    /// assert_eq!(vec.at([2, 1]), 5);
    ///
    /// // vec.at([1, 1]); // panics!
    /// assert_eq!(vec.try_at([1, 1]), None);
    /// ```
    fn at(&self, idx: impl IntoIdx<D>) -> T;

    /// Returns the `i`-th child of the vector.
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
    /// let vec = vec![
    ///     vec![0, 1, 2],
    ///     vec![3],
    ///     vec![4, 5],
    /// ];
    ///
    /// // child is a D1 vec
    /// let child = vec.child(2);
    ///
    /// assert_eq!(child.num_children(), 2);
    ///
    /// assert_eq!(child.at([0]), 4);
    /// assert_eq!(child.at([1]), 5);
    /// ```
    fn child(&self, i: D::ChildIdx) -> impl NVec<D::PrevDim, T>;

    /// Returns a flattened iterator over all scalar (D0) elements of the vector.
    ///
    /// In this sense, `all` can be considered similar to recursively
    /// called flat map on higher dimensional collections.
    ///
    /// See [`all_in`] for creating an iterator over a given domain.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_v::*;
    ///
    /// let vec = vec![
    ///     vec![0, 1],
    ///     vec![],
    ///     vec![2],
    /// ];
    ///
    /// let mut all = vec.all();
    /// assert_eq!(all.next(), Some(0));
    /// assert_eq!(all.next(), Some(1));
    /// assert_eq!(all.next(), Some(2));
    /// assert_eq!(all.next(), None);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the vector [`is_unbounded`] as this will lead to an infinite loop.
    ///
    /// The following are examples for unbounded vectors:
    /// * [`ConstantVec`] => `V.d1().constant(42)`
    /// * [`SparseVec`] => `V.d2().sparse(42)`
    /// * [`FunVec`] => `V.d2().fun(|[i, j]| i + 42 * j)`
    ///
    /// Finite domain of these vectors can be set by calling:
    /// * `bounded` for [`D1`] vectors,
    /// * `with_rectangular_bounds` or `with_variable_bounds` for higher dimensional vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v1 = V.d1().constant(42).bounded(4);
    /// assert_eq!(
    ///     v1.all().collect::<Vec<_>>(),
    ///     vec![42, 42, 42, 42],
    /// );
    ///
    /// let mut v2 = V.d2().sparse(0).with_rectangular_bounds([2, 3]);
    /// *v2.at_mut([0, 2]) = 42;
    /// assert_eq!(
    ///     v2.all().collect::<Vec<_>>(),
    ///     vec![0, 0, 42, 0, 0, 0],
    /// );
    ///
    /// let num_cols = [1, 0, 2, 1];
    /// let v2 = V.d2().fun(|[i, j]| 10 * i + j).with_variable_bounds(&num_cols);
    /// assert_eq!(
    ///     v2.equality(&[vec![0], vec![], vec![20, 21], vec![30]]),
    ///     Equality::Equal,
    /// );
    /// assert_eq!(
    ///     v2.all().collect::<Vec<_>>(),
    ///     vec![0, 20, 21, 30],
    /// );
    /// ```
    ///
    /// [`all_in`]: crate::NVec::all_in
    /// [`is_unbounded`]: crate::NVec::is_unbounded
    /// [`ConstantVec`]: crate::ConstantVec
    /// [`SparseVec`]: crate::SparseVec
    /// [`FunVec`]: crate::FunVec
    /// [`D1`]: crate::D1
    fn all(&self) -> impl Iterator<Item = T>;

    // provided - nvec-card

    /// Returns the number of children of the vector; i.e., number of
    /// elements of the one lower dimension.
    ///
    /// If this vector is of dimension D2; `num_children` returns the
    /// number of D1 children (V1) of this vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_v::*;
    ///
    /// let vec = vec![
    ///     vec![0, 1, 2],
    ///     vec![3],
    ///     vec![4, 5],
    /// ];
    ///
    /// assert_eq!(vec.num_children(), 3);
    ///
    /// assert_eq!(vec.child(2).num_children(), 2);
    /// ```
    #[inline(always)]
    fn num_children(&self) -> usize {
        <Self as NVecCoreSealed<D, T>>::core_num_children(self)
    }

    /// Returns the cardinality of the vec in any of the lower dimensions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_v::*;
    ///
    /// let v2 = [
    ///     vec![0, 2, 3],
    ///     vec![1, 7],
    ///     vec![],
    ///     vec![10, 3, 4, 8],
    /// ];
    ///
    /// // outer-most cardinality
    /// assert_eq!(v2.card([]), 4);
    ///
    /// // cardinality of the first-degree child
    /// assert_eq!(v2.card([3]), 4);
    /// // equivalent to:
    /// assert_eq!(v2.child(3).card([]), 4);
    /// ```
    ///
    /// This logic works similarly for higher dimensions.
    ///
    /// ```rust
    /// use orx_v::*;
    ///
    /// let v4 = [
    ///     vec![ // 0
    ///         vec![ // 0, 0
    ///             vec![3],    // 0, 0, 0
    ///             vec![1, 2], // 0, 0, 1
    ///         ],
    ///         vec![ // 0, 1
    ///             vec![6, 7, 8], // 0, 1, 0
    ///             vec![],        // 0, 1, 1
    ///             vec![9],       // 0, 1, 2
    ///         ],
    ///     ]
    /// ];
    ///
    /// assert_eq!(v4.card([]), 1);
    /// assert_eq!(v4.card([0]), 2);
    /// assert_eq!(v4.card([0, 0]), 2);
    /// assert_eq!(v4.card([0, 0, 0]), 1);
    /// assert_eq!(v4.card([0, 0, 1]), 2);
    /// assert_eq!(v4.card([0, 1]), 3);
    /// assert_eq!(v4.card([0, 1, 0]), 3);
    /// assert_eq!(v4.card([0, 1, 1]), 0);
    /// assert_eq!(v4.card([0, 1, 2]), 1);
    /// ```
    #[inline(always)]
    fn card(&self, idx: impl Into<D::CardIdx>) -> usize {
        <Self as NVecCoreSealed<D, T>>::core_card(self, idx)
    }

    /// Returns whether or not the vector is bounded.
    ///
    /// The following are examples for unbounded vectors:
    /// * [`ConstantVec`] => `V.d1().constant(42)`
    /// * [`SparseVec`] => `V.d2().sparse(42)`
    /// * [`FunVec`] => `V.d2().fun(|[i, j]| i + 42 * j)`
    ///
    /// Finite domain of these vectors can be set by calling:
    /// * `bounded` for [`D1`] vectors,
    /// * `with_rectangular_bounds` or `with_variable_bounds` for higher dimensional vectors.
    ///
    /// # Example
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v2 = vec![
    ///     vec![0, 1],
    ///     vec![],
    ///     vec![2],
    /// ];
    /// assert!(v2.is_bounded());
    ///
    /// let v1: &[usize] = v2[0].as_slice();
    /// assert!(v1.is_bounded());
    ///
    /// // constant
    ///
    /// let v1 = V.d1().constant(42);
    /// assert!(v1.is_unbounded());
    ///
    /// let v1 = V.d1().constant(42).bounded(10);
    /// assert!(v1.is_bounded());
    ///
    /// // sparse
    ///
    /// let mut v2 = V.d2().sparse(0);
    /// *v2.at_mut([0, 2]) = 42;
    /// assert!(v2.is_unbounded());
    ///
    /// let mut v2 = V.d2().sparse(0).with_rectangular_bounds([2, 3]);
    /// *v2.at_mut([0, 2]) = 42;
    /// assert!(v2.is_bounded());
    ///
    /// // fun
    ///
    /// let v2 = V.d2().fun(|[i, j]| 10 * i + j);
    /// assert!(v2.is_unbounded());
    ///
    /// let num_cols = [1, 0, 2, 1];
    /// let v2 = V.d2().fun(|[i, j]| 10 * i + j).with_variable_bounds(&num_cols);
    /// assert!(v2.is_bounded());
    /// ```
    ///
    /// [`ConstantVec`]: crate::ConstantVec
    /// [`SparseVec`]: crate::SparseVec
    /// [`FunVec`]: crate::FunVec
    #[inline(always)]
    fn is_bounded(&self) -> bool {
        <Self as NVecCoreSealed<D, T>>::core_num_children(self) < usize::MAX
    }

    /// Returns whether or not the cardinalities of the vector are rectangular.
    /// A rectangular vector of dimension `D` has the same number of children
    /// at a given lower dimension for all indices.
    ///
    /// * All empty vectors are rectangular.
    /// * All `D1` vectors are dimensional.
    /// * Two and higher dimensional matrices are rectangular.
    ///
    /// # Examples
    ///
    /// You may see examples of rectangular vectors below.
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let vec = V.d3().empty::<i64>();
    /// assert!(vec.is_rectangular());
    ///
    /// let vec = vec![1, 3, 4];
    /// assert!(vec.is_rectangular());
    ///
    /// let vec = V.d4().constant(42).with_rectangular_bounds([1, 3, 2, 7]);
    /// assert!(vec.is_rectangular());
    ///
    /// let vec = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6],
    /// ];
    /// assert!(vec.is_rectangular());
    ///
    /// let vec = vec![
    ///     vec![
    ///         vec![1, 2, 3],
    ///     ],
    ///     vec![
    ///         vec![4, 5, 6],
    ///     ],
    /// ];
    /// assert!(vec.is_rectangular());
    ///
    /// let flat_vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
    /// let row_end_indices = V.d1().fun(|[i]| 4 * (i + 1)).bounded(2);
    /// let vec = flat_vec.as_jagged(&row_end_indices);
    /// assert!(vec.is_rectangular());
    ///
    /// let vec = V.d2().fun(|[i, j]| i + j).with_rectangular_bounds([4, 2]);
    /// assert!(vec.is_rectangular());
    /// ```
    ///
    /// And below are examples for the non-rectangular or jagged vectors.
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let lengths = vec![3, 2, 3];
    /// let vec = V.d2().constant(42).with_variable_bounds(lengths);
    /// assert!(!vec.is_rectangular());
    ///
    /// let vec = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6, 7],
    /// ];
    /// assert!(!vec.is_rectangular());
    ///
    /// let vec = vec![
    ///     vec![
    ///         vec![1, 2, 3],
    ///     ],
    ///     vec![
    ///         vec![4, 5, 6, 7],
    ///     ],
    /// ];
    /// assert!(!vec.is_rectangular());
    ///
    /// let vec = vec![
    ///     vec![
    ///         vec![1, 2, 3],
    ///     ],
    ///     vec![
    ///         vec![4, 5, 6],
    ///         vec![7, 8, 9],
    ///     ],
    /// ];
    /// assert!(!vec.is_rectangular());
    ///
    /// let flat_vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
    /// let row_end_indices = vec![2, 5, 8];
    /// let vec = flat_vec.as_jagged(&row_end_indices);
    /// assert!(!vec.is_rectangular());
    ///
    /// let card = vec![3, 2, 3];
    /// let vec = V.d2().fun(|[i, j]| i + j).with_variable_bounds(card);
    /// assert!(!vec.is_rectangular());
    /// ```
    fn is_rectangular(&self) -> bool {
        self.core_is_rectangular()
    }

    /// Returns whether or not the vector is unbounded.
    ///
    /// The following are examples for unbounded vectors:
    /// * [`ConstantVec`] => `V.d1().constant(42)`
    /// * [`SparseVec`] => `V.d2().sparse(42)`
    /// * [`FunVec`] => `V.d2().fun(|[i, j]| i + 42 * j)`
    ///
    /// Finite domain of these vectors can be set by calling:
    /// * `bounded` for [`D1`] vectors,
    /// * `with_rectangular_bounds` or `with_variable_bounds` for higher dimensional vectors.
    ///
    /// # Example
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v2 = vec![
    ///     vec![0, 1],
    ///     vec![],
    ///     vec![2],
    /// ];
    /// assert!(v2.is_bounded());
    ///
    /// let v1: &[usize] = v2[0].as_slice();
    /// assert!(v1.is_bounded());
    ///
    /// // constant
    ///
    /// let v1 = V.d1().constant(42);
    /// assert!(v1.is_unbounded());
    ///
    /// let v1 = V.d1().constant(42).bounded(10);
    /// assert!(v1.is_bounded());
    ///
    /// // sparse
    ///
    /// let mut v2 = V.d2().sparse(0);
    /// *v2.at_mut([0, 2]) = 42;
    /// assert!(v2.is_unbounded());
    ///
    /// let mut v2 = V.d2().sparse(0).with_rectangular_bounds([2, 3]);
    /// *v2.at_mut([0, 2]) = 42;
    /// assert!(v2.is_bounded());
    ///
    /// // fun
    ///
    /// let v2 = V.d2().fun(|[i, j]| 10 * i + j);
    /// assert!(v2.is_unbounded());
    ///
    /// let num_cols = [1, 0, 2, 1];
    /// let v2 = V.d2().fun(|[i, j]| 10 * i + j).with_variable_bounds(&num_cols);
    /// assert!(v2.is_bounded());
    /// ```
    ///
    /// [`ConstantVec`]: crate::ConstantVec
    /// [`SparseVec`]: crate::SparseVec
    /// [`FunVec`]: crate::FunVec
    #[inline(always)]
    fn is_unbounded(&self) -> bool {
        <Self as NVecCoreSealed<D, T>>::core_num_children(self) == usize::MAX
    }

    /// Returns whether or not the given `idx` is in bounds.
    ///
    /// Note that the index can be the same dimension as the vector
    /// or any of the lower dimensions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_v::*;
    ///
    /// let vec = vec![
    ///     vec![1, 2, 3],
    ///     vec![4]
    /// ];
    ///
    /// // d2
    /// assert_eq!(vec.in_bounds([0, 2]), true);  // element 3
    /// assert_eq!(vec.in_bounds([1, 0]), true);  // element 4
    /// assert_eq!(vec.in_bounds([1, 1]), false); // X
    /// assert_eq!(vec.in_bounds([2, 0]), false); // X
    ///
    /// // d1
    /// assert_eq!(vec.in_bounds([0]), true);  // V1 [1, 2, 3]
    /// assert_eq!(vec.in_bounds([1]), true);  // V1 [4]
    /// assert_eq!(vec.in_bounds([2]), false); // X
    ///
    /// // d0
    /// assert_eq!(vec.in_bounds([]), true); // V2 [[1, 2, 3], [4]]
    /// ```
    #[inline(always)]
    fn in_bounds(&self, idx: impl Into<D::LeqIdx>) -> bool {
        idx.into().in_leq_bounds(self)
    }

    /// Returns the cardinality equality of this vec with the `other`:
    /// * Returns [`CardEquality::Equal`] iff the cardinality of the structures and
    ///   all their corresponding children have equal cardinalities.
    /// * Returns [`CardEquality::Unequal`] if cardinalities do not agree at at least one
    ///   level. The tuple `(idx, card1, card2)` represents the following:
    ///   * `idx` is the place the inequality in cardinalities are observed;
    ///   * `card1` and `card2` are the unequal cardinalities at the given `idx` in the first and
    ///     second vectors, respectively.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_v::*;
    ///
    /// let a = vec![
    ///     vec![0, 1, 2],
    ///     vec![3, 4, 5, 6],
    /// ];
    ///
    /// let b1 = vec![
    ///     vec![7, 0, 3],
    ///     vec![5, 2, 9, 5],
    /// ];
    /// let b2 = vec![
    ///     vec![7, 0, 3],
    ///     vec![5, 2, 9, 5, 42],
    /// ];
    /// let b3 = vec![
    ///     vec![7, 0, 3],
    ///     vec![5, 2, 9, 5],
    ///     vec![],
    /// ];
    ///
    /// // cardinalities are equal
    /// assert_eq!(a.card_equality(&b1), CardEquality::Equal);
    ///
    /// // cardinalities of the 1-st-level children with index 1
    /// // (`a.child(1)`) are different (4 != 5)
    /// assert_eq!(
    ///     a.card_equality(&b2),
    ///     CardEquality::Unequal(IdxLeqD1::IdxD1([1]), 4, 5)
    /// );
    ///
    /// // outer-most cardinalities are different (2 != 3)
    /// assert_eq!(
    ///     a.card_equality(&b3),
    ///     CardEquality::Unequal(IdxLeqD1::IdxD0([]), 2, 3)
    /// );
    /// ```
    fn card_equality(&self, other: &impl NVec<D, T>) -> CardEquality<D> {
        D::CardIdx::card_equality(self, other)
    }

    // provided

    /// Returns the element at the `idx`-th position of the vector if the
    /// index is `in_bounds`; returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_v::*;
    ///
    /// let vec = vec![
    ///     vec![0, 1, 2],
    ///     vec![3],
    ///     vec![4, 5],
    /// ];
    ///
    /// assert_eq!(vec.try_at([0, 1]), Some(1));
    /// assert_eq!(vec.try_at([1, 0]), Some(3));
    /// assert_eq!(vec.try_at([2, 1]), Some(5));
    ///
    /// // vec.at([1, 1]); // panics!
    /// assert_eq!(vec.try_at([1, 1]), None);
    /// ```
    fn try_at(&self, idx: impl IntoIdx<D>) -> Option<T> {
        match D::in_bounds(idx, self) {
            true => Some(self.at(idx)),
            false => None,
        }
    }

    /// Returns the equality of this vec with the `other`:
    /// * Returns [`Equality::Equal`] iff the cardinality of the structures as
    ///   well as all values at corresponding positions are equal.
    /// * Returns [`Equality::UnequalCard`] if cardinalities do not agree at at least one
    ///   level. The tuple `(idx, card1, card2)` represents the following:
    ///   * `idx` is the place the inequality in cardinalities are observed;
    ///   * `card1` and `card2` are the unequal cardinalities at the given `idx` in the first and
    ///     second vectors, respectively.
    /// * Returns [`Equality::UnequalValue`] if any of the values are different.
    ///   The `(idx)` represents the index where the value inequality is observed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_v::*;
    ///
    /// let a = vec![
    ///     vec![0, 1, 2],
    ///     vec![3, 4, 5, 6],
    /// ];
    ///
    /// let b1 = vec![
    ///     vec![0, 1, 2],
    ///     vec![3, 4, 5, 6],
    /// ];
    /// let b2 = vec![
    ///     vec![0, 1, 2],
    ///     vec![3, 4, 42, 6],
    /// ];
    /// let b3 = vec![
    ///     vec![0, 1, 2],
    ///     vec![3, 4, 5, 6, 42],
    /// ];
    /// let b4 = vec![
    ///     vec![0, 1, 2],
    ///     vec![3, 4, 5, 6],
    ///     vec![42],
    /// ];
    ///
    /// // vectors are equal
    /// assert_eq!(a.equality(&b1), Equality::Equal);
    ///
    /// // values at [1, 2] are different
    /// assert_eq!(a.equality(&b2), Equality::UnequalValue([1, 2]));
    ///
    /// // cardinalities of the 1-st-level children with index 1
    /// // (`a.child(1)`) are different (3 != 4)
    /// assert_eq!(
    ///     a.equality(&b3),
    ///     Equality::UnequalCard(IdxLeqD1::IdxD1([1]), 4, 5)
    /// );
    ///
    /// // outer-most cardinalities are different (2 != 3)
    /// assert_eq!(
    ///     a.equality(&b4),
    ///     Equality::UnequalCard(IdxLeqD1::IdxD0([]), 2, 3)
    /// );
    /// ```
    fn equality(&self, other: &impl NVec<D, T>) -> Equality<D>
    where
        T: PartialEq,
    {
        D::CardIdx::equality(self, other)
    }

    /// Returns an iterator of all children of the vector.
    ///
    /// Note that child has a dimension that is one less than the dimension
    /// of this vector.
    ///
    /// ```rust
    /// use orx_v::*;
    ///
    /// // D2
    /// let vec = vec![
    ///     vec![0, 1, 2],
    ///     vec![3],
    ///     vec![4, 5],
    /// ];
    ///
    /// let mut children = vec.children();
    ///
    /// assert_eq!(children.next().unwrap().equality(&[0, 1, 2]), Equality::Equal);
    /// assert_eq!(children.next().unwrap().equality(&[3]), Equality::Equal);
    /// assert_eq!(children.next().unwrap().equality(&[4, 5]), Equality::Equal);
    /// assert!(children.next().is_none());
    /// ```
    fn children(&self) -> impl Iterator<Item = impl NVec<D::PrevDim, T>> {
        (0..self.core_num_children()).map(|i| self.child(i.into()))
    }

    /// Returns an iterator of elements for the given `indices`.
    ///
    /// This method is useful especially when the vector `is_unbounded` and we would
    /// like to iterate only over the given indices.
    ///
    /// # Panics
    ///
    /// Panics if any of the indices that `indices` iterator yields is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_v::*;
    ///
    /// let vec = V.d1().constant(42);
    /// assert!(vec.is_unbounded());
    /// let mut all = vec.all_in(0..3);
    /// assert_eq!(all.next(), Some(42));
    /// assert_eq!(all.next(), Some(42));
    /// assert_eq!(all.next(), Some(42));
    /// assert_eq!(all.next(), None);
    ///
    /// let vec = vec![
    ///     vec![0, 1],
    ///     vec![],
    ///     vec![2],
    /// ];
    ///
    /// let mut all = vec.all_in([[0, 1], [2, 0]].into_iter());
    /// assert_eq!(all.next(), Some(1));
    /// assert_eq!(all.next(), Some(2));
    /// assert_eq!(all.next(), None);
    /// ```
    fn all_in(&self, indices: impl Iterator<Item = impl IntoIdx<D>>) -> impl Iterator<Item = T> {
        indices.map(|idx| self.at(idx.into_idx()))
    }
}

// &V auto impl

impl<T, D: Dim, V: NVec<D, T>> NVec<D, T> for &V {
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D>) -> T {
        <V as NVec<D, T>>::at(self, idx)
    }

    #[inline(always)]
    fn child(&self, i: <D as Dim>::ChildIdx) -> impl NVec<<D as Dim>::PrevDim, T> {
        <V as NVec<D, T>>::child(self, i)
    }

    fn all(&self) -> impl Iterator<Item = T> {
        <V as NVec<D, T>>::all(self)
    }
}

// &mut V auto impl

impl<T, D: Dim, V: NVec<D, T>> NVec<D, T> for &mut V {
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D>) -> T {
        <V as NVec<D, T>>::at(self, idx)
    }

    #[inline(always)]
    fn child(&self, i: <D as Dim>::ChildIdx) -> impl NVec<<D as Dim>::PrevDim, T> {
        <V as NVec<D, T>>::child(self, i)
    }

    fn all(&self) -> impl Iterator<Item = T> {
        <V as NVec<D, T>>::all(self)
    }
}
