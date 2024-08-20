use crate::*;
use alloc::collections::btree_map::BTreeMap;

fn pairs_to_map<T>(pairs: Vec<(usize, T)>) -> BTreeMap<usize, T> {
    pairs.into_iter().map(|x| (x.0, x.1)).collect()
}

fn d2_to_ref<'a, V>(map: &'a V)
where
    V: NVecRef<D2, Element<'a> = Option<&'a i32>>,
{
    assert_eq!(map.ref_at([1, 2]), Some(&42));
}

fn d2_to_ref2<V>(map: &V)
where
    V: for<'a> NVecRef<D2, Element<'a> = Option<&'a i32>>,
{
    assert_eq!(map.ref_at([1, 2]), Some(&42));
}

fn d2_to_val<V>(map: &V)
where
    V: NVec<D2, Option<i32>>,
{
    assert_eq!(map.at((1, 2)), Some(42));
    assert_eq!(map.at((1, 2)), Some(42));
}

#[test]
fn btree_map_as_nvec_d2_recursive() {
    let arr_map = || {
        vec![
            (0usize, vec![(0usize, 1)]),
            (1, vec![(0, 2), (1, 5), (4, 1), (3, 3), (2, 42), (5, 7)]),
            (2, vec![]),
            (3, vec![(0, 4)]),
        ]
    };

    let map: BTreeMap<usize, BTreeMap<usize, i32>> = arr_map()
        .into_iter()
        .map(|x| (x.0, pairs_to_map(x.1)))
        .collect();

    assert_eq!(map.at([1, 2]), Some(Some(42)));
    assert_eq!(map.at([1, 111]), Some(None));
    assert_eq!(map.at([111, 0]), None);

    assert_eq!(map.ref_at([1, 2]), Some(Some(&42)));
    assert_eq!(map.ref_at([1, 111]), Some(None));
    assert_eq!(map.ref_at([111, 0]), None);

    let flattened = map.flattened();

    assert_eq!(flattened.at([1, 2]), Some(42));
    assert_eq!(flattened.at([1, 111]), None);
    assert_eq!(flattened.at([111, 0]), None);

    assert_eq!(flattened.ref_at([1, 2]), Some(&42));
    assert_eq!(flattened.ref_at([1, 111]), None);
    assert_eq!(flattened.ref_at([111, 0]), None);
}

#[test]
fn btree_map_as_nvec_d2_recursive_with_vec() {
    let arr_map = || {
        vec![
            (0usize, vec![1]),
            (1, vec![2, 5, 42, 3, 3, 7]),
            (2, vec![]),
            (3, vec![4]),
        ]
    };

    let map: BTreeMap<usize, Vec<i32>> = arr_map().into_iter().collect();

    assert_eq!(map.at([1, 2]), Some(42));
    assert_eq!(map.at([111, 0]), None);
    d2_to_val(&map);

    assert_eq!(map.ref_at([1, 2]), Some(&42));
    assert_eq!(map.ref_at([111, 0]), None);
    d2_to_ref(&map);
    d2_to_ref2(&map);
}
