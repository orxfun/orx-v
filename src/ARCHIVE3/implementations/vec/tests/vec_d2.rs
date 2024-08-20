use crate::*;
use alloc::collections::btree_map::BTreeMap;

fn d2_to_ref<'a, V>(vec: &'a V)
where
    V: NVecRef<D2, Element<'a> = &'a i32>,
{
    assert_eq!(vec.ref_at((1, 2)), &42);
}

fn d2_to_ref2<V>(vec: &V)
where
    V: for<'a> NVecRef<D2, Element<'a> = &'a i32>,
{
    assert_eq!(vec.ref_at((1, 2)), &42);
}

fn d2_to_val<V>(vec: &V)
where
    V: NVec<D2, i32>,
{
    assert_eq!(vec.at((1, 2)), 42);
    assert_eq!(vec.at((1, 2)), 42);
}

#[test]
fn vec_vec_as_nvec_d2() {
    let vec: Vec<Vec<i32>> = vec![vec![0], vec![1, 2, 42], vec![3]];

    assert_eq!(vec.ref_at([1, 2]), &42);
    d2_to_ref(&vec);
    d2_to_ref2(&vec);

    assert_eq!(vec.at([1, 2]), 42);
    d2_to_val(&vec);
}
#[test]
fn vec_map_as_nvec_d2() {
    let vec: Vec<BTreeMap<usize, i32>> = vec![
        BTreeMap::from_iter([(7, 3)].into_iter()),
        BTreeMap::from_iter([(2, 42)].into_iter()),
        BTreeMap::from_iter([(0, 3), (9, 4)].into_iter()),
    ];

    assert_eq!(vec.ref_at([1, 2]), Some(&42));
    assert_eq!(vec.at([1, 2]), Some(42));
}
