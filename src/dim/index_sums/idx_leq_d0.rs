use core::fmt::Debug;

/// Indices that are less than or equal to dimension 0.
#[derive(PartialEq)]
pub enum IdxLeqD0 {
    /// Index for dimension 0.
    IdxD0([usize; 0]),
}

impl Debug for IdxLeqD0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::IdxD0(arg0) => arg0.fmt(f),
        }
    }
}

impl From<[usize; 0]> for IdxLeqD0 {
    #[inline(always)]
    fn from(value: [usize; 0]) -> Self {
        Self::IdxD0(value)
    }
}
