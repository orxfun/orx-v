use super::layout::{V1LayoutRowMajor, V1MatrixLayout};
use crate::{Dim, IntoIdx, NVec, NVecCoreSealed, NVecMut, D1};
use core::marker::PhantomData;

pub struct Row<T, V>
where
    V: NVec<D1, T>,
{
    data: V,
    layout: V1LayoutRowMajor,
    i: usize,
    phantom: PhantomData<T>,
}

impl<T, V> Row<T, V>
where
    V: NVec<D1, T>,
{
    pub(super) fn new(data: V, layout: V1LayoutRowMajor, i: usize) -> Self {
        Self {
            data,
            layout,
            i,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    fn v1_idx(&self, j: usize) -> usize {
        self.layout.v1_idx(self.i, j)
    }
}

impl<T, V> NVecCoreSealed<D1, T> for Row<T, V>
where
    V: NVec<D1, T>,
{
    fn core_num_children(&self) -> usize {
        self.layout.num_cols()
    }

    fn core_card(&self, _: impl Into<<D1 as Dim>::CardIdx>) -> usize {
        self.layout.num_cols()
    }

    fn core_child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D1 as Dim>::PrevDim, T> {
        self
    }

    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D1>, f: &mut F) -> O {
        let [j] = idx.into_idx();
        self.data.core_map(self.v1_idx(j), f)
    }

    fn core_is_rectangular(&self) -> bool {
        true
    }
}

impl<T, V> NVec<D1, T> for Row<T, V>
where
    V: NVec<D1, T>,
{
    fn at(&self, idx: impl IntoIdx<D1>) -> T {
        let [j] = idx.into_idx();
        self.data.at(self.v1_idx(j))
    }

    fn child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVec<<D1 as Dim>::PrevDim, T> {
        self
    }

    fn all(&self) -> impl Iterator<Item = T> {
        (0..self.core_num_children()).map(|j| self.data.at(self.v1_idx(j)))
    }
}

impl<T, V> NVecMut<D1, T> for Row<T, V>
where
    V: NVecMut<D1, T>,
{
    fn at_mut<Idx: IntoIdx<D1>>(&mut self, idx: Idx) -> &mut T {
        let [j] = idx.into_idx();
        self.data.at_mut(self.v1_idx(j))
    }

    fn set<Idx: IntoIdx<D1>>(&mut self, idx: Idx, value: T) {
        let [j] = idx.into_idx();
        self.data.set(self.v1_idx(j), value);
    }

    fn child_mut(&mut self, _: <D1 as Dim>::ChildIdx) -> impl NVecMut<<D1 as Dim>::PrevDim, T> {
        self
    }

    fn mut_all<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        for j in 0..self.core_num_children() {
            f(self.at_mut(j))
        }
    }

    fn reset_all(&mut self, value: T)
    where
        T: PartialEq + Copy,
    {
        for j in 0..self.core_num_children() {
            self.data.set([j], value);
        }
    }
}
