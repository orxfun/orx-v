use super::{D3, Dim};

#[derive(Clone, Copy)]
pub struct D4;

impl Dim for D4 {
    type PrevDim = D3;

    const D: usize = 4;

    type Idx = [usize; Self::D];

    fn combine_child_and_remining_indices(
        c: usize,
        [i, j, k]: <Self::PrevDim as Dim>::Idx,
    ) -> Self::Idx {
        [c, i, j, k]
    }
}
