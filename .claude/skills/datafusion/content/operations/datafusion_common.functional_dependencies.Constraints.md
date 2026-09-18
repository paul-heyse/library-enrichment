# `datafusion_common::functional_dependencies::Constraints`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.functional_dependencies.Constraints.json).

<a id="op-0bfe130db6a7dd4e538aa60e"></a>
## Constraints

`struct` · `datafusion_common::functional_dependencies::Constraints` · datafusion-common 55.1.0

```rust
struct Constraints
```

Source: `src/functional_dependencies.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This object encapsulates a list of functional constraints:

<a id="op-a62144355a7fdb707349f958"></a>
## IntoIter

`assoc_type` · `datafusion_common::functional_dependencies::Constraints::IntoIter` · datafusion-common 55.1.0

```rust
IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraints", "path": "Constraints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [99, 2], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/functional_dependencies.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d43981b802d6b76086b4e4e"></a>
## Item

`assoc_type` · `datafusion_common::functional_dependencies::Constraints::Item` · datafusion-common 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraints", "path": "Constraints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [99, 2], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/functional_dependencies.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-573100caef1bc89f4731c5d8"></a>
## Target

`assoc_type` · `datafusion_common::functional_dependencies::Constraints::Target` · datafusion-common 55.1.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraints", "path": "Constraints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [119, 2], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/functional_dependencies.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5dbc497d7b571f42c731daae"></a>
## clone

`function` · `datafusion_common::functional_dependencies::Constraints::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> Constraints
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraints", "path": "Constraints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/functional_dependencies.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c03cf7e99875c9290d0c16b"></a>
## default

`function` · `datafusion_common::functional_dependencies::Constraints::default` · datafusion-common 55.1.0

```rust
fn default() -> Constraints
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraints", "path": "Constraints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 24], "end": [39, 31], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/functional_dependencies.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b117462620fc6297d591d601"></a>
## deref

`function` · `datafusion_common::functional_dependencies::Constraints::deref` · datafusion-common 55.1.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraints", "path": "Constraints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [119, 2], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/functional_dependencies.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d573eac169a7ad9b22b60875"></a>
## eq

`function` · `datafusion_common::functional_dependencies::Constraints::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &Constraints) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraints", "path": "Constraints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 43], "end": [39, 52], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/functional_dependencies.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de4b156b0fe22a69ee6156d6"></a>
## extend

`function` · `datafusion_common::functional_dependencies::Constraints::extend` · datafusion-common 55.1.0

```rust
fn extend(&mut self, other: Constraints)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraints", "path": "Constraints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [90, 2], "filename": "src/functional_dependencies.rs"}, "trait": null, "trait_path": null}`

Source: `src/functional_dependencies.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Extends the current constraints with the given `other` constraints.

<a id="op-5c959e27468d7556feeec6e5"></a>
## fmt

`function` · `datafusion_common::functional_dependencies::Constraints::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraints", "path": "Constraints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 17], "end": [39, 22], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/functional_dependencies.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b379b6752990ed673702ed0"></a>
## fmt

`function` · `datafusion_common::functional_dependencies::Constraints::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraints", "path": "Constraints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [111, 2], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/functional_dependencies.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7ae7303696dc718e3cb762c"></a>
## hash

`function` · `datafusion_common::functional_dependencies::Constraints::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraints", "path": "Constraints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 37], "end": [39, 41], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/functional_dependencies.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84c5a770aa4de5940299bc34"></a>
## into_iter

`function` · `datafusion_common::functional_dependencies::Constraints::into_iter` · datafusion-common 55.1.0

```rust
fn into_iter(self) -> Self::IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraints", "path": "Constraints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [99, 2], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/functional_dependencies.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae7b268a02fa682fce49e693"></a>
## new_unverified

`function` · `datafusion_common::functional_dependencies::Constraints::new_unverified` · datafusion-common 55.1.0

```rust
fn new_unverified(constraints: Vec<Constraint>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraints", "path": "Constraints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [90, 2], "filename": "src/functional_dependencies.rs"}, "trait": null, "trait_path": null}`

Source: `src/functional_dependencies.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new [`Constraints`](../operations/datafusion_common.functional_dependencies.Constraints.md#op-0bfe130db6a7dd4e538aa60e) object from the given `constraints`.
Users should use the [`Constraints::default`](../operations/datafusion_common.functional_dependencies.Constraints.md#op-0c03cf7e99875c9290d0c16b) or [`SqlToRel::new_constraint_from_table_constraints`]
functions for constructing [`Constraints`](../operations/datafusion_common.functional_dependencies.Constraints.md#op-0bfe130db6a7dd4e538aa60e) instances. This constructor
is for internal purposes only and does not check whether the argument
is valid. The user is responsible for supplying a valid vector of
[`Constraint`](../operations/datafusion_common.functional_dependencies.Constraint.md#op-6a1d40180d911ee98e5b60ee) objects.

[`SqlToRel::new_constraint_from_table_constraints`]: https://docs.rs/datafusion/latest/datafusion/sql/planner/struct.SqlToRel.html#method.new_constraint_from_table_constraints

<a id="op-f716b2e39ae0cc9f7e486db1"></a>
## partial_cmp

`function` · `datafusion_common::functional_dependencies::Constraints::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &Constraints) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraints", "path": "Constraints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 54], "end": [39, 64], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/functional_dependencies.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b1a54368b9a7f23a09e18e5"></a>
## project

`function` · `datafusion_common::functional_dependencies::Constraints::project` · datafusion-common 55.1.0

```rust
fn project(&self, proj_indices: &[usize]) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraints", "path": "Constraints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [90, 2], "filename": "src/functional_dependencies.rs"}, "trait": null, "trait_path": null}`

Source: `src/functional_dependencies.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Projects constraints using the given projection indices. Returns `None`
if any of the constraint columns are not included in the projection.
