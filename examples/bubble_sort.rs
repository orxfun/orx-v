use orx_v::*;
use std::{collections::HashSet, fmt::Debug, hash::Hash};

// implementation taken and adapted from TheAlgorithms repository:
// https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/bubble_sort.rs

fn bubble_sort<T: Ord>(arr: &mut impl V1Mut<T>) {
    let mut n = arr.card([]);

    if n == 0 {
        return;
    }

    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            let vi = arr.at(i);
            if vi > arr.at(i + 1) {
                arr.set(i, arr.at(i + 1));
                arr.set(i + 1, vi);
                sorted = false;
            }
        }
        n -= 1;
    }
}

fn is_sorted<T: PartialOrd>(arr: &impl V1<T>) -> bool {
    match arr.card([]) {
        0 | 1 => true,
        n => {
            for i in 0..n - 1 {
                if arr.at(i) > arr.at(i + 1) {
                    return false;
                }
            }
            true
        }
    }
}

fn have_same_elements<T>(a: impl V1<T>, b: impl V1<T>) -> bool
where
    T: PartialOrd + Eq + Hash,
{
    match a.card([]) == b.card([]) {
        true => {
            // This is O(n^2) but performs better on smaller data sizes
            //b.iter().all(|item| a.contains(item))

            // This is O(n), performs well on larger data sizes
            let set_a: HashSet<T> = a.all().collect();
            let set_b: HashSet<T> = b.all().collect();
            set_a == set_b
        }
        false => false,
    }
}

fn sort_and_validate<T>(original: impl V1<T>, mut v: impl V1Mut<T>)
where
    T: Ord + Eq + Hash + Debug,
{
    bubble_sort(&mut v);
    assert!(is_sorted(&v) && have_same_elements(&v, &original));
}

fn main() {
    let mut descending = vec![6, 5, 4, 3, 2, 1];
    sort_and_validate(descending.clone(), &mut descending);

    let mut ascending = [1, 2, 3, 4, 5, 6];
    sort_and_validate(ascending.clone(), &mut ascending);

    let mut empty = Vec::<u64>::new();
    sort_and_validate(empty.clone(), &mut empty);

    let mut mixed = vec![3, 2, 6, 0, 1, 5, 4];
    sort_and_validate(mixed.clone(), &mut mixed);

    let mut empty = Vec::<usize>::new();
    sort_and_validate(empty.clone(), &mut empty);

    // with non-slice data types

    #[cfg(feature = "orx-split-vec")]
    {
        use orx_split_vec::SplitVec;

        let mut split_vec: SplitVec<_> = [3, 2, 6, 0, 1, 5, 4].into_iter().collect();
        assert_eq!(split_vec.fragments().len(), 2);
        assert_eq!(split_vec.fragments()[0], &[3, 2, 6, 0]);
        assert_eq!(split_vec.fragments()[1], &[1, 5, 4]);
        sort_and_validate(split_vec.clone(), &mut split_vec);
    }
}
