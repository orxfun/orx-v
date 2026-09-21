use super::super::{D3, D4, Dim};
use super::FunVec;

pub struct FunVec3<T, F>(FunVec<D3, T, F>)
where
    F: Fn(<D3 as Dim>::Idx) -> T;

pub struct FunVec3ChildOfD4<T, F>(FunVec<D4, T, F>, usize)
where
    F: Fn(<D4 as Dim>::Idx) -> T;
