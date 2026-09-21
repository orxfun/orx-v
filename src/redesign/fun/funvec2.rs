use super::super::{D2, D3, D4, Dim};
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
