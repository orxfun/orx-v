use super::{Dim, IdxNever};

pub struct D0;

impl Dim for D0 {
    const D: usize = 0;

    type Idx = IdxNever;

    type ChildIdx = IdxNever;

    type PrevDim = Self;
}
