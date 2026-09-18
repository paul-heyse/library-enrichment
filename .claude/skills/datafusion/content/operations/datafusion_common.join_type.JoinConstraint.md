# `datafusion_common::join_type::JoinConstraint`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.join_type.JoinConstraint.json).

<a id="op-832121e2e5fb95d38378f520"></a>
## JoinConstraint

`enum` · `datafusion_common::join_type::JoinConstraint` · datafusion-common 55.1.0

```rust
enum JoinConstraint
```

Source: `src/join_type.rs:220`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Join constraint

<a id="op-7e0b7cdd6599be377ce8b17b"></a>
## On

`variant` · `datafusion_common::join_type::JoinConstraint::On` · datafusion-common 55.1.0

```rust
On
```

Source: `src/join_type.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Join ON

<a id="op-f58495b2920f2801ec8507ca"></a>
## Using

`variant` · `datafusion_common::join_type::JoinConstraint::Using` · datafusion-common 55.1.0

```rust
Using
```

Source: `src/join_type.rs:224`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Join USING

<a id="op-ad6085bcb986949ea2ae5c13"></a>
## clone

`function` · `datafusion_common::join_type::JoinConstraint::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> JoinConstraint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinConstraint", "path": "JoinConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 17], "end": [219, 22], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/join_type.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a58e7165b00f9ddcb02fefb"></a>
## eq

`function` · `datafusion_common::join_type::JoinConstraint::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &JoinConstraint) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinConstraint", "path": "JoinConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 30], "end": [219, 39], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/join_type.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33f066e5c6227e29fdb1db7e"></a>
## fmt

`function` · `datafusion_common::join_type::JoinConstraint::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinConstraint", "path": "JoinConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 10], "end": [219, 15], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/join_type.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-daf1de2777fb8205e8c968c4"></a>
## hash

`function` · `datafusion_common::join_type::JoinConstraint::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinConstraint", "path": "JoinConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 57], "end": [219, 61], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/join_type.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8937b2a0789f5119a660cc99"></a>
## partial_cmp

`function` · `datafusion_common::join_type::JoinConstraint::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &JoinConstraint) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinConstraint", "path": "JoinConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 45], "end": [219, 55], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/join_type.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
