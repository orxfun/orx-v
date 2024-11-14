use orx_v::*;

#[test]
fn constant_v1() {
    let v1 = V.d1().constant(3);

    for i in 0..100 {
        assert_eq!(v1.at([i]), 3);
    }
}

#[test]
fn constant_v2() {
    let v2 = V.d2().constant(3);

    for i in 0..100 {
        for j in 0..34 {
            assert_eq!(v2.at([i, j]), 3);
        }
    }
}
