use super::{DNever, Dim, IdxNever};

#[derive(Clone, Copy)]
pub struct D1;

impl Dim for D1 {
    type PrevDim = DNever;

    const D: usize = 1;

    type Idx = usize;

    type ChildIdx = IdxNever;

    fn combine_child_and_remining_indices(
        _: IdxNever,
        _: <Self::PrevDim as Dim>::Idx,
    ) -> Self::Idx {
        unreachable!()
    }
}
