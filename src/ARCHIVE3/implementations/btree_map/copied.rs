use crate::{dimensions::dim::*, FromIndex, FunVecBuilder, NVec, NVecRef};
use alloc::collections::btree_map::BTreeMap;

// copied_btree_map

pub trait CopiedBTreeMap<'v, N: Dim, T> {
    fn copied(&'v self) -> impl NVec<N, Option<T>>;
}

impl<'v, K, T> CopiedBTreeMap<'v, D1, T> for BTreeMap<K, &'v T>
where
    K: FromIndex<D1>,
    T: Copy,
{
    fn copied(&'v self) -> impl NVec<D1, Option<T>> {
        FunVecBuilder::d1().copied_btree_map(self)
    }
}

impl<'v, K, T> CopiedBTreeMap<'v, D2, T> for BTreeMap<K, &'v T>
where
    K: FromIndex<D2>,
    T: Copy,
{
    fn copied(&'v self) -> impl NVec<D2, Option<T>> {
        FunVecBuilder::d2().copied_btree_map(self)
    }
}

macro_rules! implement {
    ($dim:tt, $fn:ident) => {
        impl<'v, K, T> CopiedBTreeMap<'v, $dim, T> for BTreeMap<K, &'v T>
        where
            K: FromIndex<$dim>,
            T: Copy,
        {
            fn copied(&'v self) -> impl NVec<$dim, Option<T>> {
                FunVecBuilder::$fn().copied_btree_map(self)
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

pub trait CopiedBTreeMapOpt<'v, N: Dim, T> {
    fn copied(&'v self) -> impl NVec<N, Option<Option<T>>>;
}

impl<'v, V, T> CopiedBTreeMapOpt<'v, D2, T> for BTreeMap<usize, V>
where
    T: Copy + 'v,
    V: NVecRef<<D2 as Dim>::PREVIOUS, Element<'v> = Option<&'v T>> + 'v,
{
    fn copied(&'v self) -> impl NVec<D2, Option<Option<T>>> {
        FunVecBuilder::d2().copied_btree_map_opt(self)
    }
}

macro_rules! implement_opt {
    ($dim:tt, $fn:ident) => {
        impl<'v, V, T> CopiedBTreeMapOpt<'v, $dim, T> for BTreeMap<usize, V>
        where
            T: Copy + 'v,
            V: NVecRef<<$dim as Dim>::PREVIOUS, Element<'v> = Option<&'v T>> + 'v,
        {
            fn copied(&'v self) -> impl NVec<$dim, Option<Option<T>>> {
                FunVecBuilder::$fn().copied_btree_map_opt(self)
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
