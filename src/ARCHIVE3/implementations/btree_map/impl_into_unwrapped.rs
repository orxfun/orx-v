// use crate::{dimensions::dim::*, FromIndex, IntoUnwrapped, Unwrapped};
// use alloc::collections::btree_map::BTreeMap;

// impl<'a, K, T> IntoUnwrapped<D1, Option<&'a T>, &'a T> for BTreeMap<K, T>
// where
//     K: FromIndex<D1>,
// {
//     fn unwrapped(
//         &self,
//     ) -> Unwrapped<'_, D1, Self, Option<&'a T>, &'a T, impl Fn(Option<&'a T>) -> &'a T> {
//         Unwrapped::new(self, |x: Option<&'a T>| {
//             x.expect("'at' called on index without a value")
//         })
//     }
// }

// impl<'a, K, T> IntoUnwrapped<D1, Option<&'a T>, &'a T> for BTreeMap<K, &'a T>
// where
//     K: FromIndex<D1>,
// {
//     fn unwrapped(
//         &self,
//     ) -> Unwrapped<'_, D1, Self, Option<&'a T>, &'a T, impl Fn(Option<&'a T>) -> &'a T> {
//         Unwrapped::new(self, |x: Option<&'a T>| {
//             x.expect("'at' called on index without a value")
//         })
//     }
// }
