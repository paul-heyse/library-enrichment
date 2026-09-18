# `deltalake_catalog_unity::models::DeltaRuntimeProperties`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.models.DeltaRuntimeProperties.json).

<a id="op-8b85073665ebaea1258e6326"></a>
## DeltaRuntimeProperties

`struct` · `deltalake_catalog_unity::models::DeltaRuntimeProperties` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaRuntimeProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L429).

Source: `crates/catalog-unity/src/models.rs:429`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f142031b8c971f160597648"></a>
## default

`function` · `deltalake_catalog_unity::models::DeltaRuntimeProperties::default` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> DeltaRuntimeProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L427).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::DeltaRuntimeProperties", "path": "DeltaRuntimeProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [427, 23], "end": [427, 30], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/catalog-unity/src/models.rs:427`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-242f7cbf81fed95004a8d0a2"></a>
## delta_runtime_properties

`struct_field` · `deltalake_catalog_unity::models::DeltaRuntimeProperties::delta_runtime_properties` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
delta_runtime_properties: std::collections::HashMap<String, String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L430).

Source: `crates/catalog-unity/src/models.rs:430`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c56daf53a8bda0ce3bfd0c9f"></a>
## deserialize

`function` · `deltalake_catalog_unity::models::DeltaRuntimeProperties::deserialize` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L427).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::DeltaRuntimeProperties", "path": "DeltaRuntimeProperties"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "_serde::__private229::Default"}}}], "generic_params": [], "type": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::DeltaRuntimeProperties", "path": "DeltaRuntimeProperties"}}}}]}, "is_negative": false, "span": {"begin": [427, 10], "end": [427, 21], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/catalog-unity/src/models.rs:427`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74e6c5becc82027c28af08b2"></a>
## fmt

`function` · `deltalake_catalog_unity::models::DeltaRuntimeProperties::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L427).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::DeltaRuntimeProperties", "path": "DeltaRuntimeProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [427, 32], "end": [427, 37], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/models.rs:427`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
