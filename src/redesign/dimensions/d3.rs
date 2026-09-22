use super::{D2, Dim};

#[derive(Clone, Copy)]
pub struct D3;

impl Dim for D3 {
    type PrevDim = D2;

    const D: usize = 3;

    type Idx = [usize; Self::D];

    fn combine_child_and_remining_indices(
        c: usize,
        [i, j]: <Self::PrevDim as Dim>::Idx,
    ) -> Self::Idx {
        [c, i, j]
    }
}
