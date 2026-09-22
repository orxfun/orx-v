use super::super::{DNever, Dim, IdxNever};
use super::Copied;

pub trait At<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> T;

    fn try_at(&self, idx: D::Idx) -> Option<T>;

    type Child<'c>: At<D::PrevDim, T>
    where
        Self: 'c;

    fn child<'c>(&'c self, c: D::ChildIdx) -> Self::Child<'c> {
        todo!()
    }
}

// never

pub struct AtNever;

impl<T> At<DNever, T> for AtNever {
    fn at(&self, _: IdxNever) -> T {
        unreachable!()
    }

    fn try_at(&self, _: IdxNever) -> Option<T> {
        unreachable!()
    }

    type Child<'c>
        = Self
    where
        Self: 'c;

    fn child<'c>(&'c self, _: IdxNever) -> Self::Child<'c> {
        unreachable!()
    }
}

// copied

pub trait AtCopied<D: Dim>: Sized {
    fn copied<'a, T>(self) -> Copied<'a, D, T, Self>
    where
        T: Copy + 'a,
        Self: At<D, &'a T>,
    {
        Copied::new(self)
    }
}

impl<D, V> AtCopied<D> for V
where
    D: Dim,
    V: Sized,
{
}
