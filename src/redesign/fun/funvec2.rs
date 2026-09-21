use super::super::{D2, Dim};
use super::FunVec;

pub struct FunVec2<T, F>(FunVec<D2, T, F>)
where
    F: Fn(<D2 as Dim>::Idx) -> T;
