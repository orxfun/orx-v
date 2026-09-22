pub trait Dim: Clone + Copy + 'static {
    type PrevDim: Dim;

    const D: usize;

    type Idx;

    fn combine_child_and_remining_indices(c: usize, r: <Self::PrevDim as Dim>::Idx) -> Self::Idx;
}
