pub trait CopyOrLazy<T> {
    fn value(&self) -> T;
}

pub struct LazyCtor<T, F>
where
    F: Fn() -> T,
{
    create: F,
}

impl<T: Copy> CopyOrLazy<T> for T {
    #[inline]
    fn value(&self) -> T {
        *self
    }
}

impl<T, F> CopyOrLazy<T> for LazyCtor<T, F>
where
    F: Fn() -> T,
{
    #[inline]
    fn value(&self) -> T {
        (self.create)()
    }
}
