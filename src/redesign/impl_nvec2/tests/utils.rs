use super::super::super::{D2, NVec};

pub fn second_first<T>(v: impl NVec<D2, T>) -> T {
    v.at([1, 0])
}
