use super::super::Dim;
use super::Copied;

pub trait At<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> T;

    fn try_at(&self, idx: D::Idx) -> Option<T>;
}

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
