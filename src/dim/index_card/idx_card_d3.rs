use super::CardIdx;
use crate::{dim::*, NVec, NVecCore, NVecCoreSealed};
use index_card::{CardEquality, Equality};
use index_sums::{IdxLeqD1, IdxLeqD2};

impl CardIdx<D3> for IdxLeqD2 {
    fn is_d0(&self) -> bool {
        matches!(self, Self::IdxD0(_))
    }

    fn card<T>(self, vec: &impl NVecCore<D3, T>) -> usize {
        match self {
            Self::IdxD0(_) => vec.core_num_children(),
            Self::IdxD1([i]) => vec.core_child(i).core_num_children(),
            Self::IdxD2([i, j]) => vec.core_child(i).core_child(j).core_num_children(),
        }
    }

    fn card_equality<T>(a: &impl NVecCore<D3, T>, b: &impl NVecCore<D3, T>) -> CardEquality<D3> {
        match (a.core_num_children(), b.core_num_children()) {
            (x, y) if x == y => {
                let unequal = (0..x)
                    .map(|i| {
                        (
                            i,
                            IdxLeqD1::card_equality(&a.core_child(i), &b.core_child(i)),
                        )
                    })
                    .find(|x| x.1 != CardEquality::Equal);

                if let Some((i, CardEquality::Unequal(idx, x, y))) = unequal {
                    return match idx {
                        IdxLeqD1::IdxD0(_) => CardEquality::Unequal(Self::IdxD1([i]), x, y),
                        IdxLeqD1::IdxD1([j]) => CardEquality::Unequal(Self::IdxD2([i, j]), x, y),
                    };
                }
                CardEquality::Equal
            }
            (x, y) => CardEquality::Unequal(Self::IdxD0([]), x, y),
        }
    }

    fn equality<T: PartialEq>(a: &impl NVec<D3, T>, b: &impl NVec<D3, T>) -> Equality<D3> {
        match (a.core_num_children(), b.core_num_children()) {
            (x, y) if x == y => {
                for i in 0..x {
                    match IdxLeqD1::equality(&a.child(i), &b.child(i)) {
                        Equality::Equal => {}
                        Equality::UnequalCard(idx, x, y) => match idx {
                            IdxLeqD1::IdxD0(_) => {
                                return Equality::UnequalCard(IdxLeqD2::IdxD1([i]), x, y)
                            }
                            IdxLeqD1::IdxD1([j]) => {
                                return Equality::UnequalCard(IdxLeqD2::IdxD2([i, j]), x, y)
                            }
                        },
                        Equality::UnequalValue([j, k]) => return Equality::UnequalValue([i, j, k]),
                    }
                }
                Equality::Equal
            }
            (x, y) => Equality::UnequalCard(Self::IdxD0([]), x, y),
        }
    }
}
