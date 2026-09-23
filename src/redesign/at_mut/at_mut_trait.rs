use super::super::{DNever, Dim, IdxNever};

pub trait AtMut<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> &T;

    fn try_at(&self, idx: D::Idx) -> Option<&T>;

    fn at_mut(&mut self, idx: D::Idx) -> &mut T;

    fn try_at_mut(&mut self, idx: D::Idx) -> Option<&mut T>;

    type ChildMut<'c>: AtMut<D::PrevDim, T>
    where
        Self: 'c;

    fn child_mut<'c>(&'c mut self, c: D::ChildIdx) -> Self::ChildMut<'c>;

    fn try_child_mut<'c>(&'c mut self, c: D::ChildIdx) -> Option<Self::ChildMut<'c>>;
}

// never

pub enum AtMutNever {}

impl<T> AtMut<DNever, T> for AtMutNever {
    fn at(&self, _: IdxNever) -> &T {
        unreachable!()
    }

    fn try_at(&self, _: IdxNever) -> Option<&T> {
        unreachable!()
    }

    fn at_mut(&mut self, _: IdxNever) -> &mut T {
        unreachable!()
    }

    fn try_at_mut(&mut self, _: IdxNever) -> Option<&mut T> {
        unreachable!()
    }

    type ChildMut<'c>
        = Self
    where
        Self: 'c;

    fn child_mut<'c>(&'c mut self, _: IdxNever) -> Self::ChildMut<'c> {
        unreachable!()
    }

    fn try_child_mut<'c>(&'c mut self, _: IdxNever) -> Option<Self::ChildMut<'c>> {
        unreachable!()
    }
}

// ref_mut

impl<D, T, V> AtMut<D, T> for &mut V
where
    D: Dim,
    V: ?Sized + AtMut<D, T>,
{
    fn at(&self, idx: D::Idx) -> &T {
        (**self).at(idx)
    }

    fn try_at(&self, idx: D::Idx) -> Option<&T> {
        (**self).try_at(idx)
    }

    fn at_mut(&mut self, idx: D::Idx) -> &mut T {
        (**self).at_mut(idx)
    }

    fn try_at_mut(&mut self, idx: D::Idx) -> Option<&mut T> {
        (**self).try_at_mut(idx)
    }

    type ChildMut<'c>
        = V::ChildMut<'c>
    where
        Self: 'c;

    fn child_mut<'c>(&'c mut self, c: D::ChildIdx) -> Self::ChildMut<'c> {
        (**self).child_mut(c)
    }

    fn try_child_mut<'c>(&'c mut self, c: D::ChildIdx) -> Option<Self::ChildMut<'c>> {
        (**self).try_child_mut(c)
    }
}
