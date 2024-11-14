mod std_order_vec;
use orx_v::*;

#[test]
fn range_v1() {
    let v1 = 3..7;

    assert_eq!(v1.card([]), 4);
    for x in 3..7 {
        assert_eq!(v1.at([x - 3]), x);
        assert_eq!(v1.try_at([x - 3]), Some(x));
    }

    let vec1: Vec<_> = v1.all().collect();
    assert_eq!(vec1, vec![3, 4, 5, 6]);

    std_order_vec::assert_std_order_v1(v1, 3);
}

#[test]
fn range_std_ord() {
    std_order_vec::assert_std_order_v1(3..100, 3);
}
