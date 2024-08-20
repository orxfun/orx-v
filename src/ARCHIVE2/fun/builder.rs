use super::funvec::FunVec;
use crate::{dimensions::dim::*, FromIndex};

#[derive(Clone, Copy, Debug)]
pub struct FunVecBuilder;

macro_rules! impl_builder {
    ($builder:tt, $dim:tt, $fn:ident) => {
        impl FunVecBuilder {
            pub fn $fn() -> $builder {
                $builder
            }
        }

        #[derive(Clone, Copy, Debug)]
        pub struct $builder;

        impl $builder {
            pub fn new<T, I, F>(self, f: F) -> FunVec<$dim, T, I, impl Fn(I) -> T>
            where
                I: FromIndex<$dim>,
                F: Fn(I) -> T,
            {
                FunVec::new(f)
            }
        }
    };
}

impl_builder!(FunVecBuilderD1, D1, d1);
impl_builder!(FunVecBuilderD2, D2, d2);
impl_builder!(FunVecBuilderD3, D3, d3);
impl_builder!(FunVecBuilderD4, D4, d4);
impl_builder!(FunVecBuilderD5, D5, d5);
impl_builder!(FunVecBuilderD6, D6, d6);
impl_builder!(FunVecBuilderD7, D7, d7);
impl_builder!(FunVecBuilderD8, D8, d8);
