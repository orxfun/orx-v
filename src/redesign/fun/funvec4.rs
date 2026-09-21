use super::super::{D4, Dim, V};
use super::FunVec;

pub struct FunVec4<T, F>(FunVec<D4, T, F>)
where
    F: Fn(<D4 as Dim>::Idx) -> T;

// impl V

impl<T, F> V<D4, T> for FunVec4<T, F>
where
    F: Fn(<D4 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D4 as Dim>::Idx) -> T {
        self.0.get(idx)
    }
}
