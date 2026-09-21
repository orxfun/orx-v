use super::super::{D1, D2, D3, D4, Dim, NVecNever, Vm};
use core::marker::PhantomData;
use orx_self_or::SoM;

pub struct FunMutVec1<D, S, T, F>(S, F, PhantomData<(D, T)>)
where
    S: SoM<D>,
    F: Fn(&mut D, <D1 as Dim>::Idx) -> &mut T;

impl<D, S, T, F> FunMutVec1<D, S, T, F>
where
    S: SoM<D>,
    F: Fn(&mut D, <D1 as Dim>::Idx) -> &mut T,
{
    pub fn new(data: S, f: F) -> Self {
        Self(data, f, PhantomData)
    }
}

// impl Vm

impl<D, S, T, F> Vm<D1, T> for FunMutVec1<D, S, T, F>
where
    S: SoM<D>,
    F: Fn(&mut D, <D1 as Dim>::Idx) -> &mut T,
{
    fn mut_at(&mut self, idx: <D1 as Dim>::Idx) -> &mut T {
        (self.1)(self.0.get_mut(), idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn abc() {
        let mut data = vec![1, 2, 3];

        fn f<'a>(d: &'a mut Vec<i32>, i: usize) -> &'a mut i32 {
            &mut d[i]
        }

        let mut v = FunMutVec1::new(&mut data, f);
        *v.mut_at(1) = 22;

        assert_eq!(data, &[1, 22, 3]);
    }
}
