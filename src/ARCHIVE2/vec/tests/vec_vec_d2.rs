use crate::*;

fn d2_to_ref<'a, V>(vec: &'a V)
where
    V: NVec<D2, Element<'a> = &'a i32>,
{
    assert_eq!(vec.at((1, 2)), &42);
}

fn d2_to_ref2<V>(vec: &V)
where
    V: for<'a> NVec<D2, Element<'a> = &'a i32>,
{
    assert_eq!(vec.at((1, 2)), &42);
}

fn d2_to_val<'a, V>(vec: &'a V)
where
    V: NVec<D2, Element<'a> = i32> + 'a,
{
    assert_eq!(vec.at((1, 2)), 42);
    assert_eq!(vec.at((1, 2)), 42);
}

#[test]
fn vec_vec_as_nvec_d2() {
    let vec = vec![vec![0], vec![1, 2, 42], vec![3]];
    assert_eq!(vec.at([1, 2]), &42);
    d2_to_ref(&vec);
    d2_to_ref2(&vec);

    let cop = vec.copied();
    assert_eq!(cop.at([1, 2]), 42);
    d2_to_val(&cop);
}
