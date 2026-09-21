use super::{D2, Dim};

pub struct D3;

impl Dim for D3 {
    const D: usize = 3;

    type Idx = [usize; 3];

    type ChildIdx = usize;

    type PrevDim = D2;
}
