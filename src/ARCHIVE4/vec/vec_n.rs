use crate::{dimensions::*, NVec, NVecMut};
use alloc::vec::Vec;

macro_rules! implement {
    ($dim:tt) => {
        impl<'v, T, E> NVec<$dim, &'v T> for &'v Vec<E>
        where
            &'v E: NVec<<$dim as Dim>::PREVIOUS, &'v T>,
            T: 'v,
        {
            fn at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> &'v T {
                let (i, index) = index.split();
                (&self[i]).at(index)
            }

            fn try_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> Option<&'v T> {
                let (i, index) = index.split();
                self.get(i).and_then(|e| e.try_at(index))
            }
        }

        impl<T, E> NVecMut<$dim, T> for Vec<E>
        where
            E: NVecMut<<$dim as Dim>::PREVIOUS, T>,
        {
            #[inline]
            fn at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> &T {
                let (i, index) = index.split();
                (&self[i]).at(index)
            }

            #[inline]
            fn try_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> Option<&T> {
                let (i, index) = index.split();
                self.get(i).and_then(|e| e.try_at(index))
            }

            #[inline]
            fn set<Idx: IntoIndex<$dim>>(&mut self, index: Idx, value: T) {
                let (i, index) = index.split();
                self[i].set(index, value)
            }
        }
    };
}

implement!(D3);
implement!(D4);
implement!(D5);
implement!(D6);
implement!(D7);
implement!(D8);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntoCopied, NVec};
    use alloc::vec;

    #[test]
    fn take_as_d3() {
        fn take<'a, V: NVec<D3, &'a i32>>(vec: V) {
            assert_eq!(vec.at([0, 0, 0]), &1);
            assert_eq!(vec.at([1, 0, 2]), &5);

            assert_eq!(vec.try_at([1, 0, 0]), Some(&3));
            assert_eq!(vec.try_at([2, 0, 0]), None);
            assert_eq!(vec.try_at([0, 0, 1]), None);
        }

        fn take_copied<V: NVec<D3, i32>>(vec: V) {
            assert_eq!(vec.at([0, 0, 0]), 1);
            assert_eq!(vec.at([1, 0, 2]), 5);

            assert_eq!(vec.try_at([1, 0, 0]), Some(3));
            assert_eq!(vec.try_at([2, 0, 0]), None);
            assert_eq!(vec.try_at([0, 0, 1]), None);
        }

        let vec: Vec<Vec<Vec<i32>>> = vec![vec![vec![1]], vec![vec![3, 4, 5]]];
        take(&vec);
        take_copied(vec.copied());
    }

    #[test]
    fn take_as_d2() {
        fn take<'a, V: NVec<D2, &'a Vec<i32>>>(vec: V) {
            assert_eq!(&vec.at([0, 0])[0], &1);
            assert_eq!(vec.at((1, 0)).at(2), &5);

            assert_eq!(vec.try_at((1, 0)).map(|x| &x[0]), Some(&3));
            assert_eq!(vec.try_at([2, 0]).map(|x| x[0]), None);
            assert_eq!(vec.try_at([0, 0]).and_then(|x| x.try_at(1)), None);
        }

        let vec: Vec<Vec<Vec<i32>>> = vec![vec![vec![1]], vec![vec![3, 4, 5]]];
        take(&vec);
    }

    #[test]
    fn take_as_d1() {
        fn take<'a, V: NVec<D1, &'a Vec<Vec<i32>>>>(vec: V) {
            assert_eq!(&vec.at([0])[0][0], &1);
            assert_eq!(vec.at(1).at(0).at(2), &5);

            assert_eq!(vec.try_at(1).map(|x| &x[0][0]), Some(&3));
            assert_eq!(vec.try_at(2).map(|x| x[0][0]), None);
        }

        let vec: Vec<Vec<Vec<i32>>> = vec![vec![vec![1]], vec![vec![3, 4, 5]]];
        take(&vec);
    }

    #[test]
    fn take_as_d3_mut() {
        fn take<'a, V: NVecMut<D3, i32>>(vec: &mut V) {
            vec.set([0, 0, 0], 42);
            vec.set([1, 0, 2], 7);

            assert_eq!(vec.at([0, 0, 0]), &42);
            assert_eq!(vec.at([1, 0, 2]), &7);
        }

        let mut vec: Vec<Vec<Vec<i32>>> = vec![vec![vec![1]], vec![vec![3, 4, 5]]];
        take(&mut vec);
    }
}
