use crate::{dimensions::dim::*, nvec_rec::NVecRec, IntoIndex, NVec};

impl<'a, T: Copy> NVecRec<D1, T> for &'a [T] {
    type Child = T;

    fn num_children(&self) -> usize {
        self.len()
    }

    fn child<Idx1: IntoIndex<D1>>(&self, index: Idx1) -> Option<&Self::Child> {
        self.get(index.into_index()[0])
    }

    fn children<'c>(&'c self) -> impl Iterator<Item = &Self::Child>
    where
        Self::Child: 'c,
    {
        self.iter()
    }
}

impl<'a, T: Copy> NVecRec<D1, T> for &'a mut [T] {
    type Child = T;

    fn num_children(&self) -> usize {
        self.len()
    }

    fn child<Idx1: IntoIndex<D1>>(&self, index: Idx1) -> Option<&Self::Child> {
        self.get(index.into_index()[0])
    }

    fn children<'c>(&'c self) -> impl Iterator<Item = &Self::Child>
    where
        Self::Child: 'c,
    {
        self.iter()
    }
}

macro_rules! implement {
    ($dim:tt) => {
        impl<'a, T: Copy, A> NVecRec<$dim, T> for &'a [A]
        where
            A: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            type Child = A;

            fn num_children(&self) -> usize {
                self.len()
            }

            fn child<Idx1: IntoIndex<D1>>(&self, index: Idx1) -> Option<&Self::Child> {
                self.get(index.into_index()[0])
            }

            fn children<'c>(&'c self) -> impl Iterator<Item = &Self::Child>
            where
                Self::Child: 'c,
            {
                self.iter()
            }
        }

        impl<'a, T: Copy, A> NVecRec<$dim, T> for &'a mut [A]
        where
            A: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            type Child = A;

            fn num_children(&self) -> usize {
                self.len()
            }

            fn child<Idx1: IntoIndex<D1>>(&self, index: Idx1) -> Option<&Self::Child> {
                self.get(index.into_index()[0])
            }

            fn children<'c>(&'c self) -> impl Iterator<Item = &Self::Child>
            where
                Self::Child: 'c,
            {
                self.iter()
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
