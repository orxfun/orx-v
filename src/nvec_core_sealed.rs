use crate::dim::*;
use core::fmt::Debug;

pub trait NVecCoreSealed<D: Dim, T>: Sized {
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
    fn core_num_children(&self) -> usize;

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
    fn core_card(&self, idx: impl Into<D::CardIdx>) -> usize;

    fn core_child(&self, i: D::ChildIdx) -> impl NVecCoreSealed<D::PrevDim, T>;

    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D>, f: &mut F) -> O;

    fn core_is_rectangular(&self) -> bool;

    // provided
    #[inline(always)]
    fn core_is_bounded(&self) -> bool {
        self.core_num_children() < usize::MAX
    }

    fn core_dbg_at(
        &self,
        idx: impl IntoIdx<D>,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result
    where
        T: Debug,
    {
        let mut fun = |x: &T| write!(f, "{:?}", x);
        self.core_map(idx, &mut fun)
    }
}

impl<D: Dim, V: NVecCoreSealed<D, T>, T> NVecCoreSealed<D, T> for &V {
    fn core_num_children(&self) -> usize {
        <V as NVecCoreSealed<D, T>>::core_num_children(self)
    }

    fn core_card(&self, idx: impl Into<<D as Dim>::CardIdx>) -> usize {
        <V as NVecCoreSealed<D, T>>::core_card(self, idx)
    }

    fn core_child(&self, i: <D as Dim>::ChildIdx) -> impl NVecCoreSealed<<D as Dim>::PrevDim, T> {
        <V as NVecCoreSealed<D, T>>::core_child(self, i)
    }

    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D>, f: &mut F) -> O {
        <V as NVecCoreSealed<D, T>>::core_map(self, idx, f)
    }

    fn core_is_rectangular(&self) -> bool {
        <V as NVecCoreSealed<D, T>>::core_is_rectangular(self)
    }
}

impl<D: Dim, V: NVecCoreSealed<D, T>, T> NVecCoreSealed<D, T> for &mut V {
    fn core_num_children(&self) -> usize {
        <V as NVecCoreSealed<D, T>>::core_num_children(self)
    }

    fn core_card(&self, idx: impl Into<<D as Dim>::CardIdx>) -> usize {
        <V as NVecCoreSealed<D, T>>::core_card(self, idx)
    }

    fn core_child(&self, i: <D as Dim>::ChildIdx) -> impl NVecCoreSealed<<D as Dim>::PrevDim, T> {
        <V as NVecCoreSealed<D, T>>::core_child(self, i)
    }

    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D>, f: &mut F) -> O {
        <V as NVecCoreSealed<D, T>>::core_map(self, idx, f)
    }

    fn core_is_rectangular(&self) -> bool {
        <V as NVecCoreSealed<D, T>>::core_is_rectangular(self)
    }
}
