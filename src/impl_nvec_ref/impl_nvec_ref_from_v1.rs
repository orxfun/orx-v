use super::v_ref::VRef;
use crate::*;
use alloc::vec::Vec;

impl<T> NVecRef<D1, T> for VRef<D1, Vec<T>> {
    fn at(&self, idx: impl IntoIdx<D1>) -> &T {
        &self.inner[idx.into_idx()[0]]
    }

    fn child<'a>(&'a self, i: <D1 as Dim>::ChildIdx) -> &'a impl NVecRef<<D1 as Dim>::PrevDim, T>
    where
        T: 'a,
    {
        self
    }

    fn all<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.inner.iter()
    }

    fn enumerate_all<'a>(&'a self) -> impl Iterator<Item = (<D1 as Dim>::Idx, &'a T)>
    where
        T: 'a,
    {
        self.inner.iter().enumerate().map(|(i, x)| ([i], x))
    }
}

impl<T, C> NVecRef<D2, T> for VRef<D2, Vec<C>>
where
    C: NVecRef<<D2 as Dim>::PrevDim, T>,
{
    fn at(&self, idx: impl IntoIdx<D2>) -> &T {
        let (i, c_idx) = idx.into_idx().split_idx();
        self.child(i).at(c_idx)
    }

    fn child<'a>(&'a self, i: <D2 as Dim>::ChildIdx) -> &'a impl NVecRef<<D2 as Dim>::PrevDim, T>
    where
        T: 'a,
    {
        &self.inner[i]
    }

    fn all<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.inner.iter().flat_map(|x| x.all())
    }

    fn enumerate_all<'a>(&'a self) -> impl Iterator<Item = (<D2 as Dim>::Idx, &'a T)>
    where
        T: 'a,
    {
        self.inner.iter().enumerate().flat_map(|(i, x)| {
            x.enumerate_all().map(move |(idx_right, y)| {
                let idx = <D2 as Dim>::left_join_from_lower_dim(i, idx_right);
                (idx, y)
            })
        })
    }
}
