use super::*;

pub trait LowerThan<Than: Dim>: Dim {}

impl LowerThan<D0> for D0 {}

impl LowerThan<D1> for D0 {}
impl LowerThan<D2> for D0 {}
impl LowerThan<D3> for D0 {}
impl LowerThan<D4> for D0 {}
impl LowerThan<D5> for D0 {}
impl LowerThan<D6> for D0 {}
impl LowerThan<D7> for D0 {}
impl LowerThan<D8> for D0 {}

impl LowerThan<D2> for D1 {}
impl LowerThan<D3> for D1 {}
impl LowerThan<D4> for D1 {}
impl LowerThan<D5> for D1 {}
impl LowerThan<D6> for D1 {}
impl LowerThan<D7> for D1 {}
impl LowerThan<D8> for D1 {}

impl LowerThan<D3> for D2 {}
impl LowerThan<D4> for D2 {}
impl LowerThan<D5> for D2 {}
impl LowerThan<D6> for D2 {}
impl LowerThan<D7> for D2 {}
impl LowerThan<D8> for D2 {}

impl LowerThan<D4> for D3 {}
impl LowerThan<D5> for D3 {}
impl LowerThan<D6> for D3 {}
impl LowerThan<D7> for D3 {}
impl LowerThan<D8> for D3 {}

impl LowerThan<D5> for D4 {}
impl LowerThan<D6> for D4 {}
impl LowerThan<D7> for D4 {}
impl LowerThan<D8> for D4 {}

impl LowerThan<D6> for D5 {}
impl LowerThan<D7> for D5 {}
impl LowerThan<D8> for D5 {}

impl LowerThan<D7> for D6 {}
impl LowerThan<D8> for D6 {}

impl LowerThan<D8> for D7 {}

pub trait HigherThan<Than: Dim>: Dim {}

impl<L: Dim, H: Dim> HigherThan<L> for H where L: LowerThan<H> {}
