use super::super::{D1, D2, D3, D4, Dim, NVecNever, V, Vzzz};
use core::marker::PhantomData;
use orx_self_or::SoR;

pub struct FunVec1<D, S, T, Fr>
where
    S: SoR<D>,
    Fr: Fn(&D, <D1 as Dim>::Idx) -> T,
{
    data: S,
    f: Fr,
    p: PhantomData<(D, T)>,
}

impl<D, S, T, Fr> FunVec1<D, S, T, Fr>
where
    S: SoR<D>,
    Fr: Fn(&D, <D1 as Dim>::Idx) -> T,
{
    pub fn new(data: S, f: Fr) -> Self {
        let p = PhantomData;
        Self { data, f, p }
    }
}

// impl V

impl<D, S, T, Fr> V<D1, T> for FunVec1<D, S, T, Fr>
where
    S: SoR<D>,
    Fr: Fn(&D, <D1 as Dim>::Idx) -> T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        (self.f)(self.data.get_ref(), idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn abc() {
        let data = vec![1, 2, 3];

        let v = FunVec1::new(&data, |d: &Vec<_>, i| d[i]);
        assert_eq!(v.at(1), 2);

        let v = FunVec1::new((), |_, i| i + 1);
        assert_eq!(v.at(1), 2);

        let one = vec![String::from("x")];
        let v = FunVec1::new((), |_, i| i + one[0].len());
        assert_eq!(v.at(1), 2);

        let one = vec![String::from("x")];
        let v = FunVec1::new((), move |_, i| i + one[0].len());
        assert_eq!(v.at(1), 2);

        let one = vec![String::from("x")];
        let v = FunVec1::new(&one, |data: &Vec<_>, i| i + data[0].len());
        assert_eq!(v.at(1), 2);

        let v = FunVec1::new(one, |data, i| i + data[0].len());
        assert_eq!(v.at(1), 2);
    }
}
