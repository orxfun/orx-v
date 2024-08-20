use crate::{IntoIndex, NVec, NVecMut, D1};

impl<T: Copy> NVec<D1, T> for &[T] {
    fn at<Idx: IntoIndex<D1>>(&self, index: Idx) -> T {
        self[index.into_index()[0]]
    }

    fn try_at<Idx: IntoIndex<D1>>(&self, index: Idx) -> Option<T> {
        self.get(index.into_index()[0]).copied()
    }
}

impl<T: Copy> NVec<D1, T> for &mut [T] {
    fn at<Idx: IntoIndex<D1>>(&self, index: Idx) -> T {
        self[index.into_index()[0]]
    }

    fn try_at<Idx: IntoIndex<D1>>(&self, index: Idx) -> Option<T> {
        self.get(index.into_index()[0]).copied()
    }
}

impl<T> NVecMut<D1, T> for &mut [T]
where
    Self: NVec<D1, T>,
{
    fn set<Idx: IntoIndex<D1>>(&mut self, index: Idx, value: T) {
        self[index.into_index()[0]] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nvec() {
        let vec = vec![0, 1, 2, 3];
        let slice = vec.as_slice();

        assert_eq!(slice.at(1), 1);
    }

    #[test]
    fn nvec_mut() {
        let mut vec = vec![0, 1, 2, 3];
        let mut slice = vec.as_mut_slice();

        assert_eq!(slice.at(1), 1);
        slice.set(1, 42);
        assert_eq!(slice.at(1), 42);
    }

    #[test]
    fn pass_nvec_mut() {
        fn take<V: NVec<D1, usize> + NVecMut<D1, usize>>(v: &mut V) {
            assert_eq!(v.at(1), 1);
            v.set(1, 42);
            assert_eq!(v.at(1), 42);
        }
        let mut vec = vec![0, 1, 2, 3];
        take(&mut vec.as_mut_slice())
    }
}
