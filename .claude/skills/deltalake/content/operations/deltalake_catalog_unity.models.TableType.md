# `deltalake_catalog_unity::models::TableType`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.models.TableType.json).

<a id="op-aedb11384242a33c82cabd99"></a>
## TableType

`enum` · `deltalake_catalog_unity::models::TableType` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum TableType
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L306).

Source: `crates/catalog-unity/src/models.rs:306`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Possible data source formats for unity tables

<a id="op-d7a5c69f258f9bf67debe215"></a>
## External

`variant` · `deltalake_catalog_unity::models::TableType::External` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
External
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L310).

Source: `crates/catalog-unity/src/models.rs:310`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-814c02b00feec75dadfad7d4"></a>
## Managed

`variant` · `deltalake_catalog_unity::models::TableType::Managed` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Managed
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L309).

Source: `crates/catalog-unity/src/models.rs:309`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b78c11192a79dad42744bc2"></a>
## MaterializedView

`variant` · `deltalake_catalog_unity::models::TableType::MaterializedView` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MaterializedView
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L312).

Source: `crates/catalog-unity/src/models.rs:312`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a44382ffd610e59ef59b25ec"></a>
## StreamingTable

`variant` · `deltalake_catalog_unity::models::TableType::StreamingTable` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
StreamingTable
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L313).

Source: `crates/catalog-unity/src/models.rs:313`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-923bcc39b3732448a85d9416"></a>
## Undefined

`variant` · `deltalake_catalog_unity::models::TableType::Undefined` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Undefined
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L308).

Source: `crates/catalog-unity/src/models.rs:308`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f0e432a798c283650d4dc8f"></a>
## View

`variant` · `deltalake_catalog_unity::models::TableType::View` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
View
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L311).

Source: `crates/catalog-unity/src/models.rs:311`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c862a9b94e61609049017cf"></a>
## clone

`function` · `deltalake_catalog_unity::models::TableType::clone` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> TableType
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L302).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TableType", "path": "TableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [302, 50], "end": [302, 55], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/catalog-unity/src/models.rs:302`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-325ee74d9cfb8fa4c6ff2e3c"></a>
## default

`function` · `deltalake_catalog_unity::models::TableType::default` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> TableType
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L302).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TableType", "path": "TableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [302, 23], "end": [302, 30], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/catalog-unity/src/models.rs:302`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ff98b42ca2333ba1cbe3005"></a>
## deserialize

`function` · `deltalake_catalog_unity::models::TableType::deserialize` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L302).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TableType", "path": "TableType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [302, 10], "end": [302, 21], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/catalog-unity/src/models.rs:302`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec2a33a571811b657c05bd3a"></a>
## eq

`function` · `deltalake_catalog_unity::models::TableType::eq` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &TableType) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L302).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TableType", "path": "TableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [302, 39], "end": [302, 48], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/catalog-unity/src/models.rs:302`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02449682b8ec7fd66fcb68b6"></a>
## fmt

`function` · `deltalake_catalog_unity::models::TableType::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L317).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TableType", "path": "TableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [316, 1], "end": [327, 2], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/catalog-unity/src/models.rs:317`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5785a52507e38a4fa02871ec"></a>
## fmt

`function` · `deltalake_catalog_unity::models::TableType::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L302).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TableType", "path": "TableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [302, 32], "end": [302, 37], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/models.rs:302`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
