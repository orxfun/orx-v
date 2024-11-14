use crate::{impl_v1_mut, impl_vn_mut, D2, D3, D4};
use tinyvec::{Array, TinyVec};

impl_v1_mut!(N, [T], TinyVec<[T; N]>, [T: Copy, [T; N]: Array<Item = T>]);
impl_vn_mut!(D2, N, [C, T], TinyVec<[C; N]>, [C: NVecMut<<D2 as Dim>::PrevDim, T>, [C; N]: Array<Item = C>]);
impl_vn_mut!(D3, N, [C, T], TinyVec<[C; N]>, [C: NVecMut<<D3 as Dim>::PrevDim, T>, [C; N]: Array<Item = C>]);
impl_vn_mut!(D4, N, [C, T], TinyVec<[C; N]>, [C: NVecMut<<D4 as Dim>::PrevDim, T>, [C; N]: Array<Item = C>]);

#[cfg(all(test, feature = "tinyvec"))]
mod tests {
    use crate::*;
    use alloc::vec;
    use alloc::vec::Vec;
    use tinyvec::{tiny_vec, TinyVec};

    #[test]
    fn tinyvec_tinyvec_d1() {
        let mut v1: TinyVec<[i32; 4]> = tiny_vec![1, 2, 3, 4];
        *v1.at_mut(1) *= 10;
        assert_eq!(v1.card([]), 4);
        assert_eq!(v1.equality(&[1, 20, 3, 4]), Equality::Equal);

        v1.push(5);
        v1.push(6);
        assert_eq!(v1.card([]), 6);
        assert_eq!(v1.equality(&[1, 20, 3, 4, 5, 6]), Equality::Equal);
    }

    #[test]
    fn tinyvec_tinyvec_recurse_d2() {
        let mut v2: TinyVec<[Vec<i32>; 4]> =
            tiny_vec![vec![1, 10], vec![2, 20], vec![3, 30], vec![4, 40]];
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
