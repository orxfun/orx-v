use crate::{dimensions::dim::*, FunVecBuilder, NVec, NVecRef};

// copied_vec

pub trait CopiedVec<'v, N: Dim, T> {
    fn copied(&'v self) -> impl NVec<N, T>;
}

impl<'v, T: Copy> CopiedVec<'v, D1, T> for Vec<&'v T> {
    fn copied(&'v self) -> impl NVec<D1, T> {
        FunVecBuilder::d1().copied_vec(self)
    }
}

impl<'v, V, T> CopiedVec<'v, D2, T> for Vec<V>
where
    V: NVec<<D2 as Dim>::PREVIOUS, &'v T> + 'v,
    T: Copy + 'v,
{
    fn copied(&'v self) -> impl NVec<D2, T> {
        FunVecBuilder::d2().copied_vec(self)
    }
}

macro_rules! implement {
    ($dim:tt, $fn:ident) => {
        impl<'v, V, T> CopiedVec<'v, $dim, T> for Vec<V>
        where
            V: NVec<<$dim as Dim>::PREVIOUS, &'v T> + 'v,
            T: Copy + 'v,
        {
            fn copied(&'v self) -> impl NVec<$dim, T> {
                FunVecBuilder::$fn().copied_vec(self)
            }
        }
    };
}

implement!(D3, d3);
implement!(D4, d4);
implement!(D5, d5);
implement!(D6, d6);
implement!(D7, d7);
implement!(D8, d8);

// copied_vec_opt

pub trait CopiedVecOpt<'v, N: Dim, T> {
    fn copied(&'v self) -> impl NVec<N, Option<T>>;
}

impl<'v, V, T> CopiedVecOpt<'v, D2, T> for Vec<V>
where
    T: Copy + 'v,
    V: NVecRef<<D2 as Dim>::PREVIOUS, Element<'v> = Option<&'v T>> + 'v,
{
    fn copied(&'v self) -> impl NVec<D2, Option<T>> {
        FunVecBuilder::d2().copied_vec_opt(self)
    }
}

macro_rules! implement_opt {
    ($dim:tt, $fn:ident) => {
        impl<'v, V, T> CopiedVecOpt<'v, $dim, T> for Vec<V>
        where
            T: Copy + 'v,
            V: NVecRef<<$dim as Dim>::PREVIOUS, Element<'v> = Option<&'v T>> + 'v,
        {
            fn copied(&'v self) -> impl NVec<$dim, Option<T>> {
                FunVecBuilder::$fn().copied_vec_opt(self)
            }
        }
    };
}

implement_opt!(D3, d3);
implement_opt!(D4, d4);
implement_opt!(D5, d5);
implement_opt!(D6, d6);
implement_opt!(D7, d7);
implement_opt!(D8, d8);
