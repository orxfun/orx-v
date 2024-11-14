/// Implements `NVec<D1, T>` for a struct which is sort of like a 1-dimensional
/// vector, such as the standard `Vec<T>`.
#[macro_export]
macro_rules! impl_v1 {
    ([$($impl_generics:tt)*], $V:ty, [$($where:tt)*]) => {
        #[allow(unused_imports)]
        use $crate::*;

        impl<$($impl_generics)*> NVec<D1, T> for $V where $($where)* {
            #[inline(always)]
            fn at(&self, idx: impl IntoIdx<D1>) -> T {
                self[idx.into_idx()[0]]
            }

            fn child(&self, _: IdxNever) -> impl NVec<<D1 as Dim>::PrevDim, T> {
                self
            }

            fn all(&self) -> impl Iterator<Item = T> {
                self.iter().copied()
            }
        }
    };
    ($const_arg:tt, [$($impl_generics:tt)*], $V:ty, [$($where:tt)*]) => {
        #[allow(unused_imports)]
        use $crate::*;

        impl<const $const_arg: usize, $($impl_generics)*> NVec<D1, T> for $V where $($where)* {
            #[inline(always)]
            fn at(&self, idx: impl IntoIdx<D1>) -> T {
                self[idx.into_idx()[0]]
            }

            fn child(&self, _: IdxNever) -> impl NVec<<D1 as Dim>::PrevDim, T> {
                self
            }

            fn all(&self) -> impl Iterator<Item = T> {
                self.iter().copied()
            }
        }
    };
}

/// Implements `NVec<D, T>` for a struct which is sort of like a 1-dimensional
/// vector, such as the standard `Vec<T>`, and a dimension `D` which is at least
/// 2-dimensional.
#[macro_export]
macro_rules! impl_vn {
    ($dim:ty, [$($impl_generics:tt)*], $V:ty, [$($where:tt)*]) => {
        #[allow(unused_imports)]
        use $crate::*;

        impl<$($impl_generics)*> NVec<$dim, T> for $V where $($where)* {
            #[inline(always)]
            fn at(&self, idx: impl IntoIdx<$dim>) -> T {
                let (i, c_idx) = idx.into_idx().split_idx();
                self.child(i).at(c_idx)
            }

            fn child(&self, i: usize) -> impl NVec<<$dim as Dim>::PrevDim, T> {
                &self[i]
            }

            fn all(&self) -> impl Iterator<Item = T> {
                self.iter().flat_map(|x| x.all())
            }
        }
    };
    ($dim:ty, $const_arg:tt, [$($impl_generics:tt)*], $V:ty, [$($where:tt)*]) => {
        #[allow(unused_imports)]
        use $crate::*;

        impl<const $const_arg: usize, $($impl_generics)*> NVec<$dim, T> for $V where $($where)* {
            #[inline(always)]
            fn at(&self, idx: impl IntoIdx<$dim>) -> T {
                let (i, c_idx) = idx.into_idx().split_idx();
                self.child(i).at(c_idx)
            }

            fn child(&self, i: usize) -> impl NVec<<$dim as Dim>::PrevDim, T> {
                &self[i]
            }

            fn all(&self) -> impl Iterator<Item = T> {
                self.iter().flat_map(|x| x.all())
            }
        }
    };
}
