# orx-v

[![orx-v crate](https://img.shields.io/crates/v/orx-v.svg)](https://crates.io/crates/orx-v)
[![orx-v documentation](https://docs.rs/orx-v/badge.svg)](https://docs.rs/orx-v)

Traits to unify all vectors!

The focus of this crate is mainly computation and algorithms. The goal is to allow for a generic algorithm implementation that can be called by many polymorphic vector types having a corresponding practical use case.

You may find an article discussing the motivation [here](https://orxfun.github.io/orxfun-notes/#/v-for-vectors-2024-11-18) and various examples in the [examples](https://github.com/orxfun/orx-v/tree/main/examples) folder of the repository.

## Traits

Two vector traits are defined: one for immutable vectors and the other one extending it to mutable vectors.

### Immutable Vectors: NVec<D, T>

[**NVec<D, T>**](https://docs.rs/orx-v/latest/orx_v/trait.NVec.html) is a `D` dimensional vector where the scalar elements are of type `T`.

At its core, the following define the vectors' shared behavior:
* `at`: efficient random access by indices
* `card`: complete knowledge of its size
* `all`: efficient serial access over all scalar elements

#### Random Access

The first shared functionality is defined by the [`at`](https://docs.rs/orx-v/latest/orx_v/trait.NVec.html#tymethod.at) method. It is analogous to the index operator of a Vec, however, extended to higher dimensions.

```rust ignore
fn at(&self, idx: impl IntoIdx<D>) -> T;
```

Below are some examples for `D1` and `D2` vectors.

```rust
use orx_v::*;

let v1 = vec![1, 7, 42];
assert_eq!(v1.at(2), 42);

let v1 = V.d1().constant(42);
assert_eq!(v1.at(121), 42);

let v1 = V.d1().fun(|[i]| 2 * i + 7);
assert_eq!(v1.at(10), 27);

let v2 = vec![vec![1, 2], vec![3, 4, 5]];
assert_eq!(v2.at([0, 1]), 2);
assert_eq!(v2.at([1, 2]), 5);

let mut v2 = V.d2().sparse(1000);
v2.set([2, 1], 21);
assert_eq!(v2.at([0, 1]), 1000);
assert_eq!(v2.at([2, 1]), 21);
assert_eq!(v2.at([1, 0]), 1000);
```

As the examples reveal, various useful concrete types already implement the vector traits, such as:
* the standard vector, arrays, slices;
* ndarray arrays such as Array1, Array2, etc.;
* sparse vectors;
* caching or memoizing vectors;
* functional vectors, and so on.

Further, due to the abstraction through traits, we can have composed definitions. For instance
* `Vec<T>` and `SparseVec<D1, T>` both implement `NVec<D1, T>`.
* Then,  `Vec<Vec<T>>` and `Vec<SparseVec<D1, T>>` both implement `NVec<D2, T>`.
* Actually, any `Vec<V>` where `V: NVec<D1, T>` automatically implements `NVec<D2, T>`.

#### Cardinality

The second shared functionality is the knowledge of the vector's and all of its children's cardinality, all the way down to scalars. This is provided by the [`card`](https://docs.rs/orx-v/latest/orx_v/trait.NVec.html#method.card) method. It is analogous to `len` method of common collections; however, extended to provide complete cardinality information rather than only the number of immediate children.

```rust ignore
fn card(&self, idx: impl Into<D::CardIdx>) -> usize;
```

Below, you may see examples for `D1` vectors. Notice that some vectors are naturally unbounded, such as a constant vector, a functional vector or a sparse vector. The examples also illustrate how to define their bounds whenever needed.

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

The `num_elements` definition also demonstrates the benefit of abstraction. In this example, we used a Vec, but we could've used any `NVec<D1, usize>` implementation. For instance, assume we have a `D2` vector where each row has 1000 elements while the last element has 1. We can easily represent this with a functional vector or a sparse vector and avoid allocating the entire vector of number of elements.

#### Sequential Access

Not to be confused with `iter()` method on collections, [`all`](https://docs.rs/orx-v/latest/orx_v/trait.NVec.html#tymethod.all) method yields the inner-most scalar elements.

In order to iterate over the immediate children, `children` method can be used instead.

```rust
use orx_v::*;

let vec = vec![vec![0, 1], vec![], vec![2]]; // V2

// inner-most elements, scalars
let mut all = vec.all();
assert_eq!(all.next(), Some(0));
assert_eq!(all.next(), Some(1));
assert_eq!(all.next(), Some(2));
assert_eq!(all.next(), None);

// immediate children belonging to previous dimension (D1 here)
let mut children = vec.children();
assert_eq!(children.next().unwrap().equality(&[0, 1]), Equality::Equal);
assert_eq!(children.next().unwrap().equality(&[]), Equality::Equal);
assert_eq!(children.next().unwrap().equality(&[2]), Equality::Equal);
assert!(children.next().is_none());
```


### Mutable Vectors: NVecMut<D, T>

As expected, [**NVecMut**](https://docs.rs/orx-v/latest/orx_v/trait.NVecMut.html) extends `NVec`; i.e., `NVecMut<D, T>: NVec<D, T>`.

Its core functionality is defined by the [`at_mut`](https://docs.rs/orx-v/latest/orx_v/trait.NVecMut.html#tymethod.at_mut) method.

```rust ignore
fn at_mut<Idx: IntoIdx<D>>(&mut self, idx: Idx) -> &mut T;
```

```rust
use orx_v::*;

let mut v1 = vec![1, 7, 42];
*v1.at_mut(1) = 7;
v1.set(2, 21); // if you prefer

let mut v2 = vec![vec![1, 2], vec![3, 4, 5]];
*v2.at_mut([0, 1]) = 21;
v2.set([1, 2], 7);

let mut v2 = V.d2().sparse(1000);
*v2.at_mut([1, 2]) = 12;
v2.set([2, 1], 21);
```

## Trait Aliases

The following trait aliases can be used instead, to fix the first generic type parameter on dimension.

```rust ignore
V1<T>     <====>   NVec<D1, T>
V1Mut<T>  <====>   NVecMut<D1, T>

V2<T>     <====>   NVec<D2, T>
V2Mut<T>  <====>   NVecMut<D2, T>

...
```

## V for Vectors!

[**V**](https://docs.rs/orx-v/latest/orx_v/struct.V.html) is basically the entry point of builders for various vector types of multi dimensional vectors. It is followed by the dimension of the vector to be created, such as `V.d1()` or `V.d3()`. Next we can call methods to create special vectors such as:
* [**ConstantVec**](https://docs.rs/orx-v/latest/orx_v/struct.ConstantVec.html)
  * `V.d1().const(42)`
  * a vector that yields only 42 for all indices
* [**EmptyVec**](https://docs.rs/orx-v/latest/orx_v/struct.EmptyVec.html)
  * `V.d3().empty::<i32>()`
  * a vector with no elements, zero cardinality
* [**SparseVec**](https://docs.rs/orx-v/latest/orx_v/struct.SparseVec.html)
  * `V.d2().sparse(1000)`
  * a sparse vector where all elements which are not explicitly set are equal to 1000
* [**FunVec**](https://docs.rs/orx-v/latest/orx_v/struct.FunVec.html)
  * `V.d2().fun(|[i, j]| euclidean(&locations[i], &locations[j]))`
  * a lazy vector which computes elements on the fly as requested
* [**CachedVec**](https://docs.rs/orx-v/latest/orx_v/struct.CachedVec.html)
  * `V.d2().fun(|[i, j]| euclidean(&locations[i], &locations[j])).into_cached()`
  * also a lazy vector vector; however, it caches or memoizes computed elements

## Practical Example

To demonstrate when and why these traits might be useful, let's assume that we are implementing the [two-opt](https://en.wikipedia.org/wiki/2-opt) which is a local search algorithm to solve the traveling salesperson problem. The algorithm takes a tour and keeps modifying it until its distance can no longer be reduced within the two-opt neighborhood. We can have our generic implementation as follows.

```rust
use orx_v::*;

fn apply_two_opt(mut tour: impl V1Mut<usize>, i: usize, j: usize) {
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

fn two_opt(distances: impl V2<u32>, mut tour: impl V1Mut<usize>) -> u32 {
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
                    apply_two_opt(&mut tour, i, j);
                }
            }
        }
    }

    improvement
}
```

This implementation is not much different than the implementation where we would use `Vec<usize>` for a tour and `Vec<Vec<u32>>` for a distance matrix.

However, it is much different in the caller side.

We can call this algorithm with a wide range of input types that make sense in different situations.

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

In addition to vector traits, specialized [**Matrix&lt;T&gt;**](https://docs.rs/orx-v/latest/orx_v/trait.Matrix.html) and [**MatrixMut&lt;T&gt;**](https://docs.rs/orx-v/latest/orx_v/trait.MatrixMut.html) traits are also defined to allow for polymorphic matrix types.

Their interface is naturally very similar to those of `V2<T>` and `V2Mut<T>` except that they require rectangular bounds.

Any `V2` vector with rectangular cardinality can be converted into or viewed as a row-major or column-major matrix by calling `into_matrix` or `as_matrix` methods of the [**V2AsMatrix**](https://docs.rs/orx-v/latest/orx_v/trait.V2AsMatrix.html) trait.

Further, any `V1` vector can be transformed or viewed as a flattened matrix by calling `v1_into_matrix` or `v1_as_matrix` methods of the [**V1AsMatrix**](https://docs.rs/orx-v/latest/orx_v/trait.V1AsMatrix.html) trait.

## Features

Vector trait implementations for vectors in well known external libraries are being included in this crate via features. For instance, you may add "ndarray" feature to be able to use "ndarray::Vector1" as a "V1", or "Vector2" as a "V2", etc.

std is enabled as the default feature, please set "default-features=false" when working in **no-std** programs.

## Contributing

Contributions, ideas and feedback are welcome!

If you notice an error, have a question or think something could be improved, or think certain data types must also implement the vector traits, please open an [issue](https://github.com/orxfun/orx-imp-vec/issues/new) or create a PR.

## License

This library is licensed under MIT license. See LICENSE for details.
