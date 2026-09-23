use super::super::super::{AtMut, D1, D2, D3, FunMutAt};
use alloc::vec;

fn increment_row(row: &mut (impl AtMut<D1, usize> + ?Sized)) {
    for i in 0..2 {
        *row.at_mut(i) += 10;
    }
}

#[test]
fn vec_vec_vec_as_at_mut3() {
    let mut values = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6]]];

    *AtMut::<D3, usize>::at_mut(&mut values, [1, 0, 1]) += 10;
    let mut outer = AtMut::<D3, usize>::child_mut(&mut values, 0);
    let mut row = AtMut::<D2, usize>::child_mut(&mut outer, 1);
    increment_row(&mut row);

    assert_eq!(*AtMut::<D3, usize>::at(&values, [1, 0, 1]), 16);
    assert_eq!(*AtMut::<D3, usize>::at(&values, [0, 1, 0]), 13);
    assert!(AtMut::<D3, usize>::try_child_mut(&mut values, 2).is_none());
}

#[test]
fn slices_as_at_mut3() {
    let mut values = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6]]];
    let values = values.as_mut_slice();

    *AtMut::<D3, usize>::at_mut(values, [0, 0, 1]) = 20;
    assert_eq!(*AtMut::<D3, usize>::at(values, [0, 0, 1]), 20);
    assert_eq!(AtMut::<D3, usize>::try_at_mut(values, [1, 1, 0]), None);
}

#[test]
fn fun_as_at_mut3() {
    let mut data = (3, vec![0; 18]);

    fn get<'a>(data: &'a (usize, alloc::vec::Vec<usize>), [i, j, k]: [usize; 3]) -> &'a usize {
        &data.1[i * data.0 * 2 + j * 2 + k]
    }
    fn get_mut<'a>(
        data: &'a mut (usize, alloc::vec::Vec<usize>),
        [i, j, k]: [usize; 3],
    ) -> &'a mut usize {
        &mut data.1[i * data.0 * 2 + j * 2 + k]
    }

    let mut values = FunMutAt::new(&mut data, get, get_mut);
    *AtMut::<D3, usize>::at_mut(&mut values, [2, 1, 1]) = 42;
    let mut child = AtMut::<D3, usize>::child_mut(&mut values, 2);
    let mut child2 = AtMut::<D2, usize>::child_mut(&mut child, 1);
    *AtMut::<D1, usize>::at_mut(&mut child2, 1) += 1;

    assert_eq!(*AtMut::<D3, usize>::at(&values, [2, 1, 1]), 43);
    assert!(AtMut::<D3, usize>::try_child_mut(&mut values, 100).is_some());
}
