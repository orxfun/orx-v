pub trait CopyOrRef<'a, T>: Copy
where
    Self: Sized,
{
    fn from_ref(t: &'a T) -> Self;
}

impl<'a, T: Copy> CopyOrRef<'a, T> for T {
    #[inline(always)]
    fn from_ref(t: &'a T) -> Self {
        *t
    }
}

impl<'a, T> CopyOrRef<'a, T> for &'a T {
    #[inline(always)]
    fn from_ref(t: &'a T) -> Self {
        t
    }
}
