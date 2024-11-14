use crate::children::{ChildD2D1, ChildD3D2, ChildD4D3};
use crate::{dim::*, NVecMut};
use ndarray::{Array, Ix2, Ix3, Ix4};

// d2 - full-indexed

impl<T: Copy> NVecMut<D2, T> for Array<T, Ix2> {
    fn at_mut<Idx: IntoIdx<D2>>(&mut self, idx: Idx) -> &mut T {
        &mut self[idx.into_idx()]
    }

    fn set<Idx: IntoIdx<D2>>(&mut self, idx: Idx, value: T) {
        self[idx.into_idx()] = value
    }

    fn child_mut(&mut self, i: <D2 as Dim>::ChildIdx) -> impl NVecMut<<D2 as Dim>::PrevDim, T> {
        ChildD2D1 {
            i,
            parent: self,
            phantom: Default::default(),
        }
    }

    fn mut_all<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        for x in self.iter_mut() {
            f(x);
        }
    }

    fn reset_all(&mut self, value: T)
    where
        T: PartialEq + Copy,
    {
        for x in self.iter_mut() {
            *x = value
        }
    }
}

// d3 - full-indexed

impl<T: Copy> NVecMut<D3, T> for Array<T, Ix3> {
    fn at_mut<Idx: IntoIdx<D3>>(&mut self, idx: Idx) -> &mut T {
        &mut self[idx.into_idx()]
    }

    fn set<Idx: IntoIdx<D3>>(&mut self, idx: Idx, value: T) {
        self[idx.into_idx()] = value
    }

    fn child_mut(&mut self, i: <D3 as Dim>::ChildIdx) -> impl NVecMut<<D3 as Dim>::PrevDim, T> {
        ChildD3D2 {
            i,
            parent: self,
            phantom: Default::default(),
        }
    }

    fn mut_all<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        for x in self.iter_mut() {
            f(x);
        }
    }

    fn reset_all(&mut self, value: T)
    where
        T: PartialEq + Copy,
    {
        for x in self.iter_mut() {
            *x = value
        }
    }
}

// d4 - full-indexed

impl<T: Copy> NVecMut<D4, T> for Array<T, Ix4> {
    fn at_mut<Idx: IntoIdx<D4>>(&mut self, idx: Idx) -> &mut T {
        &mut self[idx.into_idx()]
    }

    fn set<Idx: IntoIdx<D4>>(&mut self, idx: Idx, value: T) {
        self[idx.into_idx()] = value
    }

    fn child_mut(&mut self, i: <D4 as Dim>::ChildIdx) -> impl NVecMut<<D4 as Dim>::PrevDim, T> {
        ChildD4D3 {
            i,
            parent: self,
            phantom: Default::default(),
        }
    }

    fn mut_all<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        for x in self.iter_mut() {
            f(x);
        }
    }

    fn reset_all(&mut self, value: T)
    where
        T: PartialEq + Copy,
    {
        for x in self.iter_mut() {
            *x = value
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use alloc::vec;
    use alloc::vec::Vec;
    use ndarray::{array, Array1, Array2, Array3};

    #[test]
    fn ndarray_d1() {
        let v1: Array1<usize> = array![1, 2, 3, 4];

        assert!(v1.is_bounded());
        assert_eq!(v1.card([]), 4);
        assert_eq!(v1.equality(&[1, 2, 3, 4]), Equality::Equal);
        assert_eq!(v1.all().collect::<Vec<_>>(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn ndarray_d2() {
        let mut v2: Array2<usize> = array![[1, 10], [2, 20], [3, 30], [4, 40]];

        v2.mut_all(|x| {
            if *x > 20 {
                *x *= 10;
            }
        });

        *v2.at_mut([0, 1]) = 100;
        v2.set([1, 1], 200);

        assert!(v2.is_bounded());
        assert_eq!(
            v2.all().collect::<Vec<_>>(),
            vec![1, 100, 2, 200, 3, 300, 4, 400]
        );
        assert_eq!(
            v2.equality(&[[1, 100], [2, 200], [3, 300], [4, 400]]),
            Equality::Equal
        );

        assert_eq!(v2.card([]), 4);
        for i in 0..4 {
            assert_eq!(v2.card([i]), 2);

            let child = v2.child(i);
            assert!(child.is_bounded());
            assert_eq!(child.card([]), 2);
            assert_eq!(child.all().collect::<Vec<_>>(), vec![i + 1, (i + 1) * 100]);
            assert_eq!(child.equality(&[i + 1, (i + 1) * 100]), Equality::Equal);
        }
    }

    #[test]
    fn ndarray_d3() {
        let mut v3: Array3<usize> = array![
            [[1, 1, 1], [10, 10, 10]],
            [[2, 2, 2], [20, 20, 20]],
            [[3, 3, 3], [30, 30, 30]],
            [[4, 4, 4], [40, 40, 40]]
        ];

        *v3.at_mut([0, 1, 0]) *= 10;
        *v3.at_mut([0, 1, 1]) *= 10;
        v3.set([0, 1, 2], 100);
        v3.child_mut(0).child_mut(1).mut_all(|x| *x /= 10);

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
