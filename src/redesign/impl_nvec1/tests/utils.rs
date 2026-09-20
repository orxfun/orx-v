use super::super::super::{D1, NVec, NVecMut, NVecRef};

pub fn second<T>(v: impl NVec<D1, T>) -> T {
    v.at(1)
}

pub fn second_ref<T>(v: &impl NVecRef<D1, T>) -> &T {
    v.at_ref(1)
}

pub fn second_mut<T>(v: &mut impl NVecMut<D1, T>) -> &mut T {
    v.at_mut(1)
}
