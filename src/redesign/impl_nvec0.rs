use super::{D0, Dim, NVec};

impl<'a, T> NVec<D0, &'a T> for &'a T {
    #[inline(always)]
    fn at(&self, []: <D0 as Dim>::Idx) -> &'a T {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get<T>(v: impl NVec<D0, T>) -> T {
        v.at([])
    }
}
