#![allow(dead_code)]
use orx_v::*;

pub fn assert_std_order_v1<V: V1<usize>>(v: V, begin: usize) {
    let all: Vec<_> = v.all().collect();
    let all_enum: Vec<_> = v.all().enumerate().collect();

    let mut x = begin;
    let mut idx = 0;
    for i in 0..v.card([]) {
        assert_eq!(v.at([i]), x);
        assert_eq!(v.try_at([i]), Some(x));
        assert_eq!(all[idx], x);
        assert_eq!(all_enum[idx], (i, x));

        idx += 1;
        x += 1;
    }

    assert_eq!(idx, all.len());
    assert_eq!(idx, all_enum.len());

    // TODO: this line breaks the recursive boundary condition
    // let x = v.children().count();
}

pub fn assert_std_order_v2<V: V2<usize>>(v: V, begin: usize) {
    let all: Vec<_> = v.all().collect();
    let rf_v = &v;
    let all_enum: Vec<_> = (0..rf_v.card([]))
        .flat_map(move |i| (0..rf_v.card([i])).map(move |j| ([i, j], rf_v.at([i, j]))))
        .collect();
    let mut children = v.children();

    let mut x = begin;
    let mut idx = 0;
    for i in 0..v.card([]) {
        assert_std_order_v1(v.child(i), x);
        assert_std_order_v1(children.next().unwrap(), x);

        for j in 0..v.card([i]) {
            assert_eq!(v.at([i, j]), x);
            assert_eq!(v.try_at([i, j]), Some(x));
            assert_eq!(all[idx], x);
            assert_eq!(all_enum[idx], ([i, j], x));

            idx += 1;
            x += 1;
        }
    }

    assert_eq!(idx, all.len());
    assert_eq!(idx, all_enum.len());
}

pub fn assert_std_order_v2_mut<V: V2<usize> + V2Mut<usize>>(mut v: V, begin: usize) {
    assert_std_order_v2(&v, begin);

    for i in 0..v.card([]) {
        let mut ch = v.child_mut(i);
        for j in 0..ch.card([]) {
            *ch.at_mut(j) += 1000;
        }
        drop(ch);

        for j in 0..v.card([i]) {
            *v.at_mut([i, j]) -= 1000;
        }
    }

    assert_std_order_v2(&v, begin);
}
