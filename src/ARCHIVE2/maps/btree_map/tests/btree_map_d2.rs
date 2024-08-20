use crate::*;
use alloc::collections::btree_map::BTreeMap;

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
fn btree_map_as_nvec_d2() {
    let arr_map = || {
        vec![
            (0, vec![(0, 1)]),
            (1, vec![(0, 2), (1, 5), (4, 1), (3, 3), (2, 42), (5, 7)]),
            (2, vec![]),
            (3, vec![(0, 4)]),
        ]
    };

    let map: BTreeMap<(usize, usize), _> = arr_map()
        .into_iter()
        .flat_map(|(i, x)| x.into_iter().map(move |(j, val)| ((i, j), val)))
        .collect();

    assert_eq!(map.at([1, 2]), Some(&42));
    d2_to_ref(&map);
    d2_to_ref2(&map);

    let cop = map.copied();
    assert_eq!(cop.at((1, 2)), Some(42));
    d2_to_val(&cop);
}
