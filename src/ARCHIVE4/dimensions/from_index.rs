use super::dim::*;

pub trait FromIndex<D: Dim>: Ord {
    fn from_index(index: D::Idx) -> Self;
}

impl<D: Dim> FromIndex<D> for D::Idx {
    #[inline(always)]
    fn from_index(index: D::Idx) -> Self {
        index
    }
}

impl FromIndex<D1> for usize {
    #[inline(always)]
    fn from_index(index: <D1 as Dim>::Idx) -> Self {
        index[0]
    }
}

impl FromIndex<D2> for (usize, usize) {
    #[inline(always)]
    fn from_index(index: <D2 as Dim>::Idx) -> Self {
        (index[0], index[1])
    }
}

impl FromIndex<D3> for (usize, usize, usize) {
    #[inline(always)]
    fn from_index(index: <D3 as Dim>::Idx) -> Self {
        (index[0], index[1], index[2])
    }
}

impl FromIndex<D4> for (usize, usize, usize, usize) {
    #[inline(always)]
    fn from_index(index: <D4 as Dim>::Idx) -> Self {
        (index[0], index[1], index[2], index[3])
    }
}

impl FromIndex<D5> for (usize, usize, usize, usize, usize) {
    #[inline(always)]
    fn from_index(index: <D5 as Dim>::Idx) -> Self {
        (index[0], index[1], index[2], index[3], index[4])
    }
}

impl FromIndex<D6> for (usize, usize, usize, usize, usize, usize) {
    #[inline(always)]
    fn from_index(index: <D6 as Dim>::Idx) -> Self {
        (index[0], index[1], index[2], index[3], index[4], index[5])
    }
}

impl FromIndex<D7> for (usize, usize, usize, usize, usize, usize, usize) {
    #[inline(always)]
    fn from_index(index: <D7 as Dim>::Idx) -> Self {
        (
            index[0], index[1], index[2], index[3], index[4], index[5], index[6],
        )
    }
}

impl FromIndex<D8> for (usize, usize, usize, usize, usize, usize, usize, usize) {
    #[inline(always)]
    fn from_index(index: <D8 as Dim>::Idx) -> Self {
        (
            index[0], index[1], index[2], index[3], index[4], index[5], index[6], index[7],
        )
    }
}
