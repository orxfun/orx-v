use super::{DNever, Dim};

pub struct D0;

impl Dim for D0 {
    const D: usize = 0;

    type Idx = [usize; 0];

    type PrevDim = DNever;
}
