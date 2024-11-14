use crate::{impl_v1_mut, impl_vn_mut, D2, D3, D4};

impl_v1_mut!(['a, T], &'a mut [T], [T: Copy]);
impl_vn_mut!(D2, ['a, C, T], &'a mut [C], [C: NVecMut<<D2 as Dim>::PrevDim, T>]);
impl_vn_mut!(D3, ['a, C, T], &'a mut [C], [C: NVecMut<<D3 as Dim>::PrevDim, T>]);
impl_vn_mut!(D4, ['a, C, T], &'a mut [C], [C: NVecMut<<D4 as Dim>::PrevDim, T>]);
