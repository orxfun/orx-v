use crate::{dimensions::dim::*, Copied, IntoCopied, NVec};

impl<'v, T> IntoCopied<'v, D1, T> for Vec<T>
where
    T: Copy + 'v,
{
    fn copied(&'v self) -> Copied<'v, D1, Self, T, impl Fn(Self::Element<'v>) -> T> {
        Copied::new(self, |x| *x)
    }
}

macro_rules! implement {
    ($dim:tt) => {
        impl<'v, C, T> IntoCopied<'v, $dim, T> for Vec<C>
        where
            C: NVec<<$dim as Dim>::PREVIOUS, Element<'v> = &'v T> + 'v,
            C: IntoCopied<'v, <$dim as Dim>::PREVIOUS, T>,
            T: Copy + 'v,
        {
            fn copied(&'v self) -> Copied<'v, $dim, Self, T, impl Fn(Self::Element<'v>) -> T> {
                Copied::new(self, |x: C::Element<'_>| *x)
            }
        }
    };
}

implement!(D2);
implement!(D3);
implement!(D4);
implement!(D5);
implement!(D6);
implement!(D7);
implement!(D8);
