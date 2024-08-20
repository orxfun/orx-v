use super::fun_vec::FunVec;
use crate::{dimensions::dim::*, overloads::kv_map::KvMap, FromIndex};

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
            pub fn sparse<T, I, F>(self, f: F) -> FunVec<$dim, T, I, impl Fn(I) -> Option<T>>
            where
                I: FromIndex<$dim>,
                F: Fn(I) -> Option<T>,
            {
                FunVec::new(f)
            }

            pub fn complete<T, I, G>(self, g: G) -> FunVec<$dim, T, I, impl Fn(I) -> Option<T>>
            where
                I: FromIndex<$dim>,
                G: Fn(I) -> T,
            {
                let f = move |idx: I| Some((g)(idx));
                FunVec::new(f)
            }

            // from map
            pub fn sparse_from_map<M, T, I>(
                self,
                map: M,
            ) -> FunVec<$dim, T, I, impl Fn(I) -> Option<T>>
            where
                M: KvMap<I, T>,
                T: Copy,
                I: FromIndex<$dim>,
            {
                FunVecBuilder::$fn().sparse(move |index: I| map.get(&index).copied())
            }

            pub fn complete_from_map<M, T, I>(
                self,
                map: M,
                missing_element_value: T,
            ) -> FunVec<$dim, T, I, impl Fn(I) -> Option<T>>
            where
                M: KvMap<I, T>,
                T: Copy,
                I: FromIndex<$dim>,
            {
                FunVecBuilder::$fn().sparse(move |index: I| {
                    Some(map.get(&index).copied().unwrap_or(missing_element_value))
                })
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
