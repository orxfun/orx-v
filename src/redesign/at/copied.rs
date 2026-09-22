use super::super::Dim;
use super::At;
use core::marker::PhantomData;
use derive_new::new;

#[derive(new)]
pub struct Copied<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Copy + 'a,
{
    v: V,
    p: PhantomData<fn() -> (D, &'a T)>,
}

impl<'a, D, T, V: Clone> Clone for Copied<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Copy + 'a,
{
    fn clone(&self) -> Self {
        Self {
            v: self.v.clone(),
            p: PhantomData,
        }
    }
}

impl<'a, D, T, V: Copy> Copy for Copied<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Copy + 'a,
{
}

impl<'a, D, T, V> At<D, T> for Copied<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Copy + 'a,
{
    fn at(&self, idx: <D as Dim>::Idx) -> T {
        *self.v.at(idx)
    }

    fn try_at(&self, idx: <D as Dim>::Idx) -> Option<T> {
        self.v.try_at(idx).copied()
    }

    type Child<'c>
        = Copied<'a, D::PrevDim, T, V::Child<'c>>
    where
        Self: 'c;

    fn child<'c>(&'c self, c: <D as Dim>::ChildIdx) -> Self::Child<'c> {
        Copied::new(self.v.child(c))
    }

    fn try_child<'c>(&'c self, c: <D as Dim>::ChildIdx) -> Option<Self::Child<'c>> {
        self.v.try_child(c).map(Copied::new)
    }
}
