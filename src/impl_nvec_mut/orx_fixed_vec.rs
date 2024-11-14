use crate::{impl_v1_mut, impl_vn_mut, D2, D3, D4};
use orx_fixed_vec::*;

impl_v1_mut!([T], FixedVec<T>, [T: Copy]);
impl_vn_mut!(D2, [T, C], FixedVec<C>, [C: NVecMut<<D2 as Dim>::PrevDim, T>]);
impl_vn_mut!(D3, [T, C], FixedVec<C>, [C: NVecMut<<D3 as Dim>::PrevDim, T>]);
impl_vn_mut!(D4, [T, C], FixedVec<C>, [C: NVecMut<<D4 as Dim>::PrevDim, T>]);

#[cfg(all(test, feature = "orx-fixed-vec"))]
mod tests {
    use crate::*;
    use alloc::vec;
    use alloc::vec::Vec;
    use orx_fixed_vec::*;

    #[test]
    fn fixed_vec_d1() {
        let mut v1: FixedVec<i32> = [1, 2, 3, 4].into_iter().collect();
        *v1.at_mut(1) *= 10;
        assert_eq!(v1.card([]), 4);
        assert_eq!(v1.equality(&[1, 20, 3, 4]), Equality::Equal);
    }

    #[test]
    fn fixed_vec_d2() {
        let mut v2: FixedVec<Vec<i32>> = [vec![1, 10], vec![2, 20], vec![3, 30], vec![4, 40]]
            .into_iter()
            .collect();
        *v2.at_mut([2, 1]) = 33;
        assert_eq!(v2.card([]), 4);
        assert_eq!(
            v2.equality(&[[1, 10], [2, 20], [3, 33], [4, 40]]),
            Equality::Equal
        );
    }
}
