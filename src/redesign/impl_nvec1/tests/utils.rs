use super::super::super::{D1, NVec, NVecMut};

pub fn second<T>(v: impl NVec<D1, T>) -> T {
    v.at(1)
}

pub fn second_mut<T>(v: &mut impl NVecMut<D1, T>) -> &mut T {
    v.at_mut(1)
}

pub fn second_mut_cloned<T: Clone>(v: &mut impl NVecMut<D1, T>) -> T {
    let v = v.cloned();
    v.at(1)
}
