use crate::{impl_v1_card, impl_vn_card, D2, D3, D4};
use orx_fixed_vec::*;

impl_v1_card!([T], FixedVec<T>, []);
impl_vn_card!(D2, [C, T], FixedVec<C>, [C: NVecCoreSealed<<D2 as Dim>::PrevDim, T>]);
impl_vn_card!(D3, [C, T], FixedVec<C>, [C: NVecCoreSealed<<D3 as Dim>::PrevDim, T>]);
impl_vn_card!(D4, [C, T], FixedVec<C>, [C: NVecCoreSealed<<D4 as Dim>::PrevDim, T>]);
