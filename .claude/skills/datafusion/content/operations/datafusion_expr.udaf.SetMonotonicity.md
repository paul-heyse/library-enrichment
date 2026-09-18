# `datafusion_expr::udaf::SetMonotonicity`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udaf.SetMonotonicity.json).

<a id="op-a7821c42e6182e86e59e092e"></a>
## SetMonotonicity

`enum` · `datafusion_expr::udaf::SetMonotonicity` · datafusion-expr 55.1.0

```rust
enum SetMonotonicity
```

Source: `src/udaf.rs:1409`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Indicates whether an aggregation function is monotonic as a set
function. A set function is monotonically increasing if its value
increases as its argument grows (as a set). Formally, `f` is a
monotonically increasing set function if `f(S) >= f(T)` whenever `S`
is a superset of `T`.

For example `COUNT` and `MAX` are monotonically increasing as their
values always increase (or stay the same) as new values are seen. On
the other hand, `MIN` is monotonically decreasing as its value always
decreases or stays the same as new values are seen.

<a id="op-19bd704a0bec4654c006c466"></a>
## Decreasing

`variant` · `datafusion_expr::udaf::SetMonotonicity::Decreasing` · datafusion-expr 55.1.0

```rust
Decreasing
```

Source: `src/udaf.rs:1413`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Aggregate value decreases or stays the same as the input set grows.

<a id="op-3f7489e4eb88319b62a8104b"></a>
## Increasing

`variant` · `datafusion_expr::udaf::SetMonotonicity::Increasing` · datafusion-expr 55.1.0

```rust
Increasing
```

Source: `src/udaf.rs:1411`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Aggregate value increases or stays the same as the input set grows.

<a id="op-a1416af06a012972c0bb5220"></a>
## NotMonotonic

`variant` · `datafusion_expr::udaf::SetMonotonicity::NotMonotonic` · datafusion-expr 55.1.0

```rust
NotMonotonic
```

Source: `src/udaf.rs:1416`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Aggregate value may increase, decrease, or stay the same as the input
set grows.

<a id="op-dd5b8ba569ba79a78f77c6e5"></a>
## clone

`function` · `datafusion_expr::udaf::SetMonotonicity::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> SetMonotonicity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::SetMonotonicity", "path": "SetMonotonicity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1408, 17], "end": [1408, 22], "filename": "src/udaf.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/udaf.rs:1408`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0521e02e7f079528810ab26"></a>
## eq

`function` · `datafusion_expr::udaf::SetMonotonicity::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &SetMonotonicity) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::SetMonotonicity", "path": "SetMonotonicity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1408, 24], "end": [1408, 33], "filename": "src/udaf.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/udaf.rs:1408`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a3c0253ac78d04ef47c116f"></a>
## fmt

`function` · `datafusion_expr::udaf::SetMonotonicity::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::SetMonotonicity", "path": "SetMonotonicity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1408, 10], "end": [1408, 15], "filename": "src/udaf.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udaf.rs:1408`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
