use crate::{dimensions::dim::*, NVec, NVecMut, NVecRecMut, ValueSetError};
use alloc::{string::ToString, vec, vec::Vec};

// d2

fn d1_set_4<T, N: NVecMut<D1, T>>(nvec: &mut N, value: T) -> Result<(), ValueSetError> {
    nvec.set(4, value)
}

fn d2_child_2<T, N: NVecRecMut<D2, T>>(nvec: &mut N) -> Option<&mut N::Child> {
    nvec.child_mut(2)
}

fn d2_set_24<T, N: NVecRecMut<D2, T>>(nvec: &mut N, value: T) -> Result<(), ValueSetError> {
    let child = d2_child_2(nvec).unwrap();
    d1_set_4(child, value)
}

#[test]
fn d2_nvec_vec_vec() {
    let mut vec = vec![vec![1], vec![], vec![2, 5, 1, 3, 42, 7], vec![4]];
    assert_eq!(d2_set_24(&mut vec, 7), Ok(()));
    assert_eq!(vec.try_at([2, 4]), Some(7));

    let other_string = 12.to_string();
    let vec_string: Vec<Vec<_>> = vec
        .iter()
        .map(|x| x.iter().map(|x| x.to_string()).collect())
        .collect();
    let mut vec_refs: Vec<Vec<_>> = vec_string.iter().map(|x| x.iter().collect()).collect();
    assert_eq!(d2_set_24(&mut vec_refs, &other_string), Ok(()));
    assert_eq!(vec_refs.try_at([2, 4]), Some(&12.to_string()));
}
