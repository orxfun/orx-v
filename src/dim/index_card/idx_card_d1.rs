use super::CardIdx;
use crate::{dim::*, NVec, NVecCore};
use index_card::{CardEquality, Equality};
use index_sums::IdxLeqD0;

impl CardIdx<D1> for IdxLeqD0 {
    fn is_d0(&self) -> bool {
        true
    }

    fn card<T>(self, vec: &impl NVecCore<D1, T>) -> usize {
        match self {
            Self::IdxD0(_) => vec.core_num_children(),
        }
    }

    fn card_equality<T>(a: &impl NVecCore<D1, T>, b: &impl NVecCore<D1, T>) -> CardEquality<D1> {
        match (a.core_num_children(), b.core_num_children()) {
            (x, y) if x == y => CardEquality::Equal,
            (x, y) => CardEquality::Unequal(Self::IdxD0([]), x, y),
        }
    }

    fn equality<T: PartialEq>(a: &impl NVec<D1, T>, b: &impl NVec<D1, T>) -> Equality<D1> {
        match (a.core_num_children(), b.core_num_children()) {
            (x, y) if x == y => {
                for i in 0..a.core_num_children() {
                    if a.at(i) != b.at(i) {
                        return Equality::UnequalValue([i]);
                    }
                }
                Equality::Equal
            }
            (x, y) => Equality::UnequalCard(Self::IdxD0([]), x, y),
        }
    }
}
