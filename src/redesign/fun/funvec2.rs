use super::super::{D2, D3, D4, Dim, NVecNever, V};
use super::{FunVec, FunVec1ChildOfD2};

pub struct FunVec2<T, F>(pub(super) F)
where
    F: Fn(<D2 as Dim>::Idx) -> T;

pub struct FunVec2ChildOfD3<T, F>(pub(super) F, pub(super) usize)
where
    F: Fn(<D3 as Dim>::Idx) -> T;

pub struct FunVec2ChildOfD4<T, F>(pub(super) F, pub(super) usize, pub(super) usize)
where
    F: Fn(<D4 as Dim>::Idx) -> T;

// impl V

impl<T, F> V<D2, T> for FunVec2<T, F>
where
    F: Fn(<D2 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D2 as Dim>::Idx) -> T {
        (self.0)(idx)
    }

    type Child = FunVec1ChildOfD2<T, F>;
}

// impl<T, F> V<D2, T> for FunVec2ChildOfD3<T, F>
// where
//     F: Fn(<D3 as Dim>::Idx) -> T,
// {
//     fn at(&self, [i, j]: <D2 as Dim>::Idx) -> T {
//         self.0.get([self.1, i, j])
//     }
// }

// impl<T, F> V<D2, T> for FunVec2ChildOfD4<T, F>
// where
//     F: Fn(<D4 as Dim>::Idx) -> T,
// {
//     fn at(&self, [i, j]: <D2 as Dim>::Idx) -> T {
//         self.0.get([self.1, self.2, i, j])
//     }
// }
