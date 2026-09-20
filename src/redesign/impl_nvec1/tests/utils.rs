use super::super::super::{D1, NVec};

pub fn second<T>(v: impl NVec<D1, T>) -> T {
    v.at(1)
}
