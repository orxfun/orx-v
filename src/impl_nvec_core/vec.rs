use crate::{impl_v1_card, impl_vn_card, D2, D3, D4};
use alloc::vec::Vec;

impl_v1_card!([T], Vec<T>, []);
impl_vn_card!(D2, [C, T], Vec<C>, [C: NVecCoreSealed<<D2 as Dim>::PrevDim, T>]);
impl_vn_card!(D3, [C, T], Vec<C>, [C: NVecCoreSealed<<D3 as Dim>::PrevDim, T>]);
impl_vn_card!(D4, [C, T], Vec<C>, [C: NVecCoreSealed<<D4 as Dim>::PrevDim, T>]);

// impl<C, T> NVecCoreSealed<D2, T> for Vec<C>
// where
//     C: NVecCoreSealed<<D2 as Dim>::PrevDim, T>,
// {
//     fn core_num_children(&self) -> usize {
//         self.len()
//     }

//     fn core_card(&self, idx: impl Into<<D2 as Dim>::CardIdx>) -> usize {
//         idx.into().card(self)
//     }

//     fn core_child(&self, i: <D2 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D2 as Dim>::PrevDim, T> {
//         &self[i]
//     }

//     #[allow(unused_imports)]
//     fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D2>, f: &mut F) -> O {
//         let (i, c_idx) = idx.into_idx().split_idx();
//         let child = <Vec<C> as NVecCoreSealed<D2, T>>::core_child(self, i);
//         child.core_map(c_idx, f)
//     }
// }
