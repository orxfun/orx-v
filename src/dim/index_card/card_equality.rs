use crate::dim::Dim;
use core::fmt::Debug;
use core::fmt::Display;

/// Result of an [`card_equality`] check of two vectors of the same dimension.
///
/// The result can be
/// * [`CardEquality::Equal`] iff the cardinality of the structures as
///   well as all values at corresponding positions are equal.
/// * [`CardEquality::Unequal`] if cardinalities do not agree at at least one
///   level.
///
/// [`card_equality`]: crate::NVec::card_equality
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CardEquality<D: Dim> {
    /// Cardinality of the structures and all their corresponding children have equal cardinalities.
    Equal,
    /// Cardinalities do not agree at at least one level.
    /// The tuple `(idx, card1, card2)` represents the following:
    /// * `idx` is the place the inequality in cardinalities are observed;
    /// * `card1` and `card2` are the unequal cardinalities at the given `idx` in the first and
    ///   second vectors, respectively.
    Unequal(D::CardIdx, usize, usize),
}

impl<D: Dim> Display for CardEquality<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<D: Dim> Debug for CardEquality<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Equal => write!(f, "Equal"),
            Self::Unequal(idx, x, y) => {
                write!(
                    f,
                    "UnequalCard {{ idx: {:?}, lhs.card({:?}): {:?}, rhs.card({:?}): {:?} }}",
                    idx, idx, x, idx, y
                )
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{IdxLeqD0, IdxLeqD1, IdxLeqD2};
//     use alloc::vec;

//     #[test]
//     fn eq_card_d1() {
//         let v1 = vec![1, 2, 3];
//         let v2 = vec![7, 3, 4];

//         assert_eq!(v1.card_equality(&v2), CardEquality::Equal);
//         assert_eq!(v2.card_equality(&v1), CardEquality::Equal);
//     }

//     #[test]
//     fn eq_card_d2() {
//         let v1 = vec![vec![1, 2], vec![1, 2, 3]];
//         let v2 = vec![vec![11, 12], vec![11, 12, 13]];

//         assert_eq!(v1.card_equality(&v2), CardEquality::Equal);
//     }

//     #[test]
//     fn eq_card_d3() {
//         let v1 = vec![vec![vec![1], vec![1, 2]], vec![vec![1, 2, 3]]];
//         let v2 = vec![vec![vec![1], vec![1, 2]], vec![vec![1, 2, 3]]];

//         assert_eq!(v1.card_equality(&v2), CardEquality::Equal);
//     }

//     #[test]
//     fn eq_card_d4() {
//         let v1 = vec![
//             vec![
//                 vec![vec![1, 2, 3, 4]],
//                 vec![vec![1, 2], vec![1, 2, 3, 4, 5]],
//             ],
//             vec![vec![vec![], vec![1, 2], vec![1, 2, 3]]],
//         ];
//         let v2 = vec![
//             vec![
//                 vec![vec![1, 2, 3, 4]],
//                 vec![vec![1, 2], vec![1, 2, 3, 4, 5]],
//             ],
//             vec![vec![vec![], vec![1, 2], vec![1, 2, 3]]],
//         ];

//         assert_eq!(v1.card_equality(&v2), CardEquality::Equal);
//     }

//     #[test]
//     fn ineq_card_d1() {
//         let v1 = vec![1, 2, 3];
//         let v2 = vec![7, 3, 4, 7];

//         assert_eq!(
//             v1.card_equality(&v2),
//             CardEquality::Unequal(IdxLeqD0::IdxD0([]), 3, 4)
//         );
//         assert_eq!(
//             v2.card_equality(&v1),
//             CardEquality::Unequal(IdxLeqD0::IdxD0([]), 4, 3)
//         );
//     }

//     #[test]
//     fn ineq_card_d2() {
//         let v1 = vec![vec![1, 2], vec![1, 2, 3]];
//         let v2 = vec![vec![11, 12], vec![11, 12, 13], vec![0]];

//         assert_eq!(
//             v1.card_equality(&v2),
//             CardEquality::Unequal(IdxLeqD1::IdxD0([]), 2, 3)
//         );

//         let v1 = vec![vec![1, 2], vec![1, 2, 3]];
//         let v2 = vec![vec![11, 12], vec![11, 12, 13, 42]];

//         assert_eq!(
//             v1.card_equality(&v2),
//             CardEquality::Unequal(IdxLeqD1::IdxD1([1]), 3, 4)
//         );
//     }

//     #[test]
//     fn ineq_card_d3() {
//         let v1 = vec![vec![vec![1], vec![1, 2]], vec![vec![1, 2, 3]]];
//         let v2 = vec![vec![vec![1], vec![1, 2]], vec![vec![1, 2, 3]], vec![]];

//         assert_eq!(
//             v1.card_equality(&v2),
//             CardEquality::Unequal(IdxLeqD2::IdxD0([]), 2, 3)
//         );

//         let v1 = vec![vec![vec![1], vec![1, 2]], vec![vec![1, 2, 3]]];
//         let v2 = vec![vec![vec![1], vec![1, 2], vec![42]], vec![vec![1, 2, 3]]];

//         assert_eq!(
//             v1.card_equality(&v2),
//             CardEquality::Unequal(IdxLeqD2::IdxD1([0]), 2, 3)
//         );

//         let v1 = vec![vec![vec![1], vec![1, 2]], vec![vec![1, 2, 3]]];
//         let v2 = vec![vec![vec![1], vec![1, 2, 42]], vec![vec![1, 2, 3]]];

//         assert_eq!(
//             v1.card_equality(&v2),
//             CardEquality::Unequal(IdxLeqD2::IdxD2([0, 1]), 2, 3)
//         );
//     }
// }
