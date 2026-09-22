pub trait Dim: Clone + Copy + 'static {
    const D: usize;

    type Idx;

    type PrevDim: Dim;

    type ChildIdx;
}
