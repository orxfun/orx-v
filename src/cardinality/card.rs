use crate::{Dim, NVec};

/// Cardinality of a vector.
///
/// Note that cardinality of a vector of dimension `D` is capable of providing the
/// number of elements of all lower dimension children of the vector.
///
/// Consider, for instance a jagged vector of dimension `D2`, say `v2`, including
/// two vectors of lengths 4 and 6. Then its cardinality is capable of creating all
/// relevant cardinality information:
/// * `v.card([])` is 2
/// * `v.card([0])` is 4
/// * `v.card([1])` is 6
pub trait Card<D: Dim> {
    /// Returns true if the cardinality is bounded and rectangular; i.e,
    /// * children of a particular dimension have the same number of children.
    ///
    /// The following are example multi-dimensional vectors with rectangular cardinality:
    /// * All `D1` vectors.
    /// * `D2` vector representing n-by-m matrices such that `card([])` is n and `card([i])`
    ///   is m for all i in 0..n.
    /// * Similarly, higher dimensional vectors representing higher dimensional matrices
    ///   such as a `D3` vector representing n-by-m-by-p matrix such that `card([])` is n,
    ///   `card([i])` is m for all i, and `card([i, j])` os p for all (i,j).
    /// * Empty vector of any dimension.
    fn is_rectangular(&self) -> bool;

    /// Returns the cardinality of the child of the vector at the given `idx`.
    fn cardinality_of(&self, idx: impl Into<D::CardIdx>) -> usize;

    /// Returns the cardinality of the child of this vector at the given
    /// `left_most_idx`.
    fn child_card(&self, left_most_idx: D::ChildIdx) -> impl Card<D::PrevDim>;

    /// Creates a function, say `new_fun`, which applies the first of the indices to
    /// `left_most_index` such that:
    ///
    /// `new_fun([i, j, k])` returns `fun([left_most_idx, i, j, k])`
    fn child_fun<T, F>(
        &self,
        left_most_idx: D::ChildIdx,
        fun: F,
    ) -> impl Fn(<D::PrevDim as Dim>::Idx) -> T
    where
        F: Fn(D::Idx) -> T;

    /// Returns an iterator over all elements of the `vec` provided that this is its
    /// cardinality.
    fn vec_all<'a, T, N>(&'a self, vec: &'a N) -> impl Iterator<Item = T>
    where
        N: NVec<D, T> + 'a;

    /// Returns an iterator over all elements of the `vec` together with their corresponding
    /// indices provided that this is its cardinality.
    fn vec_enumerate_all<'a, T, N>(&'a self, vec: &'a N) -> impl Iterator<Item = (D::Idx, T)>
    where
        N: NVec<D, T> + 'a;
}

// impl ref

impl<D: Dim, C: Card<D>> Card<D> for &C {
    fn is_rectangular(&self) -> bool {
        <C as Card<D>>::is_rectangular(self)
    }

    fn cardinality_of(&self, idx: impl Into<<D as Dim>::CardIdx>) -> usize {
        <C as Card<D>>::cardinality_of(self, idx)
    }

    fn child_card(&self, left_most_idx: D::ChildIdx) -> impl Card<<D as Dim>::PrevDim> {
        <C as Card<D>>::child_card(self, left_most_idx)
    }

    fn child_fun<T, F>(
        &self,
        left_most_idx: D::ChildIdx,
        fun: F,
    ) -> impl Fn(<<D as Dim>::PrevDim as Dim>::Idx) -> T
    where
        F: Fn(<D as Dim>::Idx) -> T,
    {
        <C as Card<D>>::child_fun(self, left_most_idx, fun)
    }

    fn vec_all<'a, T, N>(&'a self, vec: &'a N) -> impl Iterator<Item = T>
    where
        N: NVec<D, T> + 'a,
    {
        <C as Card<D>>::vec_all(self, vec)
    }

    fn vec_enumerate_all<'a, T, N>(
        &'a self,
        vec: &'a N,
    ) -> impl Iterator<Item = (<D as Dim>::Idx, T)>
    where
        N: NVec<D, T> + 'a,
    {
        <C as Card<D>>::vec_enumerate_all(self, vec)
    }
}

// helpers

pub(super) fn child_fun_unchecked<D, T, F>(
    i: D::ChildIdx,
    fun: F,
) -> impl Fn(<D::PrevDim as Dim>::Idx) -> T
where
    D: Dim,
    F: Fn(D::Idx) -> T,
{
    let i: usize = i.into();
    move |idx: <D::PrevDim as Dim>::Idx| {
        let idx = D::left_join_from_lower_dim(i, idx);
        (fun)(idx)
    }
}

#[allow(clippy::panic)]
pub(crate) fn panic_on_all_when_udd(is_udd: bool) {
    if is_udd {
        panic!(
            "Called `all` or `enumerate_all` on an unbounded vector. \
        You may use `all_in` to iterate over explicit indices. \
        Alternatively, you may transform the vector into a bounded vector by \
        calling `bounded`, `with_rectangular_bounds` or `with_variable_bounds` \
        methods."
        );
    }
}
