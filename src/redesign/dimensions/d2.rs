use super::{D1, Dim};

#[derive(Clone, Copy)]
pub struct D2;

impl Dim for D2 {
    type PrevDim = D1;

    const D: usize = 2;

    type Idx = [usize; Self::D];

    type ChildIdx = usize;

    fn combine_child_and_remining_indices(c: usize, i: <Self::PrevDim as Dim>::Idx) -> Self::Idx {
        [c, i]
    }
}
