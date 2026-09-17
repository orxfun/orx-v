use super::super::super::{D1, NVec};

pub fn sum(v: impl NVec<D1, i32>) -> i32
where
{
    v.at(1) + v.at(3)
}

pub fn sum_ref<'a>(v: impl NVec<D1, &'a i32>) -> i32
where
{
    v.at(1) + v.at(3)
}
