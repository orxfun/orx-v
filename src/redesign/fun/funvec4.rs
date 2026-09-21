use super::super::{D4, Dim, V};
use super::FunVec3ChildOfD4;

pub struct FunVec4<'a, T, F>(&'a F)
where
    F: Fn(<D4 as Dim>::Idx) -> T;

// impl V

impl<T, F> V<D4, T> for FunVec4<'_, T, F>
where
    F: Fn(<D4 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D4 as Dim>::Idx) -> T {
        (self.0)(idx)
    }

    type Child<'a>
        = FunVec3ChildOfD4<'a, T, F>
    where
        Self: 'a;

    fn child(&self, idx: <D4 as Dim>::ChildIdx) -> Self::Child<'_> {
        FunVec3ChildOfD4(self.0, idx)
    }
}
