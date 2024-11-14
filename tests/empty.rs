use orx_v::*;

#[test]
fn empty_v1() {
    let v1 = V.d1().empty::<usize>();

    assert_eq!(v1.num_children(), 0);
    assert_eq!(v1.card([]), 0);

    assert_eq!(v1.all().count(), 0);
    assert_eq!(v1.try_at([0]), None);

    assert_eq!(v1.in_bounds([]), true);
    assert_eq!(v1.in_bounds([0]), false);
}

#[test]
fn empty_v2() {
    let v2 = V.d2().empty::<usize>();

    assert_eq!(v2.num_children(), 0);
    assert_eq!(v2.card([]), 0);

    assert_eq!(v2.all().count(), 0);
    assert_eq!(v2.try_at([0, 0]), None);

    assert_eq!(v2.in_bounds([]), true);
    assert_eq!(v2.in_bounds([0]), false);
    assert_eq!(v2.in_bounds([0, 0]), false);
}
