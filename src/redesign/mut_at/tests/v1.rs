use super::super::super::D1;
use super::super::{FunMutAt, MutAt};
use alloc::string::{String, ToString};
use alloc::vec;

fn target_fun<'a>(v1: &mut impl MutAt<D1, usize>, mut v2: impl MutAt<D1, String>) {
    v2.mut_at(1).push_str("z");
    let x = v2.mut_at(1).len();
    *v1.mut_at(2) += x;
}

#[test]
fn vec_as_mut_at1() {
    let mut v1 = vec![1, 2, 3];
    let v2 = vec!["x".to_string(), "y".to_string()];
    target_fun(&mut v1, v2);
    assert_eq!(v1[2], 5);
}

#[test]
fn slice_as_mut_at1() {
    let mut vec1 = vec![1, 2, 3];
    let mut v1 = vec1.as_mut_slice();

    let mut vec2 = vec!["x".to_string(), "y".to_string()];
    let v2 = vec2.as_mut_slice();

    target_fun(&mut v1, v2);
    assert_eq!(v1[2], 5);
    assert_eq!(&vec2[1], "yz");
}

#[test]
fn fun_as_mut_at1() {
    use alloc::vec::Vec;
    fn get<'a>(v: &'a mut Vec<i32>, i: usize) -> &'a mut i32 {
        todo!()
    }

    let mut vec1 = vec![1, 2, 3];
    let v1 = FunMutAt::<'_, D1, _, _>::new(|i| &mut vec1[i]);

    // let v1 = FunAt::new(|i| i + 1);

    // let vec2 = vec!["x".to_string(), "y".to_string()];
    // let v2 = FunAt::new(|i| &vec2[i]);

    // let res = target_fun(&v1, &v2);
    // assert_eq!(res, 4);
}
