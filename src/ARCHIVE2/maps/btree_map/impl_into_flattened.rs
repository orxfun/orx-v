use crate::{dimensions::dim::*, Flattened, FromIndex, IntoFlattened, NVec};
use alloc::collections::btree_map::BTreeMap;

// impl<'v, I, T> IntoFlattened<'v, D2, Option<T>> for BTreeMap<I, T>
// where
//     I: FromIndex<D2> + 'v,
//     T: Copy + 'v,
// {
//     fn flattened(
//         &'v self,
//     ) -> Flattened<'v, D2, Self, Option<T>, impl Fn(Self::Element<'v>) -> Option<T>> {
//         Flattened::new(self, |x| x.copied())
//     }
// }

// impl<C> NVec<$dim> for BTreeMap<usize, C>
//         where
//             C: NVec<<$dim as Dim>::PREVIOUS>,
//         {
//             type Element<'e> = Option<C::Element<'e>> where Self: 'e;

//             fn at<'e, Idx: IntoIndex<$dim>>(&'e self, index: Idx) -> Self::Element<'e>
//             where
//                 Self: 'e,
//             {
//                 let (i, index) = index.split();
//                 self.get(&i).map(|c| c.at(index))
//             }
//         }

impl<'v, C, T> IntoFlattened<'v, D2, T> for BTreeMap<usize, C>
where
    Self: NVec<D2, Element<'v> = Option<T>>,
    C: NVec<<D2 as Dim>::PREVIOUS> + 'v,
    T: 'v,
{
    fn flattened(&'v self) -> Flattened<'v, D2, Self, T, impl Fn(Self::Element<'v>) -> T> {
        Flattened::new(self, |x: Option<T>| x.unwrap())
    }
}
