use crate::{Dim, IntoIndex, NVec};
use core::marker::PhantomData;

pub struct Hooked<N, T, V, H>
where
    N: Dim,
    H: Fn(N::Idx, Option<&T>),
{
    inner: V,
    hook: H,
    phantom: PhantomData<(N, T)>,
}

// nvecs

impl<N, T, V, H> NVec<N, T> for Hooked<N, T, V, H>
where
    N: Dim,
    V: NVec<N, T>,
    H: Fn(N::Idx, Option<&T>),
{
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
        let idx = index.into_index();
        let value = self.inner.at(index);
        (self.hook)(idx, Some(&value));
        value
    }

    fn try_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T> {
        let idx = index.into_index();
        let maybe = self.inner.try_at(index);
        (self.hook)(idx, maybe.as_ref());
        maybe
    }
}

// into

pub trait IntoHooked<N, T, H>
where
    N: Dim,
    Self: NVec<N, T> + Sized,
    H: Fn(N::Idx, Option<&T>),
{
    fn hooked(self, hook: H) -> Hooked<N, T, Self, H>;
}

impl<N, T, V, H> IntoHooked<N, T, H> for V
where
    N: Dim,
    V: NVec<N, T>,
    H: Fn(N::Idx, Option<&T>),
{
    fn hooked(self, hook: H) -> Hooked<N, T, V, H> {
        Hooked {
            inner: self,
            hook,
            phantom: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use core::cell::RefCell;
    use std::collections::BTreeMap;

    fn run<V: NVec<D2, char>>(vec: V) {
        let _ = vec.at([2, 1]);
        let _ = vec.at([0, 0]);
        let _ = vec.try_at([4, 4]);
    }

    fn log([i, j]: [usize; 2], value: Option<&char>) {
        println!("x[{},{}] = {:?}", i, j, value)
    }

    #[test]
    fn log_hook() {
        let vec_vec = vec![vec!['1', '2'], vec![], vec!['3', '4', '5', '6']];
        run(vec_vec.hooked(log));

        let map_vec: BTreeMap<usize, Vec<char>> = [
            (0, vec!['1']),
            (4, vec!['2', '3', '4', '5', '6']),
            (2, vec!['7', '8']),
        ]
        .into_iter()
        .collect();
        run((&map_vec).hooked(log));
    }

    #[test]
    fn recursively_hooked() {
        let num_called = RefCell::new(0);
        let hook_increment = |_: [usize; 2], _: Option<&char>| *num_called.borrow_mut() += 1;

        let vec_vec = vec![vec!['1', '2'], vec![], vec!['3', '4', '5', '6']];
        run(vec_vec.hooked(log).hooked(hook_increment));

        assert_eq!(*num_called.borrow(), 3);
    }
}
