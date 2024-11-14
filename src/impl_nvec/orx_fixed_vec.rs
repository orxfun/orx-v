use crate::{impl_v1, impl_vn, D2, D3, D4};
use orx_fixed_vec::*;

impl_v1!([T], FixedVec<T>, [T: Copy]);
impl_vn!(D2, [T, C], FixedVec<C>, [C: NVec<<D2 as Dim>::PrevDim, T>]);
impl_vn!(D3, [T, C], FixedVec<C>, [C: NVec<<D3 as Dim>::PrevDim, T>]);
impl_vn!(D4, [T, C], FixedVec<C>, [C: NVec<<D4 as Dim>::PrevDim, T>]);

#[cfg(all(test, feature = "orx-fixed-vec"))]
mod tests {
    use crate::*;
    use alloc::vec;
    use alloc::vec::Vec;
    use orx_fixed_vec::*;

    #[test]
    fn fixed_vec_d1() {
        let v1: FixedVec<i32> = [1, 2, 3, 4].into_iter().collect();
        assert_eq!(v1.card([]), 4);
        assert_eq!(v1.equality(&[1, 2, 3, 4]), Equality::Equal);
        assert_eq!(v1.all().collect::<Vec<_>>(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn fixed_vec_recurse_d2() {
        let v2: FixedVec<Vec<i32>> = [vec![1, 10], vec![2, 20], vec![3, 30], vec![4, 40]]
            .into_iter()
            .collect();
        assert_eq!(v2.card([]), 4);
        assert_eq!(
            v2.equality(&[[1, 10], [2, 20], [3, 30], [4, 40]]),
            Equality::Equal
        );
        assert_eq!(
            v2.all().collect::<Vec<_>>(),
            vec![1, 10, 2, 20, 3, 30, 4, 40]
        );
    }
}
