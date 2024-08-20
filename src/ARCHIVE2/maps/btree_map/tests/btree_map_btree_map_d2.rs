use crate::*;
use alloc::collections::btree_map::BTreeMap;

fn pairs_to_map<T>(pairs: Vec<(usize, T)>) -> BTreeMap<usize, T> {
    pairs.into_iter().map(|x| (x.0, x.1)).collect()
}

fn d2_to_ref<'a, V>(map: &'a V)
where
    V: NVec<D2, Element<'a> = Option<&'a i32>>,
{
    assert_eq!(map.at([1, 2]), Some(&42));
}

fn d2_to_ref2<V>(map: &V)
where
    V: for<'a> NVec<D2, Element<'a> = Option<&'a i32>>,
{
    assert_eq!(map.at([1, 2]), Some(&42));
}

fn d2_to_val<'a, V>(map: &'a V)
where
    V: NVec<D2, Element<'a> = Option<i32>> + 'a,
{
    assert_eq!(map.at((1, 2)), Some(42));
    assert_eq!(map.at((1, 2)), Some(42));
}

#[test]
fn btree_map_btree_map_as_nvec_d2() {
    let vec = vec![
        (0, pairs_to_map(vec![(0, 1), (1, 4), (3, 13)])),
        (7, pairs_to_map(vec![(0, 1), (1, 3), (3, 13)])),
        (3, pairs_to_map(vec![(0, 1), (1, 7), (3, 13)])),
        (1, pairs_to_map(vec![(0, 1), (2, 42), (3, 13)])),
    ];

    let map = pairs_to_map(vec);
    assert_eq!(map.at([1, 2]), Some(Some(&42)));

    let flat = map.flattened();
    assert_eq!(flat.at([1, 2]), Some(&42));
    assert_eq!(flat.at([1, 2]), Some(&42));
    // d2_to_ref(&map.flattened());
    // d2_to_ref2(&flat);
}
