use crate::{D2, D3, D4, impl_v1_card, impl_vn_card};
use orx_split_vec::*;

impl_v1_card!([G, T], SplitVec<T, G>, [G: Growth]);
impl_vn_card!(D2, [G, C, T], SplitVec<C, G>, [G: Growth, C: NVecCore<<D2 as Dim>::PrevDim, T>]);
impl_vn_card!(D3, [G, C, T], SplitVec<C, G>, [G: Growth, C: NVecCore<<D3 as Dim>::PrevDim, T>]);
impl_vn_card!(D4, [G, C, T], SplitVec<C, G>, [G: Growth, C: NVecCore<<D4 as Dim>::PrevDim, T>]);
