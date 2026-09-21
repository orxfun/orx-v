use super::{D0, Dim, NVec, NVecMut, NVecRef};

// ref

impl<'a, T> NVecRef<D0, T> for T {
    #[inline(always)]
    fn at_ref(&self, _: <D0 as Dim>::Idx) -> &T {
        self
    }
}

// mut

impl<'a, T> NVecMut<D0, T> for T {
    #[inline(always)]
    fn at_mut(&mut self, _: <D0 as Dim>::Idx) -> &mut T {
        self
    }
}

// nvec

impl<'a, T> NVec<D0, &'a T> for &'a T {
    #[inline(always)]
    fn at<'r>(&'r self, _: <D0 as Dim>::Idx) -> &'a T
    where
        &'a T: 'r,
    {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get<T>(v: impl NVec<D0, T>) -> T {
        v.at([])
    }

    fn get_ref<T>(v: &impl NVecRef<D0, T>) -> &T {
        v.at_ref([])
    }

    fn get_mut<T>(v: &mut impl NVecMut<D0, T>) -> &mut T {
        v.at_mut([])
    }

    #[test]
    fn scalar_as_v0() {
        let mut v = 2;

        assert_eq!(get(&v), &2);
        assert_eq!(get(v.cloned()), 2);
        assert_eq!(get(v.copied()), 2);

        assert_eq!(get_ref(&v), &2);
        assert_eq!(get_mut(&mut v), &mut 2);
    }
}
