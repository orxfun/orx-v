use num::Num;
use orx_v::*;
use std::ops::AddAssign;

fn dot_product<N>(a: impl V1<N>, b: impl V1<N>) -> N
where
    N: Num + AddAssign,
{
    assert_eq!(a.card([]), b.card([]));

    let mut dot = N::zero();

    for (x, y) in a.all().zip(b.all()) {
        dot += x * y;
    }

    dot
}

fn main() {
    let v1 = vec![1, 2, 3, 4];
    let v2 = vec![2, 2, 2, 2];
    assert_eq!(dot_product(&v1, &v2), 20);

    let v1 = vec![1, 2, 3, 4];
    let v2 = V.d1().constant(2).bounded(v1.card([]));
    assert_eq!(dot_product(&v1, &v2), 20);
}
