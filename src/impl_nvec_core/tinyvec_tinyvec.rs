use crate::{impl_v1_card, impl_vn_card, D2, D3, D4};
use tinyvec::{Array, TinyVec};

impl_v1_card!(N, [T], TinyVec<[T; N]>, [[T; N]: Array<Item = T>]);
impl_vn_card!(D2, N, [C, T], TinyVec<[C; N]>, [C: NVecCoreSealed<<D2 as Dim>::PrevDim, T>, [C; N]: Array<Item = C>]);
impl_vn_card!(D3, N, [C, T], TinyVec<[C; N]>, [C: NVecCoreSealed<<D3 as Dim>::PrevDim, T>, [C; N]: Array<Item = C>]);
impl_vn_card!(D4, N, [C, T], TinyVec<[C; N]>, [C: NVecCoreSealed<<D4 as Dim>::PrevDim, T>, [C; N]: Array<Item = C>]);
