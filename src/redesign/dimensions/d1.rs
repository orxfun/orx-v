use super::Dim;

pub struct D1;

impl Dim for D1 {
    const D: usize = 1;

    type Idx = usize;
}
