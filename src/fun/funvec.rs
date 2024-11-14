use crate::common_trait_helpers::debug::*;
use crate::{dim::*, Card, NVec, UnboundedCard};
use core::fmt::Debug;
use core::marker::PhantomData;

/// A functional vector of dimension `D` and cardinality `C`
/// having `T` scalar elements.
///
/// So much generics, but the important bit is:
///
/// * any `FunVec<D, T, F, C>` implements `NVec<D, T>`
///
/// hence,
///
/// * any `FunVec<D1, T, F, C>` implements `V1<T>`,
/// * any `FunVec<D2, T, F, C>` implements `V2<T>`,
/// * and so on.
///
/// Examples will hopefully clarify.
///
/// # Examples
///
/// ```
/// use orx_v::*;
/// use std::collections::HashMap;
///
/// #[derive(Clone, Copy)]
/// struct Geocode(i32, i32);
///
/// fn compute_using_coordinates<V: V1<Geocode>>(geocodes: V) { /* todo */ }
///
/// // we can pass a Vec of coordinates
/// compute_using_coordinates(&vec![Geocode(0, 0), Geocode(1, 1)]);
///
/// // or we can pass a functional vector
/// let geocodes = V.d1().fun(|[i]| Geocode(i as i32, i as i32));
/// compute_using_coordinates(geocodes);
///
/// // also see sparse (V.d1().sparse(Geocode(0, 0)))
/// let known_geocodes: HashMap<usize, Geocode> = [(3, Geocode(1, 1)), (7, Geocode(2, 1))].into_iter().collect();
/// let geocodes = V.d1().fun(|[i]| match known_geocodes.get(&i) {
///     Some(geo) => *geo,
///     None => Geocode(0, 0),
/// });
/// compute_using_coordinates(geocodes);
///
/// // the functional vectors are unbounded on construction
/// assert!(geocodes.is_unbounded());
/// assert_eq!(geocodes.card([]), usize::MAX);
/// ```
///
/// The function of the functional geocodes vector can be anything that can compute a geocode
/// based on its index. This is exactly the same in higher dimensional vectors.
///
/// If we break down all generic parameters:
/// * `D = D1` since it is a 1-dimensional vector,
/// * `T = Geocode` which is the scalar that the vector returns,
/// * `F` is any function implementing `Fn([usize; 1]) -> T`, we can never type its name,
/// * `C = UnboundedCard<D1>`.
///
/// You might guess that, all functional vectors in the above examples are capable of computing
/// a geocode for any given index. Their domains are unbounded; hence, the generic cardinality
/// parameter is `C = UnboundedCard<D1>`. This makes sense at one hand; however, having a bounded
/// domain is often more practical on the other hand. We can introduce bounds to the domain of
/// the functional vector by using:
/// * `bounded` if the vector is of dimension `D1`,
/// * `with_rectangular_bounds` or `with_variable_bounds` otherwise.
#[derive(Copy)]
pub struct FunVec<D, T, F, C = UnboundedCard<D>>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
    C: Card<D>,
{
    pub(crate) fun: F,
    pub(crate) card: C,
    phantom: PhantomData<(D, T)>,
}

impl<D, T, F, C> FunVec<D, T, F, C>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
    C: Card<D>,
{
    pub(crate) fn new(fun: F, card: C) -> Self {
        Self {
            fun,
            card,
            phantom: PhantomData,
        }
    }
}

impl<D, T, F, C> Clone for FunVec<D, T, F, C>
where
    D: Dim,
    F: Fn(D::Idx) -> T + Clone,
    C: Card<D> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            fun: self.fun.clone(),
            card: self.card.clone(),
            phantom: Default::default(),
        }
    }
}

macro_rules! impl_debug {
    ($dim:ty, $dbg_fn:ident) => {
        impl<T, F, C> Debug for FunVec<$dim, T, F, C>
        where
            F: Fn(<$dim as Dim>::Idx) -> T,
            C: Card<$dim>,
            T: Debug,
            Self: NVec<$dim, T>,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(
                    f,
                    "{{ kind: FunVec, dim: D{}, is_bounded: {}, values: ",
                    <$dim as Dim>::dimension(),
                    self.is_bounded(),
                )?;
                $dbg_fn(f, self)?;
                write!(f, " }}")
            }
        }
    };
}

impl_debug!(D1, dbg_values_d1);
impl_debug!(D2, dbg_values_d2);
impl_debug!(D3, dbg_values_d3);
impl_debug!(D4, dbg_values_d4);
