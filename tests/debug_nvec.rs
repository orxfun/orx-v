use core::fmt::Debug;
use orx_v::*;

fn dbg_str<X: Debug>(x: &X) -> String {
    format!("{:?}", x)
}

#[test]
fn empty_vec() {
    let v1 = V.d1().empty::<usize>();
    assert_eq!(
        dbg_str(&v1).as_str(),
        "{ kind: EmptyVec, dim: D1, values: [] }"
    );

    let v2 = V.d2().empty::<usize>();
    assert_eq!(
        dbg_str(&v2).as_str(),
        "{ kind: EmptyVec, dim: D2, values: [] }"
    );

    let v3 = V.d3().empty::<usize>();
    assert_eq!(
        dbg_str(&v3).as_str(),
        "{ kind: EmptyVec, dim: D3, values: [] }"
    );

    let v4 = V.d4().empty::<usize>();
    assert_eq!(
        dbg_str(&v4).as_str(),
        "{ kind: EmptyVec, dim: D4, values: [] }"
    );
}

#[test]
fn const_vec() {
    let v1 = V.d1().constant(3);
    assert_eq!(
        dbg_str(&v1).as_str(),
        "{ kind: ConstantVec, dim: D1, is_bounded: false, values: [3, 3, 3, 3, ..] }"
    );

    let v2 = V.d2().constant(3);
    assert_eq!(
        dbg_str(&v2).as_str(),
        "{ kind: ConstantVec, dim: D2, is_bounded: false, values: [[3, 3, 3, 3, ..], [3, 3, 3, 3, ..], [3, 3, 3, 3, ..], [3, 3, 3, 3, ..], ..]] }"
    );

    let v1 = V.d1().constant(3).bounded(3);
    assert_eq!(
        dbg_str(&v1).as_str(),
        "{ kind: ConstantVec, dim: D1, is_bounded: true, values: [3, 3, 3] }"
    );

    let v2 = V.d2().constant(3).with_rectangular_bounds([2, 3]);
    assert_eq!(
        dbg_str(&v2).as_str(),
        "{ kind: ConstantVec, dim: D2, is_bounded: true, values: [[3, 3, 3], [3, 3, 3]] }"
    );

    let v2 = V.d2().constant(3).with_variable_bounds([2, 3, 1]);
    assert_eq!(
        dbg_str(&v2).as_str(),
        "{ kind: ConstantVec, dim: D2, is_bounded: true, values: [[3, 3], [3, 3, 3], [3]] }"
    );
}

#[test]
fn sparse_vec() {
    // d1

    let v1 = V.d1().sparse(2);
    assert_eq!(
        dbg_str(&v1).as_str(),
        "{ kind: SparseVec, dim: D1, is_bounded: false, default_value: 2, lookup_len: 0, values: [2, 2, 2, 2, ..] }"
    );

    let v1 = V.d1().sparse(2).bounded(3);
    assert_eq!(
        dbg_str(&v1).as_str(),
        "{ kind: SparseVec, dim: D1, is_bounded: true, default_value: 2, lookup_len: 0, values: [2, 2, 2] }"
    );

    let mut v1 = V.d1().sparse(2);
    v1.set(3, 7);
    v1.set(1111, 11);
    assert_eq!(
        dbg_str(&v1).as_str(),
        "{ kind: SparseVec, dim: D1, is_bounded: false, default_value: 2, lookup_len: 2, values: [2, 2, 2, 7, ..] }"
    );

    let mut v1 = V.d1().sparse(2).bounded(5);
    v1.set(3, 7);
    assert_eq!(
        dbg_str(&v1).as_str(),
        "{ kind: SparseVec, dim: D1, is_bounded: true, default_value: 2, lookup_len: 1, values: [2, 2, 2, 7, 2] }"
    );

    // d2

    let v2 = V.d2().sparse(2);
    assert_eq!(
        dbg_str(&v2).as_str(),
        "{ kind: SparseVec, dim: D2, is_bounded: false, default_value: 2, lookup_len: 0, values: [[2, 2, 2, 2, ..], [2, 2, 2, 2, ..], [2, 2, 2, 2, ..], [2, 2, 2, 2, ..], ..]] }"
    );

    let v2 = V.d2().sparse(2).with_rectangular_bounds([2, 3]);
    assert_eq!(
        dbg_str(&v2).as_str(),
        "{ kind: SparseVec, dim: D2, is_bounded: true, default_value: 2, lookup_len: 0, values: [[2, 2, 2], [2, 2, 2]] }"
    );

    let mut v2 = V.d2().sparse(2);
    v2.set([0, 2], 7);
    v2.set([1111, 34], 11);
    assert_eq!(
        dbg_str(&v2).as_str(),
        "{ kind: SparseVec, dim: D2, is_bounded: false, default_value: 2, lookup_len: 2, values: [[2, 2, 7, 2, ..], [2, 2, 2, 2, ..], [2, 2, 2, 2, ..], [2, 2, 2, 2, ..], ..]] }"
    );

    let mut v2 = V.d2().sparse(2).with_rectangular_bounds([2, 3]);
    v2.set([0, 2], 7);
    v2.set([1, 0], 3);
    assert_eq!(
        dbg_str(&v2).as_str(),
        "{ kind: SparseVec, dim: D2, is_bounded: true, default_value: 2, lookup_len: 2, values: [[2, 2, 7], [3, 2, 2]] }"
    );
}

#[test]
fn fun_vec() {
    let v1 = V.d1().fun(|[i]| i + 1);
    assert_eq!(
        dbg_str(&v1).as_str(),
        "{ kind: FunVec, dim: D1, is_bounded: false, values: [1, 2, 3, 4, ..] }"
    );

    let v1 = V.d1().fun(|[i]| i + 1).bounded(7);
    assert_eq!(
        dbg_str(&v1).as_str(),
        "{ kind: FunVec, dim: D1, is_bounded: true, values: [1, 2, 3, 4, 5, 6, 7] }"
    );

    let v2 = V.d2().fun(|[i, j]| i + j);
    assert_eq!(
        dbg_str(&v2).as_str(),
        "{ kind: FunVec, dim: D2, is_bounded: false, values: [[0, 1, 2, 3, ..], [1, 2, 3, 4, ..], [2, 3, 4, 5, ..], [3, 4, 5, 6, ..], ..]] }"
    );

    let v2 = V.d2().fun(|[i, j]| i + j).with_rectangular_bounds([2, 3]);
    assert_eq!(
        dbg_str(&v2).as_str(),
        "{ kind: FunVec, dim: D2, is_bounded: true, values: [[0, 1, 2], [1, 2, 3]] }"
    );

    let v2 = V.d2().fun(|[i, j]| i + j).with_variable_bounds([2, 3, 1]);
    assert_eq!(
        dbg_str(&v2).as_str(),
        "{ kind: FunVec, dim: D2, is_bounded: true, values: [[0, 1], [1, 2, 3], [2]] }"
    );

    let v3 = V.d3().fun(|[i, j, k]| i + j + k);
    assert_eq!(
        dbg_str(&v3).as_str(),
        "{ kind: FunVec, dim: D3, is_bounded: false, values: [[[0, 1, 2, 3, ..], [1, 2, 3, 4, ..], [2, 3, 4, 5, ..], [3, 4, 5, 6, ..], ..]], [[1, 2, 3, 4, ..], [2, 3, 4, 5, ..], [3, 4, 5, 6, ..], [4, 5, 6, 7, ..], ..]], [[2, 3, 4, 5, ..], [3, 4, 5, 6, ..], [4, 5, 6, 7, ..], [5, 6, 7, 8, ..], ..]], [[3, 4, 5, 6, ..], [4, 5, 6, 7, ..], [5, 6, 7, 8, ..], [6, 7, 8, 9, ..], ..]], ..]] }"
    );

    let v3 = V
        .d3()
        .fun(|[i, j, k]| i + j + k)
        .with_rectangular_bounds([2, 3, 4]);
    assert_eq!(
        dbg_str(&v3).as_str(),
        "{ kind: FunVec, dim: D3, is_bounded: true, values: [[[0, 1, 2, 3], [1, 2, 3, 4], [2, 3, 4, 5]], [[1, 2, 3, 4], [2, 3, 4, 5], [3, 4, 5, 6]]] }"
    );

    let v3 = V
        .d3()
        .fun(|[i, j, k]| i + j + k)
        .with_variable_bounds([vec![2, 3], vec![3, 1, 4]]);
    assert_eq!(
        dbg_str(&v3).as_str(),
        "{ kind: FunVec, dim: D3, is_bounded: true, values: [[[0, 1], [1, 2, 3]], [[1, 2, 3], [2], [3, 4, 5, 6]]] }"
    );
}

#[test]
fn jagged() {
    let v1 = vec![0, 1, 2, 3, 4, 5];
    let v2 = v1.as_jagged_from_row_lengths(&[2, 3, 0, 1]);
    assert_eq!(
        dbg_str(&v2).as_str(),
        "{ kind: FlatJagged, dim: D2, values: [[0, 1], [2, 3, 4], [], [5]] }"
    );
}

#[test]
fn cached() {
    // d1

    let v1 = V.d1().fun(|[i]| i + 1).into_cached();
    assert_eq!(
        dbg_str(&v1).as_str(),
        "{ kind: CachedVec, dim: D1, is_bounded: false, cache_len: 0, values: [1, 2, 3, 4, ..] }"
    );

    let v1 = V.d1().fun(|[i]| i + 1).bounded(5).into_cached();
    assert_eq!(
        dbg_str(&v1).as_str(),
        "{ kind: CachedVec, dim: D1, is_bounded: true, cache_len: 0, values: [1, 2, 3, 4, 5] }"
    );

    let v1 = V.d1().fun(|[i]| i + 1).into_cached();
    let _ = v1.at([0]);
    let _ = v1.at([3]);
    let _ = v1.at([3]);
    assert_eq!(
        dbg_str(&v1).as_str(),
        "{ kind: CachedVec, dim: D1, is_bounded: false, cache_len: 2, values: [1, 2, 3, 4, ..] }"
    );

    let v1 = V.d1().fun(|[i]| i + 1).bounded(5).into_cached();
    let _ = v1.at([0]);
    let _ = v1.at([3]);
    let _ = v1.at([3]);
    assert_eq!(
        dbg_str(&v1).as_str(),
        "{ kind: CachedVec, dim: D1, is_bounded: true, cache_len: 2, values: [1, 2, 3, 4, 5] }"
    );

    // d2

    let v2 = V.d2().fun(|[i, j]| i + j).into_cached();
    let _ = v2.at([0, 3]);
    let _ = v2.at([0, 3]);
    let _ = v2.at([1, 2]);
    assert_eq!(
        dbg_str(&v2).as_str(),
        "{ kind: CachedVec, dim: D2, is_bounded: false, cache_len: 2, values: [[0, 1, 2, 3, ..], [1, 2, 3, 4, ..], [2, 3, 4, 5, ..], [3, 4, 5, 6, ..], ..]] }"
    );

    let v2 = V
        .d2()
        .fun(|[i, j]| i + j)
        .with_rectangular_bounds([2, 3])
        .into_cached();
    let _ = v2.at([0, 3]);
    let _ = v2.at([0, 3]);
    let _ = v2.at([1, 2]);
    assert_eq!(
        dbg_str(&v2).as_str(),
        "{ kind: CachedVec, dim: D2, is_bounded: true, cache_len: 2, values: [[0, 1, 2], [1, 2, 3]] }"
    );
}
