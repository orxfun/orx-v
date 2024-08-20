// use super::transformed::Transformed;
// use crate::{dimensions::dim::*, IntoIndex, NVec, NVecRef};

// pub struct Unwrapped<'v, N, V, T1, T2, F>(Transformed<'v, N, V, T1, T2, F>)
// where
//     N: Dim,
//     F: Fn(T1) -> T2;

// impl<'v, N, V, T1, T2, F> Unwrapped<'v, N, V, T1, T2, F>
// where
//     N: Dim,
//     F: Fn(T1) -> T2,
// {
//     pub(crate) fn new(inner: &'v V, transform: F) -> Self {
//         Self(Transformed::new(inner, transform))
//     }
// }

// impl<'v, N, V, T, F> NVec<N, T> for Unwrapped<'v, N, V, Option<T>, T, F>
// where
//     N: Dim,
//     T: Copy,
//     F: Fn(Option<T>) -> T,
//     V: NVec<N, Option<T>>,
// {
//     #[inline]
//     fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
//         (self.0.transform)(self.0.inner.at(index))
//     }
// }

// impl<'v, N, V, T, F> NVecRef<N> for Unwrapped<'v, N, V, Option<&'v T>, &'v T, F>
// where
//     N: Dim,
//     V: NVecRef<N, Element<'v> = Option<&'v T>>,
//     F: Fn(Option<&'v T>) -> &'v T,
// {
//     type Element<'e> = &'e T where Self: 'e;

//     #[inline]
//     fn ref_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Self::Element<'_> {
//         (self.0.transform)(self.0.inner.ref_at(index))
//     }
// }

// // INTO

// pub trait IntoUnwrapped<N, T1, T2>
// where
//     N: Dim,
//     Self: Sized,
// {
//     fn unwrapped(&self) -> Unwrapped<'_, N, Self, T1, T2, impl Fn(T1) -> T2>;
// }
