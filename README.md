# orx-v

Vector traits to unify all vectors!

The focus of this crate is computation and algorithms. The goal is to allow algorithm developer to provide a single generic algorithm implementation and that can be called by many polymorphic vector types with a corresponding practical use case.

You may find an article discussing the motivation [here](https://orxfun.github.io/orxfun-notes/#/v-4-vectors-2024-11-18) and various examples in the [examples](https://github.com/orxfun/orx-v/tree/main/examples) folder of the repository.

## Traits

Two vector traits are defined:

### Immutable Vectors: NVec<D, T>

`NVec<D, T>` is a `D` dimensional vector where the scalar elements are of type `T`.

It has a few required methods and more provided methods and functionalities that tend to grow. However, it has the following two core methods defining the common behavior.

#### Random Access

The first shared functionality is defined by the `at` method which is analogous to the index operator of a Vec, however, extended to higher dimensions.

```rust ignore
fn at(&self, idx: impl IntoIdx<D>) -> T;
```

Below are some examples for `D1` and `D2` vectors.

```rust
use orx_v::*;

let v1 = vec![1, 7, 42];
assert_eq!(v1.at(2), 42);

let v1 = V.d1().constant(42);
assert_eq!(v1.at([121]), 42);

let v1 = V.d1().fun(|[i]| 2 * i + 7);
assert_eq!(v1.at([10]), 27);

let v2 = vec![vec![1, 2], vec![3, 4, 5]];
assert_eq!(v2.at([0, 1]), 2);
assert_eq!(v2.at([1, 2]), 5);

let mut v2 = V.d2().sparse(1000);
v2.set([2, 1], 21);
assert_eq!(v2.at([0, 1]), 1000);
assert_eq!(v2.at([2, 1]), 21);
assert_eq!(v2.at([1, 0]), 1000);
```

The examples above also hints the goal that various useful concrete types already implement the vector traits, such as:
* the standard vector, arrays, slices;
* ndarray arrays such as Array1, Array2, etc.;
* sparse vectors;
* caching or memoizing vectors;
* functional vectors, and so on.

Further, due to the abstraction through traits, we can have composed definitions. For instance
* Both `Vec<T>` and `SparseVec<D1, T>` implements `NVec<D1, T>`.
* Then, both `Vec<Vec<T>>` and `Vec<SparseVec<D1, T>>` implement `NVec<D2, T>`.
* Actually, any `Vec<V>` where `V: NVec<D1, T>` implements `NVec<D2, T>`.

#### Cardinality

The second shared functionality is the knowledge of the vectors and all of its children's cardinality all the way down to the scalars. This is provided by the `card` method that is analogous to `len` method extended to provide complete cardinality information rather than the number of immediate children.

```rust ignore
fn card(&self, idx: impl Into<D::CardIdx>) -> usize;
```

Below, you may see examples for `D1` vectors. Notice that some vectors are naturally unbounded, such as a constant vector, a functional vector or a sparse vector. The examples also illustrates how to defined their bounds.

```rust
use orx_v::*;

let v1 = vec![2, 3, 4];
assert_eq!(v1.card([]), 3);

// unbounded vectors

let sparse_elements: DefaultLookup<D1, _> = [([1], 42), ([7], 12)].into_iter().collect();
let v1 = V.d1().sparse_from(sparse_elements, 0);

assert_eq!(v1.at(7), 12);
assert_eq!(v1.at(123456), 0); // never out of bounds

assert_eq!(v1.card([]), usize::MAX);
assert!(!v1.is_bounded());

// add bound to unbounded vectors
let v1 = v1.bounded(10);

assert_eq!(v1.card([]), 10);
assert!(v1.is_bounded());

assert_eq!(v1.try_at(123456), None);
```

The following `D2` examples illustrate how `card` method can be used for all lower dimensions.

```rust
use orx_v::*;

let v2 = vec![
    vec![1, 2],
    vec![],
    vec![3, 4, 5],
];

assert_eq!(v2.card([]), 3);

assert_eq!(v2.card([1]), 0);
assert_eq!(v2.card([2]), 3);
// this is equivalent to
assert_eq!(v2.child(2).card([]), 3);

// unbounded vectors
let v2 = V.d2().fun(|[i, j]| 2 * i + j);

assert_eq!(v2.card([]), usize::MAX);
assert!(!v2.is_bounded());

assert_eq!(v2.card([100]), usize::MAX);
assert!(!v2.child(100).is_bounded());

// add rectangular bounds as in matrices
let v2 = V
    .d2()
    .fun(|[i, j]| 2 * i + j)
    .with_rectangular_bounds([2, 3]);

assert_eq!(v2.card([]), 2);
assert_eq!(v2.card([0]), 3);
assert_eq!(v2.card([1]), 3);

// add variable bounds as in jagged arrays
let num_elements = vec![2, 0, 3]; // any NVec<D1, usize>
let v2 = V
    .d2()
    .fun(|[i, j]| 2 * i + j)
    .with_variable_bounds(&num_elements);
assert_eq!(v2.card([]), 3);
assert_eq!(v2.card([0]), 2);
assert_eq!(v2.card([1]), 0);
assert_eq!(v2.card([2]), 3);
```

You may again notice the benefit of the abstraction in the `num_elements` definition above. In this example, we used a Vec but actually we could use any `NVec<D1, usize>` implementation. For instance, assume we have a `D2` vector where each row has 1000 elements while the last element has 1. We can easily represent this with a functional vector or a sparse vector and avoid allocating the entire vector of number of elements.

### Mutable Vectors: NVecMut<D, T>

As expected, `NVecMut<D, T>: NVec<D, T>`. Again the provided methods of this trait tend to grow; however, its core ability is defined by the `at_mut` method.

```rust ignore
fn at_mut<Idx: IntoIdx<D>>(&mut self, idx: Idx) -> &mut T;
```

And below examples show the use case together with the `set` method.

```rust
use orx_v::*;

let mut v1 = vec![1, 7, 42];
*v1.at_mut(1) = 7;
v1.set(2, 21); // if you will

let mut v2 = vec![vec![1, 2], vec![3, 4, 5]];
*v2.at_mut([0, 1]) = 21;
v2.set([1, 2], 7);

let mut v2 = V.d2().sparse(1000);
*v2.at_mut([1, 2]) = 12;
v2.set([2, 1], 21);
```

## Trait Aliases

The following trait aliases can be used instead.

```rust ignore
V1<T>     <====>   NVec<D1, T>
V1Mut<T>  <====>   NVecMut<D1, T>

V2<T>     <====>   NVec<D2, T>
V2Mut<T>  <====>   NVecMut<D2, T>

...
```

## V for Vectors!

You might have noticed in the above examples the use of `V`. This is basically the entry point of builders of different types of multi dimensional vectors. It is followed by the dimension of the vector to be created, such as `V.d1()` or `V.d3()`. Next we can call methods to create special vectors such as:
* `ConstVec` => `V.d1().const(42)`
* `EmptyVec` => `V.d3().empty::<i32>()`
* `SparseVec` => `V.d2().sparse(1000)`;
* `FunVec` => `V.d2().fun(|[i, j]| euclidean(&locations[i], &locations[j]))`
* `CachedVec` => `V.d2().fun(|[i, j]| euclidean(&locations[i], &locations[j])).into_cached()`

## Practical Example

To have a practical example, let's assume that our algorithm is the [two-opt](https://en.wikipedia.org/wiki/2-opt) which is a local search algorithm to solve the traveling salesperson problem. The algorithm takes a tour and keeps modifying it until its distance can no longer be reduced within the two-opt neighborhood. We can have our generic implementation as follows.

```rust
use orx_v::*;

fn apply_two_opt(tour: &mut impl V1Mut<usize>, i: usize, j: usize) {
    let mut i = i + 1;
    let mut j = j;
    while i < j {
        let t = tour.at(i);
        *tour.at_mut(i) = tour.at(j);
        *tour.at_mut(j) = t;
        i += 1;
        j -= 1;
    }
}

fn two_opt(distances: impl V2<u32>, tour: &mut impl V1Mut<usize>) -> u32 {
    let mut improvement = 0;
    let d = distances;
    let n = tour.card([]);

    let mut improved = true;
    while improved {
        improved = false;

        for i in 0..(n - 1) {
            let i1 = tour.at(i);
            let i2 = tour.at(i + 1);

            for j in (i + 2)..n {
                let j1 = tour.at(j);
                let j2 = tour.at((j + 1) % n);

                let removed_len = d.at([i1, i2]) + d.at([j1, j2]);
                let added_len = d.at([i1, j1]) + d.at([i2, j2]);

                if removed_len > added_len {
                    improved = true;
                    improvement += removed_len - added_len;
                    apply_two_opt(tour, i, j);
                }
            }
        }
    }

    improvement
}
```

Notice that this implementation resembles the implementation where we would use `&mut [usize]` for a tour and `Vec<Vec<u32>>` for a distance matrix. However, we can call this method with many more input types that make sense in different situations.

```rust ignore
let n = 100;
let mut tour: Vec<_> = initial_tour(n);

// complete matrix stored as a V2
{
    // Vec<Vec<u32>>
    let distances: Vec<Vec<u32>> = complete_distance_matrix_d2(n);
    let _improvement = two_opt(&distances, &mut tour);

    // ndarray::Array2
    let distances: Array2<u32> = complete_ndarray_d2(n);
    let _improvement = two_opt(&distances, &mut tour);
}

// complete matrix stored as a flattened V1
{
    // Vec<u32> as flattened matrix
    let distances: Vec<u32> = complete_distance_matrix_d1(n);
    let _improvement = two_opt(distances.as_jagged_with_uniform_lengths(n), &mut tour);

    // ndarray::Array1 as flattened matrix
    let distances: Array1<u32> = complete_ndarray_d1(n);
    let _improvement = two_opt(distances.as_jagged_with_uniform_lengths(n), &mut tour);
}

// sparse matrix
let finite_distances: HashMap<[usize; 2], u32> = finite_distances_map(n);
let distances = V.d2().sparse_from(finite_distances, 10000);
let _improvement = two_opt(&distances, &mut tour);

// functional matrix
let locations: Vec<Location> = get_locations(n);
let distances = V
    .d2()
    .fun(|[i, j]| euclidean_distance(&locations[i], &locations[j]));
let _improvement = two_opt(&distances, &mut tour);

// functional matrix: ignore from-to depot (node 0) links
let locations: Vec<Location> = get_locations(n);
let distances = V.d2().fun(|[i, j]| match (i, j) {
    (0, _) => 0,
    (_, 0) => 0,
    _ => euclidean_distance(&locations[i], &locations[j]),
});
let _improvement = two_opt(&distances, &mut tour);

// cached matrix
let locations: Vec<Location> = get_locations(n);
let distances = V
    .d2()
    .fun(|[i, j]| routing_engine(&locations[i], &locations[j]))
    .into_cached();
let _improvement = two_opt(&distances, &mut tour);

// uniform distances
let distances = V.d2().constant(10);
let _improvement = two_opt(&distances, &mut tour);
```

## Matrices

`Matrix<T>` and `MatrixMut<T>` traits are also defined to allow for polymorphic matrix types. Their interface is quite similar to those of `V2<T>` and `V2Mut<T>` except that they require rectangular bounds.

They can be crated by calling `as_matrix` or `v1_as_matrix` methods on one-dimensional (flattened matrix) and two-dimensional vectors.

## Features

Vector trait implementations for vectors in well known external libraries are being included in this crate via features. For instance, you may add "ndarray" feature to be able to use "ndarray::Vector1" as a "V1", or "Vector2" as a "V2", etc.

## Contributing

Contributions are welcome! Feedback based on experiences are appreciated

If you notice an error, have a question or think something could be improved, or think certain data types must also implement the vector traits, please open an [issue](https://github.com/orxfun/orx-imp-vec/issues/new) or create a PR.

## License

This library is licensed under MIT license. See LICENSE for details.
