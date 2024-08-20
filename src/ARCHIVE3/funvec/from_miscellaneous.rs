use super::{builder::*, FunVec};
use crate::dimensions::dim::*;

impl FunVecBuilderD1 {
    pub fn constant<T>(self, constant_value: T) -> FunVec<D1, T, usize, impl Fn(usize) -> T>
    where
        T: Copy,
    {
        FunVecBuilder::d1().new(move |_: usize| constant_value)
    }
}

impl FunVecBuilderD2 {
    pub fn constant<T>(
        self,
        constant_value: T,
    ) -> FunVec<D2, T, [usize; 2], impl Fn([usize; 2]) -> T>
    where
        T: Copy,
    {
        FunVecBuilder::d2().new(move |_: [usize; 2]| constant_value)
    }
}

// >= d3

macro_rules! impl_builder {
    ($builder:tt, $dim:tt, $fn:ident, $n:expr) => {
        impl $builder {
            pub fn constant<T>(
                self,
                constant_value: T,
            ) -> FunVec<$dim, T, [usize; $n], impl Fn([usize; $n]) -> T>
            where
                T: Copy,
            {
                FunVecBuilder::$fn().new(move |_: [usize; $n]| constant_value)
            }
        }
    };
}

impl_builder!(FunVecBuilderD3, D3, d3, 3);
impl_builder!(FunVecBuilderD4, D4, d4, 4);
impl_builder!(FunVecBuilderD5, D5, d5, 5);
impl_builder!(FunVecBuilderD6, D6, d6, 6);
impl_builder!(FunVecBuilderD7, D7, d7, 7);
impl_builder!(FunVecBuilderD8, D8, d8, 8);
