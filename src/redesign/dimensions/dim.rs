pub trait Dim {
    const D: usize;

    type Idx;

    type PrevDim: Dim;
}
