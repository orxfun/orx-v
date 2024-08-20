use crate::*;
use alloc::collections::btree_map::BTreeMap;

fn pairs_to_map<T>(pairs: Vec<(usize, T)>) -> BTreeMap<usize, T> {
    pairs.into_iter().map(|x| (x.0, x.1)).collect()
}

fn d1_to_ref<'a, V>(vec: &'a V)
where
    V: NVecRef<D1, Element<'a> = Option<&'a i32>>,
{
    assert_eq!(vec.ref_at(1), Some(&42));
    assert_eq!(vec.ref_at(42), None);
}

fn d1_to_ref2<V>(vec: &V)
where
    V: for<'a> NVecRef<D1, Element<'a> = Option<&'a i32>>,
{
    d1_to_ref(vec)
}

fn d1_to_val<V>(vec: &V)
where
    V: NVec<D1, Option<i32>>,
{
    assert_eq!(vec.at(1), Some(42));
    assert_eq!(vec.at(1), Some(42));
}

#[test]
fn btree_map_as_nvec_d1() {
    let map: BTreeMap<usize, i32> = pairs_to_map(vec![(0, 1), (1, 42), (3, 13)]);

    assert_eq!(map.ref_at(1), Some(&42));
    assert_eq!(map.ref_at(42), None);
    d1_to_ref(&map);
    d1_to_ref2(&map);

    assert_eq!(map.at(1), Some(42));
    assert_eq!(map.at(42), None);
    d1_to_val(&map);
}
