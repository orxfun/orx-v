use crate::{impl_v1_card, impl_vn_card, D2, D3, D4};
use ndarray::{Array, Ix1};

impl_v1_card!([T], Array<T, Ix1>, []);
impl_vn_card!(D2, [C, T], Array<C, Ix1>, [C: NVecCoreSealed<<D2 as Dim>::PrevDim, T>]);
impl_vn_card!(D3, [C, T], Array<C, Ix1>, [C: NVecCoreSealed<<D3 as Dim>::PrevDim, T>]);
impl_vn_card!(D4, [C, T], Array<C, Ix1>, [C: NVecCoreSealed<<D4 as Dim>::PrevDim, T>]);
