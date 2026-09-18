# `datafusion_common::functional_dependencies::Constraint`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.functional_dependencies.Constraint.json).

<a id="op-6a1d40180d911ee98e5b60ee"></a>
## Constraint

`enum` · `datafusion_common::functional_dependencies::Constraint` · datafusion-common 55.1.0

```rust
enum Constraint
```

Source: `src/functional_dependencies.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This object defines a constraint on a table.

<a id="op-6a369ee9e2d7d7f6044b9561"></a>
## PrimaryKey

`variant` · `datafusion_common::functional_dependencies::Constraint::PrimaryKey` · datafusion-common 55.1.0

```rust
PrimaryKey
```

Source: `src/functional_dependencies.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Columns with the given indices form a composite primary key (they are
jointly unique and not nullable):

<a id="op-e875fa892582037ad111f4b3"></a>
## Unique

`variant` · `datafusion_common::functional_dependencies::Constraint::Unique` · datafusion-common 55.1.0

```rust
Unique
```

Source: `src/functional_dependencies.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Columns with the given indices form a composite unique key:

<a id="op-5989c2249301c63ed2c5b794"></a>
## clone

`function` · `datafusion_common::functional_dependencies::Constraint::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> Constraint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraint", "path": "Constraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 17], "end": [29, 22], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/functional_dependencies.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98df542a12118e9ae93333f7"></a>
## eq

`function` · `datafusion_common::functional_dependencies::Constraint::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &Constraint) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraint", "path": "Constraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 24], "end": [29, 33], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/functional_dependencies.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa429305850d2facc8fdc823"></a>
## fmt

`function` · `datafusion_common::functional_dependencies::Constraint::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraint", "path": "Constraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 10], "end": [29, 15], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/functional_dependencies.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a090732d007b54183833b82"></a>
## hash

`function` · `datafusion_common::functional_dependencies::Constraint::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraint", "path": "Constraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 51], "end": [29, 55], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/functional_dependencies.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-751da2e9732c6818861f841a"></a>
## partial_cmp

`function` · `datafusion_common::functional_dependencies::Constraint::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &Constraint) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Constraint", "path": "Constraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 39], "end": [29, 49], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/functional_dependencies.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
