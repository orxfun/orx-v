use super::{DNever, Dim, IdxNever};

pub struct D1;

impl Dim for D1 {
    const D: usize = 1;

    type Idx = usize;

    type ChildIdx = IdxNever;

    type PrevDim = DNever;
}
