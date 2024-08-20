use super::into_index::IntoIndex;

pub trait Dim: Sized {
    const DIM: usize;

    type PREVIOUS: Dim;
    type Idx: Ord + IntoIndex<Self>;
}

pub trait MultiDim: Dim {}

pub struct D0;
impl Dim for D0 {
    const DIM: usize = 0;

    type PREVIOUS = D0;
    type Idx = [usize; 0];
}

pub struct D1;
impl Dim for D1 {
    const DIM: usize = 1;

    type PREVIOUS = D0;
    type Idx = [usize; Self::DIM];
}

pub struct D2;
impl Dim for D2 {
    const DIM: usize = 2;

    type PREVIOUS = D1;
    type Idx = [usize; Self::DIM];
}
impl MultiDim for D2 {}

pub struct D3;
impl Dim for D3 {
    const DIM: usize = 3;

    type PREVIOUS = D2;
    type Idx = [usize; Self::DIM];
}
impl MultiDim for D3 {}

pub struct D4;
impl Dim for D4 {
    const DIM: usize = 4;

    type PREVIOUS = D3;
    type Idx = [usize; Self::DIM];
}
impl MultiDim for D4 {}

pub struct D5;
impl Dim for D5 {
    const DIM: usize = 5;

    type PREVIOUS = D4;
    type Idx = [usize; Self::DIM];
}
impl MultiDim for D5 {}

pub struct D6;
impl Dim for D6 {
    const DIM: usize = 6;

    type PREVIOUS = D5;
    type Idx = [usize; Self::DIM];
}
impl MultiDim for D6 {}

pub struct D7;
impl Dim for D7 {
    const DIM: usize = 7;

    type PREVIOUS = D6;
    type Idx = [usize; Self::DIM];
}
impl MultiDim for D7 {}

pub struct D8;
impl Dim for D8 {
    const DIM: usize = 8;

    type PREVIOUS = D7;
    type Idx = [usize; Self::DIM];
}
impl MultiDim for D8 {}
