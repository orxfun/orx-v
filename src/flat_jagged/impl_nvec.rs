use super::{FlatJagged, FlatJaggedRowMut};
use crate::{CardD1, Dim, FunVec, IntoIdx, NVec, D1, D2};

// row

impl<'a, V, I, T> NVec<D1, T> for FlatJaggedRowMut<'a, V, I, T>
where
    V: NVec<D1, T>,
    I: NVec<D1, usize>,
{
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D1>) -> T {
        let [j] = idx.into_idx();
        let idx = self.jagged.to_d1_idx([self.i, j]);
        self.jagged.flat_vec.at(idx)
    }

    fn child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVec<<D1 as Dim>::PrevDim, T> {
        self
    }

    fn all(&self) -> impl Iterator<Item = T> {
        let (begin, end) = self.jagged.row_range(self.i);
        self.jagged.flat_vec.all().skip(begin).take(end - begin)
    }
}

// vec

impl<V, I, T> NVec<D2, T> for FlatJagged<V, I, T>
where
    V: NVec<D1, T>,
    I: NVec<D1, usize>,
{
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D2>) -> T {
        let idx = self.to_d1_idx(idx.into_idx());
        self.flat_vec.at(idx)
    }

    fn child(&self, i: <D2 as Dim>::ChildIdx) -> impl NVec<<D2 as Dim>::PrevDim, T> {
        let (begin, end) = self.row_range(i);
        FunVec::new(move |[j]| self.at([i, j]), CardD1::from(end - begin))
    }

    fn all(&self) -> impl Iterator<Item = T> {
        self.flat_vec.all()
    }
}
