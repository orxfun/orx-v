use super::Dim;

pub enum IdxNever {}

#[derive(Clone, Copy)]
pub struct DNever;

impl Dim for DNever {
    const D: usize = 0;

    type Idx = IdxNever;

    type PrevDim = Self;

    fn combine_child_and_remining_indices(_: usize, _: <Self::PrevDim as Dim>::Idx) -> Self::Idx {
        unreachable!()
    }
}
