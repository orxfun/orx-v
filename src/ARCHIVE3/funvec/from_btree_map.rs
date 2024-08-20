use super::{builder::*, FunVec};
use crate::dimensions::dim::*;
use crate::overloads::CopyOrRef;
use crate::{FromIndex, KvMap};

impl FunVecBuilderD1 {
    pub fn complete_from_map<'m, M, C, T, I>(
        self,
        map: &'m M,
        missing_element_value: C,
    ) -> FunVec<D1, C, I, impl Fn(I) -> C + 'm>
    where
        T: 'm,
        M: KvMap<I, T> + 'm,
        I: FromIndex<D1> + 'm,
        C: CopyOrRef<'m, T> + 'm,
    {
        FunVecBuilder::d1().new(move |index: I| {
            map.get(&index)
                .map(|x| C::from_ref(x))
                .unwrap_or(missing_element_value)
        })
    }
}

impl FunVecBuilderD2 {
    pub fn complete_from_map<'m, M, C, T, I>(
        self,
        map: &'m M,
        missing_element_value: C,
    ) -> FunVec<D2, C, I, impl Fn(I) -> C + 'm>
    where
        T: 'm,
        M: KvMap<I, T> + 'm,
        I: FromIndex<D2> + 'm,
        C: CopyOrRef<'m, T> + 'm,
    {
        FunVecBuilder::d2().new(move |index: I| {
            map.get(&index)
                .map(|x| C::from_ref(x))
                .unwrap_or(missing_element_value)
        })
    }
}

// >= d3

macro_rules! impl_builder {
    ($builder:tt, $dim:tt, $fn:ident) => {
        impl $builder {
            pub fn complete_from_map<'m, M, T, I>(
                self,
                map: &'m M,
                missing_element_value: T,
            ) -> FunVec<$dim, T, I, impl Fn(I) -> T + 'm>
            where
                T: Copy + 'm,
                M: KvMap<I, T>,
                I: FromIndex<$dim> + 'm,
            {
                FunVecBuilder::$fn()
                    .new(move |index: I| map.get(&index).copied().unwrap_or(missing_element_value))
            }
        }
    };
}

impl_builder!(FunVecBuilderD3, D3, d3);
impl_builder!(FunVecBuilderD4, D4, d4);
impl_builder!(FunVecBuilderD5, D5, d5);
impl_builder!(FunVecBuilderD6, D6, d6);
impl_builder!(FunVecBuilderD7, D7, d7);
impl_builder!(FunVecBuilderD8, D8, d8);
