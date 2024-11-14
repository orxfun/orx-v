/// Implements `V1Mut<T>` (`NVecMut<D1, T>`) for a struct which is sort of like a 1-dimensional
/// vector, such as the standard `Vec<T>`.
#[macro_export]
macro_rules! impl_v1mut_for_v1 {
    ($v1:ty) => {
        #[allow(unused_imports)]
        use $crate::*;
        impl<T: Copy> NVecMut<D1, T> for $v1 {
            #[inline(always)]
            fn at_mut<Idx: IntoIdx<D1>>(&mut self, idx: Idx) -> &mut T {
                &mut self[idx.into_idx()[0]]
            }

            #[inline(always)]
            fn set<Idx: IntoIdx<D1>>(&mut self, idx: Idx, value: T) {
                self[idx.into_idx()[0]] = value
            }

            fn child_mut(&mut self, _: <D1 as Dim>::ChildIdx) -> impl NVecMut<<D1 as Dim>::PrevDim, T> {
                self
            }

            fn all_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
            where
                T: 'a,
            {
                self.iter_mut()
            }

            fn enumerate_all_mut<'a>(&'a mut self) -> impl Iterator<Item = (<D1 as Dim>::Idx, &'a mut T)>
            where
                T: 'a,
            {
                self.iter_mut().enumerate().map(|(i, x)| ([i], x))
            }
        }
    };

    ($v1:ty, $arg:ident, {$($where:tt)*}) => {
        #[allow(unused_imports)]
        use $crate::*;
        impl<T: Copy, $arg> NVecMut<D1, T> for $v1
        where
            $($where)*
        {
            #[inline(always)]
            fn at_mut<Idx: IntoIdx<D1>>(&mut self, idx: Idx) -> &mut T {
                &mut self[idx.into_idx()[0]]
            }

            #[inline(always)]
            fn set<Idx: IntoIdx<D1>>(&mut self, idx: Idx, value: T) {
                self[idx.into_idx()[0]] = value
            }

            fn child_mut(&mut self, _: <D1 as Dim>::ChildIdx) -> impl NVecMut<<D1 as Dim>::PrevDim, T> {
                self
            }

            fn all_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
            where
                T: 'a,
            {
                self.iter_mut()
            }

            fn enumerate_all_mut<'a>(&'a mut self) -> impl Iterator<Item = (<D1 as Dim>::Idx, &'a mut T)>
            where
                T: 'a,
            {
                self.iter_mut().enumerate().map(|(i, x)| ([i], x))
            }
        }
    };

    ($v1:ty, $const_arg:ident, {$($where:tt)*}, $is_const:tt) => {
        #[allow(unused_imports)]
        use $crate::*;
        impl<const $const_arg: usize, T: Copy> NVecMut<D1, T> for $v1
        where
            $($where)*
        {
            #[inline(always)]
            fn at_mut<Idx: IntoIdx<D1>>(&mut self, idx: Idx) -> &mut T {
                &mut self[idx.into_idx()[0]]
            }

            #[inline(always)]
            fn set<Idx: IntoIdx<D1>>(&mut self, idx: Idx, value: T) {
                self[idx.into_idx()[0]] = value
            }

            fn child_mut(&mut self, _: <D1 as Dim>::ChildIdx) -> impl NVecMut<<D1 as Dim>::PrevDim, T> {
                self
            }

            fn all_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
            where
                T: 'a,
            {
                self.iter_mut()
            }

            fn enumerate_all_mut<'a>(&'a mut self) -> impl Iterator<Item = (<D1 as Dim>::Idx, &'a mut T)>
            where
                T: 'a,
            {
                self.iter_mut().enumerate().map(|(i, x)| ([i], x))
            }
        }
    };
}

/// Implements `NVecMut<D, T>` for a struct which is sort of like a 1-dimensional
/// vector, such as the standard `Vec<T>`, and a dimension `D` which is at least
/// 2-dimensional.
#[macro_export]
macro_rules! impl_vmut_recursive_for_v1 {
    ($dim:ty, $v1:ty) => {
        #[allow(unused_imports)]
        use $crate::*;

        impl<C, T> NVecMut<$dim, T> for $v1
        where
            C: NVecMut<<$dim as Dim>::PrevDim, T>,
        {
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

            #[inline(always)]
            fn child_mut(&mut self, i: <$dim as Dim>::ChildIdx) -> impl NVecMut<<$dim as Dim>::PrevDim, T> {
                &mut self[i]
            }

            fn all_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
            where
                T: 'a,
            {
                self.iter_mut().flat_map(|x| x.all_mut())
            }

            fn enumerate_all_mut<'a>(&'a mut self) -> impl Iterator<Item = (<$dim as Dim>::Idx, &'a mut T)>
            where
                T: 'a,
            {
                self.iter_mut().enumerate().flat_map(|(i, x)| {
                    x.enumerate_all_mut().map(move |(idx_right, y)| {
                        let idx = <$dim as Dim>::left_join_from_lower_dim(i, idx_right);
                        (idx, y)
                    })
                })
            }
        }
    };

    ($dim:ty, $v1:ty, $arg:ident, {$($where:tt)*}) => {
        #[allow(unused_imports)]
        use $crate::*;
        impl<C, T, $arg> NVecMut<$dim, T> for $v1
        where
            C: NVecMut<<$dim as Dim>::PrevDim, T>,
            $($where)*
        {
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

            #[inline(always)]
            fn child_mut(&mut self, i: <$dim as Dim>::ChildIdx) -> impl NVecMut<<$dim as Dim>::PrevDim, T> {
                &mut self[i]
            }

            fn all_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
            where
                T: 'a,
            {
                self.iter_mut().flat_map(|x| x.all_mut())
            }

            fn enumerate_all_mut<'a>(&'a mut self) -> impl Iterator<Item = (<$dim as Dim>::Idx, &'a mut T)>
            where
                T: 'a,
            {
                self.iter_mut().enumerate().flat_map(|(i, x)| {
                    x.enumerate_all_mut().map(move |(idx_right, y)| {
                        let idx = <$dim as Dim>::left_join_from_lower_dim(i, idx_right);
                        (idx, y)
                    })
                })
            }
        }
    };

    ($dim:ty, $v1:ty, $const_arg:ident, {$($where:tt)*}, $is_const:tt) => {
        #[allow(unused_imports)]
        use $crate::*;
        impl<const $const_arg: usize, C, T> NVecMut<$dim, T> for $v1
        where
            C: NVecMut<<$dim as Dim>::PrevDim, T>,
            $($where)*
        {
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

            #[inline(always)]
            fn child_mut(&mut self, i: <$dim as Dim>::ChildIdx) -> impl NVecMut<<$dim as Dim>::PrevDim, T> {
                &mut self[i]
            }

            fn all_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
            where
                T: 'a,
            {
                self.iter_mut().flat_map(|x| x.all_mut())
            }

            fn enumerate_all_mut<'a>(&'a mut self) -> impl Iterator<Item = (<$dim as Dim>::Idx, &'a mut T)>
            where
                T: 'a,
            {
                self.iter_mut().enumerate().flat_map(|(i, x)| {
                    x.enumerate_all_mut().map(move |(idx_right, y)| {
                        let idx = <$dim as Dim>::left_join_from_lower_dim(i, idx_right);
                        (idx, y)
                    })
                })
            }
        }
    };
}
