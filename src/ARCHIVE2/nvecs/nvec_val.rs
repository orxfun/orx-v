// use crate::{Dim, IntoIndex, NVec};

// pub trait NVecVal<N: Dim, T> {
//     fn val_at<Idx: IntoIndex<N>>(&self, index: Idx) -> T;
// }

// impl<N: Dim, T, V> NVecVal<N, T> for V
// where
//     V: for<'a> NVec<N, Element<'a> = T>,
// {
//     fn val_at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
//         todo!()
//     }
// }

// impl<'a, N: Dim, V> NVecVal<N, V::Element<'a>> for V
// where
//     V: NVec<N>,
//     Self: 'a,
//     for<'b> V::Element<'b>: 'a,
//     V: 'a,
// {
//     fn val_at<Idx: IntoIndex<N>>(&self, index: Idx) -> V::Element<'a> {
//         let x = self.at(index);
//         x
//     }
// }

// impl<'a, N: Dim, V> NVecVal<N, V::Element<'a>> for V
// where
//     V: NVec<N>,
//     // 'a: 'static,
//     'static: 'a,
// {
//     fn val_at<Idx: IntoIndex<N>>(&self, index: Idx) -> V::Element<'a> {
//         let x = self.at(index);
//         x
//     }
// }

// let x = self.at(index);
// x

// TODO !!!
