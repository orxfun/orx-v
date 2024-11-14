use crate::{impl_v1, impl_vn, D2, D3, D4};

impl_v1!(N, [T], [T; N], [T: Copy]);
impl_vn!(D2, N, [C, T], [C; N], [C: NVec<<D2 as Dim>::PrevDim, T>]);
impl_vn!(D3, N, [C, T], [C; N], [C: NVec<<D3 as Dim>::PrevDim, T>]);
impl_vn!(D4, N, [C, T], [C; N], [C: NVec<<D4 as Dim>::PrevDim, T>]);
