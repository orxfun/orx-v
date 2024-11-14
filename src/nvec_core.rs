use super::nvec_core_sealed::NVecCoreSealed;
use crate::Dim;

/// Core & common functionalities of vectors.
pub trait NVecCore<D: Dim, T>: NVecCoreSealed<D, T> {}

impl<D: Dim, V: NVecCoreSealed<D, T>, T> NVecCore<D, T> for V {}
