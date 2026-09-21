use super::super::{D1, Dim};
use super::FunVec;

pub struct FunVec1<T, F>(FunVec<D1, T, F>)
where
    F: Fn(<D1 as Dim>::Idx) -> T;
