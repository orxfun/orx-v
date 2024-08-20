// use crate::dimensions::dim::*;
// use core::marker::PhantomData;

// pub struct Transformed<'v, N, V, T1, T2, F>
// where
//     N: Dim,
//     F: Fn(T1) -> T2,
// {
//     pub(super) inner: &'v V,
//     pub(super) transform: F,
//     phantom: PhantomData<(N, T1)>,
// }

// impl<'v, N, V, T1, T2, F> Transformed<'v, N, V, T1, T2, F>
// where
//     N: Dim,

//     F: Fn(T1) -> T2,
// {
//     pub(super) fn new(inner: &'v V, transform: F) -> Self {
//         Self {
//             inner,
//             transform,
//             phantom: Default::default(),
//         }
//     }
// }
