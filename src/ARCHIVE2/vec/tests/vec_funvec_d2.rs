use crate::*;

fn d2_to_val<'a, V>(vec: &'a V)
where
    V: NVec<D2, Element<'a> = i32> + 'a,
{
    assert_eq!(vec.at([1, 21]), 42);
    assert_eq!(vec.at([2, 6]), 42);
}

#[test]
fn vec_funvec_as_nvec_d2() {
    fn just_zero(_: usize) -> i32 {
        0
    }
    fn get(j: usize) -> i32 {
        match j {
            6 | 21 => 42,
            _ => j as i32 + 1,
        }
    }

    let vec: Vec<_> = [just_zero, get, get]
        .iter()
        .map(|f| FunVecBuilder::d1().new(f))
        .collect();
    assert_eq!(vec.at([1, 21]), 42);
    assert_eq!(vec.at([2, 6]), 42);
    d2_to_val(&vec);
}

#[test]
fn vec_funvec_as_nvec_d2_capture() {
    let vec: Vec<_> = [5, 2, 7]
        .iter()
        .map(|coefficient| {
            let calc = move |j: usize| coefficient * j as i32;
            FunVecBuilder::d1().new(calc)
        })
        .collect();
    assert_eq!(vec.at([1, 21]), 42);
    assert_eq!(vec.at([2, 6]), 42);
    d2_to_val(&vec);
}
