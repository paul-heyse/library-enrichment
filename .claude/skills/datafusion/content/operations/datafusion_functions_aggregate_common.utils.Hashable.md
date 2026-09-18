# `datafusion_functions_aggregate_common::utils::Hashable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.utils.Hashable.json).

<a id="op-a2e7368c7b771a9bd9a3a921"></a>
## Hashable

`struct` · `datafusion_functions_aggregate_common::utils::Hashable` · datafusion-functions-aggregate-common 55.1.0

```rust
struct Hashable<T>
```

Source: `src/utils.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

A wrapper around a type to provide hash for floats

<a id="op-daf64fca73d1bdf8b187986d"></a>
## 0

`struct_field` · `datafusion_functions_aggregate_common::utils::Hashable::0` · datafusion-functions-aggregate-common 55.1.0

```rust
0: T
```

Source: `src/utils.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e934e98ec38777531175ef06"></a>
## clone

`function` · `datafusion_functions_aggregate_common::utils::Hashable::clone` · datafusion-functions-aggregate-common 55.1.0

```rust
fn clone(&self) -> Hashable<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::utils::Hashable", "path": "Hashable"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 16], "end": [74, 21], "filename": "src/utils.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/utils.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d99d0f1e22713a0f604a5b10"></a>
## eq

`function` · `datafusion_functions_aggregate_common::utils::Hashable::eq` · datafusion-functions-aggregate-common 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::utils::Hashable", "path": "Hashable"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::arithmetic::ArrowNativeTypeOp", "path": "ArrowNativeTypeOp"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [87, 2], "filename": "src/utils.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/utils.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14e2503ef21450dd58fe24d6"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::utils::Hashable::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::utils::Hashable", "path": "Hashable"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 23], "end": [74, 28], "filename": "src/utils.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/utils.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f87962cc0f433eb20068164"></a>
## hash

`function` · `datafusion_functions_aggregate_common::utils::Hashable::hash` · datafusion-functions-aggregate-common 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::utils::Hashable", "path": "Hashable"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ToByteSlice", "path": "ToByteSlice"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [81, 2], "filename": "src/utils.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/utils.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
