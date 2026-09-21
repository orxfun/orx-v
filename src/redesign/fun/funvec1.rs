use super::super::{D1, D2, D3, D4, Dim, V};
use super::FunVec;

pub struct FunVec1<T, F>(FunVec<D1, T, F>)
where
    F: Fn(<D1 as Dim>::Idx) -> T;

pub struct FunVec1ChildOfD2<T, F>(FunVec<D2, T, F>, usize)
where
    F: Fn(<D2 as Dim>::Idx) -> T;

pub struct FunVec1ChildOfD3<T, F>(FunVec<D3, T, F>, usize, usize)
where
    F: Fn(<D3 as Dim>::Idx) -> T;

pub struct FunVec1ChildOfD4<T, F>(FunVec<D4, T, F>, usize, usize, usize)
where
    F: Fn(<D4 as Dim>::Idx) -> T;

// impl V

impl<T, F> V<D1, T> for FunVec1<T, F>
where
    F: Fn(<D1 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        self.0.get(idx)
    }
}

impl<T, F> V<D1, T> for FunVec1ChildOfD2<T, F>
where
    F: Fn(<D2 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        self.0.get([self.1, idx])
    }
}

impl<T, F> V<D1, T> for FunVec1ChildOfD3<T, F>
where
    F: Fn(<D3 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        self.0.get([self.1, self.2, idx])
    }
}

impl<T, F> V<D1, T> for FunVec1ChildOfD4<T, F>
where
    F: Fn(<D4 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        self.0.get([self.1, self.2, self.3, idx])
    }
}
