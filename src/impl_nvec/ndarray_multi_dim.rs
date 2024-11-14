use crate::children::{ChildD2D1, ChildD3D2, ChildD4D3};
use crate::dim::*;
use crate::nvec::NVec;
use ndarray::{Array, Ix2, Ix3, Ix4};

// d2 - full-indexed

impl<T: Copy> NVec<D2, T> for Array<T, Ix2> {
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D2>) -> T {
        self[idx.into_idx()]
    }

    fn child(&self, i: <D2 as Dim>::ChildIdx) -> impl NVec<<D2 as Dim>::PrevDim, T> {
        ChildD2D1 {
            i,
            parent: self,
            phantom: Default::default(),
        }
    }

    fn all(&self) -> impl Iterator<Item = T> {
        self.iter().copied()
    }
}

// d3 - full-indexed

impl<T: Copy> NVec<D3, T> for Array<T, Ix3> {
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D3>) -> T {
        self[idx.into_idx()]
    }

    fn child(&self, i: usize) -> impl NVec<<D3 as Dim>::PrevDim, T> {
        ChildD3D2 {
            i,
            parent: self,
            phantom: Default::default(),
        }
    }

    fn all(&self) -> impl Iterator<Item = T> {
        self.iter().copied()
    }
}

// d4 - full-indexed

impl<T: Copy> NVec<D4, T> for Array<T, Ix4> {
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D4>) -> T {
        self[idx.into_idx()]
    }

    fn child(&self, i: usize) -> impl NVec<<D4 as Dim>::PrevDim, T> {
        ChildD4D3 {
            i,
            parent: self,
            phantom: Default::default(),
        }
    }

    fn all(&self) -> impl Iterator<Item = T> {
        self.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use alloc::vec;
    use alloc::vec::Vec;
    use ndarray::{array, Array2, Array3};

    #[test]
    fn ndarray_d2() {
        let v2: Array2<usize> = array![[1, 10], [2, 20], [3, 30], [4, 40]];
        assert!(v2.is_bounded());
        assert_eq!(
            v2.all().collect::<Vec<_>>(),
            vec![1, 10, 2, 20, 3, 30, 4, 40]
        );
        assert_eq!(
            v2.equality(&[[1, 10], [2, 20], [3, 30], [4, 40]]),
            Equality::Equal
        );

        assert_eq!(v2.card([]), 4);
        for i in 0..4 {
            assert_eq!(v2.card([i]), 2);

            let child = v2.child(i);
            assert!(child.is_bounded());
            assert_eq!(child.card([]), 2);
            assert_eq!(child.all().collect::<Vec<_>>(), vec![i + 1, (i + 1) * 10]);
            assert_eq!(child.equality(&[i + 1, (i + 1) * 10]), Equality::Equal);
        }
    }

    #[test]
    fn ndarray_d3() {
        let v3: Array3<usize> = array![
            [[1, 1, 1], [10, 10, 10]],
            [[2, 2, 2], [20, 20, 20]],
            [[3, 3, 3], [30, 30, 30]],
            [[4, 4, 4], [40, 40, 40]]
        ];

        assert_eq!(
            v3.all().collect::<Vec<_>>(),
            vec![
                1, 1, 1, 10, 10, 10, 2, 2, 2, 20, 20, 20, 3, 3, 3, 30, 30, 30, 4, 4, 4, 40, 40, 40
            ]
        );
        assert_eq!(
            v3.equality(&[
                [[1, 1, 1], [10, 10, 10]],
                [[2, 2, 2], [20, 20, 20]],
                [[3, 3, 3], [30, 30, 30]],
                [[4, 4, 4], [40, 40, 40]]
            ]),
            Equality::Equal
        );

        assert!(v3.is_bounded());
        assert_eq!(v3.card([]), 4);
        for i in 0..4 {
            assert_eq!(v3.card([i]), 2);

            let child = v3.child(i);
            assert!(child.is_bounded());
            assert_eq!(child.card([]), 2);
            assert_eq!(
                child.equality(&[
                    [i + 1, i + 1, i + 1],
                    [10 * (i + 1), 10 * (i + 1), 10 * (i + 1)]
                ]),
                Equality::Equal
            );
            assert_eq!(
                child.all().collect::<Vec<_>>(),
                vec![
                    i + 1,
                    i + 1,
                    i + 1,
                    10 * (i + 1),
                    10 * (i + 1),
                    10 * (i + 1)
                ]
            );

            for j in 0..2 {
                let coef = match j {
                    0 => 1,
                    _ => 10,
                };
                let child = child.child(j);
                assert!(child.is_bounded());
                assert_eq!(child.card([]), 3);
                assert_eq!(
                    child.all().collect::<Vec<_>>(),
                    vec![coef * (i + 1), coef * (i + 1), coef * (i + 1)]
                );
                assert_eq!(
                    child.equality(&[coef * (i + 1), coef * (i + 1), coef * (i + 1)]),
                    Equality::Equal
                );

                assert_eq!(v3.card([i, j]), 3);
            }
        }
    }
}
