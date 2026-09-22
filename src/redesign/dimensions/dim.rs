pub trait Dim: Clone + Copy {
    const D: usize;

    type Idx;

    type PrevDim: Dim;

    type ChildIdx;
}
