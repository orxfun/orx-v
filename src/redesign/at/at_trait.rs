use super::super::{DNever, Dim};
use super::Copied;

pub trait At<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> T;

    fn try_at(&self, idx: D::Idx) -> Option<T>;

    type Child<'c>: At<D::PrevDim, T>
    where
        Self: 'c;
}

// never

pub struct AtNever;

impl<T> At<DNever, T> for AtNever {
    fn at(&self, _: <DNever as Dim>::Idx) -> T {
        unreachable!()
    }

    fn try_at(&self, _: <DNever as Dim>::Idx) -> Option<T> {
        unreachable!()
    }

    type Child<'c>
        = Self
    where
        Self: 'c;
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
