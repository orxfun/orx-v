use crate::{dimensions::*, NVec, NVecMut};
use alloc::vec::Vec;

impl<'v, T> NVec<D1, &'v T> for &'v Vec<T> {
    #[inline]
    fn at<Idx: IntoIndex<D1>>(&self, index: Idx) -> &'v T {
        &self[index.into_index()[0]]
    }

    #[inline]
    fn try_at<Idx: IntoIndex<D1>>(&self, index: Idx) -> Option<&'v T> {
        self.get(index.into_index()[0])
    }
}

impl<T> NVecMut<D1, T> for Vec<T> {
    #[inline]
    fn at<Idx: IntoIndex<D1>>(&self, index: Idx) -> &T {
        &self[index.into_index()[0]]
    }

    #[inline]
    fn try_at<Idx: IntoIndex<D1>>(&self, index: Idx) -> Option<&T> {
        self.get(index.into_index()[0])
    }

    #[inline]
    fn set<Idx: IntoIndex<D1>>(&mut self, index: Idx, value: T) {
        self[index.into_index()[0]] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntoCopied, IntoFilled};
    use alloc::{
        string::{String, ToString},
        vec,
    };

    #[test]
    fn nvec() {
        let vec = vec![1i32, 2, 3];
        let nvec = &vec;
        assert_eq!(nvec.at(1), &2);
        assert_eq!(nvec.try_at(1), Some(&2));
        assert_eq!(nvec.try_at(3), None);

        let vec = vec![1.to_string(), 2.to_string(), 3.to_string()];
        let nvec = &vec;
        assert_eq!(nvec.at(1), &2.to_string());
        assert_eq!(nvec.try_at(3), None);

        let empty = String::default();
        let nvec = nvec.filled_with(&empty);
        assert_eq!(nvec.at(3), &empty);
        assert_eq!(nvec.try_at(3), Some(&empty));
    }

    #[test]
    fn nvec_mut() {
        let mut vec = vec![1i32, 2, 3];

        vec.set(1, 42);
        assert_eq!(vec.at(1), &42);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let vec = vec![1, 2, 3];
        let nvec = &vec;
        let _ = nvec.at(3);
    }

    #[test]
    fn copied() {
        let vec = vec![1i32, 2, 3];
        let nvec = &vec;
        let nvec = nvec.copied();
        assert_eq!(nvec.at(1), 2);
        assert_eq!(nvec.try_at(1), Some(2));
    }
}
