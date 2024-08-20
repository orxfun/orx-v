use crate::{Dim, IntoIndex};

pub trait NVec<N: Dim, T> {
    fn try_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T>;

    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
        self.try_at(index)
            .expect("'at' called on index without a value")
    }
}

pub struct VecVec<T> {
    data: Vec<Vec<T>>,
}

pub trait NVec2<N: Dim> {
    type Yield<'c>
    where
        Self: 'c;

    fn try_at<'a, Idx: IntoIndex<N>>(&'a self, index: Idx) -> Self::Yield<'a>
    where
        Self: 'a;
}

impl<T> NVec2<crate::D1> for VecVec<T> {
    type Yield<'c> = &'c Vec<T>
    where
        Self: 'c;

    fn try_at<'a, Idx: IntoIndex<crate::D1>>(&'a self, index: Idx) -> Self::Yield<'a>
    where
        Self: 'a,
    {
        self.data.get(0).unwrap()
    }
}

pub struct MyFun<F: Fn(usize) -> usize> {
    fun: F,
}
impl<F: Fn(usize) -> usize> NVec2<crate::D1> for MyFun<F> {
    type Yield<'c> = usize
    where
        Self: 'c;

    fn try_at<'a, Idx: IntoIndex<crate::D1>>(&'a self, index: Idx) -> Self::Yield<'a>
    where
        Self: 'a,
    {
        let x = (self.fun)(0);
        x
    }
}

fn abc() {
    let vecvec: VecVec<usize> = VecVec { data: vec![] };

    fn takenvec2<'a, N: NVec2<crate::D1>>(x: N) {
        let first = x.try_at(0);
        let second = x.try_at(0);
    }

    takenvec2(vecvec);
}

impl<T> NVec2<crate::D1> for Vec<T> {
    type Yield<'c> = &'c T
    where
        Self: 'c;

    fn try_at<'a, Idx: IntoIndex<crate::D1>>(&'a self, index: Idx) -> Self::Yield<'a>
    where
        Self: 'a,
    {
        &self[0]
    }
}

struct VecCopy<T: Copy>(Vec<T>);

impl<T: Copy> NVec2<crate::D1> for VecCopy<T> {
    type Yield<'c> = T
    where
        Self: 'c;

    fn try_at<'a, Idx: IntoIndex<crate::D1>>(&'a self, index: Idx) -> Self::Yield<'a>
    where
        Self: 'a,
    {
        self.0[0]
    }
}
