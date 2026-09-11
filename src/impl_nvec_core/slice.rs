use crate::{impl_v1_card, impl_vn_card, D2, D3, D4};

impl_v1_card!(['a, T], &'a [T], []);
impl_vn_card!(D2, ['a, C, T], &'a [C], [C: NVecCore<<D2 as Dim>::PrevDim, T>]);
impl_vn_card!(D3, ['a, C, T], &'a [C], [C: NVecCore<<D3 as Dim>::PrevDim, T>]);
impl_vn_card!(D4, ['a, C, T], &'a [C], [C: NVecCore<<D4 as Dim>::PrevDim, T>]);

impl_v1_card!(['a, T], &'a mut [T], []);
impl_vn_card!(D2, ['a, C, T], &'a mut [C], [C: NVecCore<<D2 as Dim>::PrevDim, T>]);
impl_vn_card!(D3, ['a, C, T], &'a mut [C], [C: NVecCore<<D3 as Dim>::PrevDim, T>]);
impl_vn_card!(D4, ['a, C, T], &'a mut [C], [C: NVecCore<<D4 as Dim>::PrevDim, T>]);
