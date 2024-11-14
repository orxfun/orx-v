use super::CardIdx;
use crate::{dim::*, NVec, NVecCore, NVecCoreSealed};
use index_card::{CardEquality, Equality};
use index_sums::{IdxLeqD2, IdxLeqD3};

impl CardIdx<D4> for IdxLeqD3 {
    fn is_d0(&self) -> bool {
        matches!(self, Self::IdxD0(_))
    }

    fn card<T>(self, vec: &impl NVecCore<D4, T>) -> usize {
        match self {
            Self::IdxD0(_) => vec.core_num_children(),
            Self::IdxD1([i]) => vec.core_child(i).core_num_children(),
            Self::IdxD2([i, j]) => vec.core_child(i).core_child(j).core_num_children(),
            Self::IdxD3([i, j, k]) => vec
                .core_child(i)
                .core_child(j)
                .core_child(k)
                .core_num_children(),
        }
    }

    fn card_equality<T>(a: &impl NVecCore<D4, T>, b: &impl NVecCore<D4, T>) -> CardEquality<D4> {
        match (a.core_num_children(), b.core_num_children()) {
            (x, y) if x == y => {
                let unequal = (0..x)
                    .map(|i| {
                        (
                            i,
                            IdxLeqD2::card_equality(&a.core_child(i), &b.core_child(i)),
                        )
                    })
                    .find(|x| x.1 != CardEquality::Equal);

                if let Some((i, CardEquality::Unequal(idx, x, y))) = unequal {
                    return match idx {
                        IdxLeqD2::IdxD0(_) => CardEquality::Unequal(Self::IdxD1([i]), x, y),
                        IdxLeqD2::IdxD1([j]) => CardEquality::Unequal(Self::IdxD2([i, j]), x, y),
                        IdxLeqD2::IdxD2([j, k]) => {
                            CardEquality::Unequal(Self::IdxD3([i, j, k]), x, y)
                        }
                    };
                }
                CardEquality::Equal
            }
            (x, y) => CardEquality::Unequal(Self::IdxD0([]), x, y),
        }
    }

    fn equality<T: PartialEq>(a: &impl NVec<D4, T>, b: &impl NVec<D4, T>) -> Equality<D4> {
        match (a.core_num_children(), b.core_num_children()) {
            (x, y) if x == y => {
                for i in 0..x {
                    match IdxLeqD2::equality(&a.child(i), &b.child(i)) {
                        Equality::Equal => {}
                        Equality::UnequalCard(idx, x, y) => match idx {
                            IdxLeqD2::IdxD0(_) => {
                                return Equality::UnequalCard(IdxLeqD3::IdxD1([i]), x, y)
                            }
                            IdxLeqD2::IdxD1([j]) => {
                                return Equality::UnequalCard(IdxLeqD3::IdxD2([i, j]), x, y)
                            }
                            IdxLeqD2::IdxD2([j, k]) => {
                                return Equality::UnequalCard(IdxLeqD3::IdxD3([i, j, k]), x, y)
                            }
                        },
                        Equality::UnequalValue([j, k, l]) => {
                            return Equality::UnequalValue([i, j, k, l])
                        }
                    }
                }
                Equality::Equal
            }
            (x, y) => Equality::UnequalCard(Self::IdxD0([]), x, y),
        }
    }
}
