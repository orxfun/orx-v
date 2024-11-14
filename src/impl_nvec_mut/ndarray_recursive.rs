use crate::{impl_v1_mut, impl_vn_mut, D2, D3, D4};
use ndarray::{Array, Ix1};

impl_v1_mut!([T], Array<T, Ix1>, [T: Copy]);
impl_vn_mut!(D2, [T, C], Array<C, Ix1>, [C: NVecMut<<D2 as Dim>::PrevDim, T>]);
impl_vn_mut!(D3, [T, C], Array<C, Ix1>, [C: NVecMut<<D3 as Dim>::PrevDim, T>]);
impl_vn_mut!(D4, [T, C], Array<C, Ix1>, [C: NVecMut<<D4 as Dim>::PrevDim, T>]);
