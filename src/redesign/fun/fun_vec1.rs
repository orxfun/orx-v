use super::super::{D1, D2, D3, D4, Dim, NVecNever, V};
use core::marker::PhantomData;
use orx_self_or::SoR;

pub struct FunVec1<D, S, T, Fr>(S, Fr, PhantomData<(D, T)>)
where
    S: SoR<D>,
    Fr: Fn(&D, <D1 as Dim>::Idx) -> &T;

impl<D, S, T, Fr> FunVec1<D, S, T, Fr>
where
    S: SoR<D>,
    Fr: Fn(&D, <D1 as Dim>::Idx) -> &T,
{
    pub fn new(data: S, f: Fr) -> Self {
        Self(data, f, PhantomData)
    }
}

// impl V

// arch

pub struct FunVec1Zzz<'a, T, F>(&'a F)
where
    F: Fn(<D1 as Dim>::Idx) -> T;

impl<'a, T, F> FunVec1Zzz<'a, T, F>
where
    F: Fn(<D1 as Dim>::Idx) -> T,
{
    pub fn new(f: &'a F) -> Self {
        Self(f)
    }
}

pub struct FunVec1ChildOfD2<'a, T, F>(pub(super) &'a F, pub(super) usize)
where
    F: Fn(<D2 as Dim>::Idx) -> T;

pub struct FunVec1ChildOfD3<'a, T, F>(pub(super) &'a F, pub(super) usize, pub(super) usize)
where
    F: Fn(<D3 as Dim>::Idx) -> T;

pub struct FunVec1ChildOfD4<'a, T, F>(
    pub(super) &'a F,
    pub(super) usize,
    pub(super) usize,
    pub(super) usize,
)
where
    F: Fn(<D4 as Dim>::Idx) -> T;

// impl V

impl<T, F> V<D1, T> for FunVec1Zzz<'_, T, F>
where
    F: Fn(<D1 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        (self.0)(idx)
    }

    type Child<'a>
        = NVecNever
    where
        Self: 'a;
}

impl<T, F> V<D1, T> for FunVec1ChildOfD2<'_, T, F>
where
    F: Fn(<D2 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        (self.0)([self.1, idx])
    }

    type Child<'a>
        = NVecNever
    where
        Self: 'a;
}

impl<T, F> V<D1, T> for FunVec1ChildOfD3<'_, T, F>
where
    F: Fn(<D3 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        (self.0)([self.1, self.2, idx])
    }

    type Child<'a>
        = NVecNever
    where
        Self: 'a;
}

impl<T, F> V<D1, T> for FunVec1ChildOfD4<'_, T, F>
where
    F: Fn(<D4 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        (self.0)([self.1, self.2, self.3, idx])
    }

    type Child<'a>
        = NVecNever
    where
        Self: 'a;
}
