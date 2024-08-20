// use super::funvec::FunVec;
// use crate::{dimensions::dim::*, FromIndex};

// #[derive(Clone, Copy, Debug)]
// pub struct FunVecBuilder;

// // d1

// impl FunVecBuilder {
//     pub fn d1() -> FunVecBuilderD1 {
//         FunVecBuilderD1
//     }
// }

// #[derive(Clone, Copy, Debug)]
// pub struct FunVecBuilderD1;

// impl FunVecBuilderD1 {
//     pub fn new<T, I, F>(self, f: F) -> FunVec<D1, T, I, impl Fn(I) -> T>
//     where
//         I: FromIndex<D1>,
//         F: Fn(I) -> T,
//     {
//         FunVec::new(f)
//     }
// }

// // d2

// impl FunVecBuilder {
//     pub fn d2() -> FunVecBuilderD2 {
//         FunVecBuilderD2
//     }
// }

// #[derive(Clone, Copy, Debug)]
// pub struct FunVecBuilderD2;

// impl FunVecBuilderD2 {
//     pub fn new<T, I, F>(self, f: F) -> FunVec<D2, T, I, impl Fn(I) -> T>
//     where
//         I: FromIndex<D2>,
//         F: Fn(I) -> T,
//     {
//         FunVec::new(f)
//     }
// }

// // >= d3

// macro_rules! impl_builder {
//     ($builder:tt, $dim:tt, $fn:ident, $n:expr) => {
//         impl FunVecBuilder {
//             pub fn $fn() -> $builder {
//                 $builder
//             }
//         }

//         #[derive(Clone, Copy, Debug)]
//         pub struct $builder;

//         impl $builder {
//             pub fn new<T, I, F>(self, f: F) -> FunVec<$dim, T, I, impl Fn(I) -> T>
//             where
//                 I: FromIndex<$dim>,
//                 F: Fn(I) -> T,
//             {
//                 FunVec::new(f)
//             }
//         }
//     };
// }

// impl_builder!(FunVecBuilderD3, D3, d3, 3);
// impl_builder!(FunVecBuilderD4, D4, d4, 4);
// impl_builder!(FunVecBuilderD5, D5, d5, 5);
// impl_builder!(FunVecBuilderD6, D6, d6, 6);
// impl_builder!(FunVecBuilderD7, D7, d7, 7);
// impl_builder!(FunVecBuilderD8, D8, d8, 8);
