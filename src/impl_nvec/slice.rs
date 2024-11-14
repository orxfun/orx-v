use crate::{impl_v1, impl_vn, D2, D3, D4};

impl_v1!(['a, T], &'a [T], [T: Copy]);
impl_vn!(D2, ['a, C, T], &'a [C], [C: NVec<<D2 as Dim>::PrevDim, T>]);
impl_vn!(D3, ['a, C, T], &'a [C], [C: NVec<<D3 as Dim>::PrevDim, T>]);
impl_vn!(D4, ['a, C, T], &'a [C], [C: NVec<<D4 as Dim>::PrevDim, T>]);

impl_v1!(['a, T], &'a mut [T], [T: Copy]);
impl_vn!(D2, ['a, C, T], &'a mut [C], [C: NVec<<D2 as Dim>::PrevDim, T>]);
impl_vn!(D3, ['a, C, T], &'a mut [C], [C: NVec<<D3 as Dim>::PrevDim, T>]);
impl_vn!(D4, ['a, C, T], &'a mut [C], [C: NVec<<D4 as Dim>::PrevDim, T>]);
