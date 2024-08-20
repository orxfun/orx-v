// use crate::{dimensions::dim::*, CopiedRef, FromIndex, IntoCopiedRef, NVecRef};
// use alloc::collections::btree_map::BTreeMap;

// impl<'a, K, T: Copy> IntoCopiedRef<D1, Option<&'a T>, Option<T>> for BTreeMap<K, T>
// where
//     K: FromIndex<D1>,
// {
//     fn copied(
//         &self,
//     ) -> CopiedRef<'_, D1, Self, Option<&'a T>, Option<T>, impl Fn(Option<&'a T>) -> Option<T>>
//     {
//         CopiedRef::new(self, |x: Option<&'a T>| x.copied())
//     }
// }

// // full-indexed

// impl<'a, K, T: Copy> IntoCopiedRef<D2, Option<&'a T>, Option<T>> for BTreeMap<K, T>
// where
//     K: FromIndex<D2>,
// {
//     fn copied(
//         &self,
//     ) -> CopiedRef<'_, D2, Self, Option<&'a T>, Option<T>, impl Fn(Option<&'a T>) -> Option<T>>
//     {
//         CopiedRef::new(self, |x: Option<&'a T>| x.copied())
//     }
// }

// macro_rules! implement_idx {
//     ($dim:tt) => {
//         impl<'a, K, T: Copy> IntoCopiedRef<$dim, Option<&'a T>, Option<T>> for BTreeMap<K, T>
//         where
//             K: FromIndex<$dim>,
//         {
//             fn copied(
//                 &self,
//             ) -> CopiedRef<
//                 '_,
//                 $dim,
//                 Self,
//                 Option<&'a T>,
//                 Option<T>,
//                 impl Fn(Option<&'a T>) -> Option<T>,
//             > {
//                 CopiedRef::new(self, |x: Option<&'a T>| x.copied())
//             }
//         }
//     };
// }

// implement_idx!(D3);
// implement_idx!(D4);
// implement_idx!(D5);
// implement_idx!(D6);
// implement_idx!(D7);
// implement_idx!(D8);

// // recursive

// impl<'a, T, V> IntoCopiedRef<D2, Option<&'a T>, Option<T>> for BTreeMap<usize, V>
// where
//     T: Copy,
//     V: NVecRef<<D2 as Dim>::PREVIOUS>,
//     Self: NVecRef<D2>,
// {
//     fn copied(
//         &self,
//     ) -> CopiedRef<'_, D2, Self, Option<&'a T>, Option<T>, impl Fn(Option<&'a T>) -> Option<T>>
//     {
//         CopiedRef::new(self, |x: Option<&'a T>| x.copied())
//     }
// }

// macro_rules! implement_recursive {
//     ($dim:tt) => {
//         impl<'a, T, V> IntoCopiedRef<$dim, Option<&'a T>, Option<T>> for BTreeMap<usize, V>
//         where
//             T: Copy,
//             V: NVecRef<<$dim as Dim>::PREVIOUS>,
//             Self: NVecRef<$dim>,
//         {
//             fn copied(
//                 &self,
//             ) -> CopiedRef<
//                 '_,
//                 $dim,
//                 Self,
//                 Option<&'a T>,
//                 Option<T>,
//                 impl Fn(Option<&'a T>) -> Option<T>,
//             > {
//                 CopiedRef::new(self, |x: Option<&'a T>| x.copied())
//             }
//         }
//     };
// }

// implement_recursive!(D3);
// implement_recursive!(D4);
// implement_recursive!(D5);
// implement_recursive!(D6);
// implement_recursive!(D7);
// implement_recursive!(D8);
