use core::fmt::Debug;

/// Indices that are less than or equal to dimension 2.
#[derive(PartialEq)]
pub enum IdxLeqD2 {
    /// Index for dimension 0.
    IdxD0([usize; 0]),
    /// Index for dimension 1.
    IdxD1([usize; 1]),
    /// Index for dimension 2.
    IdxD2([usize; 2]),
}

impl Debug for IdxLeqD2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::IdxD0(arg0) => arg0.fmt(f),
            Self::IdxD1(arg0) => arg0.fmt(f),
            Self::IdxD2(arg0) => arg0.fmt(f),
        }
    }
}

impl From<[usize; 0]> for IdxLeqD2 {
    #[inline(always)]
    fn from(value: [usize; 0]) -> Self {
        Self::IdxD0(value)
    }
}

impl From<[usize; 1]> for IdxLeqD2 {
    #[inline(always)]
    fn from(value: [usize; 1]) -> Self {
        Self::IdxD1(value)
    }
}

impl From<usize> for IdxLeqD2 {
    #[inline(always)]
    fn from(value: usize) -> Self {
        Self::IdxD1([value])
    }
}

impl From<[usize; 2]> for IdxLeqD2 {
    #[inline(always)]
    fn from(value: [usize; 2]) -> Self {
        Self::IdxD2(value)
    }
}
