mod array;
mod impl_nvec_mut_from_v1;
mod slice;
mod vec;

#[cfg(feature = "arrayvec")]
mod arrayvec;

#[cfg(feature = "ndarray")]
mod ndarray_recursive;

#[cfg(feature = "ndarray")]
mod ndarray_multi_dim;

#[cfg(feature = "orx-fixed-vec")]
mod orx_fixed_vec;

#[cfg(feature = "orx-split-vec")]
mod orx_split_vec;

#[cfg(feature = "smallvec")]
mod smallvec;

#[cfg(feature = "tinyvec")]
mod tinyvec_arrayvec;

#[cfg(feature = "tinyvec")]
mod tinyvec_tinyvec;
