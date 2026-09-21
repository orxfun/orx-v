use super::super::super::{D1, D2, D3, D4, Dim, NVecNever, V};
use super::fun_trait::{Fun1, FunWithData1, FunWithoutData1};
use core::marker::PhantomData;
use orx_self_or::SoR;

pub struct FunVec1<D, S, T, F>
where
    S: SoR<D>,
    F: Fun1<D, T>,
{
    data: S,
    f: F,
    p: PhantomData<(D, T)>,
}

impl<D, S, T, F> FunVec1<D, S, T, FunWithData1<D, T, F>>
where
    S: SoR<D>,
    F: Fn(&D, usize) -> T,
{
    pub fn with_data(data: S, f: F) -> Self {
        let f = FunWithData1::new(f);
        let p = PhantomData;
        Self { data, f, p }
    }
}

impl<T, F> FunVec1<(), (), T, FunWithoutData1<T, F>>
where
    F: Fn(usize) -> T,
{
    pub fn new(f: F) -> Self {
        let f = FunWithoutData1::new(f);
        let p = PhantomData;
        let data = ();
        Self { data, f, p }
    }
}

// impl V

impl<D, S, T, F> V<D1, T> for FunVec1<D, S, T, F>
where
    S: SoR<D>,
    F: Fun1<D, T>,
{
    fn at(&self, i: <D1 as Dim>::Idx) -> T {
        self.f.at(self.data.get_ref(), i)
    }
}
