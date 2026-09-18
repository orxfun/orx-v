use super::super::super::{D2, NVec};

pub fn sum(v: impl NVec<D2, i32>) -> i32
where
{
    v.at([1, 0]) + v.at([2, 1])
}

pub fn sum_ref<'a>(v: impl NVec<D2, &'a i32>) -> i32
where
{
    v.at([1, 0]) + v.at([2, 1])
}
