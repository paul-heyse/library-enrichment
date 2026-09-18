# `deltalake_catalog_unity::models::TemporaryTableCredentialsRequest`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.models.TemporaryTableCredentialsRequest.json).

<a id="op-73e902b668c751e84228ad8a"></a>
## TemporaryTableCredentialsRequest

`struct` · `deltalake_catalog_unity::models::TemporaryTableCredentialsRequest` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct TemporaryTableCredentialsRequest
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L590).

Source: `crates/catalog-unity/src/models.rs:590`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab39c1a2dce02d988f5c3eee"></a>
## clone

`function` · `deltalake_catalog_unity::models::TemporaryTableCredentialsRequest::clone` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> TemporaryTableCredentialsRequest
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L589).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TemporaryTableCredentialsRequest", "path": "TemporaryTableCredentialsRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [589, 28], "end": [589, 33], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/catalog-unity/src/models.rs:589`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bac443259fd3fed8d4faf55"></a>
## fmt

`function` · `deltalake_catalog_unity::models::TemporaryTableCredentialsRequest::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L589).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TemporaryTableCredentialsRequest", "path": "TemporaryTableCredentialsRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [589, 21], "end": [589, 26], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/models.rs:589`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a988f6724e2a4aaff25c2f56"></a>
## new

`function` · `deltalake_catalog_unity::models::TemporaryTableCredentialsRequest::new` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(table_id: &str, operation: &str) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L596).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TemporaryTableCredentialsRequest", "path": "TemporaryTableCredentialsRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [595, 1], "end": [602, 2], "filename": "crates/catalog-unity/src/models.rs"}, "trait": null, "trait_path": null}`

Source: `crates/catalog-unity/src/models.rs:596`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d4b4181aa007c9e9c0e8f93"></a>
## operation

`struct_field` · `deltalake_catalog_unity::models::TemporaryTableCredentialsRequest::operation` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
operation: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L592).

Source: `crates/catalog-unity/src/models.rs:592`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2a14ba64c9738180363bb75"></a>
## serialize

`function` · `deltalake_catalog_unity::models::TemporaryTableCredentialsRequest::serialize` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L589).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TemporaryTableCredentialsRequest", "path": "TemporaryTableCredentialsRequest"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [589, 10], "end": [589, 19], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/catalog-unity/src/models.rs:589`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83ca68755f1a5c6577cb466f"></a>
## table_id

`struct_field` · `deltalake_catalog_unity::models::TemporaryTableCredentialsRequest::table_id` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
table_id: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L591).

Source: `crates/catalog-unity/src/models.rs:591`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
