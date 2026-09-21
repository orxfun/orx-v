use super::super::{D2, D3, D4, Dim, V};
use super::{FunVec1ChildOfD2, FunVec1ChildOfD3, FunVec1ChildOfD4};

pub struct FunVec2<'a, T, F>(&'a F)
where
    F: Fn(<D2 as Dim>::Idx) -> T;

impl<'a, T, F> FunVec2<'a, T, F>
where
    F: Fn(<D2 as Dim>::Idx) -> T,
{
    pub fn new(f: &'a F) -> Self {
        Self(f)
    }
}

pub struct FunVec2ChildOfD3<'a, T, F>(pub(super) &'a F, pub(super) usize)
where
    F: Fn(<D3 as Dim>::Idx) -> T;

pub struct FunVec2ChildOfD4<'a, T, F>(pub(super) &'a F, pub(super) usize, pub(super) usize)
where
    F: Fn(<D4 as Dim>::Idx) -> T;

// impl V

// TODO: impl V for self (FunVec2) too

impl<T, F> V<D2, T> for &FunVec2<'_, T, F>
where
    F: Fn(<D2 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D2 as Dim>::Idx) -> T {
        (self.0)(idx)
    }

    type Child<'a>
        = FunVec1ChildOfD2<'a, T, F>
    where
        Self: 'a;

    fn child(&self, idx: <D2 as Dim>::ChildIdx) -> Self::Child<'_> {
        FunVec1ChildOfD2(self.0, idx)
    }
}

impl<T, F> V<D2, T> for FunVec2ChildOfD3<'_, T, F>
where
    F: Fn(<D3 as Dim>::Idx) -> T,
{
    fn at(&self, [i, j]: <D2 as Dim>::Idx) -> T {
        (self.0)([self.1, i, j])
    }

    type Child<'a>
        = FunVec1ChildOfD3<'a, T, F>
    where
        Self: 'a;

    fn child(&self, idx: <D2 as Dim>::ChildIdx) -> Self::Child<'_> {
        FunVec1ChildOfD3(self.0, self.1, idx)
    }
}

impl<T, F> V<D2, T> for FunVec2ChildOfD4<'_, T, F>
where
    F: Fn(<D4 as Dim>::Idx) -> T,
{
    fn at(&self, [i, j]: <D2 as Dim>::Idx) -> T {
        (self.0)([self.1, self.2, i, j])
    }

    type Child<'a>
        = FunVec1ChildOfD4<'a, T, F>
    where
        Self: 'a;

    fn child(&self, idx: <D2 as Dim>::ChildIdx) -> Self::Child<'_> {
        FunVec1ChildOfD4(self.0, self.1, self.2, idx)
    }
}
