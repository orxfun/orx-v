use crate::dim::*;

pub trait SplitIdx {
    type LesserIdx;

    fn split_idx(self) -> (usize, Self::LesserIdx);
}

impl SplitIdx for <D1 as Dim>::Idx {
    type LesserIdx = [usize; 0];
    #[inline(always)]
    fn split_idx(self) -> (usize, Self::LesserIdx) {
        (self[0], [])
    }
}

impl SplitIdx for <D2 as Dim>::Idx {
    type LesserIdx = <D1 as Dim>::Idx;
    #[inline(always)]
    fn split_idx(self) -> (usize, Self::LesserIdx) {
        (self[0], [self[1]])
    }
}

impl SplitIdx for <D3 as Dim>::Idx {
    type LesserIdx = <D2 as Dim>::Idx;
    #[inline(always)]
    fn split_idx(self) -> (usize, Self::LesserIdx) {
        (self[0], [self[1], self[2]])
    }
}

impl SplitIdx for <D4 as Dim>::Idx {
    type LesserIdx = <D3 as Dim>::Idx;
    #[inline(always)]
    fn split_idx(self) -> (usize, Self::LesserIdx) {
        (self[0], [self[1], self[2], self[3]])
    }
}
