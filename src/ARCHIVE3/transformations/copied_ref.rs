// use super::{transformed::Transformed, IntoUnwrapped, Unwrapped};
// use crate::{dimensions::dim::*, IntoIndex, NVec, NVecRef};

// pub struct CopiedRef<'v, N, V, T1, T2, F>(Transformed<'v, N, V, T1, T2, F>)
// where
//     N: Dim,
//     F: Fn(T1) -> T2;

// impl<'v, N, V, T1, T2, F> CopiedRef<'v, N, V, T1, T2, F>
// where
//     N: Dim,
//     F: Fn(T1) -> T2,
// {
//     pub(crate) fn new(inner: &'v V, transform: F) -> Self {
//         Self(Transformed::new(inner, transform))
//     }
// }

// impl<'v, N, V, T, F> NVec<N, T> for CopiedRef<'v, N, V, V::Element<'v>, T, F>
// where
//     N: Dim,
//     F: Fn(V::Element<'v>) -> T,
//     V: NVecRef<N>,
// {
//     fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
//         (self.0.transform)(self.0.inner.ref_at(index))
//     }
// }

// // INTO

// pub trait IntoCopiedRef<N, T1, T2>
// where
//     N: Dim,
//     Self: NVecRef<N> + Sized,
// {
//     fn copied(&self) -> CopiedRef<'_, N, Self, T1, T2, impl Fn(T1) -> T2>;
// }

// // IntoUnwrapped

// impl<'v, 'e, N, V, T, F> IntoUnwrapped<N, Option<T>, T>
//     for CopiedRef<'v, N, V, Option<&'e T>, Option<T>, F>
// where
//     N: Dim,
//     F: Fn(Option<&'e T>) -> Option<T>,
// {
//     #[inline]
//     fn unwrapped(&self) -> Unwrapped<'_, N, Self, Option<T>, T, impl Fn(Option<T>) -> T> {
//         Unwrapped::new(self, |x: Option<T>| {
//             x.expect("'at' called on index without a value")
//         })
//     }
// }
