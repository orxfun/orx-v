// use crate::{flat_jagged::FlatJagged, Dim, IntoIdx, NVec, NVecMut, V1Mut, D2, V1};

// impl<V, I, T> NVecMut<D2, T> for FlatJagged<V, I, T>
// where
//     V: V1Mut<T>,
//     I: V1<usize>,
// {
//     fn at_mut<Idx: IntoIdx<D2>>(&mut self, idx: Idx) -> &mut T {
//         let idx = self.to_d1_idx(idx.into_idx());
//         self.flat_vec.at_mut(idx)
//     }

//     fn set<Idx: IntoIdx<D2>>(&mut self, idx: Idx, value: T) {
//         let idx = self.to_d1_idx(idx.into_idx());
//         self.flat_vec.set(idx, value);
//     }

//     fn child_mut(&mut self, i: <D2 as Dim>::ChildIdx) -> impl NVecMut<<D2 as Dim>::PrevDim, T> {
//         todo!()
//     }

//     fn all_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
//     where
//         T: 'a,
//     {
//         self.flat_vec.all_mut()
//     }

//     fn enumerate_all_mut<'a>(&'a mut self) -> impl Iterator<Item = (<D2 as Dim>::Idx, &'a mut T)>
//     where
//         T: 'a,
//     {
//         (0..self.num_children())
//             .flat_map(move |i| (0..self.card([i])).map(move |j| ([i, j], self.at_mut([i, j]))))
//     }
// }
