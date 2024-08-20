use crate::{dimensions::dim::*, nvec_mut::NVecMut, NVec, ValueSetError};
use alloc::{string::ToString, vec, vec::Vec};

// d1

fn d1_set_1<T, N: NVecMut<D1, T>>(nvec: &mut N, value: T) -> Result<(), ValueSetError> {
    nvec.set(1, value)
}

#[test]
fn d1_nvec_mut_vec() {
    let mut vec = vec![1, 42, 13];
    assert_eq!(d1_set_1(&mut vec.as_mut_slice(), 7), Ok(()));
    assert_eq!(vec.try_at(1), Some(7));

    let other_string = 7.to_string();
    let vec_string: Vec<_> = vec.iter().map(|x| x.to_string()).collect();
    let mut vec_refs: Vec<_> = vec_string.iter().collect();
    assert_eq!(
        d1_set_1(&mut vec_refs.as_mut_slice(), &other_string),
        Ok(())
    );
    assert_eq!(vec_refs.try_at(1), Some(&7.to_string()));
}

// d2

fn d2_set_24<T, N: NVecMut<D2, T>>(nvec: &mut N, value: T) -> Result<(), ValueSetError> {
    nvec.set((2, 4), value)
}

#[test]
fn d2_nvec_mut_vec_vec() {
    let mut vec = vec![vec![1], vec![], vec![2, 5, 1, 3, 42, 7], vec![4]];
    assert_eq!(d2_set_24(&mut vec.as_mut_slice(), 7), Ok(()));
    assert_eq!(vec.try_at((2, 4)), Some(7));

    let vec_string: Vec<Vec<_>> = vec
        .iter()
        .map(|x| x.iter().map(|x| x.to_string()).collect())
        .collect();
    let vec_refs: Vec<Vec<_>> = vec_string.iter().map(|x| x.iter().collect()).collect();
    assert_eq!(vec_refs.try_at((2, 4)), Some(&7.to_string()));
}
