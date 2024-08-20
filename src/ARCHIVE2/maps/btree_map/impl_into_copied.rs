use crate::{dimensions::dim::*, Copied, FromIndex, IntoCopied};
use alloc::collections::btree_map::BTreeMap;

impl<'v, I, T> IntoCopied<'v, D1, Option<T>> for BTreeMap<I, T>
where
    I: FromIndex<D1> + 'v,
    T: Copy + 'v,
{
    fn copied(
        &'v self,
    ) -> Copied<'v, D1, Self, Option<T>, impl Fn(Self::Element<'v>) -> Option<T>> {
        Copied::new(self, |x| x.copied())
    }
}

// full-indexed

macro_rules! implement_idx {
    ($dim:tt) => {
        impl<'v, I, T> IntoCopied<'v, $dim, Option<T>> for BTreeMap<I, T>
        where
            I: FromIndex<$dim> + 'v,
            T: Copy + 'v,
        {
            fn copied(
                &'v self,
            ) -> Copied<'v, $dim, Self, Option<T>, impl Fn(Self::Element<'v>) -> Option<T>> {
                Copied::new(self, |x| x.copied())
            }
        }
    };
}

implement_idx!(D2);
implement_idx!(D3);
implement_idx!(D4);
implement_idx!(D5);
implement_idx!(D6);
implement_idx!(D7);
implement_idx!(D8);
