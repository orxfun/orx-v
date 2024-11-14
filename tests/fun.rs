mod std_order_vec;
use orx_v::*;

#[test]
fn fun_d1() {
    let v1 = V.d1().fun(|[i]| 10 + i).bounded(10);
    std_order_vec::assert_std_order_v1(v1, 10);
}

#[test]
fn fun_d2() {
    let v2 = V
        .d2()
        .fun(|[i, j]| i * 20 + j)
        .with_rectangular_bounds([10, 20]);
    std_order_vec::assert_std_order_v2(v2, 0);

    let num_cols = V.d1().constant(10).bounded(20);
    let v2 = V
        .d2()
        .fun(|[i, j]| i * 10 + j)
        .with_variable_bounds(num_cols);
    std_order_vec::assert_std_order_v2(v2, 0);
}
