use crate::*;

fn d1_to_val<'a, V>(vec: &'a V)
where
    V: NVec<D1, Element<'a> = i32> + 'a,
{
    assert_eq!(vec.at(1), 42);
    assert_eq!(vec.at(1), 42);
}

#[test]
fn fun_vec_as_nvec_d1() {
    fn get(i: usize) -> i32 {
        match i {
            1 => 42,
            _ => i as i32 + 1,
        }
    }

    let vec = FunVecBuilder::d1().new(get);
    assert_eq!(vec.at(1), 42);
    d1_to_val(&vec);

    let get = |i| match i {
        1 => 42,
        _ => i as i32 + 1,
    };
    let vec = FunVecBuilder::d1().new(get);
    assert_eq!(vec.at(1), 42);
    d1_to_val(&vec);
}

#[test]
fn fun_vec_as_nvec_d1_capture() {
    let forty_two = Box::new(42);

    let get = |i| match i {
        1 => *forty_two,
        _ => i as i32 + 1,
    };
    let vec = FunVecBuilder::d1().new(get);
    assert_eq!(vec.at(1), 42);
    d1_to_val(&vec);
}
