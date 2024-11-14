use super::{uniform_end_indices::UniformEndIndices, FlatJagged};
use crate::{NVec, D1};
use alloc::vec::Vec;

/// Transforms a `D1` vector into a jagged `D2` vector via `into_x` methods;
/// or alternatively, creates a jagged `D2` vector view `as_x` methods.
pub trait IntoJagged<T>: Sized + NVec<D1, T> {
    /// Converts a `D1` vector into a jagged `D2` vector where each row is
    /// identified by `row_end_indices`.
    ///
    /// Notice that `row_end_indices` is any `V1<usize>` which might be
    /// a vector of indices or a functional vector, etc.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// * `row_end_indices` is **not** non-decreasing, or
    /// * last element of `row_end_indices` is **not** equal to the length of this
    ///   flat vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let end_indices = vec![1, 1, 4, 10]; // lengths => [1, 0, 3, 6]
    /// let v2 = v1.into_jagged(end_indices);
    /// assert_eq!(
    ///     v2.equality(&[vec![0], vec![], vec![1, 2, 3], vec![4, 5, 6, 7, 8, 9]]),
    ///     Equality::Equal
    /// );
    ///
    /// let v1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let end_indices = V.d1().fun(|[i]| (i + 1) * 5).bounded(2); // lengths => [5, 5]
    /// let v2 = v1.into_jagged(end_indices);
    /// assert_eq!(
    ///     v2.equality(&[vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9]]),
    ///     Equality::Equal
    /// );
    /// ```
    fn into_jagged<I>(self, row_end_indices: I) -> FlatJagged<Self, I, T>
    where
        I: NVec<D1, usize>,
    {
        validate_row_end_indices(&self, &row_end_indices);
        FlatJagged::new(self, row_end_indices)
    }

    /// From a flat `D1` vector, creates a jagged `D2` vector view where each row is
    /// identified by `row_end_indices`.
    ///
    /// Notice that `row_end_indices` is any `V1<usize>` which might be
    /// a vector of indices or a functional vector, etc.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// * `row_end_indices` is **not** non-decreasing, or
    /// * last element of `row_end_indices` is **not** equal to the length of this
    ///   flat vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ///
    /// let end_indices = vec![1, 1, 4, 10]; // lengths => [1, 0, 3, 6]
    /// let v2 = v1.as_jagged(&end_indices);
    /// assert_eq!(
    ///     v2.equality(&[vec![0], vec![], vec![1, 2, 3], vec![4, 5, 6, 7, 8, 9]]),
    ///     Equality::Equal
    /// );
    ///
    /// let end_indices = V.d1().fun(|[i]| (i + 1) * 5).bounded(2); // lengths => [5, 5]
    /// let v2 = v1.as_jagged(&end_indices);
    /// assert_eq!(
    ///     v2.equality(&[vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9]]),
    ///     Equality::Equal
    /// );
    /// ```
    fn as_jagged<I>(&self, row_end_indices: I) -> FlatJagged<&Self, I, T>
    where
        I: NVec<D1, usize>,
    {
        validate_row_end_indices(self, &row_end_indices);
        FlatJagged::new(self, row_end_indices)
    }

    /// From a flat `D1` vector, creates a mutable jagged `D2` vector view where each row is
    /// identified by `row_end_indices`.
    ///
    /// Notice that `row_end_indices` is any `V1<usize>` which might be
    /// a vector of indices or a functional vector, etc.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// * `row_end_indices` is **not** non-decreasing, or
    /// * last element of `row_end_indices` is **not** equal to the length of this
    ///   flat vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let mut v1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ///
    /// let end_indices = vec![1, 1, 4, 10]; // lengths => [1, 0, 3, 6]
    /// let mut v2 = v1.as_jagged_mut(&end_indices);
    /// *v2.at_mut([2, 2]) += 10;
    /// *v2.at_mut([3, 4]) *= 10;
    /// assert_eq!(
    ///     v2.equality(&[vec![0], vec![], vec![1, 2, 13], vec![4, 5, 6, 7, 80, 9]]),
    ///     Equality::Equal
    /// );
    ///
    /// let end_indices = V.d1().fun(|[i]| (i + 1) * 5).bounded(2); // lengths => [5, 5]
    /// let mut v2 = v1.as_jagged_mut(&end_indices);
    /// *v2.at_mut([0, 4]) += 100;
    /// assert_eq!(
    ///     v2.equality(&[vec![0, 1, 2, 13, 104], vec![5, 6, 7, 80, 9]]),
    ///     Equality::Equal
    /// );
    /// ```
    fn as_jagged_mut<I>(&mut self, row_end_indices: I) -> FlatJagged<&mut Self, I, T>
    where
        I: NVec<D1, usize>,
    {
        validate_row_end_indices(self, &row_end_indices);
        FlatJagged::new(self, row_end_indices)
    }

    // from row lengths

    /// Converts a `D1` vector into a jagged `D2` vector where each row is
    /// identified by `row_lengths`.
    ///
    /// Notice that `row_lengths` is any `V1<usize>` which might be
    /// a vector of indices or a functional vector, etc.
    ///
    /// Internally, this method will evaluate `row_end_indices`, store it in a
    /// `Vec<usize>` and call `into_jagged` method.
    ///
    /// # Panics
    ///
    /// Panics if the sum of `row_lengths` do not add up to the length of this vector;
    /// i.e., `self.card([])`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let row_lengths = vec![1, 0, 3, 6];
    /// let v2 = v1.into_jagged_from_row_lengths(&row_lengths);
    /// assert_eq!(
    ///     v2.equality(&[vec![0], vec![], vec![1, 2, 3], vec![4, 5, 6, 7, 8, 9]]),
    ///     Equality::Equal
    /// );
    ///
    /// let v1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let row_lengths = V.d1().constant(5).bounded(2); // lengths => [5, 5]
    /// let v2 = v1.into_jagged_from_row_lengths(&row_lengths);
    /// assert_eq!(
    ///     v2.equality(&[vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9]]),
    ///     Equality::Equal
    /// );
    /// ```
    fn into_jagged_from_row_lengths<I>(self, row_lengths: &I) -> FlatJagged<Self, Vec<usize>, T>
    where
        I: NVec<D1, usize>,
    {
        validate_row_end_lengths(&self, row_lengths);
        self.into_jagged(row_indices_from_row_lengths(row_lengths))
    }

    /// From a flat `D1` vector, creates a jagged `D2` vector view where each row is
    /// identified by `row_lengths`.
    ///
    /// Notice that `row_lengths` is any `V1<usize>` which might be
    /// a vector of indices or a functional vector, etc.
    ///
    /// Internally, this method will evaluate `row_end_indices`, store it in a
    /// `Vec<usize>` and call `into_jagged` method.
    ///
    /// # Panics
    ///
    /// Panics if the sum of `row_lengths` do not add up to the length of this vector;
    /// i.e., `self.card([])`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ///
    /// let row_lengths = vec![1, 0, 3, 6]; // lengths => [1, 0, 3, 6]
    /// let v2 = v1.as_jagged_from_row_lengths(&row_lengths);
    /// assert_eq!(
    ///     v2.equality(&[vec![0], vec![], vec![1, 2, 3], vec![4, 5, 6, 7, 8, 9]]),
    ///     Equality::Equal
    /// );
    ///
    /// let row_lengths = V.d1().constant(5).bounded(2); // lengths => [5, 5]
    /// let v2 = v1.as_jagged_from_row_lengths(&row_lengths);
    /// assert_eq!(
    ///     v2.equality(&[vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9]]),
    ///     Equality::Equal
    /// );
    /// ```
    fn as_jagged_from_row_lengths<I>(&self, row_lengths: &I) -> FlatJagged<&Self, Vec<usize>, T>
    where
        I: NVec<D1, usize>,
    {
        validate_row_end_lengths(self, row_lengths);
        self.as_jagged(row_indices_from_row_lengths(row_lengths))
    }

    /// From a flat `D1` vector, creates a mutable jagged `D2` vector view where each row is
    /// identified by `row_lengths`.
    ///
    /// Notice that `row_lengths` is any `V1<usize>` which might be
    /// a vector of indices or a functional vector, etc.
    ///
    /// Internally, this method will evaluate `row_end_indices`, store it in a
    /// `Vec<usize>` and call `into_jagged` method.
    ///
    /// # Panics
    ///
    /// Panics if the sum of `row_lengths` do not add up to the length of this vector;
    /// i.e., `self.card([])`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let mut v1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ///
    /// let row_lengths = vec![1, 0, 3, 6]; // lengths => [1, 0, 3, 6]
    /// let mut v2 = v1.as_jagged_mut_from_row_lengths(&row_lengths);
    /// *v2.at_mut([2, 2]) += 10;
    /// *v2.at_mut([3, 4]) *= 10;
    /// assert_eq!(
    ///     v2.equality(&[vec![0], vec![], vec![1, 2, 13], vec![4, 5, 6, 7, 80, 9]]),
    ///     Equality::Equal
    /// );
    ///
    /// let row_lengths = V.d1().constant(5).bounded(2); // lengths => [5, 5]
    /// let mut v2 = v1.as_jagged_mut_from_row_lengths(&row_lengths);
    /// *v2.at_mut([0, 4]) += 100;
    /// assert_eq!(
    ///     v2.equality(&[vec![0, 1, 2, 13, 104], vec![5, 6, 7, 80, 9]]),
    ///     Equality::Equal
    /// );
    fn as_jagged_mut_from_row_lengths<I>(
        &mut self,
        row_lengths: &I,
    ) -> FlatJagged<&mut Self, Vec<usize>, T>
    where
        I: NVec<D1, usize>,
    {
        validate_row_end_lengths(self, row_lengths);
        self.as_jagged_mut(row_indices_from_row_lengths(row_lengths))
    }

    // with max child size

    /// Converts a `D1` vector into a jagged `D2` vector where each row has the given
    /// `uniform_length`, except that the last row might have fewer elements if the
    /// cardinality of the `D1` vector is not divisible by the uniform length.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v1 = [1, 2, 3, 4, 5, 6, 7, 8];
    ///
    /// let v2 = v1.into_jagged_with_uniform_lengths(3);
    ///
    /// assert_eq!(v2.card([]), 3);
    /// assert_eq!(v2.card([0]), 3);
    /// assert_eq!(v2.at([1, 2]), 6);
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8]]),
    ///     Equality::Equal
    /// );
    /// ```
    fn into_jagged_with_uniform_lengths(
        self,
        uniform_length: usize,
    ) -> FlatJagged<Self, UniformEndIndices, T> {
        assert!(uniform_length > 0);
        let row_end_indices = UniformEndIndices::new(uniform_length, self.card([]));
        FlatJagged::new(self, row_end_indices)
    }

    /// From a flat `D1` vector, creates a jagged `D2` vector view where each row has
    /// the given `uniform_length`, except that the last row might have fewer elements
    /// if the cardinality of the `D1` vector is not divisible by the uniform length.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v1 = [1, 2, 3, 4, 5, 6, 7, 8];
    ///
    /// let v2 = v1.as_jagged_with_uniform_lengths(3);
    ///
    /// assert_eq!(v2.card([]), 3);
    /// assert_eq!(v2.card([0]), 3);
    /// assert_eq!(v2.at([1, 2]), 6);
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8]]),
    ///     Equality::Equal
    /// );
    /// ```
    fn as_jagged_with_uniform_lengths(
        &self,
        uniform_length: usize,
    ) -> FlatJagged<&Self, UniformEndIndices, T> {
        assert!(uniform_length > 0);
        let row_end_indices = UniformEndIndices::new(uniform_length, self.card([]));
        FlatJagged::new(self, row_end_indices)
    }

    /// From a flat `D1` vector, creates a mutable jagged `D2` vector view where each row has
    /// the given `uniform_length`, except that the last row might have fewer elements
    /// if the cardinality of the `D1` vector is not divisible by the uniform length.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let mut v1 = [1, 2, 3, 4, 5, 6, 7, 8];
    ///
    /// let mut v2 = v1.as_jagged_mut_with_uniform_lengths(3);
    ///
    /// v2.set([1, 2], 66);
    /// *v2.at_mut([2, 0]) = 77;
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![1, 2, 3], vec![4, 5, 66], vec![77, 8]]),
    ///     Equality::Equal
    /// );
    /// ```
    fn as_jagged_mut_with_uniform_lengths(
        &mut self,
        uniform_length: usize,
    ) -> FlatJagged<&mut Self, UniformEndIndices, T> {
        assert!(uniform_length > 0);
        let row_end_indices = UniformEndIndices::new(uniform_length, self.card([]));
        FlatJagged::new(self, row_end_indices)
    }
}

impl<T, V: Sized + NVec<D1, T>> IntoJagged<T> for V {}

// helpers

fn row_indices_from_row_lengths<I>(row_lengths: &I) -> Vec<usize>
where
    I: NVec<D1, usize>,
{
    row_lengths
        .all()
        .scan(0, |x, y| {
            *x += y;
            Some(*x)
        })
        .collect()
}

fn validate_row_end_indices<T, V: NVec<D1, T>, I: NVec<D1, usize>>(flat_vec: &V, end_indices: I) {
    let mut begin = 0;
    for end in end_indices.all() {
        assert!(end >= begin,
            "`row_end_indices` must be a non-decreasing vector. \
            For example, end indices [1, 2, 2, 5] represents a jagged vector with row lengths of [1, 1, 0, 3]. \
            However, received a decreasing sequence [.., {}, {}, ..].",
            begin, end
        );
        begin = end;
    }
    assert_eq!(flat_vec.card([]), begin,
        "Last entry of the `row_end_indices` must equal the cardinality of the flat V1 storage. \
        For example, end indices [1, 2, 2, 5] for flat storage [0, 1, 2, 3, 4] represents a jagged vector of \
        [ [0], [1], [], [2, 3, 4] ]. \
        However, received a flat storage cardinality of {} while the last entry of row end indices is {}.",
        flat_vec.card([]), begin
    );
}

fn validate_row_end_lengths<T, V: NVec<D1, T>, I: NVec<D1, usize>>(flat_vec: &V, lengths: I) {
    let total_len: usize = lengths.all().sum();
    assert_eq!(flat_vec.card([]), total_len,
        "Sum of elements of `row_lengths` must equal the cardinality of the flat V1 storage. \
        For example, row lengths [1, 1, 0, 3] for flat storage [0, 1, 2, 3, 4] represents a jagged vector of \
        [ [0], [1], [], [2, 3, 4] ]. \
        However, received a flat storage cardinality of {} while sum of row lengths is {}.",
        flat_vec.card([]), total_len
    );
}
