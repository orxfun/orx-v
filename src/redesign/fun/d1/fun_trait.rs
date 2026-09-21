use core::marker::PhantomData;

pub trait Fun1<D, T> {
    fn exe(&self, data: &D, i: usize) -> T;
}

// with data

pub struct FunWithData1<D, T, F>
where
    F: Fn(&D, usize) -> T,
{
    fun: F,
    p: PhantomData<D>,
}

impl<D, T, F> FunWithData1<D, T, F>
where
    F: Fn(&D, usize) -> T,
{
    pub fn new(fun: F) -> Self {
        let p = PhantomData;
        Self { fun, p }
    }
}

impl<D, T, F> Fun1<D, T> for FunWithData1<D, T, F>
where
    F: Fn(&D, usize) -> T,
{
    #[inline(always)]
    fn exe(&self, data: &D, i: usize) -> T {
        (self.fun)(data, i)
    }
}

// no data

pub struct FunWithoutData1<T, F>
where
    F: Fn(usize) -> T,
{
    fun: F,
}

impl<T, F> FunWithoutData1<T, F>
where
    F: Fn(usize) -> T,
{
    pub fn new(fun: F) -> Self {
        Self { fun }
    }
}

impl<T, F> Fun1<(), T> for FunWithoutData1<T, F>
where
    F: Fn(usize) -> T,
{
    #[inline(always)]
    fn exe(&self, _: &(), i: usize) -> T {
        (self.fun)(i)
    }
}
