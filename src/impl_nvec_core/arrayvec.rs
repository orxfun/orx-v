use crate::{impl_v1_card, impl_vn_card, D2, D3, D4};
use arrayvec::ArrayVec;

impl_v1_card!(N, [T], ArrayVec<T, N>, []);
impl_vn_card!(D2, N, [C, T], ArrayVec<C, N>, [C: NVecCoreSealed<<D2 as Dim>::PrevDim, T>]);
impl_vn_card!(D3, N, [C, T], ArrayVec<C, N>, [C: NVecCoreSealed<<D3 as Dim>::PrevDim, T>]);
impl_vn_card!(D4, N, [C, T], ArrayVec<C, N>, [C: NVecCoreSealed<<D4 as Dim>::PrevDim, T>]);
