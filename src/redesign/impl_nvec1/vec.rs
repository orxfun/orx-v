use super::super::{D1, Dim, NVec, NVecMut};
use alloc::vec::Vec;

impl<'a, T> NVec<D1, &'a T> for &'a Vec<T> {
    #[inline(always)]
    fn at(&self, idx: <D1 as Dim>::Idx) -> &'a T {
        &self[idx]
    }
}

impl<T> NVecMut<D1, T> for Vec<T> {
    #[inline(always)]
    fn at(&self, idx: <D1 as Dim>::Idx) -> &T {
        &self[idx]
    }

    #[inline(always)]
    fn at_mut(&mut self, idx: <D1 as Dim>::Idx) -> &mut T {
        &mut self[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    fn second<T>(v: &mut impl NVecMut<D1, T>) -> &mut T {
        v.at_mut(1)
    }

    #[test]
    fn abc() {
        let mut vec = vec![1, 2, 3];
        let sec = second(&mut vec);
        *sec = 22;
        let third = second(&mut vec);
        *third = 22;

        assert_eq!(vec, vec![1, 22, 3]);
    }

    #[test]
    fn creates_aliasing_mutable_references() {
        let mut v = vec![0, 1];

        let mut r: &mut Vec<_> = &mut v;
        // let shared: &&mut Vec<_> = &r;

        let first = r.at_mut(0);
        *first = 10;

        let second = r.at_mut(1);
        *second = 20;
    }
}
