# `datafusion_functions_aggregate_common::tdigest::Centroid`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.tdigest.Centroid.json).

<a id="op-a41984a513b5d982d464b95b"></a>
## Centroid

`struct` · `datafusion_functions_aggregate_common::tdigest::Centroid` · datafusion-functions-aggregate-common 55.1.0

```rust
struct Centroid
```

Source: `src/tdigest.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Centroid implementation to the cluster mentioned in the paper.

<a id="op-0925580edf5799e29775337f"></a>
## add

`function` · `datafusion_functions_aggregate_common::tdigest::Centroid::add` · datafusion-functions-aggregate-common 55.1.0

```rust
fn add(&mut self, sum: f64, weight: f64) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::Centroid", "path": "Centroid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [85, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfa31c23834d2929b15f2b4c"></a>
## clone

`function` · `datafusion_functions_aggregate_common::tdigest::Centroid::clone` · datafusion-functions-aggregate-common 55.1.0

```rust
fn clone(&self) -> Centroid
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::Centroid", "path": "Centroid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 28], "end": [53, 33], "filename": "src/tdigest.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/tdigest.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c017acbd089fae23c18a513"></a>
## cmp_mean

`function` · `datafusion_functions_aggregate_common::tdigest::Centroid::cmp_mean` · datafusion-functions-aggregate-common 55.1.0

```rust
fn cmp_mean(&self, other: &Self) -> Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::Centroid", "path": "Centroid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [85, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-141159baa9a4ddb3c921c830"></a>
## default

`function` · `datafusion_functions_aggregate_common::tdigest::Centroid::default` · datafusion-functions-aggregate-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::Centroid", "path": "Centroid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [94, 2], "filename": "src/tdigest.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/tdigest.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0fa22ee339a63b037e100e8"></a>
## eq

`function` · `datafusion_functions_aggregate_common::tdigest::Centroid::eq` · datafusion-functions-aggregate-common 55.1.0

```rust
fn eq(&self, other: &Centroid) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::Centroid", "path": "Centroid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 17], "end": [53, 26], "filename": "src/tdigest.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/tdigest.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6240f8716c515e2fb4d88f7"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::tdigest::Centroid::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::Centroid", "path": "Centroid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 10], "end": [53, 15], "filename": "src/tdigest.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tdigest.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fa4c08a74ddf29f37094476"></a>
## mean

`function` · `datafusion_functions_aggregate_common::tdigest::Centroid::mean` · datafusion-functions-aggregate-common 55.1.0

```rust
fn mean(&self) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::Centroid", "path": "Centroid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [85, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b4d2641aceb3a2afb9ded45"></a>
## new

`function` · `datafusion_functions_aggregate_common::tdigest::Centroid::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new(mean: f64, weight: f64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::Centroid", "path": "Centroid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [85, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-669d02d94d37fd92cd9b7793"></a>
## weight

`function` · `datafusion_functions_aggregate_common::tdigest::Centroid::weight` · datafusion-functions-aggregate-common 55.1.0

```rust
fn weight(&self) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::tdigest::Centroid", "path": "Centroid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [85, 2], "filename": "src/tdigest.rs"}, "trait": null, "trait_path": null}`

Source: `src/tdigest.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
