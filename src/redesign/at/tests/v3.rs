use super::super::super::{At, D1, D2, D3, FunAt};

#[test]
fn fun_as_at3() {
    let values = FunAt::new(|[i, j, k]: [usize; 3]| i * 100 + j * 10 + k);

    assert_eq!(At::<D3, usize>::at(&values, [2, 3, 4]), 234);
    let child = At::<D3, usize>::child(&values, 2);
    assert_eq!(At::<D2, usize>::at(&child, [3, 4]), 234);
    let child2 = At::<D2, usize>::child(&child, 3);
    assert_eq!(At::<D1, usize>::at(&child2, 4), 234);
    assert!(At::<D3, usize>::try_child(&values, 100).is_some());
}
