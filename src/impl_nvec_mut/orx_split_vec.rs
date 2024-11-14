use crate::{impl_v1_mut, impl_vn_mut, D2, D3, D4};
use orx_split_vec::*;

impl_v1_mut!([T, G], SplitVec<T, G>, [T: Copy, G: Growth]);
impl_vn_mut!(D2, [G, T, C], SplitVec<C, G>, [C: NVecMut<<D2 as Dim>::PrevDim, T>, G: Growth]);
impl_vn_mut!(D3, [G, T, C], SplitVec<C, G>, [C: NVecMut<<D3 as Dim>::PrevDim, T>, G: Growth]);
impl_vn_mut!(D4, [G, T, C], SplitVec<C, G>, [C: NVecMut<<D4 as Dim>::PrevDim, T>, G: Growth]);

#[cfg(all(test, feature = "orx-split-vec"))]
mod tests {
    use crate::*;
    use alloc::vec;
    use alloc::vec::Vec;
    use orx_split_vec::*;

    #[test]
    fn split_vec_d1() {
        let mut v1: SplitVec<i32> = [1, 2, 3, 4].into_iter().collect();
        *v1.at_mut(1) *= 10;
        assert_eq!(v1.card([]), 4);
        assert_eq!(v1.equality(&[1, 20, 3, 4]), Equality::Equal);

        v1.push(5);
        v1.push(6);
        assert_eq!(v1.card([]), 6);
        assert_eq!(v1.equality(&[1, 20, 3, 4, 5, 6]), Equality::Equal);
    }

    #[test]
    fn split_vec_d2() {
        let mut v2: SplitVec<Vec<i32>> = [vec![1, 10], vec![2, 20], vec![3, 30], vec![4, 40]]
            .into_iter()
            .collect();
        assert_eq!(v2.card([]), 4);
        assert_eq!(
            v2.equality(&[[1, 10], [2, 20], [3, 30], [4, 40]]),
            Equality::Equal
        );

        v2.push(vec![5]);
        v2.push(vec![6, 60, 600]);

        *v2.at_mut([1, 0]) = 200;

        assert_eq!(v2.card([]), 6);
        assert_eq!(
            v2.equality(&[
                vec![1, 10],
                vec![200, 20],
                vec![3, 30],
                vec![4, 40],
                vec![5],
                vec![6, 60, 600]
            ]),
            Equality::Equal
        );
    }
}
