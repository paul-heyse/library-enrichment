# `deltalake_catalog_unity::models::AwsTempCredentials`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.models.AwsTempCredentials.json).

<a id="op-ca6f395ddaf1ee626577d78d"></a>
## AwsTempCredentials

`struct` · `deltalake_catalog_unity::models::AwsTempCredentials` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct AwsTempCredentials
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L505).

Source: `crates/catalog-unity/src/models.rs:505`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b12454fadba60e7bfaaa5658"></a>
## access_key_id

`struct_field` · `deltalake_catalog_unity::models::AwsTempCredentials::access_key_id` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
access_key_id: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L506).

Source: `crates/catalog-unity/src/models.rs:506`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad18bc6e826833a94fcc9eeb"></a>
## access_point

`struct_field` · `deltalake_catalog_unity::models::AwsTempCredentials::access_point` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
access_point: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L509).

Source: `crates/catalog-unity/src/models.rs:509`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca433d7499547d3af03eea51"></a>
## clone

`function` · `deltalake_catalog_unity::models::AwsTempCredentials::clone` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> AwsTempCredentials
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L504).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::AwsTempCredentials", "path": "AwsTempCredentials"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 30], "end": [504, 35], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/catalog-unity/src/models.rs:504`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ec1194e9fdbbc08de45821c"></a>
## deserialize

`function` · `deltalake_catalog_unity::models::AwsTempCredentials::deserialize` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L504).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::AwsTempCredentials", "path": "AwsTempCredentials"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 10], "end": [504, 21], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/catalog-unity/src/models.rs:504`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d73c2a9863036000b650e887"></a>
## fmt

`function` · `deltalake_catalog_unity::models::AwsTempCredentials::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L504).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::AwsTempCredentials", "path": "AwsTempCredentials"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 23], "end": [504, 28], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/models.rs:504`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79260292d40d6de5c5224184"></a>
## secret_access_key

`struct_field` · `deltalake_catalog_unity::models::AwsTempCredentials::secret_access_key` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
secret_access_key: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L507).

Source: `crates/catalog-unity/src/models.rs:507`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4498d98e8154f6086ef6f418"></a>
## session_token

`struct_field` · `deltalake_catalog_unity::models::AwsTempCredentials::session_token` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
session_token: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L508).

Source: `crates/catalog-unity/src/models.rs:508`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
