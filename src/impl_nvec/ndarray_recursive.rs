use crate::{impl_v1, impl_vn, D2, D3, D4};
use ndarray::{Array, Ix1};

impl_v1!([T], Array<T, Ix1>, [T: Copy]);
impl_vn!(D2, [T, C], Array<C, Ix1>, [C: NVec<<D2 as Dim>::PrevDim, T>]);
impl_vn!(D3, [T, C], Array<C, Ix1>, [C: NVec<<D3 as Dim>::PrevDim, T>]);
impl_vn!(D4, [T, C], Array<C, Ix1>, [C: NVec<<D4 as Dim>::PrevDim, T>]);

#[cfg(test)]
mod tests {
    use crate::*;
    use alloc::vec;
    use alloc::vec::Vec;
    use ndarray::{array, Array1};

    #[test]
    fn ndarray_d1() {
        let v1: Array1<usize> = array![1, 2, 3, 4];

        assert!(v1.is_bounded());
        assert_eq!(v1.card([]), 4);
        assert_eq!(v1.equality(&[1, 2, 3, 4]), Equality::Equal);
        assert_eq!(v1.all().collect::<Vec<_>>(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn ndarray_recurse_d2() {
        let v2: Array1<Vec<usize>> = array![vec![1], vec![2, 2], vec![3, 3, 3], vec![4, 4, 4, 4]];

        assert!(v2.is_bounded());
        assert_eq!(v2.card([]), 4);
        assert_eq!(
            v2.equality(&[vec![1], vec![2, 2], vec![3, 3, 3], vec![4, 4, 4, 4]]),
            Equality::Equal
        );
        assert_eq!(
            v2.all().collect::<Vec<_>>(),
            vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4]
        );

        for i in 0..4 {
            assert_eq!(v2.card(i), i + 1);
            let child = v2.child(i);
            assert_eq!(child.card([]), i + 1);
        }
    }
}
