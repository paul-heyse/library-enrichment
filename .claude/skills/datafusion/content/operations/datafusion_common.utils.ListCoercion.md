# `datafusion_common::utils::ListCoercion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.ListCoercion.json).

<a id="op-ebd835a23f8f8027faf3d07b"></a>
## ListCoercion

`enum` · `datafusion_common::utils::ListCoercion` · datafusion-common 55.1.0

```rust
enum ListCoercion
```

Source: `src/utils/mod.rs:752`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Information about how to coerce lists.

<a id="op-115463342e4fc128aa36d510"></a>
## FixedSizedListToList

`variant` · `datafusion_common::utils::ListCoercion::FixedSizedListToList` · datafusion-common 55.1.0

```rust
FixedSizedListToList
```

Source: `src/utils/mod.rs:754`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

[`DataType::FixedSizeList`](../operations/arrow_schema.datatype.DataType.md#op-69f9b11fdd4d79bf23810b3a) should be coerced to [`DataType::List`](../operations/arrow_schema.datatype.DataType.md#op-83ec578cb0e12f00905856b8).

<a id="op-362a689566bb96c47175eec1"></a>
## clone

`function` · `datafusion_common::utils::ListCoercion::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ListCoercion
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::ListCoercion", "path": "ListCoercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 17], "end": [751, 22], "filename": "src/utils/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/utils/mod.rs:751`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5f6eebefe1914e6f2afdf5d"></a>
## eq

`function` · `datafusion_common::utils::ListCoercion::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ListCoercion) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::ListCoercion", "path": "ListCoercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 24], "end": [751, 33], "filename": "src/utils/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/utils/mod.rs:751`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-575c54eb1842bdbf2a176047"></a>
## fmt

`function` · `datafusion_common::utils::ListCoercion::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::ListCoercion", "path": "ListCoercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 10], "end": [751, 15], "filename": "src/utils/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/utils/mod.rs:751`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0372bf2fb43020a064eb0b9d"></a>
## hash

`function` · `datafusion_common::utils::ListCoercion::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::ListCoercion", "path": "ListCoercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 51], "end": [751, 55], "filename": "src/utils/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/utils/mod.rs:751`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9731f81c94d100c7b4a8a35"></a>
## partial_cmp

`function` · `datafusion_common::utils::ListCoercion::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &ListCoercion) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::ListCoercion", "path": "ListCoercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 39], "end": [751, 49], "filename": "src/utils/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/utils/mod.rs:751`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
