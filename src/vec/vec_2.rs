use crate::{dimensions::*, NVec, NVecMut};

impl<T, E> NVec<D2, T> for Vec<E>
where
    E: NVec<<D2 as Dim>::PREVIOUS, T>,
{
    fn at<Idx: IntoIndex<D2>>(&self, index: Idx) -> T {
        let (i, index) = index.split();
        self[i].at(index)
    }

    fn try_at<Idx: IntoIndex<D2>>(&self, index: Idx) -> Option<T> {
        let (i, index) = index.split();
        self.get(i).and_then(|x| x.try_at(index))
    }
}

impl<T, E> NVecMut<D2, T> for Vec<E>
where
    E: NVecMut<<D2 as Dim>::PREVIOUS, T>,
{
    fn set<Idx: IntoIndex<D2>>(&mut self, index: Idx, value: T) {
        let (i, index) = index.split();
        self[i].set(index, value)
    }
}

// &

impl<T, E> NVec<D2, T> for &Vec<E>
where
    E: NVec<<D2 as Dim>::PREVIOUS, T>,
{
    fn at<Idx: IntoIndex<D2>>(&self, index: Idx) -> T {
        let (i, index) = index.split();
        self[i].at(index)
    }

    fn try_at<Idx: IntoIndex<D2>>(&self, index: Idx) -> Option<T> {
        let (i, index) = index.split();
        self.get(i).and_then(|x| x.try_at(index))
    }
}

// &mut

impl<T, E> NVec<D2, T> for &mut Vec<E>
where
    E: NVec<<D2 as Dim>::PREVIOUS, T>,
{
    fn at<Idx: IntoIndex<D2>>(&self, index: Idx) -> T {
        let (i, index) = index.split();
        self[i].at(index)
    }

    fn try_at<Idx: IntoIndex<D2>>(&self, index: Idx) -> Option<T> {
        let (i, index) = index.split();
        self.get(i).and_then(|x| x.try_at(index))
    }
}

impl<T, E> NVecMut<D2, T> for &mut Vec<E>
where
    E: NVecMut<<D2 as Dim>::PREVIOUS, T>,
{
    fn set<Idx: IntoIndex<D2>>(&mut self, index: Idx, value: T) {
        let (i, index) = index.split();
        self[i].set(index, value)
    }
}
