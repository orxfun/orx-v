use super::super::super::FunVec;
use super::utils::second;

#[test]
fn funvec_as_v1() {
    let f = |i: usize| i + 1;
    let v = FunVec::new(f);

    assert_eq!(second(&v), 2);
}
