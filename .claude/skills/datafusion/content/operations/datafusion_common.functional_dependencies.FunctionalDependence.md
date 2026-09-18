# `datafusion_common::functional_dependencies::FunctionalDependence`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.functional_dependencies.FunctionalDependence.json).

<a id="op-0febe70ca7a2be0a30b41bd9"></a>
## FunctionalDependence

`struct` · `datafusion_common::functional_dependencies::FunctionalDependence` · datafusion-common 55.1.0

```rust
struct FunctionalDependence
```

Source: `src/functional_dependencies.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This object defines a functional dependence in the schema. A functional
dependence defines a relationship between determinant keys and dependent
columns. A determinant key is a column, or a set of columns, whose value
uniquely determines values of some other (dependent) columns. If two rows
have the same determinant key, dependent columns in these rows are
necessarily the same. If the determinant key is unique, the set of
dependent columns is equal to the entire schema and the determinant key can
serve as a primary key. Note that a primary key may "downgrade" into a
determinant key due to an operation such as a join, and this object is
used to track dependence relationships in such cases. For more information
on functional dependencies, see:
<https://www.scaler.com/topics/dbms/functional-dependency-in-dbms/>

<a id="op-be151451729fd49049b6f88f"></a>
## clone

`function` · `datafusion_common::functional_dependencies::FunctionalDependence::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> FunctionalDependence
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependence", "path": "FunctionalDependence"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 17], "end": [133, 22], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/functional_dependencies.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0963cd07fef2d60562bca93"></a>
## eq

`function` · `datafusion_common::functional_dependencies::FunctionalDependence::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &FunctionalDependence) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependence", "path": "FunctionalDependence"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 24], "end": [133, 33], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/functional_dependencies.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acc7ce06c1241c6cb7607b02"></a>
## fmt

`function` · `datafusion_common::functional_dependencies::FunctionalDependence::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependence", "path": "FunctionalDependence"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 10], "end": [133, 15], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/functional_dependencies.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a28a565d320b49ff904ae2b"></a>
## mode

`struct_field` · `datafusion_common::functional_dependencies::FunctionalDependence::mode` · datafusion-common 55.1.0

```rust
mode: Dependency
```

Source: `src/functional_dependencies.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c01aa0c6cb9825400d9218b"></a>
## new

`function` · `datafusion_common::functional_dependencies::FunctionalDependence::new` · datafusion-common 55.1.0

```rust
fn new(source_indices: Vec<usize>, target_indices: Vec<usize>, nullable: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependence", "path": "FunctionalDependence"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [180, 2], "filename": "src/functional_dependencies.rs"}, "trait": null, "trait_path": null}`

Source: `src/functional_dependencies.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7d9d8f16d0eb63c151c358f"></a>
## nullable

`struct_field` · `datafusion_common::functional_dependencies::FunctionalDependence::nullable` · datafusion-common 55.1.0

```rust
nullable: bool
```

Source: `src/functional_dependencies.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Flag indicating whether one of the `source_indices` can receive NULL values.
For a data source, if the constraint in question is `Constraint::Unique`,
this flag is `true`. If the constraint in question is `Constraint::PrimaryKey`,
this flag is `false`.
Note that as the schema changes between different stages in a plan,
such as after LEFT JOIN or RIGHT JOIN operations, this property may
change.

<a id="op-40ed09da36564af081ca2780"></a>
## source_indices

`struct_field` · `datafusion_common::functional_dependencies::FunctionalDependence::source_indices` · datafusion-common 55.1.0

```rust
source_indices: Vec<usize>
```

Source: `src/functional_dependencies.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-515b5f35ad141233e2c702be"></a>
## target_indices

`struct_field` · `datafusion_common::functional_dependencies::FunctionalDependence::target_indices` · datafusion-common 55.1.0

```rust
target_indices: Vec<usize>
```

Source: `src/functional_dependencies.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0420fe01b7d672af7cc6d5d3"></a>
## with_mode

`function` · `datafusion_common::functional_dependencies::FunctionalDependence::with_mode` · datafusion-common 55.1.0

```rust
fn with_mode(self, mode: Dependency) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependence", "path": "FunctionalDependence"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [180, 2], "filename": "src/functional_dependencies.rs"}, "trait": null, "trait_path": null}`

Source: `src/functional_dependencies.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
