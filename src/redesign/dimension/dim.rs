pub trait Dim: Clone + Copy + 'static {
    type PrevDim: Dim;

    const D: usize;

    type Idx;

    type ChildIdx;

    fn combine_child_and_remining_indices(
        c: Self::ChildIdx,
        r: <Self::PrevDim as Dim>::Idx,
    ) -> Self::Idx;
}
