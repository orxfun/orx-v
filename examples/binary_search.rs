use orx_v::*;
use std::cmp::Ordering;

// implementation taken and adapted from TheAlgorithms repository:
// https://github.com/TheAlgorithms/Rust/blob/master/src/searching/binary_search.rs

fn binary_search<T: Ord>(item: &T, arr: impl V1<T>) -> Option<usize> {
    let is_asc = is_asc_arr(&arr);

    let mut left = 0;
    let mut right = arr.card([]);

    while left < right {
        if match_compare(item, &arr, &mut left, &mut right, is_asc) {
            return Some(left);
        }
    }

    None
}

fn match_compare<T: Ord>(
    item: &T,
    arr: impl V1<T>,
    left: &mut usize,
    right: &mut usize,
    is_asc: bool,
) -> bool {
    let mid = *left + (*right - *left) / 2;
    let cmp_result = item.cmp(&arr.at(mid));

    match (is_asc, cmp_result) {
        (true, Ordering::Less) | (false, Ordering::Greater) => {
            *right = mid;
        }
        (true, Ordering::Greater) | (false, Ordering::Less) => {
            *left = mid + 1;
        }
        (_, Ordering::Equal) => {
            *left = mid;
            return true;
        }
    }

    false
}

fn is_asc_arr<T: Ord>(arr: impl V1<T>) -> bool {
    arr.card([]) > 1 && arr.at(0) < arr.at(arr.card([]) - 1)
}

fn main() {
    let arr = [1, 3, 8, 11];
    assert_eq!(binary_search(&7, &arr), None);
    assert_eq!(binary_search(&8, &arr), Some(2));

    // with non-slice data types

    #[cfg(feature = "orx-split-vec")]
    {
        use orx_split_vec::SplitVec;

        let split: SplitVec<_> = [1, 3, 8, 11, 12, 14, 17].into_iter().collect();
        assert_eq!(split.fragments().len(), 2);
        assert_eq!(split.fragments()[0], &[1, 3, 8, 11]);
        assert_eq!(split.fragments()[1], &[12, 14, 17]);
        assert_eq!(binary_search(&7, &split), None);
        assert_eq!(binary_search(&12, &split), Some(4));
    }

    let mut sparse = V.d1().sparse(1000).bounded(6);
    sparse.set(0, 7);
    sparse.set(1, 12);
    assert_eq!(binary_search(&8, &sparse), None);
    assert_eq!(binary_search(&12, &sparse), Some(1));

    let constant = V.d1().constant(42).bounded(6);
    assert_eq!(binary_search(&8, &constant), None);
    assert_eq!(binary_search(&42, &constant), Some(3)); // first 42 found in the mid

    let fun_vec = V.d1().fun(|[i]| 2 * i + 1).bounded(6); // => 1, 3, 5, 7, 9, 11
    assert_eq!(binary_search(&8, &fun_vec), None);
    assert_eq!(binary_search(&7, &fun_vec), Some(3));
}
