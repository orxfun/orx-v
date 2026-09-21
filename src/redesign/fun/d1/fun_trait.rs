use core::marker::PhantomData;
use derive_new::new;

pub trait Fun1<'a, D, T> {
    fn at(&self, data: &'a D, i: usize) -> T;
}

// ref - with data

#[derive(new)]
pub struct FunRefWithData1<'a, D, T, F>
where
    F: Fn(&'a D, usize) -> &'a T,
    D: 'a,
    T: 'a,
{
    fun: &'a F,
    p: PhantomData<D>,
}

impl<'a, D, T, F> Fun1<'a, D, &'a T> for FunRefWithData1<'a, D, T, F>
where
    F: Fn(&'a D, usize) -> &'a T,
    D: 'a,
    T: 'a,
{
    fn at(&self, data: &'a D, i: usize) -> &'a T {
        (self.fun)(data, i)
    }
}

// val - with data

#[derive(new)]
pub struct FunValWithData1<D, T, F>
where
    F: Fn(&D, usize) -> T,
{
    fun: F,
    p: PhantomData<D>,
}

impl<D, T, F> Fun1<'_, D, T> for FunValWithData1<D, T, F>
where
    F: Fn(&D, usize) -> T,
{
    fn at(&self, data: &'_ D, i: usize) -> T {
        (self.fun)(data, i)
    }
}

// ref - without data

#[derive(new)]
pub struct FunRefWithoutData1<'a, T, F>
where
    F: Fn(usize) -> &'a T,
    T: 'a,
{
    fun: F,
}

impl<'a, T, F> Fun1<'a, (), &'a T> for FunRefWithoutData1<'a, T, F>
where
    F: Fn(usize) -> &'a T,
    T: 'a,
{
    fn at(&self, _: &'a (), i: usize) -> &'a T {
        (self.fun)(i)
    }
}

// val - without data

#[derive(new)]
pub struct FunValWithoutData1<T, F>
where
    F: Fn(usize) -> T,
{
    fun: F,
}

impl<T, F> Fun1<'_, (), T> for FunValWithoutData1<T, F>
where
    F: Fn(usize) -> T,
{
    fn at(&self, _: &'_ (), i: usize) -> T {
        (self.fun)(i)
    }
}
