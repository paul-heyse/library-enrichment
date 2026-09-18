# `deltalake_catalog_unity::models::IsolationMode`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.models.IsolationMode.json).

<a id="op-b807f223b72631cc3efccd59"></a>
## IsolationMode

`enum` · `deltalake_catalog_unity::models::IsolationMode` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum IsolationMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L134).

Source: `crates/catalog-unity/src/models.rs:134`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether the current securable is accessible from all workspaces or a specific set of workspaces.

<a id="op-c5e6954932f1766f977db120"></a>
## Isolated

`variant` · `deltalake_catalog_unity::models::IsolationMode::Isolated` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Isolated
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L138).

Source: `crates/catalog-unity/src/models.rs:138`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bbc493fb2d996697bc2cd2e"></a>
## Open

`variant` · `deltalake_catalog_unity::models::IsolationMode::Open` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Open
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L137).

Source: `crates/catalog-unity/src/models.rs:137`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c377a4c9b80918598c3e2c39"></a>
## Undefined

`variant` · `deltalake_catalog_unity::models::IsolationMode::Undefined` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Undefined
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L136).

Source: `crates/catalog-unity/src/models.rs:136`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2ade71a967f5353753ca874"></a>
## default

`function` · `deltalake_catalog_unity::models::IsolationMode::default` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> IsolationMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L130).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::IsolationMode", "path": "IsolationMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 23], "end": [130, 30], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/catalog-unity/src/models.rs:130`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79174012fa6f7d7f88b2320c"></a>
## deserialize

`function` · `deltalake_catalog_unity::models::IsolationMode::deserialize` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L130).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::IsolationMode", "path": "IsolationMode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 10], "end": [130, 21], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/catalog-unity/src/models.rs:130`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e7e190b2d9d1a3d3283a7fa"></a>
## fmt

`function` · `deltalake_catalog_unity::models::IsolationMode::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L130).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::IsolationMode", "path": "IsolationMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 32], "end": [130, 37], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/models.rs:130`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
