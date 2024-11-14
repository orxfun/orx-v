use crate::{NVecCore, D1, D2, D3, D4};
use core::fmt::Debug;

// impl

const UNBOUNDED_VIEW_LEN: usize = 4;
fn view_len(n: usize) -> usize {
    match n {
        usize::MAX => UNBOUNDED_VIEW_LEN,
        x => x,
    }
}

pub(crate) fn dbg_values_d4<T: Debug, V: NVecCore<D4, T>>(
    f: &mut core::fmt::Formatter<'_>,
    vec: &V,
) -> core::fmt::Result {
    let n = vec.core_card([]);

    write!(f, "[")?;
    if n > 0 {
        dbg_values_d3(f, &vec.core_child(0))?;
    }
    for i in 1..view_len(n) {
        write!(f, ", ")?;
        dbg_values_d3(f, &vec.core_child(i))?;
    }
    if n == usize::MAX {
        write!(f, ", ..]")?;
    }
    write!(f, "]")
}

pub(crate) fn dbg_values_d3<T: Debug, V: NVecCore<D3, T>>(
    f: &mut core::fmt::Formatter<'_>,
    vec: &V,
) -> core::fmt::Result {
    let n = vec.core_card([]);

    write!(f, "[")?;
    if n > 0 {
        dbg_values_d2(f, &vec.core_child(0))?;
    }
    for i in 1..view_len(n) {
        write!(f, ", ")?;
        dbg_values_d2(f, &vec.core_child(i))?;
    }
    if n == usize::MAX {
        write!(f, ", ..]")?;
    }
    write!(f, "]")
}

pub(crate) fn dbg_values_d2<T: Debug, V: NVecCore<D2, T>>(
    f: &mut core::fmt::Formatter<'_>,
    vec: &V,
) -> core::fmt::Result {
    let n = vec.core_card([]);

    write!(f, "[")?;
    if n > 0 {
        dbg_values_d1(f, &vec.core_child(0))?;
    }
    for i in 1..view_len(n) {
        write!(f, ", ")?;
        dbg_values_d1(f, &vec.core_child(i))?;
    }
    if n == usize::MAX {
        write!(f, ", ..]")?;
    }
    write!(f, "]")
}

pub(crate) fn dbg_values_d1<T: Debug, V: NVecCore<D1, T>>(
    f: &mut core::fmt::Formatter<'_>,
    vec: &V,
) -> core::fmt::Result {
    let n = vec.core_card([]);

    write!(f, "[")?;
    if n > 0 {
        vec.core_dbg_at(0, f)?;
    }
    for i in 1..view_len(n) {
        write!(f, ", ")?;
        vec.core_dbg_at(i, f)?;
    }
    if n == usize::MAX {
        write!(f, ", ..")?;
    }
    write!(f, "]")
}
