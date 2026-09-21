use super::super::{D2, D3, D4, Dim, V};
use super::FunVec;

pub struct FunVec2<T, F>(FunVec<D2, T, F>)
where
    F: Fn(<D2 as Dim>::Idx) -> T;

pub struct FunVec2ChildOfD3<T, F>(FunVec<D3, T, F>, usize)
where
    F: Fn(<D3 as Dim>::Idx) -> T;

pub struct FunVec2ChildOfD4<T, F>(FunVec<D4, T, F>, usize, usize)
where
    F: Fn(<D4 as Dim>::Idx) -> T;

// impl V

impl<T, F> V<D2, T> for FunVec2<T, F>
where
    F: Fn(<D2 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D2 as Dim>::Idx) -> T {
        self.0.get(idx)
    }
}

impl<T, F> V<D2, T> for FunVec2ChildOfD3<T, F>
where
    F: Fn(<D3 as Dim>::Idx) -> T,
{
    fn at(&self, [i, j]: <D2 as Dim>::Idx) -> T {
        self.0.get([self.1, i, j])
    }
}

impl<T, F> V<D2, T> for FunVec2ChildOfD4<T, F>
where
    F: Fn(<D4 as Dim>::Idx) -> T,
{
    fn at(&self, [i, j]: <D2 as Dim>::Idx) -> T {
        self.0.get([self.1, self.2, i, j])
    }
}
