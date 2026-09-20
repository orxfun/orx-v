use super::Dim;

pub enum DNever {}

impl Dim for DNever {
    const D: usize = 0;

    type Idx = [usize; 0];

    type PrevDim = Self;
}
