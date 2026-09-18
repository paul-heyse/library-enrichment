# `datafusion_common::types::logical::TypeParameter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.logical.TypeParameter.json).

<a id="op-8f9deaac22e330fa41d27bcd"></a>
## TypeParameter

`enum` · `datafusion_common::types::logical::TypeParameter` · datafusion-common 55.1.0

```rust
enum TypeParameter<'a>
```

Source: `src/types/logical.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddba050878295c914fc6092b"></a>
## Number

`variant` · `datafusion_common::types::logical::TypeParameter::Number` · datafusion-common 55.1.0

```rust
Number
```

Source: `src/types/logical.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-256b35abb64d2005aa6a4822"></a>
## Type

`variant` · `datafusion_common::types::logical::TypeParameter::Type` · datafusion-common 55.1.0

```rust
Type
```

Source: `src/types/logical.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba92c3812f92342a4b7b3e3b"></a>
## clone

`function` · `datafusion_common::types::logical::TypeParameter::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> TypeParameter<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_common::types::logical::TypeParameter", "path": "TypeParameter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 17], "end": [39, 22], "filename": "src/types/logical.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/types/logical.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e5a42bac47b46cb44a16af5"></a>
## cmp

`function` · `datafusion_common::types::logical::TypeParameter::cmp` · datafusion-common 55.1.0

```rust
fn cmp(&self, other: &TypeParameter<'a>) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_common::types::logical::TypeParameter", "path": "TypeParameter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 57], "end": [39, 60], "filename": "src/types/logical.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/types/logical.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24e0951554fe83c4f961d722"></a>
## eq

`function` · `datafusion_common::types::logical::TypeParameter::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &TypeParameter<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_common::types::logical::TypeParameter", "path": "TypeParameter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 24], "end": [39, 33], "filename": "src/types/logical.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/types/logical.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15ad68dff3607cdceb5dfff7"></a>
## fmt

`function` · `datafusion_common::types::logical::TypeParameter::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_common::types::logical::TypeParameter", "path": "TypeParameter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/types/logical.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types/logical.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60087d0ea7a814136ce6e519"></a>
## hash

`function` · `datafusion_common::types::logical::TypeParameter::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_common::types::logical::TypeParameter", "path": "TypeParameter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 39], "end": [39, 43], "filename": "src/types/logical.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/types/logical.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d14bca1e190b742b814172d9"></a>
## partial_cmp

`function` · `datafusion_common::types::logical::TypeParameter::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &TypeParameter<'a>) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_common::types::logical::TypeParameter", "path": "TypeParameter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 45], "end": [39, 55], "filename": "src/types/logical.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/types/logical.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
