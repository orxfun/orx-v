use super::super::{D4, Dim};
use super::FunVec;

pub struct FunVec4<T, F>(FunVec<D4, T, F>)
where
    F: Fn(<D4 as Dim>::Idx) -> T;
