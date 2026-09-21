use super::super::{D3, D4, Dim, V};
use super::FunVec;

pub struct FunVec3<T, F>(FunVec<D3, T, F>)
where
    F: Fn(<D3 as Dim>::Idx) -> T;

pub struct FunVec3ChildOfD4<T, F>(FunVec<D4, T, F>, usize)
where
    F: Fn(<D4 as Dim>::Idx) -> T;

// impl V

impl<T, F> V<D3, T> for FunVec3<T, F>
where
    F: Fn(<D3 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D3 as Dim>::Idx) -> T {
        self.0.get(idx)
    }
}

impl<T, F> V<D3, T> for FunVec3ChildOfD4<T, F>
where
    F: Fn(<D4 as Dim>::Idx) -> T,
{
    fn at(&self, [i, j, k]: <D3 as Dim>::Idx) -> T {
        self.0.get([self.1, i, j, k])
    }
}
