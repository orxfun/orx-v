use crate::{cardinality::panic_on_all_when_udd, Dim, IntoIdx, NVec, D1, D2};
use crate::{NVecCore, NVecCoreSealed, NVecMut, D3, D4};
use core::fmt::Debug;
use core::marker::PhantomData;

// D2 -> D1
pub struct ChildD2D1<V, T>
where
    V: NVecCore<D2, T>,
{
    pub(crate) parent: V,
    pub(crate) i: usize,
    pub(crate) phantom: PhantomData<T>,
}

impl<V, T> Debug for ChildD2D1<V, T>
where
    T: Debug,
    V: NVecCore<D2, T>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ kind: VecChild, dim: D1, is_bounded: {}, values: ",
            self.parent.core_is_bounded(),
        )?;
        crate::common_trait_helpers::debug::dbg_values_d1(f, self)?;
        write!(f, " }}")
    }
}

impl<V, T> NVecCoreSealed<D1, T> for ChildD2D1<V, T>
where
    V: NVecCore<D2, T>,
{
    #[inline(always)]
    fn core_num_children(&self) -> usize {
        self.parent.core_card([self.i])
    }

    #[inline(always)]
    fn core_card(&self, _: impl Into<<D1 as Dim>::CardIdx>) -> usize {
        self.parent.core_card([self.i])
    }

    fn core_child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D1 as Dim>::PrevDim, T> {
        self
    }

    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D1>, f: &mut F) -> O {
        let [j] = idx.into_idx();
        let idx = [self.i, j];
        self.parent.core_map(idx, f)
    }

    fn core_is_rectangular(&self) -> bool {
        true
    }
}

impl<V, T> NVec<D1, T> for ChildD2D1<V, T>
where
    V: NVec<D2, T>,
{
    fn at(&self, idx: impl IntoIdx<D1>) -> T {
        let [j] = idx.into_idx();
        let idx = [self.i, j];
        self.parent.at(idx)
    }

    fn child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVec<<D1 as Dim>::PrevDim, T> {
        self
    }

    fn all(&self) -> impl Iterator<Item = T> {
        panic_on_all_when_udd(!self.parent.core_is_bounded());
        (0..self.core_num_children()).map(|j| self.parent.at([self.i, j]))
    }
}

impl<V, T> NVecMut<D1, T> for ChildD2D1<V, T>
where
    V: NVecMut<D2, T>,
{
    fn at_mut<Idx: IntoIdx<D1>>(&mut self, idx: Idx) -> &mut T {
        let [k] = idx.into_idx();
        self.parent.at_mut([self.i, k])
    }

    fn set<Idx: IntoIdx<D1>>(&mut self, idx: Idx, value: T) {
        let [k] = idx.into_idx();
        self.parent.set([self.i, k], value);
    }

    fn child_mut(&mut self, _: <D1 as Dim>::ChildIdx) -> impl NVecMut<<D1 as Dim>::PrevDim, T> {
        self
    }

    fn mut_all<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        panic_on_all_when_udd(!self.parent.core_is_bounded());
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

// D3 -> D1
pub struct ChildD3D1<V, T>
where
    V: NVecCoreSealed<D3, T>,
{
    pub(crate) parent: V,
    pub(crate) i: usize,
    pub(crate) j: usize,
    pub(crate) phantom: PhantomData<T>,
}

impl<V, T> Debug for ChildD3D1<V, T>
where
    T: Debug,
    V: NVecCoreSealed<D3, T>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ kind: VecChild, dim: D1, is_bounded: {}, values: ",
            self.parent.core_is_bounded(),
        )?;
        crate::common_trait_helpers::debug::dbg_values_d1(f, self)?;
        write!(f, " }}")
    }
}

impl<V, T> NVecCoreSealed<D1, T> for ChildD3D1<V, T>
where
    V: NVecCoreSealed<D3, T>,
{
    #[inline(always)]
    fn core_num_children(&self) -> usize {
        self.parent.core_card([self.i, self.j])
    }

    #[inline(always)]
    fn core_card(&self, _: impl Into<<D1 as Dim>::CardIdx>) -> usize {
        self.parent.core_card([self.i, self.j])
    }

    fn core_child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D1 as Dim>::PrevDim, T> {
        self
    }

    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D1>, f: &mut F) -> O {
        let [k] = idx.into_idx();
        let idx = [self.i, self.j, k];
        self.parent.core_map(idx, f)
    }

    fn core_is_rectangular(&self) -> bool {
        true
    }
}

impl<V, T> NVec<D1, T> for ChildD3D1<V, T>
where
    V: NVec<D3, T>,
{
    fn at(&self, idx: impl IntoIdx<D1>) -> T {
        let [k] = idx.into_idx();
        let idx = [self.i, self.j, k];
        self.parent.at(idx)
    }

    fn child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVec<<D1 as Dim>::PrevDim, T> {
        self
    }

    fn all(&self) -> impl Iterator<Item = T> {
        panic_on_all_when_udd(!self.parent.core_is_bounded());
        (0..self.core_num_children()).map(|k| self.parent.at([self.i, self.j, k]))
    }
}

impl<V, T> NVecMut<D1, T> for ChildD3D1<V, T>
where
    V: NVecMut<D3, T>,
{
    fn at_mut<Idx: IntoIdx<D1>>(&mut self, idx: Idx) -> &mut T {
        let [k] = idx.into_idx();
        let idx = [self.i, self.j, k];
        self.parent.at_mut(idx)
    }

    fn set<Idx: IntoIdx<D1>>(&mut self, idx: Idx, value: T) {
        let [k] = idx.into_idx();
        let idx = [self.i, self.j, k];
        self.parent.set(idx, value);
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

// D4 -> D1
pub struct ChildD4D1<V, T>
where
    V: NVecCoreSealed<D4, T>,
{
    pub(crate) parent: V,
    pub(crate) i: usize,
    pub(crate) j: usize,
    pub(crate) k: usize,
    pub(crate) phantom: PhantomData<T>,
}

impl<V, T> Debug for ChildD4D1<V, T>
where
    T: Debug,
    V: NVecCoreSealed<D4, T>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ kind: VecChild, dim: D1, is_bounded: {}, values: ",
            self.parent.core_is_bounded(),
        )?;
        crate::common_trait_helpers::debug::dbg_values_d1(f, self)?;
        write!(f, " }}")
    }
}

impl<V, T> NVecCoreSealed<D1, T> for ChildD4D1<V, T>
where
    V: NVecCoreSealed<D4, T>,
{
    #[inline(always)]
    fn core_num_children(&self) -> usize {
        self.parent.core_card([self.i, self.j, self.k])
    }

    #[inline(always)]
    fn core_card(&self, _: impl Into<<D1 as Dim>::CardIdx>) -> usize {
        self.parent.core_card([self.i, self.j, self.k])
    }

    fn core_child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D1 as Dim>::PrevDim, T> {
        self
    }

    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D1>, f: &mut F) -> O {
        let [l] = idx.into_idx();
        let idx = [self.i, self.j, self.k, l];
        self.parent.core_map(idx, f)
    }

    fn core_is_rectangular(&self) -> bool {
        true
    }
}

impl<V, T> NVec<D1, T> for ChildD4D1<V, T>
where
    V: NVec<D4, T>,
{
    fn at(&self, idx: impl IntoIdx<D1>) -> T {
        let [l] = idx.into_idx();
        let idx = [self.i, self.j, self.k, l];
        self.parent.at(idx)
    }

    fn child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVec<<D1 as Dim>::PrevDim, T> {
        self
    }

    fn all(&self) -> impl Iterator<Item = T> {
        panic_on_all_when_udd(!self.parent.core_is_bounded());
        (0..self.core_num_children()).map(|l| self.parent.at([self.i, self.j, self.k, l]))
    }
}

impl<V, T> NVecMut<D1, T> for ChildD4D1<V, T>
where
    V: NVecMut<D4, T>,
{
    fn at_mut<Idx: IntoIdx<D1>>(&mut self, idx: Idx) -> &mut T {
        let [l] = idx.into_idx();
        let idx = [self.i, self.j, self.k, l];
        self.parent.at_mut(idx)
    }

    fn set<Idx: IntoIdx<D1>>(&mut self, idx: Idx, value: T) {
        let [l] = idx.into_idx();
        let idx = [self.i, self.j, self.k, l];
        self.parent.set(idx, value);
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
