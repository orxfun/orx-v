use crate::{Dim, NVec};

pub trait IntoCopied<N, T>
where
    N: Dim,
{
    fn copied(&self) -> impl NVec<N, T>;
}

// use super::{transformed::Transformed, IntoUnwrapped, Unwrapped};
// use crate::{dimensions::dim::*, IntoIndex, NVec};

// pub struct Copied<'v, N, V, T1, T2, F>(Transformed<'v, N, V, T1, T2, F>)
// where
//     N: Dim,
//     F: Fn(T1) -> T2;

// impl<'v, N, V, T1, T2, F> Copied<'v, N, V, T1, T2, F>
// where
//     N: Dim,
//     F: Fn(T1) -> T2,
// {
//     pub(crate) fn new(inner: &'v V, transform: F) -> Self {
//         Self(Transformed::new(inner, transform))
//     }
// }

// impl<'v, 'a, N, V, T, F> NVec<N, T> for Copied<'v, N, V, &'a T, T, F>
// where
//     N: Dim,
//     T: Copy,
//     F: Fn(&'a T) -> T,
//     V: NVec<N, &'a T>,
// {
//     #[inline]
//     fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
//         (self.0.transform)(self.0.inner.at(index))
//     }
// }

// impl<'v, 'a, N, V, T, F> NVec<N, Option<T>> for Copied<'v, N, V, Option<&'a T>, Option<T>, F>
// where
//     N: Dim,
//     T: Copy,
//     F: Fn(Option<&'a T>) -> Option<T>,
//     V: NVec<N, Option<&'a T>>,
// {
//     #[inline]
//     fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T> {
//         (self.0.transform)(self.0.inner.at(index))
//     }
// }

// // INTO

// pub trait IntoCopied<N, T1, T2>
// where
//     N: Dim,
//     Self: NVec<N, T1> + Sized,
// {
//     fn copied(&self) -> Copied<'_, N, Self, T1, T2, impl Fn(T1) -> T2>;
// }

// // OLD

// pub struct CopiedZzz<'v, N, V, T1, T2, F>(Transformed<'v, N, V, T1, T2, F>)
// where
//     N: Dim,
//     F: Fn(T1) -> T2;

// impl<'v, N, V, T1, T2, F> CopiedZzz<'v, N, V, T1, T2, F>
// where
//     N: Dim,
//     F: Fn(T1) -> T2,
// {
//     pub(crate) fn new(inner: &'v V, transform: F) -> Self {
//         Self(Transformed::new(inner, transform))
//     }
// }

// impl<'v, 'a, N, V, T, F> NVec<N, T> for CopiedZzz<'v, N, V, &'a T, T, F>
// where
//     N: Dim,
//     T: Copy,
//     F: Fn(&'a T) -> T,
//     V: NVec<N, &'a T>,
// {
//     #[inline]
//     fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
//         (self.0.transform)(self.0.inner.at(index))
//     }
// }

// impl<'v, 'a, N, V, T, F> NVec<N, Option<T>> for CopiedZzz<'v, N, V, Option<&'a T>, Option<T>, F>
// where
//     N: Dim,
//     T: Copy,
//     F: Fn(Option<&'a T>) -> Option<T>,
//     V: NVec<N, Option<&'a T>>,
// {
//     #[inline]
//     fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T> {
//         (self.0.transform)(self.0.inner.at(index))
//     }
// }

// // INTO

// pub trait IntoCopiedZzz<N, T1, T2>
// where
//     N: Dim,
//     Self: NVec<N, T1> + Sized,
// {
//     fn copied(&self) -> CopiedZzz<'_, N, Self, T1, T2, impl Fn(T1) -> T2>;
// }

// // IntoUnwrapped

// impl<'v, 'e, N, V, T, F> IntoUnwrapped<N, Option<T>, T>
//     for CopiedZzz<'v, N, V, Option<&'e T>, Option<T>, F>
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
