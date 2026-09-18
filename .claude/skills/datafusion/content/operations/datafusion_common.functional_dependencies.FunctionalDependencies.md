# `datafusion_common::functional_dependencies::FunctionalDependencies`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.functional_dependencies.FunctionalDependencies.json).

<a id="op-9f9fb036c7740b17011bc94f"></a>
## FunctionalDependencies

`struct` · `datafusion_common::functional_dependencies::FunctionalDependencies` · datafusion-common 55.1.0

```rust
struct FunctionalDependencies
```

Source: `src/functional_dependencies.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This object encapsulates all functional dependencies in a given relation.

<a id="op-b0c3105e5f18a3fd773610a6"></a>
## Target

`assoc_type` · `datafusion_common::functional_dependencies::FunctionalDependencies::Target` · datafusion-common 55.1.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependencies", "path": "FunctionalDependencies"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 1], "end": [417, 2], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/functional_dependencies.rs:412`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8400126309ed4286ad46f835"></a>
## add_offset

`function` · `datafusion_common::functional_dependencies::FunctionalDependencies::add_offset` · datafusion-common 55.1.0

```rust
fn add_offset(&mut self, offset: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependencies", "path": "FunctionalDependencies"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [409, 2], "filename": "src/functional_dependencies.rs"}, "trait": null, "trait_path": null}`

Source: `src/functional_dependencies.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Adds the `offset` value to `source_indices` and `target_indices` for
each functional dependency.

<a id="op-17f6a1469860dd4f904990a8"></a>
## clone

`function` · `datafusion_common::functional_dependencies::FunctionalDependencies::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> FunctionalDependencies
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependencies", "path": "FunctionalDependencies"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 17], "end": [183, 22], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/functional_dependencies.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-125064f20842129293bda0b2"></a>
## deref

`function` · `datafusion_common::functional_dependencies::FunctionalDependencies::deref` · datafusion-common 55.1.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependencies", "path": "FunctionalDependencies"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 1], "end": [417, 2], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/functional_dependencies.rs:414`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9d622baf1288b1ab650d598"></a>
## empty

`function` · `datafusion_common::functional_dependencies::FunctionalDependencies::empty` · datafusion-common 55.1.0

```rust
fn empty() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependencies", "path": "FunctionalDependencies"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [409, 2], "filename": "src/functional_dependencies.rs"}, "trait": null, "trait_path": null}`

Source: `src/functional_dependencies.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates an empty `FunctionalDependencies` object.

<a id="op-e4781a43404873c0db3cee57"></a>
## eq

`function` · `datafusion_common::functional_dependencies::FunctionalDependencies::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &FunctionalDependencies) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependencies", "path": "FunctionalDependencies"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 24], "end": [183, 33], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/functional_dependencies.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5670eaa53038dc0f2569119c"></a>
## extend

`function` · `datafusion_common::functional_dependencies::FunctionalDependencies::extend` · datafusion-common 55.1.0

```rust
fn extend(&mut self, other: FunctionalDependencies)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependencies", "path": "FunctionalDependencies"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [409, 2], "filename": "src/functional_dependencies.rs"}, "trait": null, "trait_path": null}`

Source: `src/functional_dependencies.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Merges the given functional dependencies with these.

<a id="op-d9f9021e4f11c0859c9ec9f6"></a>
## extend_target_indices

`function` · `datafusion_common::functional_dependencies::FunctionalDependencies::extend_target_indices` · datafusion-common 55.1.0

```rust
fn extend_target_indices(&mut self, n_out: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependencies", "path": "FunctionalDependencies"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [409, 2], "filename": "src/functional_dependencies.rs"}, "trait": null, "trait_path": null}`

Source: `src/functional_dependencies.rs:395`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This function ensures that functional dependencies involving uniquely
occurring determinant keys cover their entire table in terms of
dependent columns.

<a id="op-1081eddf4dc763e71bcb4852"></a>
## fmt

`function` · `datafusion_common::functional_dependencies::FunctionalDependencies::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependencies", "path": "FunctionalDependencies"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 10], "end": [183, 15], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/functional_dependencies.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5e9af624b37fc0d22228bb4"></a>
## is_valid

`function` · `datafusion_common::functional_dependencies::FunctionalDependencies::is_valid` · datafusion-common 55.1.0

```rust
fn is_valid(&self, n_field: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependencies", "path": "FunctionalDependencies"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [409, 2], "filename": "src/functional_dependencies.rs"}, "trait": null, "trait_path": null}`

Source: `src/functional_dependencies.rs:248`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Sanity checks if functional dependencies are valid. For example, if
there are 10 fields, we cannot receive any index further than 9.

<a id="op-9483eee7389fab0ed864ec0d"></a>
## join

`function` · `datafusion_common::functional_dependencies::FunctionalDependencies::join` · datafusion-common 55.1.0

```rust
fn join(&self, other: &FunctionalDependencies, join_type: &JoinType, left_cols_len: usize) -> FunctionalDependencies
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependencies", "path": "FunctionalDependencies"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [409, 2], "filename": "src/functional_dependencies.rs"}, "trait": null, "trait_path": null}`

Source: `src/functional_dependencies.rs:333`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This function joins this set of functional dependencies with the `other`
according to the given `join_type`.

<a id="op-04691b9c9dd2baf0a7e4852d"></a>
## new

`function` · `datafusion_common::functional_dependencies::FunctionalDependencies::new` · datafusion-common 55.1.0

```rust
fn new(dependencies: Vec<FunctionalDependence>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependencies", "path": "FunctionalDependencies"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [409, 2], "filename": "src/functional_dependencies.rs"}, "trait": null, "trait_path": null}`

Source: `src/functional_dependencies.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a new `FunctionalDependencies` object from a vector of
`FunctionalDependence` objects.

<a id="op-a1d2be36b0856cfb649da977"></a>
## new_from_constraints

`function` · `datafusion_common::functional_dependencies::FunctionalDependencies::new_from_constraints` · datafusion-common 55.1.0

```rust
fn new_from_constraints(constraints: Option<&Constraints>, n_field: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependencies", "path": "FunctionalDependencies"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [409, 2], "filename": "src/functional_dependencies.rs"}, "trait": null, "trait_path": null}`

Source: `src/functional_dependencies.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a new `FunctionalDependencies` object from the given constraints.

<a id="op-44707a1972a14e52eab9e4ce"></a>
## project_functional_dependencies

`function` · `datafusion_common::functional_dependencies::FunctionalDependencies::project_functional_dependencies` · datafusion-common 55.1.0

```rust
fn project_functional_dependencies(&self, proj_indices: &[usize], n_out: usize) -> FunctionalDependencies
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependencies", "path": "FunctionalDependencies"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [409, 2], "filename": "src/functional_dependencies.rs"}, "trait": null, "trait_path": null}`

Source: `src/functional_dependencies.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Updates `source_indices` and `target_indices` of each functional
dependence using the index mapping given in `proj_indices`.

Assume that `proj_indices` is \[2, 5, 8\] and we have a functional
dependence \[5\] (`source_indices`) -> \[5, 8\] (`target_indices`).
In the updated schema, fields at indices \[2, 5, 8\] will transform
to \[0, 1, 2\]. Therefore, the resulting functional dependence will
be \[1\] -> \[1, 2\].

<a id="op-244ed7dee05b694f9036a5ea"></a>
## with_dependency

`function` · `datafusion_common::functional_dependencies::FunctionalDependencies::with_dependency` · datafusion-common 55.1.0

```rust
fn with_dependency(self, mode: Dependency) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::FunctionalDependencies", "path": "FunctionalDependencies"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [409, 2], "filename": "src/functional_dependencies.rs"}, "trait": null, "trait_path": null}`

Source: `src/functional_dependencies.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
