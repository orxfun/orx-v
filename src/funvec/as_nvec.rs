use super::FunVec;
use crate::{dimensions::*, FromIndex, IntoIndex, NVec};

impl<N, T, I, F> NVec<N, T> for FunVec<N, T, I, F>
where
    N: Dim,
    I: FromIndex<N>,
    F: Fn(I) -> T,
{
    #[inline]
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
        (self.fun)(I::from_index(index.into_index()))
    }

    #[inline]
    fn try_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T> {
        Some(self.at(index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FunVecBuilder, IntoFunVec};

    #[test]
    fn d1() {
        let f = |i: usize| i as u32 + 1;

        let fun = f.to_funvec();
        assert_eq!(fun.at(0), 1);

        let fun = FunVecBuilder::d1().new(f);
        assert_eq!(fun.at(0), 1);
    }

    #[test]
    fn d2() {
        let f = |(i, j)| i + j;

        let fun = f.to_funvec();
        assert_eq!(fun.at([0, 0]), 0);
        assert_eq!(fun.at([40, 2]), 42);

        let fun = FunVecBuilder::d2().new(f);
        assert_eq!(fun.at([0, 0]), 0);
        assert_eq!(fun.at([40, 2]), 42);
    }

    #[test]
    fn fn_to_funvec() {
        fn inc(i: usize) -> u32 {
            i as u32 + 1
        }
        let fun = inc.to_funvec();
        assert_eq!(fun.at(0), 1);

        fn add(i: usize, j: usize) -> usize {
            i + j
        }
        let fun = (|[i, j]: [usize; 2]| add(i, j)).to_funvec();
        assert_eq!(fun.at((3, 4)), 7);
    }
}
