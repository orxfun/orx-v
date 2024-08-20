use crate::{dimensions::dim::*, nvec_mut::NVecMut, IntoIndex, ValueSetError};
use alloc::vec::Vec;

impl<T: Copy> NVecMut<D1, T> for Vec<T> {
    #[inline(always)]
    fn set<Idx: IntoIndex<D1>>(&mut self, index: Idx, value: T) -> Result<(), ValueSetError> {
        let i = index.into_index()[0];
        match i < self.len() {
            true => {
                *unsafe { self.get_unchecked_mut(i) } = value;
                Ok(())
            }
            false => Err(ValueSetError::ElementDoesNotExist),
        }
    }
}

macro_rules! implement {
    ($dim:tt) => {
        impl<T: Copy, A> NVecMut<$dim, T> for Vec<A>
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
                match self.get_mut(i) {
                    Some(x) => x.set(index, value),
                    None => Err(ValueSetError::ElementDoesNotExist),
                }
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
