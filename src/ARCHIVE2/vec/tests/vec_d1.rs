use crate::*;

fn d1_to_ref<'a, V>(vec: &'a V)
where
    V: NVec<D1, Element<'a> = &'a i32>,
{
    assert_eq!(vec.at(1), &42);
}

fn d1_to_ref2<V>(vec: &V)
where
    V: for<'a> NVec<D1, Element<'a> = &'a i32>,
{
    assert_eq!(vec.at(1), &42);
}

fn d1_to_val<'a, V>(vec: &'a V)
where
    V: NVec<D1, Element<'a> = i32> + 'a,
{
    assert_eq!(vec.at(1), 42);
    assert_eq!(vec.at(1), 42);
}

#[test]
fn vec_as_nvec_d1() {
    let vec = vec![0, 42, 2, 3, 4, 5];
    assert_eq!(vec.at(1), &42);
    d1_to_ref(&vec);
    d1_to_ref2(&vec);

    let cop = vec.copied();
    assert_eq!(cop.at(1), 42);
    d1_to_val(&cop);
}
