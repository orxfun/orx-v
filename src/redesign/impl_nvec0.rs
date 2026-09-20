use super::{D0, Dim, NVec};

// impl<'a, T> NVec<D0, &'a T> for &'a T {
//     #[inline(always)]
//     fn at(&self, []: <D0 as Dim>::Idx) -> &'a T {
//         self
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use alloc::string::ToString;

//     fn get<T>(v: impl NVec<D0, T>) -> T {
//         v.at([])
//     }

//     #[test]
//     fn scalar_as_v0_copy() {
//         let a = 12;

//         assert_eq!(get(&a), &12);
//         assert_eq!(get(a.cloned()), 12);
//         assert_eq!(get(a.copied()), 12);
//     }

//     #[test]
//     fn scalar_as_v0_clone() {
//         let a = "x".to_string();

//         assert_eq!(get(&a), &"x".to_string());
//         assert_eq!(get(a.cloned()), "x".to_string());
//     }
// }
