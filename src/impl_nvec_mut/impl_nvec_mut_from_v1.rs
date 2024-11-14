/// Implements `NVecMut<D1, T>` for a struct which is sort of like a 1-dimensional
/// vector, such as the standard `Vec<T>`.
#[macro_export]
macro_rules! impl_v1_mut {
    ([$($impl_generics:tt)*], $V:ty, [$($where:tt)*]) => {
        #[allow(unused_imports)]
        use $crate::*;

        impl<$($impl_generics)*> NVecMut<D1, T> for $V where $($where)* {
            #[inline(always)]
            fn at_mut<Idx: IntoIdx<D1>>(&mut self, idx: Idx) -> &mut T {
                &mut self[idx.into_idx()[0]]
            }

            #[inline(always)]
            fn set<Idx: IntoIdx<D1>>(&mut self, idx: Idx, value: T) {
                self[idx.into_idx()[0]] = value;
            }

            fn child_mut(&mut self, _: IdxNever) -> impl NVecMut<<D1 as Dim>::PrevDim, T> {
                self
            }

            fn mut_all<F>(&mut self, mut f: F)
            where
                F: FnMut(&mut T),
            {
                for x in self.iter_mut() {
                    f(x);
                }
            }

            fn reset_all(&mut self, value: T)
            where
                T: PartialEq + Copy,
            {
                self.mut_all(|x| *x = value);
            }
        }
    };
    ($const_arg:tt, [$($impl_generics:tt)*], $V:ty, [$($where:tt)*]) => {
        #[allow(unused_imports)]
        use $crate::*;

        impl<const $const_arg: usize, $($impl_generics)*> NVecMut<D1, T> for $V where $($where)* {
            #[inline(always)]
            fn at_mut<Idx: IntoIdx<D1>>(&mut self, idx: Idx) -> &mut T {
                &mut self[idx.into_idx()[0]]
            }

            #[inline(always)]
            fn set<Idx: IntoIdx<D1>>(&mut self, idx: Idx, value: T) {
                self[idx.into_idx()[0]] = value;
            }

            fn child_mut(&mut self, _: IdxNever) -> impl NVecMut<<D1 as Dim>::PrevDim, T> {
                self
            }

            fn mut_all<F>(&mut self, mut f: F)
            where
                F: FnMut(&mut T),
            {
                for x in self.iter_mut() {
                    f(x);
                }
            }

            fn reset_all(&mut self, value: T)
            where
                T: PartialEq + Copy,
            {
                self.mut_all(|x| *x = value);
            }
        }
    };
}

/// Implements `NVecMut<D, T>` for a struct which is sort of like a 1-dimensional
/// vector, such as the standard `Vec<T>`, and a dimension `D` which is at least
/// 2-dimensional.
#[macro_export]
macro_rules! impl_vn_mut {
    ($dim:ty, [$($impl_generics:tt)*], $V:ty, [$($where:tt)*]) => {
        #[allow(unused_imports)]
        use $crate::*;

        impl<$($impl_generics)*> NVecMut<$dim, T> for $V where $($where)* {
            #[inline(always)]
            fn at_mut<Idx: IntoIdx<$dim>>(&mut self, idx: Idx) -> &mut T {
                let (i, c_idx) = idx.into_idx().split_idx();
                self[i].at_mut(c_idx)
            }

            #[inline(always)]
            fn set<Idx: IntoIdx<$dim>>(&mut self, idx: Idx, value: T) {
                let (i, c_idx) = idx.into_idx().split_idx();
                self[i].set(c_idx, value);
            }

            fn child_mut(&mut self, i: <$dim as Dim>::ChildIdx) -> impl NVecMut<<$dim as Dim>::PrevDim, T> {
                &mut self[i]
            }

            fn mut_all<F>(&mut self, mut f: F)
            where
                F: FnMut(&mut T),
            {
                for x in self.iter_mut() {
                    x.mut_all(&mut f);
                }
            }

            fn reset_all(&mut self, value: T)
            where
                T: PartialEq + Copy,
            {
                self.mut_all(|x| *x = value);
            }
        }
    };
    ($dim:ty, $const_arg:tt, [$($impl_generics:tt)*], $V:ty, [$($where:tt)*]) => {
        #[allow(unused_imports)]
        use $crate::*;

        impl<const $const_arg: usize, $($impl_generics)*> NVecMut<$dim, T> for $V where $($where)* {
            #[inline(always)]
            fn at_mut<Idx: IntoIdx<$dim>>(&mut self, idx: Idx) -> &mut T {
                let (i, c_idx) = idx.into_idx().split_idx();
                self[i].at_mut(c_idx)
            }

            #[inline(always)]
            fn set<Idx: IntoIdx<$dim>>(&mut self, idx: Idx, value: T) {
                let (i, c_idx) = idx.into_idx().split_idx();
                self[i].set(c_idx, value);
            }

            fn child_mut(&mut self, i: <$dim as Dim>::ChildIdx) -> impl NVecMut<<$dim as Dim>::PrevDim, T> {
                &mut self[i]
            }

            fn mut_all<F>(&mut self, mut f: F)
            where
                F: FnMut(&mut T),
            {
                for x in self.iter_mut() {
                    x.mut_all(&mut f);
                }
            }

            fn reset_all(&mut self, value: T)
            where
                T: PartialEq + Copy,
            {
                self.mut_all(|x| *x = value);
            }
        }
    };
}
