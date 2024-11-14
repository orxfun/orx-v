use crate::dim::Dim;
use core::fmt::Debug;
use core::fmt::Display;

/// Result of an [`equality`] check of two vectors of the same dimension.
///
/// The result can be
/// * [`Equality::Equal`] iff the cardinality of the structures as
///   well as all values at corresponding positions are equal.
/// * [`Equality::UnequalCard`] if cardinalities do not agree at at least one
///   level.
/// * [`Equality::UnequalValue`] if any of the values are different.
///
/// [`equality`]: crate::NVec::equality
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Equality<D: Dim> {
    /// Cardinality of the structures as well as all values at corresponding positions are equal.
    Equal,
    /// Cardinalities do not agree at at least one level.
    /// The tuple `(idx, card1, card2)` represents the following:
    /// * `idx` is the place the inequality in cardinalities are observed;
    /// * `card1` and `card2` are the unequal cardinalities at the given `idx` in the first and
    ///   second vectors, respectively.
    UnequalCard(D::CardIdx, usize, usize),
    /// Values of at least one of the element is different.
    /// The `(idx)` represents the index where the value inequality is observed.
    UnequalValue(D::Idx),
}

impl<D: Dim> Debug for Equality<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Equal => write!(f, "Equal"),
            Self::UnequalCard(idx, x, y) => {
                write!(
                    f,
                    "UnequalCard {{ idx: {:?}, lhs.card({:?}): {:?}, rhs.card({:?}): {:?} }}",
                    idx, idx, x, idx, y
                )
            }
            Self::UnequalValue(idx) => {
                write!(f, "UnequalValue {{ idx: {:?} }}", idx)
            }
        }
    }
}

impl<D: Dim> Display for Equality<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{dim::Equality, IdxLeqD0, IdxLeqD1, IdxLeqD2, IdxLeqD3, NVec, NVecCoreSealed};
//     use alloc::vec;

//     #[test]
//     fn eq_d1() {
//         let v1 = vec![1, 2, 3];
//         let v2 = vec![1, 2, 3];

//         assert_eq!(v1.equality(&v2), Equality::Equal);
//         assert_eq!(v2.equality(&v1), Equality::Equal);
//     }

//     #[test]
//     fn eq_d2() {
//         let v1 = vec![vec![1, 2], vec![1, 2, 3]];
//         let v2 = vec![vec![1, 2], vec![1, 2, 3]];

//         assert_eq!(v1.equality(&v2), Equality::Equal);
//     }

//     #[test]
//     fn eq_d3() {
//         let v1 = vec![vec![vec![1], vec![1, 2]], vec![vec![1, 2, 3]]];
//         let v2 = vec![vec![vec![1], vec![1, 2]], vec![vec![1, 2, 3]]];

//         assert_eq!(v1.equality(&v2), Equality::Equal);
//     }

//     #[test]
//     fn eq_d4() {
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

//         assert_eq!(v1.equality(&v2), Equality::Equal);
//     }

//     #[test]
//     fn ineq_card_d1() {
//         let v1 = vec![1, 2, 3];
//         let v2 = vec![1, 2, 3, 4];

//         assert_eq!(
//             v1.equality(&v2),
//             Equality::UnequalCard(IdxLeqD0::IdxD0([]), 3, 4)
//         );
//     }

//     #[test]
//     fn ineq_card_d2() {
//         let v1 = vec![vec![1, 2], vec![1, 2, 3, 4]];
//         let v2 = vec![vec![1, 2], vec![1, 2, 3]];

//         assert_eq!(
//             v1.equality(&v2),
//             Equality::UnequalCard(IdxLeqD1::IdxD1([1]), 4, 3)
//         );
//     }

//     #[test]
//     fn ineq_card_d3() {
//         let v1 = vec![vec![vec![1], vec![1, 2]], vec![vec![1, 2, 3]]];
//         let v2 = vec![vec![vec![1], vec![1, 2]], vec![vec![1, 2, 3]], vec![]];

//         assert_eq!(
//             v1.equality(&v2),
//             Equality::UnequalCard(IdxLeqD2::IdxD0([]), 2, 3)
//         );
//     }

//     #[test]
//     fn ineq_card_d4() {
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
//             vec![vec![], vec![vec![], vec![1, 2], vec![1, 2, 3]]],
//         ];

//         assert_eq!(
//             v1.equality(&v2),
//             Equality::UnequalCard(IdxLeqD3::IdxD1([1]), 1, 2)
//         );
//     }

//     #[test]
//     fn ineq_val_d1() {
//         let v1 = vec![1, 2, 3];
//         let v2 = vec![1, 22, 3];

//         assert_eq!(v1.equality(&v2), Equality::UnequalValue([1]));
//     }

//     #[test]
//     fn ineq_val_d2() {
//         let v1 = vec![vec![1, 2], vec![1, 2, 3]];
//         let v2 = vec![vec![1, 2], vec![1, 42, 3]];

//         assert_eq!(v1.equality(&v2), Equality::UnequalValue([1, 1]));
//     }

//     #[test]
//     fn ineq_val_d3() {
//         let v1 = vec![vec![vec![1], vec![1, 2]], vec![vec![1, 2, 3]]];
//         let v2 = vec![vec![vec![1], vec![1, 2]], vec![vec![42, 2, 3]]];

//         assert_eq!(v1.equality(&v2), Equality::UnequalValue([1, 0, 0]));
//     }

//     #[test]
//     fn ineq_val_d4() {
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
//                 vec![vec![1, 2], vec![1, 2, 42, 4, 5]],
//             ],
//             vec![vec![vec![], vec![1, 2], vec![1, 2, 3]]],
//         ];

//         assert_eq!(v1.equality(&v2), Equality::UnequalValue([0, 1, 1, 2]));
//     }
// }
