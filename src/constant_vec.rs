use crate::common_trait_helpers::debug::*;
use crate::{
    dim::*, Card, CardD1, IntoIdx, NVec, NVecCoreSealed, RectangularCardD2, RectangularCardD3,
    RectangularCardD4, UnboundedCard, VariableCardD2, VariableCardD3, VariableCardD4, V1, V2, V3,
};
use core::fmt::Debug;
use core::marker::PhantomData;

/// A constant vector which returns the same value for any input index.
///
/// # Example
///
/// ```
/// use orx_v::*;
///
/// let v1 = V.d1().constant(42).bounded(4);
///
/// assert_eq!(v1.at([2]), 42);
/// assert_eq!(
///     v1.equality(&[42, 42, 42, 42]),
///     Equality::Equal
/// );
///
/// let v2 = V.d2().constant(42).with_rectangular_bounds([2, 3]);
/// assert_eq!(v2.at([0, 2]), 42);
/// assert_eq!(v2.at([1, 0]), 42);
/// assert_eq!(
///     v2.equality(&vec![vec![42, 42, 42], vec![42, 42, 42]]),
///     Equality::Equal
/// );
/// ```
#[derive(Clone, Copy)]
pub struct ConstantVec<D, T, C = UnboundedCard<D>>
where
    D: Dim,
    T: Copy,
    C: Card<D>,
{
    value: T,
    card: C,
    phantom: PhantomData<D>,
}

macro_rules! impl_debug {
    ($dim:ty, $dbg_fn:ident) => {
        impl<T, C> Debug for ConstantVec<$dim, T, C>
        where
            T: Copy + Debug,
            C: Card<$dim>,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(
                    f,
                    "{{ kind: ConstantVec, dim: D{}, is_bounded: {}, values: ",
                    <$dim as Dim>::dimension(),
                    self.is_bounded(),
                )?;
                $dbg_fn(f, self)?;
                write!(f, " }}")
            }
        }
    };
}

impl_debug!(D1, dbg_values_d1);
impl_debug!(D2, dbg_values_d2);
impl_debug!(D3, dbg_values_d3);
impl_debug!(D4, dbg_values_d4);

impl<D, T, C> ConstantVec<D, T, C>
where
    D: Dim,
    T: Copy,
    C: Card<D>,
{
    /// Creates a new constant vector with the given `card` where all elements are
    /// equal to `value`.
    ///
    /// Alternatively, unbounded constant vectors of different dimensions can be
    /// created by `V.d1().constant(42)`, `V.d2().constant(42)`, etc., which can
    /// later be transformed into bounded vectors by applying `bounded`,
    /// `with_rectangular_bounds` or `with_variable_bounds` transformations.
    pub fn new(value: T, card: C) -> Self {
        Self {
            value,
            card,
            phantom: PhantomData,
        }
    }
}

impl<D, T, C> NVecCoreSealed<D, T> for ConstantVec<D, T, C>
where
    D: Dim,
    T: Copy,
    C: Card<D>,
{
    fn core_num_children(&self) -> usize {
        self.card.cardinality_of([])
    }

    fn core_card(&self, idx: impl Into<<D as Dim>::CardIdx>) -> usize {
        self.card.cardinality_of(idx)
    }

    fn core_child(&self, i: <D as Dim>::ChildIdx) -> impl NVecCoreSealed<<D as Dim>::PrevDim, T> {
        ConstantVec {
            card: self.card.child_card(i),
            value: self.value,
            phantom: Default::default(),
        }
    }

    fn core_map<F: FnMut(&T) -> O, O>(&self, _: impl IntoIdx<D>, f: &mut F) -> O {
        f(&self.value)
    }

    fn core_is_rectangular(&self) -> bool {
        self.card.is_rectangular()
    }
}

impl<D, T, C> NVec<D, T> for ConstantVec<D, T, C>
where
    D: Dim,
    T: Copy,
    C: Card<D>,
{
    #[inline(always)]
    fn at(&self, _: impl IntoIdx<D>) -> T {
        self.value
    }

    fn child(&self, i: <D as Dim>::ChildIdx) -> impl NVec<<D as Dim>::PrevDim, T> {
        ConstantVec {
            card: self.card.child_card(i),
            value: self.value,
            phantom: Default::default(),
        }
    }

    fn all(&self) -> impl Iterator<Item = T> {
        self.card.vec_all(self)
    }
}

// into bounded

// D1

impl<T> ConstantVec<D1, T, UnboundedCard<D1>>
where
    T: Copy,
{
    /// Transforms an unbounded constant vector into one with a fixed length.
    ///
    /// # Example
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v1 = V.d1().constant(42);
    ///
    /// assert_eq!(v1.card([]), usize::MAX);
    /// assert_eq!(v1.at([2]), 42);
    /// assert_eq!(v1.at([42]), 42);
    ///
    /// let v1 = v1.bounded(4);
    /// // or
    /// let v1 = V.d1().constant(42).bounded(4);
    ///
    /// assert_eq!(v1.card([]), 4);
    /// assert_eq!(v1.at([2]), 42);
    /// assert_eq!(v1.try_at([2]), Some(42));
    /// assert_eq!(v1.try_at([42]), None);
    /// assert_eq!(v1.all().collect::<Vec<_>>(), vec![42, 42, 42, 42]);
    ///
    /// assert_eq!(v1.at([42]), 42); // (!) un-compromised performance
    /// ```
    ///
    /// ***(!) un-compromised performance***
    ///
    /// *Main reason to add bounds to a functional vector is to set its domain.
    /// However, calling `at` with an out-of-bounds index can still produce a valid
    /// element in order not to compromise performance.
    /// If we want to check whether or not an index is in bounds or not, we can
    /// use the `in_bounds` or `card` methods, or use `try_at` instead which would
    /// return `None` if the index is out of bounds.*
    pub fn bounded(self, num_elements: usize) -> ConstantVec<D1, T, CardD1> {
        ConstantVec::new(self.value, num_elements.into())
    }
}

// D2

impl<T> ConstantVec<D2, T, UnboundedCard<D2>>
where
    T: Copy,
{
    /// Transforms an unbounded constant vector into one with rectangular bounds as in
    /// matrices:
    /// * the vector has `dimensions[0]` children, and
    /// * each children has `dimensions[1]` elements.
    ///
    /// # Example
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v2 = V.d2().constant(42).with_rectangular_bounds([2, 3]);
    ///
    /// assert_eq!(v2.card([]), 2);
    /// assert_eq!(v2.card([0]), 3);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.at([0, 2]), 42);
    /// assert_eq!(v2.at([1, 0]), 42);
    /// assert_eq!(
    ///     v2.equality(&vec![vec![42, 42, 42], vec![42, 42, 42]]),
    ///     Equality::Equal
    /// );
    ///
    /// assert_eq!(v2.try_at([42, 113]), None);
    /// assert_eq!(v2.at([42, 113]), 42); // (!) un-compromised performance
    /// ```
    ///
    /// ***(!) un-compromised performance***
    ///
    /// *Main reason to add bounds to a functional vector is to set its domain.
    /// However, calling `at` with an out-of-bounds index can still produce a valid
    /// element in order not to compromise performance.
    /// If we want to check whether or not an index is in bounds or not, we can
    /// use the `in_bounds` or `card` methods, or use `try_at` instead which would
    /// return `None` if the index is out of bounds.*
    pub fn with_rectangular_bounds(
        self,
        dimensions: [usize; 2],
    ) -> ConstantVec<D2, T, RectangularCardD2> {
        ConstantVec::new(self.value, dimensions.into())
    }

    /// Transforms an unbounded constant vector into one with variable bounds as in
    /// jagged arrays:
    /// * the vector has `cardinality.card([])` children, and
    /// * i-th child has `cardinality.at([i])` elements.
    ///
    /// # Example
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // jagged => [ [42, 42], [42, 42, 42], [42] ]
    /// let num_cols = vec![2, 3, 1];
    /// let v2 = V.d2().constant(42).with_variable_bounds(&num_cols);
    ///
    /// assert_eq!(v2.card([]), 3);
    /// assert_eq!(v2.card([0]), 2);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.card([2]), 1);
    /// assert_eq!(v2.at([1, 2]), 42);
    /// assert_eq!(v2.at([2, 0]), 42);
    /// assert_eq!(
    ///     v2.equality(&vec![vec![42, 42], vec![42, 42, 42], vec![42]]),
    ///     Equality::Equal
    /// );
    ///
    /// assert_eq!(v2.try_at([42, 113]), None);
    /// assert_eq!(v2.at([42, 113]), 42); // (!) un-compromised performance
    /// ```
    ///
    /// ***(!) un-compromised performance***
    ///
    /// *Main reason to add bounds to a functional vector is to set its domain.
    /// However, calling `at` with an out-of-bounds index can still produce a valid
    /// element in order not to compromise performance.
    /// If we want to check whether or not an index is in bounds or not, we can
    /// use the `in_bounds` or `card` methods, or use `try_at` instead which would
    /// return `None` if the index is out of bounds.*
    pub fn with_variable_bounds<C>(self, cardinality: C) -> ConstantVec<D2, T, VariableCardD2<C>>
    where
        C: V1<usize>,
    {
        ConstantVec::new(self.value, cardinality.into())
    }
}

// D3

impl<T> ConstantVec<D3, T, UnboundedCard<D3>>
where
    T: Copy,
{
    /// Transforms an unbounded constant vector into one with rectangular bounds as in
    /// multi-dimensional matrices. The matrix has
    /// `dimensions[0]` x `dimensions[1]` x `dimensions[2]` elements.
    ///
    /// # Example
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v2 = V.d2().constant(42).with_rectangular_bounds([2, 3]);
    ///
    /// assert_eq!(v2.card([]), 2);
    /// assert_eq!(v2.card([0]), 3);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.at([0, 2]), 42);
    /// assert_eq!(v2.at([1, 0]), 42);
    /// assert_eq!(
    ///     v2.equality(&vec![vec![42, 42, 42], vec![42, 42, 42]]),
    ///     Equality::Equal
    /// );
    ///
    /// assert_eq!(v2.try_at([42, 113]), None);
    /// assert_eq!(v2.at([42, 113]), 42); // (!) un-compromised performance
    /// ```
    ///
    /// ***(!) un-compromised performance***
    ///
    /// *Main reason to add bounds to a functional vector is to set its domain.
    /// However, calling `at` with an out-of-bounds index can still produce a valid
    /// element in order not to compromise performance.
    /// If we want to check whether or not an index is in bounds or not, we can
    /// use the `in_bounds` or `card` methods, or use `try_at` instead which would
    /// return `None` if the index is out of bounds.*
    pub fn with_rectangular_bounds(
        self,
        dimensions: [usize; 3],
    ) -> ConstantVec<D3, T, RectangularCardD3> {
        ConstantVec::new(self.value, dimensions.into())
    }

    /// Transforms an unbounded constant vector into one with variable bounds as in
    /// jagged arrays:
    /// * the vector has `cardinality.card([])` children, and
    /// * i-th child has `cardinality.at([i])` elements.
    ///
    /// # Example
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // jagged => [ [42, 42], [42, 42, 42], [42] ]
    /// let num_cols = vec![2, 3, 1];
    /// let v2 = V.d2().constant(42).with_variable_bounds(&num_cols);
    ///
    /// assert_eq!(v2.card([]), 3);
    /// assert_eq!(v2.card([0]), 2);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.card([2]), 1);
    /// assert_eq!(v2.at([1, 2]), 42);
    /// assert_eq!(v2.at([2, 0]), 42);
    /// assert_eq!(
    ///     v2.equality(&vec![vec![42, 42], vec![42, 42, 42], vec![42]]),
    ///     Equality::Equal
    /// );
    ///
    /// assert_eq!(v2.try_at([42, 113]), None);
    /// assert_eq!(v2.at([42, 113]), 42); // (!) un-compromised performance
    /// ```
    ///
    /// ***(!) un-compromised performance***
    ///
    /// *Main reason to add bounds to a functional vector is to set its domain.
    /// However, calling `at` with an out-of-bounds index can still produce a valid
    /// element in order not to compromise performance.
    /// If we want to check whether or not an index is in bounds or not, we can
    /// use the `in_bounds` or `card` methods, or use `try_at` instead which would
    /// return `None` if the index is out of bounds.*
    pub fn with_variable_bounds<C>(self, cardinality: C) -> ConstantVec<D3, T, VariableCardD3<C>>
    where
        C: V2<usize>,
    {
        ConstantVec::new(self.value, cardinality.into())
    }
}

// D4

impl<T> ConstantVec<D4, T, UnboundedCard<D4>>
where
    T: Copy,
{
    /// Transforms an unbounded constant vector into one with rectangular bounds as in
    /// multi-dimensional matrices. The matrix has
    /// `dimensions[0]` x `dimensions[1]` x `dimensions[2]` x `dimensions[3]` elements.
    ///
    /// # Example
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v2 = V.d2().constant(42).with_rectangular_bounds([2, 3]);
    ///
    /// assert_eq!(v2.card([]), 2);
    /// assert_eq!(v2.card([0]), 3);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.at([0, 2]), 42);
    /// assert_eq!(v2.at([1, 0]), 42);
    /// assert_eq!(
    ///     v2.equality(&vec![vec![42, 42, 42], vec![42, 42, 42]]),
    ///     Equality::Equal
    /// );
    ///
    /// assert_eq!(v2.try_at([42, 113]), None);
    /// assert_eq!(v2.at([42, 113]), 42); // (!) un-compromised performance
    /// ```
    ///
    /// ***(!) un-compromised performance***
    ///
    /// *Main reason to add bounds to a functional vector is to set its domain.
    /// However, calling `at` with an out-of-bounds index can still produce a valid
    /// element in order not to compromise performance.
    /// If we want to check whether or not an index is in bounds or not, we can
    /// use the `in_bounds` or `card` methods, or use `try_at` instead which would
    /// return `None` if the index is out of bounds.*
    pub fn with_rectangular_bounds(
        self,
        dimensions: [usize; 4],
    ) -> ConstantVec<D4, T, RectangularCardD4> {
        ConstantVec::new(self.value, dimensions.into())
    }

    /// Transforms an unbounded constant vector into one with variable bounds as in
    /// jagged arrays:
    /// * the vector has `cardinality.card([])` children, and
    /// * i-th child has `cardinality.at([i])` elements.
    ///
    /// # Example
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // jagged => [ [42, 42], [42, 42, 42], [42] ]
    /// let num_cols = vec![2, 3, 1];
    /// let v2 = V.d2().constant(42).with_variable_bounds(&num_cols);
    ///
    /// assert_eq!(v2.card([]), 3);
    /// assert_eq!(v2.card([0]), 2);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.card([2]), 1);
    /// assert_eq!(v2.at([1, 2]), 42);
    /// assert_eq!(v2.at([2, 0]), 42);
    /// assert_eq!(
    ///     v2.equality(&vec![vec![42, 42], vec![42, 42, 42], vec![42]]),
    ///     Equality::Equal
    /// );
    ///
    /// assert_eq!(v2.try_at([42, 113]), None);
    /// assert_eq!(v2.at([42, 113]), 42); // (!) un-compromised performance
    /// ```
    ///
    /// ***(!) un-compromised performance***
    ///
    /// *Main reason to add bounds to a functional vector is to set its domain.
    /// However, calling `at` with an out-of-bounds index can still produce a valid
    /// element in order not to compromise performance.
    /// If we want to check whether or not an index is in bounds or not, we can
    /// use the `in_bounds` or `card` methods, or use `try_at` instead which would
    /// return `None` if the index is out of bounds.*
    pub fn with_variable_bounds<C>(self, cardinality: C) -> ConstantVec<D4, T, VariableCardD4<C>>
    where
        C: V3<usize>,
    {
        ConstantVec::new(self.value, cardinality.into())
    }
}
