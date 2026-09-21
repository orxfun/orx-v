use super::super::{D1, D2, D3, D4, Dim, NVecNever, Vm};
use core::marker::PhantomData;
use orx_self_or::SoM;

pub struct FunMutVec1<D, S, T, Fr, Fm>
where
    S: SoM<D>,
    Fr: Fn(&D, <D1 as Dim>::Idx) -> &T,
    Fm: Fn(&mut D, <D1 as Dim>::Idx) -> &mut T,
{
    data: S,
    f: Fr,
    m: Fm,
    p: PhantomData<(D, T)>,
}

impl<D, S, T, Fr, Fm> FunMutVec1<D, S, T, Fr, Fm>
where
    S: SoM<D>,
    Fr: Fn(&D, <D1 as Dim>::Idx) -> &T,
    Fm: Fn(&mut D, <D1 as Dim>::Idx) -> &mut T,
{
    pub fn new(data: S, f: Fr, m: Fm) -> Self {
        let p = PhantomData;
        Self { data, f, m, p }
    }
}

// impl Vm

impl<D, S, T, Fr, Fm> Vm<D1, T> for FunMutVec1<D, S, T, Fr, Fm>
where
    S: SoM<D>,
    Fr: Fn(&D, <D1 as Dim>::Idx) -> &T,
    Fm: Fn(&mut D, <D1 as Dim>::Idx) -> &mut T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> &T {
        (self.f)(self.data.get_ref(), idx)
    }

    fn mut_at(&mut self, idx: <D1 as Dim>::Idx) -> &mut T {
        (self.m)(self.data.get_mut(), idx)
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

        fn f<'a>(d: &'a Vec<i32>, i: usize) -> &'a i32 {
            &d[i]
        }

        fn m<'a>(d: &'a mut Vec<i32>, i: usize) -> &'a mut i32 {
            &mut d[i]
        }

        let mut v = FunMutVec1::new(&mut data, f, m);
        assert_eq!(v.at(1), &2);
        *v.mut_at(1) = 22;
        assert_eq!(v.at(1), &22);

        assert_eq!(data, &[1, 22, 3]);
    }
}
