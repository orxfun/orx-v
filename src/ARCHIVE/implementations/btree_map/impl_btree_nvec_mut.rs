use crate::{dimensions::dim::*, nvec_mut::NVecMut, FromIndex, IntoIndex, ValueSetError};
use alloc::collections::btree_map::BTreeMap;

impl<I, T: Copy> NVecMut<D1, T> for BTreeMap<I, T>
where
    I: FromIndex<D1>,
{
    #[inline(always)]
    fn set<Idx: IntoIndex<D1>>(&mut self, index: Idx, value: T) -> Result<(), ValueSetError> {
        let index = I::from_index(index.into_index());
        match self.get_mut(&index) {
            Some(x) => {
                *x = value;
                Ok(())
            }
            None => Err(ValueSetError::ElementDoesNotExist),
        }
    }
}

// full-indexed

macro_rules! implement_idx {
    ($dim:tt) => {
        impl<I, T: Copy> NVecMut<$dim, T> for BTreeMap<I, T>
        where
            I: FromIndex<$dim>,
        {
            #[inline(always)]
            fn set<Idx: IntoIndex<$dim>>(
                &mut self,
                index: Idx,
                value: T,
            ) -> Result<(), ValueSetError> {
                let index = I::from_index(index.into_index());
                match self.get_mut(&index) {
                    Some(x) => {
                        *x = value;
                        Ok(())
                    }
                    None => Err(ValueSetError::ElementDoesNotExist),
                }
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
        impl<A, T: Copy> NVecMut<$dim, T> for BTreeMap<usize, A>
        where
            A: NVecMut<<$dim as Dim>::PREVIOUS, T>,
        {
            #[inline(always)]
            fn set<Idx: IntoIndex<$dim>>(
                &mut self,
                index: Idx,
                value: T,
            ) -> Result<(), ValueSetError> {
                let (i, index) = index.split();
                match self.get_mut(&i) {
                    Some(c) => c.set(index, value),
                    None => Err(ValueSetError::ElementDoesNotExist),
                }
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
