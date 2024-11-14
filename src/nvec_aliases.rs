use crate::dim::*;
use crate::NVec;
use crate::NVecMut;

/// A type alias: `V1<T>` is equivalent to `NVec<D1, T>`.
pub trait V1<T>: NVec<D1, T> {}
impl<T, N: NVec<D1, T>> V1<T> for N {}

/// A type alias: `V2<T>` is equivalent to `NVec<D2, T>`.
pub trait V2<T>: NVec<D2, T> {}
impl<T, N: NVec<D2, T>> V2<T> for N {}

/// A type alias: `V3<T>` is equivalent to `NVec<D3, T>`.
pub trait V3<T>: NVec<D3, T> {}
impl<T, N: NVec<D3, T>> V3<T> for N {}

/// A type alias: `V4<T>` is equivalent to `NVec<D4, T>`.
pub trait V4<T>: NVec<D4, T> {}
impl<T, N: NVec<D4, T>> V4<T> for N {}

// mut

/// A type alias: `V1Mut<T>` is equivalent to `NVecMut<D1, T>`.
pub trait V1Mut<T>: NVecMut<D1, T> {}
impl<T, N: NVecMut<D1, T>> V1Mut<T> for N {}

/// A type alias: `V2Mut<T>` is equivalent to `NVecMut<D2, T>`.
pub trait V2Mut<T>: NVecMut<D2, T> {}
impl<T, N: NVecMut<D2, T>> V2Mut<T> for N {}

/// A type alias: `V3Mut<T>` is equivalent to `NVecMut<D3, T>`.
pub trait V3Mut<T>: NVecMut<D3, T> {}
impl<T, N: NVecMut<D3, T>> V3Mut<T> for N {}

/// A type alias: `V4Mut<T>` is equivalent to `NVecMut<D4, T>`.
pub trait V4Mut<T>: NVecMut<D4, T> {}
impl<T, N: NVecMut<D4, T>> V4Mut<T> for N {}
