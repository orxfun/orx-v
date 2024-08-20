use crate::{dimensions::dim::*, IntoIndex, NVec};

impl<T> NVec<D1> for Vec<T> {
    type Element<'e> = &'e T where Self: 'e;

    #[inline]
    fn at<'e, Idx: IntoIndex<D1>>(&'e self, index: Idx) -> Self::Element<'e>
    where
        Self: 'e,
    {
        &self[index.into_index()[0]]
    }
}

macro_rules! implement {
    ($dim:tt) => {
        impl<C> NVec<$dim> for Vec<C>
        where
            C: NVec<<$dim as Dim>::PREVIOUS>,
        {
            type Element<'e> = C::Element<'e> where Self: 'e;

            #[inline]
            fn at<'e, Idx: IntoIndex<$dim>>(&'e self, index: Idx) -> Self::Element<'e>
            where
                Self: 'e,
            {
                let (i, index) = index.split();
                self[i].at(index)
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
