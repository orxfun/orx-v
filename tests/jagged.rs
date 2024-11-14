mod std_order_vec;
use orx_v::*;

#[test]
fn jagged_from_ends() {
    let storage = vec![7, 8, 9, 10, 11, 12];
    let row_ends = [3, 4, 6];

    let jagged = storage.as_jagged(&row_ends);
    std_order_vec::assert_std_order_v2(&jagged, 7);

    assert_eq!(jagged.card([]), 3);
    assert_eq!(jagged.card([0]), 3);
    assert_eq!(jagged.card([1]), 1);
    assert_eq!(jagged.card([2]), 2);
}

#[test]
fn jagged_from_lengths() {
    let storage = vec![7, 8, 9, 10, 11, 12];
    let row_lens = [3, 1, 2];

    let jagged = storage.as_jagged_from_row_lengths(&row_lens);
    std_order_vec::assert_std_order_v2(&jagged, 7);

    assert_eq!(jagged.card([]), 3);
    assert_eq!(jagged.card([0]), 3);
    assert_eq!(jagged.card([1]), 1);
    assert_eq!(jagged.card([2]), 2);
}

#[test]
fn jagged_mut() {
    let storage: Vec<_> = (100..1000).collect();
    let row_lens = [100, 300, 200, 150, 150];
    let jagged = storage.into_jagged_from_row_lengths(&row_lens);
    std_order_vec::assert_std_order_v2_mut(jagged, 100);
}
