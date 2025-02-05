use super::{FlatJagged, FlatJaggedRowMut};
use crate::{CardD1, Dim, FunVec, IdxLeqD0, IdxLeqD1, IntoIdx, NVec, NVecCoreSealed, D1, D2};

// row

impl<V, I, T> NVecCoreSealed<D1, T> for FlatJaggedRowMut<'_, V, I, T>
where
    V: NVec<D1, T>,
    I: NVec<D1, usize>,
{
    fn core_num_children(&self) -> usize {
        self.jagged.card([self.i])
    }

    fn core_card(&self, idx: impl Into<<D1 as Dim>::CardIdx>) -> usize {
        match idx.into() {
            IdxLeqD0::IdxD0(_) => self.jagged.card([self.i]),
        }
    }

    fn core_child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D1 as Dim>::PrevDim, T> {
        self
    }

    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D1>, f: &mut F) -> O {
        let [j] = idx.into_idx();
        let idx = self.jagged.to_d1_idx([self.i, j]);
        f(&self.jagged.flat_vec.at(idx))
    }

    fn core_is_rectangular(&self) -> bool {
        true
    }
}

// vec

impl<V, I, T> NVecCoreSealed<D2, T> for FlatJagged<V, I, T>
where
    V: NVec<D1, T>,
    I: NVec<D1, usize>,
{
    fn core_num_children(&self) -> usize {
        self.num_rows()
    }

    fn core_card(&self, idx: impl Into<<D2 as Dim>::CardIdx>) -> usize {
        match idx.into() {
            IdxLeqD1::IdxD0(_) => self.num_rows(),
            IdxLeqD1::IdxD1([i]) => {
                let (begin, end) = self.row_range(i);
                end - begin
            }
        }
    }

    fn core_child(&self, i: <D2 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D2 as Dim>::PrevDim, T> {
        let (begin, end) = self.row_range(i);
        FunVec::new(move |[j]| self.at([i, j]), CardD1::from(end - begin))
    }

    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D2>, f: &mut F) -> O {
        let idx = self.to_d1_idx(idx.into_idx());
        f(&self.flat_vec.at(idx))
    }

    fn core_is_rectangular(&self) -> bool {
        self.is_rectangular()
    }
}
