use super::super::super::{D2, NVec, NVecMut, NVecRef};

pub fn second_first<T>(v: &impl NVec<D2, T>) -> T {
    v.at([1, 0])
}

pub fn second_first_ref<T>(v: &impl NVecRef<D2, T>) -> &T {
    v.at_ref([1, 0])
}

pub fn second_first_mut<T>(v: &mut impl NVecMut<D2, T>) -> &mut T {
    v.at_mut([1, 0])
}
