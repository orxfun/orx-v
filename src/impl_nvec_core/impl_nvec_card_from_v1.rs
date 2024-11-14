/// Implements `NVecCoreSealed<D1>` for a struct which is sort of like a 1-dimensional
/// vector, such as the standard `Vec<T>`.
#[macro_export]
macro_rules! impl_v1_card {
    ([$($impl_generics:tt)*], $V:ty, [$($where:tt)*]) => {
        #[allow(unused_imports)]
        use $crate::*;

        impl<$($impl_generics)*> NVecCoreSealed<D1, T> for $V where $($where)* {
            #[inline(always)]
            fn core_num_children(&self) -> usize {
                self.len()
            }

            #[inline(always)]
            fn core_card(&self, _: impl Into<<D1 as Dim>::CardIdx>) -> usize {
                self.len()
            }

            fn core_child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D1 as Dim>::PrevDim, T> {
                self
            }

            #[inline(always)]
            fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D1>, f: &mut F) -> O {
                f(&self[idx.into_idx()[0]])
            }

            fn core_is_rectangular(&self) -> bool{
                true
            }
        }
    };
    ($const_arg:tt, [$($impl_generics:tt)*], $V:ty, [$($where:tt)*]) => {
        #[allow(unused_imports)]
        use $crate::*;

        impl<const $const_arg: usize, $($impl_generics)*> NVecCoreSealed<D1, T> for $V where $($where)* {
            #[inline(always)]
            fn core_num_children(&self) -> usize {
                self.len()
            }

            #[inline(always)]
            fn core_card(&self, _: impl Into<<D1 as Dim>::CardIdx>) -> usize {
                self.len()
            }

            fn core_child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D1 as Dim>::PrevDim, T> {
                self
            }

            #[inline(always)]
            fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D1>, f: &mut F) -> O {
                f(&self[idx.into_idx()[0]])
            }

            fn core_is_rectangular(&self) -> bool{
                true
            }
        }
    };
}

/// Implements `NVecCoreSealed<D>` for a struct which is sort of like a 1-dimensional
/// vector, such as the standard `Vec<T>`, and a dimension `D` which is at least
/// 2-dimensional.
#[macro_export]
macro_rules! impl_vn_card {
    ($dim:ty, [$($impl_generics:tt)*], $V:ty, [$($where:tt)*]) => {
        #[allow(unused_imports)]
        use $crate::*;

        impl<$($impl_generics)*> NVecCoreSealed<$dim, T> for $V where $($where)* {
            #[inline(always)]
            fn core_num_children(&self) -> usize {
                self.len()
            }

            #[inline(always)]
            fn core_card(&self, idx: impl Into<<$dim as Dim>::CardIdx>) -> usize {
                idx.into().card(self)
            }

            fn core_child(&self, i: <$dim as Dim>::ChildIdx) -> impl NVecCoreSealed<<$dim as Dim>::PrevDim, T> {
                &self[i]
            }

            #[allow(unused_imports)]
            fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<$dim>, f: &mut F) -> O {
                let (i, c_idx) = idx.into_idx().split_idx();
                let child = <$V as NVecCoreSealed<$dim, T>>::core_child(self, i);
                child.core_map(c_idx, f)
            }

            fn core_is_rectangular(&self) -> bool{
                <$dim as $crate::cardinality::IsRectangular>::is_rectangular(self)
            }
        }
    };
    ($dim:ty, $const_arg:tt, [$($impl_generics:tt)*], $V:ty, [$($where:tt)*]) => {
        #[allow(unused_imports)]
        use $crate::*;

        impl<const $const_arg: usize, $($impl_generics)*> NVecCoreSealed<$dim, T> for $V where $($where)* {
            #[inline(always)]
            fn core_num_children(&self) -> usize {
                self.len()
            }

            #[inline(always)]
            fn core_card(&self, idx: impl Into<<$dim as Dim>::CardIdx>) -> usize {
                idx.into().card(self)
            }

            fn core_child(&self, i: <$dim as Dim>::ChildIdx) -> impl NVecCoreSealed<<$dim as Dim>::PrevDim, T> {
                &self[i]
            }

            #[allow(unused_imports)]
            fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<$dim>, f: &mut F) -> O {
                let (i, c_idx) = idx.into_idx().split_idx();
                let child = <$V as NVecCoreSealed<$dim, T>>::core_child(self, i);
                child.core_map(c_idx, f)
            }

            fn core_is_rectangular(&self) -> bool{
                <$dim as $crate::cardinality::IsRectangular>::is_rectangular(self)
            }
        }
    };
}
