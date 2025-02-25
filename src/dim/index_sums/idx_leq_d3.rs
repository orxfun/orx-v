use core::fmt::Debug;

/// Indices that are less than or equal to dimension 3.
#[derive(PartialEq)]
pub enum IdxLeqD3 {
    /// Index for dimension 0.
    IdxD0([usize; 0]),
    /// Index for dimension 1.
    IdxD1([usize; 1]),
    /// Index for dimension 2.
    IdxD2([usize; 2]),
    /// Index for dimension 3.
    IdxD3([usize; 3]),
}

impl Debug for IdxLeqD3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::IdxD0(arg0) => arg0.fmt(f),
            Self::IdxD1(arg0) => arg0.fmt(f),
            Self::IdxD2(arg0) => arg0.fmt(f),
            Self::IdxD3(arg0) => arg0.fmt(f),
        }
    }
}

impl From<[usize; 0]> for IdxLeqD3 {
    #[inline(always)]
    fn from(value: [usize; 0]) -> Self {
        Self::IdxD0(value)
    }
}

impl From<[usize; 1]> for IdxLeqD3 {
    #[inline(always)]
    fn from(value: [usize; 1]) -> Self {
        Self::IdxD1(value)
    }
}

impl From<usize> for IdxLeqD3 {
    #[inline(always)]
    fn from(value: usize) -> Self {
        Self::IdxD1([value])
    }
}

impl From<[usize; 2]> for IdxLeqD3 {
    #[inline(always)]
    fn from(value: [usize; 2]) -> Self {
        Self::IdxD2(value)
    }
}

impl From<[usize; 3]> for IdxLeqD3 {
    #[inline(always)]
    fn from(value: [usize; 3]) -> Self {
        Self::IdxD3(value)
    }
}
