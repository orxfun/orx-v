use super::super::{D3, Dim};
use super::FunVec;

pub struct FunVec3<T, F>(FunVec<D3, T, F>)
where
    F: Fn(<D3 as Dim>::Idx) -> T;
