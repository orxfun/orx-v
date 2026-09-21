use super::super::super::{D1, D2, NVec, NVecMut, NVecRef};

pub fn second_first<T>(v: &impl NVec<D2, T>) -> T {
    v.at([1, 0])
}

pub fn second_first_ref<T>(v: &impl NVecRef<D2, T>) -> &T {
    v.at_ref([1, 0])
}

pub fn second_first_mut<T>(v: &mut impl NVecMut<D2, T>) -> &mut T {
    v.at_mut([1, 0])
}

pub fn reduce_first_child<T>(v: &impl NVecRef<D2, T>, mut init: T, f: impl Fn(T, &T) -> T) -> T {
    let c = v.child(1);
    for j in 0..3 {
        init = f(init, c.at_ref(j));
    }
    init
}
