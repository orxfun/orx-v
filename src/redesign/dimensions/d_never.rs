use super::Dim;

pub enum IdxNever {}

pub enum DNever {}

impl Dim for DNever {
    const D: usize = 0;

    type Idx = IdxNever;

    type ChildIdx = IdxNever;

    type PrevDim = Self;
}
