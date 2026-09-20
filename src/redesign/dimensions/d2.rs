use super::{D1, Dim};

pub struct D2;

impl Dim for D2 {
    const D: usize = 2;

    type Idx = [usize; 2];

    type PrevDim = D1;
}
