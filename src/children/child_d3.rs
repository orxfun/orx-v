use super::child_d2::ChildD4D2;
use crate::{cardinality::panic_on_all_when_udd, Dim, IdxLeqD2, IntoIdx, NVec, D3, D4};
use crate::{NVecCore, NVecCoreSealed, NVecMut};
use core::fmt::Debug;
use core::marker::PhantomData;

// D4 -> D3
pub struct ChildD4D3<V, T>
where
    V: NVecCore<D4, T>,
{
    pub(crate) parent: V,
    pub(crate) i: usize,
    pub(crate) phantom: PhantomData<T>,
}

impl<V, T> Debug for ChildD4D3<V, T>
where
    T: Debug,
    V: NVecCore<D4, T>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ kind: VecChild, dim: D3, is_bounded: {}, values: ",
            self.parent.core_is_bounded(),
        )?;
        crate::common_trait_helpers::debug::dbg_values_d3(f, self)?;
        write!(f, " }}")
    }
}

impl<V, T> NVecCoreSealed<D3, T> for ChildD4D3<V, T>
where
    V: NVecCore<D4, T>,
{
    fn core_num_children(&self) -> usize {
        self.parent.core_card([self.i])
    }

    fn core_card(&self, idx: impl Into<<D3 as Dim>::CardIdx>) -> usize {
        match idx.into() {
            IdxLeqD2::IdxD0(_) => self.parent.core_card([self.i]),
            IdxLeqD2::IdxD1([j]) => self.parent.core_card([self.i, j]),
            IdxLeqD2::IdxD2([j, k]) => self.parent.core_card([self.i, j, k]),
        }
    }

    fn core_child(&self, j: <D3 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D3 as Dim>::PrevDim, T> {
        ChildD4D2::<_, T> {
            i: self.i,
            j,
            parent: &self.parent,
            phantom: Default::default(),
        }
    }

    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D3>, f: &mut F) -> O {
        let [j, k, l] = idx.into_idx();
        let idx = [self.i, j, k, l];
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

impl<V, T> NVec<D3, T> for ChildD4D3<V, T>
where
    V: NVec<D4, T>,
{
    fn at(&self, idx: impl IntoIdx<D3>) -> T {
        let [j, k, l] = idx.into_idx();
        let idx = [self.i, j, k, l];
        self.parent.at(idx)
    }

    fn child(&self, j: <D3 as Dim>::ChildIdx) -> impl NVec<<D3 as Dim>::PrevDim, T> {
        ChildD4D2 {
            i: self.i,
            j,
            parent: &self.parent,
            phantom: Default::default(),
        }
    }

    fn all(&self) -> impl Iterator<Item = T> {
        panic_on_all_when_udd(self.core_num_children() == usize::MAX);
        (0..self.core_num_children()).flat_map(move |j| {
            (0..self.parent.core_card([self.i, j])).flat_map(move |k| {
                (0..self.parent.core_card([self.i, j, k]))
                    .map(move |l| self.parent.at([self.i, j, k, l]))
            })
        })
    }
}

impl<V, T> NVecMut<D3, T> for ChildD4D3<V, T>
where
    V: NVecMut<D4, T>,
{
    fn at_mut<Idx: IntoIdx<D3>>(&mut self, idx: Idx) -> &mut T {
        let [j, k, l] = idx.into_idx();
        let idx = [self.i, j, k, l];
        self.parent.at_mut(idx)
    }

    fn set<Idx: IntoIdx<D3>>(&mut self, idx: Idx, value: T) {
        let [j, k, l] = idx.into_idx();
        let idx = [self.i, j, k, l];
        self.parent.set(idx, value);
    }

    fn child_mut(&mut self, j: <D3 as Dim>::ChildIdx) -> impl NVecMut<<D3 as Dim>::PrevDim, T> {
        ChildD4D2 {
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
                for l in 0..self.card([j, k]) {
                    f(self.at_mut([j, k, l]));
                }
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
