use super::{D3, Dim};

#[derive(Clone, Copy)]
pub struct D4;

impl Dim for D4 {
    const D: usize = 4;

    type Idx = [usize; 4];

    type ChildIdx = usize;

    type PrevDim = D3;
}
