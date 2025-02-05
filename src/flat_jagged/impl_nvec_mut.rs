use super::{FlatJagged, FlatJaggedRowMut};
use crate::{Dim, IntoIdx, NVec, NVecMut, D1, D2};

// row

impl<V, I, T> NVecMut<D1, T> for FlatJaggedRowMut<'_, V, I, T>
where
    V: NVec<D1, T> + NVecMut<D1, T>,
    I: NVec<D1, usize>,
{
    fn at_mut<Idx: IntoIdx<D1>>(&mut self, idx: Idx) -> &mut T {
        let [j] = idx.into_idx();
        let idx = self.jagged.to_d1_idx([self.i, j]);
        self.jagged.flat_vec.at_mut(idx)
    }

    fn set<Idx: IntoIdx<D1>>(&mut self, idx: Idx, value: T) {
        let [j] = idx.into_idx();
        let idx = self.jagged.to_d1_idx([self.i, j]);
        self.jagged.flat_vec.set(idx, value);
    }

    fn child_mut(&mut self, _: <D1 as Dim>::ChildIdx) -> impl NVecMut<<D1 as Dim>::PrevDim, T> {
        self
    }

    fn mut_all<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        for j in 0..self.num_children() {
            f(self.at_mut(j));
        }
    }

    fn reset_all(&mut self, value: T)
    where
        T: PartialEq + Copy,
    {
        self.mut_all(|x| *x = value);
    }
}

// vec

impl<V, I, T> NVecMut<D2, T> for FlatJagged<V, I, T>
where
    V: NVec<D1, T> + NVecMut<D1, T>,
    I: NVec<D1, usize>,
{
    fn at_mut<Idx: IntoIdx<D2>>(&mut self, idx: Idx) -> &mut T {
        let idx = self.to_d1_idx(idx.into_idx());
        self.flat_vec.at_mut(idx)
    }

    fn set<Idx: IntoIdx<D2>>(&mut self, idx: Idx, value: T) {
        let idx = self.to_d1_idx(idx.into_idx());
        self.flat_vec.set(idx, value);
    }

    fn child_mut(&mut self, i: <D2 as Dim>::ChildIdx) -> impl NVecMut<<D2 as Dim>::PrevDim, T> {
        self.row_mut(i)
    }

    fn mut_all<F>(&mut self, f: F)
    where
        F: FnMut(&mut T),
    {
        self.flat_vec.mut_all(f);
    }

    fn reset_all(&mut self, value: T)
    where
        T: PartialEq + Copy,
    {
        self.flat_vec.mut_all(|x| *x = value);
    }
}
