use super::child_d1::{ChildD3D1, ChildD4D1};
use crate::{cardinality::panic_on_all_when_udd, Dim, IdxLeqD1, IntoIdx, NVec, D2, D3, D4};
use crate::{NVecCore, NVecCoreSealed, NVecMut};
use core::fmt::Debug;
use core::marker::PhantomData;

// D3 -> D2
pub struct ChildD3D2<V, T>
where
    V: NVecCore<D3, T>,
{
    pub(crate) parent: V,
    pub(crate) i: usize,
    pub(crate) phantom: PhantomData<T>,
}

impl<V, T> Debug for ChildD3D2<V, T>
where
    T: Debug,
    V: NVecCore<D3, T>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ kind: VecChild, dim: D2, is_bounded: {}, values: ",
            self.parent.core_is_bounded(),
        )?;
        crate::common_trait_helpers::debug::dbg_values_d2(f, self)?;
        write!(f, " }}")
    }
}

impl<V, T> NVecCoreSealed<D2, T> for ChildD3D2<V, T>
where
    V: NVecCore<D3, T>,
{
    fn core_num_children(&self) -> usize {
        self.parent.core_card([self.i])
    }

    fn core_card(&self, idx: impl Into<<D2 as Dim>::CardIdx>) -> usize {
        match idx.into() {
            IdxLeqD1::IdxD0(_) => self.parent.core_card([self.i]),
            IdxLeqD1::IdxD1([j]) => self.parent.core_card([self.i, j]),
        }
    }

    fn core_child(&self, j: <D2 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D2 as Dim>::PrevDim, T> {
        ChildD3D1::<_, T> {
            i: self.i,
            j,
            parent: &self.parent,
            phantom: Default::default(),
        }
    }

    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D2>, f: &mut F) -> O {
        let [j, k] = idx.into_idx();
        let idx = [self.i, j, k];
        self.parent.core_map(idx, f)
    }

    fn core_is_rectangular(&self) -> bool {
        match self.parent.core_card([self.i]) {
            0 => true,
            n => {
                let m = self.parent.core_card([self.i, 0]);
                for j in 1..n {
                    if self.parent.core_card([self.i, j]) != m {
                        return false;
                    }
                }
                true
            }
        }
    }
}

impl<V, T> NVec<D2, T> for ChildD3D2<V, T>
where
    V: NVec<D3, T>,
{
    fn at(&self, idx: impl IntoIdx<D2>) -> T {
        let [j, k] = idx.into_idx();
        let idx = [self.i, j, k];
        self.parent.at(idx)
    }

    fn child(&self, j: <D2 as Dim>::ChildIdx) -> impl NVec<<D2 as Dim>::PrevDim, T> {
        ChildD3D1 {
            i: self.i,
            j,
            parent: &self.parent,
            phantom: Default::default(),
        }
    }

    fn all(&self) -> impl Iterator<Item = T> {
        panic_on_all_when_udd(self.core_num_children() == usize::MAX);
        (0..self.core_num_children()).flat_map(move |j| {
            (0..self.parent.core_card([self.i, j])).map(move |k| self.parent.at([self.i, j, k]))
        })
    }
}

impl<V, T> NVecMut<D2, T> for ChildD3D2<V, T>
where
    V: NVecMut<D3, T>,
{
    fn at_mut<Idx: IntoIdx<D2>>(&mut self, idx: Idx) -> &mut T {
        let [j, k] = idx.into_idx();
        let idx = [self.i, j, k];
        self.parent.at_mut(idx)
    }

    fn set<Idx: IntoIdx<D2>>(&mut self, idx: Idx, value: T) {
        let [j, k] = idx.into_idx();
        let idx = [self.i, j, k];
        self.parent.set(idx, value);
    }

    fn child_mut(&mut self, j: <D2 as Dim>::ChildIdx) -> impl NVecMut<<D2 as Dim>::PrevDim, T> {
        ChildD3D1 {
            i: self.i,
            j,
            parent: &mut self.parent,
            phantom: Default::default(),
        }
    }

    fn mut_all<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        for j in 0..self.num_children() {
            for k in 0..self.card([j]) {
                f(self.at_mut([j, k]));
            }
        }
    }

    fn reset_all(&mut self, value: T)
    where
        T: PartialEq + Copy,
    {
        self.mut_all(|x| *x = value);
    }
}

// D4 -> D2
pub struct ChildD4D2<V, T>
where
    V: NVecCore<D4, T>,
{
    pub(crate) parent: V,
    pub(crate) i: usize,
    pub(crate) j: usize,
    pub(crate) phantom: PhantomData<T>,
}

impl<V, T> Debug for ChildD4D2<V, T>
where
    T: Debug,
    V: NVecCore<D4, T>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ kind: VecChild, dim: D2, is_bounded: {}, values: ",
            self.parent.core_is_bounded(),
        )?;
        crate::common_trait_helpers::debug::dbg_values_d2(f, self)?;
        write!(f, " }}")
    }
}

impl<V, T> NVecCoreSealed<D2, T> for ChildD4D2<V, T>
where
    V: NVecCore<D4, T>,
{
    fn core_num_children(&self) -> usize {
        self.parent.core_card([self.i, self.j])
    }

    fn core_card(&self, idx: impl Into<<D2 as Dim>::CardIdx>) -> usize {
        match idx.into() {
            IdxLeqD1::IdxD0(_) => self.parent.core_card([self.i, self.j]),
            IdxLeqD1::IdxD1([k]) => self.parent.core_card([self.i, self.j, k]),
        }
    }

    fn core_child(&self, k: <D2 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D2 as Dim>::PrevDim, T> {
        ChildD4D1::<_, T> {
            i: self.i,
            j: self.j,
            k,
            parent: &self.parent,
            phantom: Default::default(),
        }
    }

    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D2>, f: &mut F) -> O {
        let [k, l] = idx.into_idx();
        let idx = [self.i, self.j, k, l];
        self.parent.core_map(idx, f)
    }

    fn core_is_rectangular(&self) -> bool {
        match self.parent.core_card([self.i, self.j]) {
            0 => true,
            n => {
                let m = self.parent.core_card([self.i, self.j, 0]);
                for k in 1..n {
                    if self.parent.core_card([self.i, self.j, k]) != m {
                        return false;
                    }
                }
                true
            }
        }
    }
}

impl<V, T> NVec<D2, T> for ChildD4D2<V, T>
where
    V: NVec<D4, T>,
{
    fn at(&self, idx: impl IntoIdx<D2>) -> T {
        let [k, l] = idx.into_idx();
        let idx = [self.i, self.j, k, l];
        self.parent.at(idx)
    }

    fn child(&self, k: <D2 as Dim>::ChildIdx) -> impl NVec<<D2 as Dim>::PrevDim, T> {
        ChildD4D1 {
            i: self.i,
            j: self.j,
            k,
            parent: &self.parent,
            phantom: Default::default(),
        }
    }

    fn all(&self) -> impl Iterator<Item = T> {
        panic_on_all_when_udd(self.core_num_children() == usize::MAX);
        (0..self.core_num_children()).flat_map(move |l| {
            (0..self.parent.core_card([self.i, self.j, l]))
                .map(move |k| self.parent.at([self.i, self.j, l, k]))
        })
    }
}

impl<V, T> NVecMut<D2, T> for ChildD4D2<V, T>
where
    V: NVecMut<D4, T>,
{
    fn at_mut<Idx: IntoIdx<D2>>(&mut self, idx: Idx) -> &mut T {
        let [k, l] = idx.into_idx();
        let idx = [self.i, self.j, k, l];
        self.parent.at_mut(idx)
    }

    fn set<Idx: IntoIdx<D2>>(&mut self, idx: Idx, value: T) {
        let [k, l] = idx.into_idx();
        let idx = [self.i, self.j, k, l];
        self.parent.set(idx, value);
    }

    fn child_mut(&mut self, k: <D2 as Dim>::ChildIdx) -> impl NVecMut<<D2 as Dim>::PrevDim, T> {
        ChildD4D1 {
            i: self.i,
            j: self.j,
            k,
            parent: &mut self.parent,
            phantom: Default::default(),
        }
    }

    fn mut_all<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        for j in 0..self.num_children() {
            for k in 0..self.card([j]) {
                f(self.at_mut([j, k]));
            }
        }
    }

    fn reset_all(&mut self, value: T)
    where
        T: PartialEq + Copy,
    {
        self.mut_all(|x| *x = value);
    }
}
