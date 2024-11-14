#![cfg(test)]
use orx_v::*;

#[test]
fn sparse_v1() {
    let mut v1 = V.d1().sparse(42).bounded(10);
    assert_eq!(v1.lookup_len(), 0);

    for x in v1.all() {
        assert_eq!(x, 42);
    }

    *v1.at_mut([1]) = 1;
    v1.set([3], 3);
    assert_eq!(v1.lookup_len(), 2);

    for (i, x) in v1.all().enumerate() {
        match i {
            1 | 3 => assert_eq!(x, i),
            _ => assert_eq!(x, 42),
        }
    }
}

#[test]
fn sparse_from_v1() {
    let map = DefaultLookup::<D1, _>::from_iter([([1], 1), ([3], 3)]);
    let mut v1 = V.d1().sparse_from(map, 42).bounded(10);
    assert_eq!(v1.lookup_len(), 2);

    for (i, x) in v1.all().enumerate() {
        match i {
            1 | 3 => assert_eq!(x, i),
            _ => assert_eq!(x, 42),
        }
    }

    *v1.at_mut([4]) = 4;
    v1.set([5], 5);
    assert_eq!(v1.lookup_len(), 4);

    for (i, x) in v1.all().enumerate() {
        match i {
            1 | 3 | 4 | 5 => assert_eq!(x, i),
            _ => assert_eq!(x, 42),
        }
    }
}
