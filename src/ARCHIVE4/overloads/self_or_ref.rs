// pub trait SelfOrRef<T> {
//     type Core;
// }

// impl<T> SelfOrRef<T> for T {
//     type Core = T;
// }

// impl<'a, T> SelfOrRef<T> for &'a T {
//     type Core = T;
// }

// // impl<'a, T> SelfOrRef<&'a T> for T {
// //     type Core = T;
// // }

// // impl<'a, 'b, T> SelfOrRef<&'a &'b T> for T {
// //     type Core = T;
// // }

// impl<'a, 'b, T> SelfOrRef<T> for &'a &'b T {
//     type Core = T;
// }
