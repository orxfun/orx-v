use super::super::{D1, Dim, NVec};
use alloc::vec::Vec;

impl<'a, T> NVec<D1, &'a T> for &'a Vec<T> {
    #[inline(always)]
    fn at(&self, idx: <D1 as Dim>::Idx) -> &'a T {
        &self[idx]
    }
}

impl<'a, T> NVec<D1, &'a mut T> for &'a mut Vec<T> {
    #[inline(always)]
    fn at(&self, idx: <D1 as Dim>::Idx) -> &'a mut T {
        let x = self as *const &mut Vec<T>;
        let y = unsafe { x.read() };
        &mut y[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    fn second<'a, T>(v: impl NVec<D1, &'a mut T>) -> &'a mut T {
        v.at(1)
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
}
