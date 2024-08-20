use crate::{dimensions::*, NVec};

impl<'v, T, E> NVec<D2, &'v T> for &'v [E]
where
    &'v E: NVec<D1, &'v T>,
    T: 'v,
{
    fn at<Idx: IntoIndex<D2>>(&self, index: Idx) -> &'v T {
        let (i, index) = index.split();
        (&self[i]).at(index)
    }

    fn try_at<Idx: IntoIndex<D2>>(&self, index: Idx) -> Option<&'v T> {
        let (i, index) = index.split();
        self.get(i).and_then(|e| e.try_at(index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntoCopied, IntoFilled, NVec};
    use alloc::vec;

    #[test]
    fn take_as_d2() {
        fn take<'a, V: NVec<D2, &'a i32>>(vec: V) {
            assert_eq!(vec.at([0, 0]), &1);
            assert_eq!(vec.at([1, 2]), &5);

            assert_eq!(vec.try_at([1, 0]), Some(&3));
            assert_eq!(vec.try_at([2, 0]), None);
            assert_eq!(vec.try_at([0, 1]), None);

            let vec = vec.filled_with(&42);
            assert_eq!(vec.try_at([2, 0]), Some(&42));
            assert_eq!(vec.try_at([0, 1]), Some(&42));
            assert_eq!(vec.at([2, 0]), &42);
        }

        fn take_copied<V: NVec<D2, i32>>(vec: V) {
            assert_eq!(vec.at([0, 0]), 1);
            assert_eq!(vec.at([1, 2]), 5);

            assert_eq!(vec.try_at([1, 0]), Some(3));
            assert_eq!(vec.try_at([2, 0]), None);
            assert_eq!(vec.try_at([0, 1]), None);

            let vec = vec.filled_with(42);
            assert_eq!(vec.try_at([2, 0]), Some(42));
            assert_eq!(vec.try_at([0, 1]), Some(42));
            assert_eq!(vec.at([2, 0]), 42);
        }

        let vec: Vec<Vec<i32>> = vec![vec![1], vec![3, 4, 5]];

        take(vec.as_slice());
        take_copied(vec.as_slice().copied());
    }

    #[test]
    fn take_as_d1() {
        fn take<'a, V: NVec<D1, &'a Vec<i32>>>(vec: V) {
            assert_eq!(&vec.at([0])[0], &1);
            assert_eq!(vec.at(1).at(2), &5);

            assert_eq!(vec.try_at(1).map(|x| &x[0]), Some(&3));
            assert_eq!(vec.try_at(2).map(|x| x[0]), None);
            assert_eq!(vec.try_at(0).and_then(|x| x.try_at(1)), None);
        }

        let vec: Vec<Vec<i32>> = vec![vec![1], vec![3, 4, 5]];
        take(vec.as_slice());
    }
}
