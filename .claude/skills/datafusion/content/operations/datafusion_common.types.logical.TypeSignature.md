# `datafusion_common::types::logical::TypeSignature`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.logical.TypeSignature.json).

<a id="op-47e68e315000adddbdb25e09"></a>
## TypeSignature

`enum` · `datafusion_common::types::logical::TypeSignature` · datafusion-common 55.1.0

```rust
enum TypeSignature<'a>
```

Source: `src/types/logical.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Signature that uniquely identifies a type among other types.

<a id="op-b05dff677956e4e8c72b1f9a"></a>
## Extension

`variant` · `datafusion_common::types::logical::TypeSignature::Extension` · datafusion-common 55.1.0

```rust
Extension
```

Source: `src/types/logical.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Represents an arrow-compatible extension type.
(<https://arrow.apache.org/docs/format/Columnar.html#extension-types>)

The `name` should contain the same value as 'ARROW:extension:name'.

<a id="op-c1ecdba80b9d1441c0da9057"></a>
## Native

`variant` · `datafusion_common::types::logical::TypeSignature::Native` · datafusion-common 55.1.0

```rust
Native
```

Source: `src/types/logical.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Represents a built-in native type.

<a id="op-27bf35253c9cc6043c911418"></a>
## clone

`function` · `datafusion_common::types::logical::TypeSignature::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> TypeSignature<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_common::types::logical::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 17], "end": [25, 22], "filename": "src/types/logical.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/types/logical.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f20535474e16e2d3db344cd0"></a>
## cmp

`function` · `datafusion_common::types::logical::TypeSignature::cmp` · datafusion-common 55.1.0

```rust
fn cmp(&self, other: &TypeSignature<'a>) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_common::types::logical::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 57], "end": [25, 60], "filename": "src/types/logical.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/types/logical.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b3b25a5f746d92575f875a6"></a>
## eq

`function` · `datafusion_common::types::logical::TypeSignature::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &TypeSignature<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_common::types::logical::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 24], "end": [25, 33], "filename": "src/types/logical.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/types/logical.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b531f84d112b2c1d3f7f6d48"></a>
## fmt

`function` · `datafusion_common::types::logical::TypeSignature::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_common::types::logical::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 10], "end": [25, 15], "filename": "src/types/logical.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types/logical.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32c1ee9bdd16238d6f0b64d2"></a>
## hash

`function` · `datafusion_common::types::logical::TypeSignature::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_common::types::logical::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 39], "end": [25, 43], "filename": "src/types/logical.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/types/logical.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b359c27db9d9068e23f004f"></a>
## partial_cmp

`function` · `datafusion_common::types::logical::TypeSignature::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &TypeSignature<'a>) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_common::types::logical::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 45], "end": [25, 55], "filename": "src/types/logical.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/types/logical.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
