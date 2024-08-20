use crate::*;

#[test]
fn funvec_as_nvec_d2_constant() {
    let fun = FunVecBuilder::d2().new(|_: (usize, usize)| 42);
    assert_eq!(fun.at((0, 0)), 42);
    assert_eq!(fun.at([1, 3]), 42);
}

#[test]
fn funvec_as_nvec_d2_constant_ref() {
    let fun = FunVecBuilder::d2().new(|_: [usize; 2]| &42);
    assert_eq!(fun.at((0, 0)), &42);
    assert_eq!(fun.at([1, 3]), &42);
}

#[test]
fn funvec_as_nvec_d2_copy_from_capture() {
    let vec = vec![vec![1], vec![], vec![2, 5, 1, 3, 42, 7], vec![4]];

    let fun = FunVecBuilder::d2()
        .new(|(i, j): (usize, usize)| vec.get(i).and_then(|x| x.get(j)).copied());

    assert_eq!(fun.at((0, 0)), Some(1));
    assert_eq!(fun.at([2, 4]), Some(42));
}

#[test]
fn funvec_as_nvec_d2_ref_from_capture() {
    let vec = vec![vec![1], vec![], vec![2, 5, 1, 3, 42, 7], vec![4]];

    let fun = FunVecBuilder::d2().new(|(i, j): (usize, usize)| vec.get(i).and_then(|x| x.get(j)));

    assert_eq!(fun.at((0, 0)), Some(&1));
    assert_eq!(fun.at([2, 4]), Some(&42));
}
