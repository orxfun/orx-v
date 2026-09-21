use super::super::{D1, D2, D3, D4, Dim, NVecNever, V};

pub struct FunVec1<T, F>(pub(super) F)
where
    F: Fn(<D1 as Dim>::Idx) -> T;

pub struct FunVec1ChildOfD2<T, F>(pub(super) F, pub(super) usize)
where
    F: Fn(<D2 as Dim>::Idx) -> T;

pub struct FunVec1ChildOfD3<T, F>(pub(super) F, pub(super) usize, pub(super) usize)
where
    F: Fn(<D3 as Dim>::Idx) -> T;

pub struct FunVec1ChildOfD4<T, F>(
    pub(super) F,
    pub(super) usize,
    pub(super) usize,
    pub(super) usize,
)
where
    F: Fn(<D4 as Dim>::Idx) -> T;

// impl V

impl<T, F> V<D1, T> for FunVec1<T, F>
where
    F: Fn(<D1 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        (self.0)(idx)
    }

    type Child = NVecNever;
}

impl<T, F> V<D1, T> for FunVec1ChildOfD2<T, F>
where
    F: Fn(<D2 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        (self.0)([self.1, idx])
    }

    type Child = NVecNever;
}

impl<T, F> V<D1, T> for FunVec1ChildOfD3<T, F>
where
    F: Fn(<D3 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        (self.0)([self.1, self.2, idx])
    }

    type Child = NVecNever;
}

impl<T, F> V<D1, T> for FunVec1ChildOfD4<T, F>
where
    F: Fn(<D4 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        (self.0)([self.1, self.2, self.3, idx])
    }

    type Child = NVecNever;
}
