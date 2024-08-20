use crate::{dimensions::*, failures::OUT_OF_BOUNDS, Completed, IntoCompleted, NVec, NVecMut};
use alloc::collections::btree_map::BTreeMap;

impl<E, T> NVec<D2, T> for BTreeMap<usize, E>
where
    E: NVec<<D2 as Dim>::PREVIOUS, T>,
{
    fn at<Idx: IntoIndex<D2>>(&self, index: Idx) -> T {
        let (i, index) = index.split();
        self.get(&i).expect(OUT_OF_BOUNDS).at(index)
    }

    fn try_at<Idx: IntoIndex<D2>>(&self, index: Idx) -> Option<T> {
        let (i, index) = index.split();
        self.get(&i).and_then(|x| x.try_at(index))
    }
}

impl<E, T> NVecMut<D2, T> for BTreeMap<usize, E>
where
    E: NVecMut<<D2 as Dim>::PREVIOUS, T>,
{
    fn set<Idx: IntoIndex<D2>>(&mut self, index: Idx, value: T) {
        let (i, index) = index.split();
        self.get_mut(&i).expect(OUT_OF_BOUNDS).set(index, value)
    }
}

impl<E, T> IntoCompleted<D2, T> for BTreeMap<usize, E> {
    fn into_completed(self, complete_with: T) -> Completed<Self, D2, T> {
        Completed::new(self, complete_with)
    }
}

// &

impl<E, T> NVec<D2, T> for &BTreeMap<usize, E>
where
    E: NVec<<D2 as Dim>::PREVIOUS, T>,
{
    fn at<Idx: IntoIndex<D2>>(&self, index: Idx) -> T {
        let (i, index) = index.split();
        self.get(&i).expect(OUT_OF_BOUNDS).at(index)
    }

    fn try_at<Idx: IntoIndex<D2>>(&self, index: Idx) -> Option<T> {
        let (i, index) = index.split();
        self.get(&i).and_then(|x| x.try_at(index))
    }
}

// &mut

impl<E, T> NVec<D2, T> for &mut BTreeMap<usize, E>
where
    E: NVec<<D2 as Dim>::PREVIOUS, T>,
{
    fn at<Idx: IntoIndex<D2>>(&self, index: Idx) -> T {
        let (i, index) = index.split();
        self.get(&i).expect(OUT_OF_BOUNDS).at(index)
    }

    fn try_at<Idx: IntoIndex<D2>>(&self, index: Idx) -> Option<T> {
        let (i, index) = index.split();
        self.get(&i).and_then(|x| x.try_at(index))
    }
}

impl<E, T> NVecMut<D2, T> for &mut BTreeMap<usize, E>
where
    E: NVecMut<<D2 as Dim>::PREVIOUS, T>,
{
    fn set<Idx: IntoIndex<D2>>(&mut self, index: Idx, value: T) {
        let (i, index) = index.split();
        self.get_mut(&i).expect(OUT_OF_BOUNDS).set(index, value)
    }
}

impl<E, T> IntoCompleted<D2, T> for &mut BTreeMap<usize, E> {
    fn into_completed(self, complete_with: T) -> Completed<Self, D2, T> {
        Completed::new(self, complete_with)
    }
}

// >= d3

macro_rules! implement {
    ($dim:tt) => {
        impl<E, T> NVec<$dim, T> for BTreeMap<usize, E>
        where
            E: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            fn at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> T {
                let (i, index) = index.split();
                self.get(&i).expect(OUT_OF_BOUNDS).at(index)
            }

            fn try_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> Option<T> {
                let (i, index) = index.split();
                self.get(&i).and_then(|x| x.try_at(index))
            }
        }

        impl<E, T> NVecMut<$dim, T> for BTreeMap<usize, E>
        where
            E: NVecMut<<$dim as Dim>::PREVIOUS, T>,
        {
            fn set<Idx: IntoIndex<$dim>>(&mut self, index: Idx, value: T) {
                let (i, index) = index.split();
                self.get_mut(&i).expect(OUT_OF_BOUNDS).set(index, value)
            }
        }

        impl<E, T> IntoCompleted<$dim, T> for BTreeMap<usize, E> {
            fn into_completed(self, complete_with: T) -> Completed<Self, $dim, T> {
                Completed::new(self, complete_with)
            }
        }

        // &

        impl<E, T> NVec<$dim, T> for &BTreeMap<usize, E>
        where
            E: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            fn at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> T {
                let (i, index) = index.split();
                self.get(&i).expect(OUT_OF_BOUNDS).at(index)
            }

            fn try_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> Option<T> {
                let (i, index) = index.split();
                self.get(&i).and_then(|x| x.try_at(index))
            }
        }

        impl<E, T> IntoCompleted<$dim, T> for &BTreeMap<usize, E> {
            fn into_completed(self, complete_with: T) -> Completed<Self, $dim, T> {
                Completed::new(self, complete_with)
            }
        }

        // &mut

        impl<E, T> NVec<$dim, T> for &mut BTreeMap<usize, E>
        where
            E: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            fn at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> T {
                let (i, index) = index.split();
                self.get(&i).expect(OUT_OF_BOUNDS).at(index)
            }

            fn try_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> Option<T> {
                let (i, index) = index.split();
                self.get(&i).and_then(|x| x.try_at(index))
            }
        }

        impl<E, T> NVecMut<$dim, T> for &mut BTreeMap<usize, E>
        where
            E: NVecMut<<$dim as Dim>::PREVIOUS, T>,
        {
            fn set<Idx: IntoIndex<$dim>>(&mut self, index: Idx, value: T) {
                let (i, index) = index.split();
                self.get_mut(&i).expect(OUT_OF_BOUNDS).set(index, value)
            }
        }

        impl<E, T> IntoCompleted<$dim, T> for &mut BTreeMap<usize, E> {
            fn into_completed(self, complete_with: T) -> Completed<Self, $dim, T> {
                Completed::new(self, complete_with)
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
