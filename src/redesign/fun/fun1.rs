use super::super::{D1, Dim};
use super::{FunRef, FunVal};

pub struct Fun1;

impl Fun1 {
    pub fn val<T, F>(f: F) -> FunVal<D1, T, F>
    where
        F: Fn(usize) -> T,
    {
        FunVal::new(f)
    }

    pub fn rf<'a, T, F>(f: F) -> FunRef<'a, D1, T, F>
    where
        F: Fn(usize) -> &'a T,
    {
        FunRef::new(f)
    }
}
