use crate::{dimensions::*, NVec, NVecMut};

macro_rules! implement {
    ($dim:tt) => {
        impl<T, E> NVec<$dim, T> for Vec<E>
        where
            E: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            fn at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> T {
                let (i, index) = index.split();
                self[i].at(index)
            }

            fn try_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> Option<T> {
                let (i, index) = index.split();
                self.get(i).and_then(|x| x.try_at(index))
            }
        }

        impl<T, E> NVecMut<$dim, T> for Vec<E>
        where
            E: NVecMut<<$dim as Dim>::PREVIOUS, T>,
        {
            fn set<Idx: IntoIndex<$dim>>(&mut self, index: Idx, value: T) {
                let (i, index) = index.split();
                self[i].set(index, value)
            }
        }

        // &

        impl<T, E> NVec<$dim, T> for &Vec<E>
        where
            E: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            fn at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> T {
                let (i, index) = index.split();
                self[i].at(index)
            }

            fn try_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> Option<T> {
                let (i, index) = index.split();
                self.get(i).and_then(|x| x.try_at(index))
            }
        }

        // &mut

        impl<T, E> NVec<$dim, T> for &mut Vec<E>
        where
            E: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            fn at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> T {
                let (i, index) = index.split();
                self[i].at(index)
            }

            fn try_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> Option<T> {
                let (i, index) = index.split();
                self.get(i).and_then(|x| x.try_at(index))
            }
        }

        impl<T, E> NVecMut<$dim, T> for &mut Vec<E>
        where
            E: NVecMut<<$dim as Dim>::PREVIOUS, T>,
        {
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
