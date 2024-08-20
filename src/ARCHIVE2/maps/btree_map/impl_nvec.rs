use crate::{dimensions::dim::*, FromIndex, IntoIndex, NVec};
use alloc::collections::btree_map::BTreeMap;

impl<I, T> NVec<D1> for BTreeMap<I, T>
where
    I: FromIndex<D1>,
{
    type Element<'e> = Option<&'e T> where Self: 'e;

    fn at<'e, Idx: IntoIndex<D1>>(&'e self, index: Idx) -> Self::Element<'e>
    where
        Self: 'e,
    {
        let index = I::from_index(index.into_index());
        self.get(&index)
    }
}

// full-indexed

macro_rules! implement_idx {
    ($dim:tt) => {
        impl<I, T> NVec<$dim> for BTreeMap<I, T>
        where
            I: FromIndex<$dim>,
        {
            type Element<'e> = Option<&'e T> where Self: 'e;

            fn at<'e, Idx: IntoIndex<$dim>>(&'e self, index: Idx) -> Self::Element<'e>
            where
                Self: 'e,
            {
                let index = I::from_index(index.into_index());
                self.get(&index)
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

// recursive

macro_rules! implement_rec {
    ($dim:tt) => {
        impl<C> NVec<$dim> for BTreeMap<usize, C>
        where
            C: NVec<<$dim as Dim>::PREVIOUS>,
        {
            type Element<'e> = Option<C::Element<'e>> where Self: 'e;

            fn at<'e, Idx: IntoIndex<$dim>>(&'e self, index: Idx) -> Self::Element<'e>
            where
                Self: 'e,
            {
                let (i, index) = index.split();
                self.get(&i).map(|c| c.at(index))
            }
        }
    };
}

implement_rec!(D2);
implement_rec!(D3);
implement_rec!(D4);
implement_rec!(D5);
implement_rec!(D6);
implement_rec!(D7);
implement_rec!(D8);
