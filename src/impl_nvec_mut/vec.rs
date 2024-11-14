use crate::{impl_v1_mut, impl_vn_mut, D2, D3, D4};
use alloc::vec::Vec;

impl_v1_mut!([T], Vec<T>, [T: Copy]);
impl_vn_mut!(D2, [T, C], Vec<C>, [C: NVecMut<<D2 as Dim>::PrevDim, T>]);
impl_vn_mut!(D3, [T, C], Vec<C>, [C: NVecMut<<D3 as Dim>::PrevDim, T>]);
impl_vn_mut!(D4, [T, C], Vec<C>, [C: NVecMut<<D4 as Dim>::PrevDim, T>]);

#[cfg(test)]
mod tests {
    use crate::*;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn vec_d1() {
        let mut v1: Vec<i32> = vec![1, 2, 3, 4];
        assert_eq!(v1.card([]), 4);
        assert_eq!(v1.equality(&[1, 2, 3, 4]), Equality::Equal);
        assert_eq!(v1.all().collect::<Vec<_>>(), vec![1, 2, 3, 4]);

        v1.push(5);
        v1.push(6);
        *v1.at_mut([0]) = 11;
        v1.set(1, 22);
        assert_eq!(v1.core_card([]), 6);
        assert_eq!(v1.equality(&[11, 22, 3, 4, 5, 6]), Equality::Equal);
    }

    #[test]
    fn vec_recurse_d2() {
        let mut v2: Vec<Vec<i32>> = vec![vec![1, 10], vec![2, 20], vec![3, 30], vec![4, 40]];
        assert_eq!(v2.card([]), 4);
        assert_eq!(
            v2.equality(&[[1, 10], [2, 20], [3, 30], [4, 40]]),
            Equality::Equal
        );
        assert_eq!(
            v2.all().collect::<Vec<_>>(),
            vec![1, 10, 2, 20, 3, 30, 4, 40]
        );

        v2.push(vec![5]);
        v2.push(vec![6, 60, 600]);

        *v2.at_mut([0, 1]) += 10;

        assert_eq!(v2.card([]), 6);
        assert_eq!(
            v2.equality(&[
                vec![1, 20],
                vec![2, 20],
                vec![3, 30],
                vec![4, 40],
                vec![5],
                vec![6, 60, 600]
            ]),
            Equality::Equal
        );
        assert_eq!(
            v2.all().collect::<Vec<_>>(),
            vec![1, 20, 2, 20, 3, 30, 4, 40, 5, 6, 60, 600]
        );
    }

    #[test]
    fn vec_recurse_d3() {
        let mut v3: Vec<Vec<Vec<i32>>> = vec![
            vec![vec![1], vec![10]],
            vec![vec![2], vec![20]],
            vec![vec![3], vec![30]],
            vec![vec![4], vec![40]],
        ];
        assert_eq!(v3.card([]), 4);
        assert_eq!(
            v3.equality(&[[[1], [10]], [[2], [20]], [[3], [30]], [[4], [40]]]),
            Equality::Equal
        );
        assert_eq!(
            v3.all().collect::<Vec<_>>(),
            vec![1, 10, 2, 20, 3, 30, 4, 40]
        );

        v3.push(vec![vec![5]]);
        v3.push(vec![vec![6], vec![60], vec![600]]);
        assert_eq!(v3.card([]), 6);

        v3.set([0, 1, 0], 42);

        assert_eq!(
            v3.equality(&[
                vec![[1], [42]],
                vec![[2], [20]],
                vec![[3], [30]],
                vec![[4], [40]],
                vec![[5]],
                vec![[6], [60], [600]]
            ]),
            Equality::Equal
        );
        assert_eq!(
            v3.all().collect::<Vec<_>>(),
            vec![1, 42, 2, 20, 3, 30, 4, 40, 5, 6, 60, 600]
        );
    }
}
