use super::super::{D3, D4, Dim, V};
use super::{FunVec2ChildOfD3, FunVec2ChildOfD4};

pub struct FunVec3<'a, T, F>(&'a F)
where
    F: Fn(<D3 as Dim>::Idx) -> T;

impl<'a, T, F> FunVec3<'a, T, F>
where
    F: Fn(<D3 as Dim>::Idx) -> T,
{
    pub fn new(f: &'a F) -> Self {
        Self(f)
    }
}

pub struct FunVec3ChildOfD4<'a, T, F>(pub(super) &'a F, pub(super) usize)
where
    F: Fn(<D4 as Dim>::Idx) -> T;

// impl V

impl<T, F> V<D3, T> for FunVec3<'_, T, F>
where
    F: Fn(<D3 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D3 as Dim>::Idx) -> T {
        (self.0)(idx)
    }

    type Child<'a>
        = FunVec2ChildOfD3<'a, T, F>
    where
        Self: 'a;

    fn child(&self, idx: <D3 as Dim>::ChildIdx) -> Self::Child<'_> {
        FunVec2ChildOfD3(self.0, idx)
    }
}

impl<T, F> V<D3, T> for FunVec3ChildOfD4<'_, T, F>
where
    F: Fn(<D4 as Dim>::Idx) -> T,
{
    fn at(&self, [i, j, k]: <D3 as Dim>::Idx) -> T {
        (self.0)([self.1, i, j, k])
    }

    type Child<'a>
        = FunVec2ChildOfD4<'a, T, F>
    where
        Self: 'a;

    fn child(&self, idx: <D3 as Dim>::ChildIdx) -> Self::Child<'_> {
        FunVec2ChildOfD4(self.0, self.1, idx)
    }
}
