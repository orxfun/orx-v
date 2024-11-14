use crate::{impl_v1_mut, impl_vn_mut, D2, D3, D4};

impl_v1_mut!(N, [T], [T; N], [T: Copy]);
impl_vn_mut!(D2, N, [C, T], [C; N], [C: NVecMut<<D2 as Dim>::PrevDim, T>]);
impl_vn_mut!(D3, N, [C, T], [C; N], [C: NVecMut<<D3 as Dim>::PrevDim, T>]);
impl_vn_mut!(D4, N, [C, T], [C; N], [C: NVecMut<<D4 as Dim>::PrevDim, T>]);
