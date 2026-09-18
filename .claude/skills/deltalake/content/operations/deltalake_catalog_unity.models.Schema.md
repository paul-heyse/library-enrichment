# `deltalake_catalog_unity::models::Schema`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.models.Schema.json).

<a id="op-42fa76415c59de194bd7b187"></a>
## Schema

`struct` · `deltalake_catalog_unity::models::Schema` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Schema
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L216).

Source: `crates/catalog-unity/src/models.rs:216`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A schema within a catalog

<a id="op-9f2fcb1f22150468025c9aa5"></a>
## catalog_name

`struct_field` · `deltalake_catalog_unity::models::Schema::catalog_name` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
catalog_name: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L235).

Source: `crates/catalog-unity/src/models.rs:235`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Name of parent catalog.

<a id="op-84e1dcea64f06f2a0f69ae9e"></a>
## catalog_type

`struct_field` · `deltalake_catalog_unity::models::Schema::catalog_type` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
catalog_type: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L232).

Source: `crates/catalog-unity/src/models.rs:232`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The type of the parent catalog.

<a id="op-5394cdee598de43ad6d82b30"></a>
## comment

`struct_field` · `deltalake_catalog_unity::models::Schema::comment` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
comment: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L251).

Source: `crates/catalog-unity/src/models.rs:251`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

User-provided free-form text description.

<a id="op-3bc3fd4d106e03b7513511fd"></a>
## created_at

`struct_field` · `deltalake_catalog_unity::models::Schema::created_at` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
created_at: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L255).

Source: `crates/catalog-unity/src/models.rs:255`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Time at which this schema was created, in epoch milliseconds.

<a id="op-5ac4cc8ef3d7524138447093"></a>
## created_by

`struct_field` · `deltalake_catalog_unity::models::Schema::created_by` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
created_by: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L219).

Source: `crates/catalog-unity/src/models.rs:219`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Username of schema creator.

<a id="op-7207c09e890888933c094cd8"></a>
## default

`function` · `deltalake_catalog_unity::models::Schema::default` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Schema
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L215).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [215, 23], "end": [215, 30], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/catalog-unity/src/models.rs:215`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8ca3945c455b779ef2c6b6f"></a>
## deserialize

`function` · `deltalake_catalog_unity::models::Schema::deserialize` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L215).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::Schema", "path": "Schema"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [215, 10], "end": [215, 21], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/catalog-unity/src/models.rs:215`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7f939187ceaa34662c6a1ce"></a>
## fmt

`function` · `deltalake_catalog_unity::models::Schema::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L215).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [215, 32], "end": [215, 37], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/models.rs:215`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efc9172bc66c496bb3b7b4d6"></a>
## full_name

`struct_field` · `deltalake_catalog_unity::models::Schema::full_name` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
full_name: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L229).

Source: `crates/catalog-unity/src/models.rs:229`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Full name of schema, in form of catalog_name.schema_name.

<a id="op-592b325f2d0231b487895f1a"></a>
## metastore_id

`struct_field` · `deltalake_catalog_unity::models::Schema::metastore_id` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
metastore_id: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L266).

Source: `crates/catalog-unity/src/models.rs:266`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Unique identifier of parent metastore.

<a id="op-e81926c9869f371692d7f01c"></a>
## name

`struct_field` · `deltalake_catalog_unity::models::Schema::name` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
name: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L222).

Source: `crates/catalog-unity/src/models.rs:222`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Name of schema, relative to parent catalog.

<a id="op-86249e262717b36b8172a083"></a>
## owner

`struct_field` · `deltalake_catalog_unity::models::Schema::owner` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
owner: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L259).

Source: `crates/catalog-unity/src/models.rs:259`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Username of current owner of schema.

<a id="op-8e63908d6bceae4da2472880"></a>
## properties

`struct_field` · `deltalake_catalog_unity::models::Schema::properties` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
properties: std::collections::HashMap<String, String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L247).

Source: `crates/catalog-unity/src/models.rs:247`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A map of key-value properties attached to the securable.

<a id="op-46be5d48f5bff32f0c77acee"></a>
## storage_location

`struct_field` · `deltalake_catalog_unity::models::Schema::storage_location` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
storage_location: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L243).

Source: `crates/catalog-unity/src/models.rs:243`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Storage location for managed tables within schema.

<a id="op-51ebbee8834ab2eef57df115"></a>
## storage_root

`struct_field` · `deltalake_catalog_unity::models::Schema::storage_root` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
storage_root: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L239).

Source: `crates/catalog-unity/src/models.rs:239`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Storage root URL for managed tables within schema.

<a id="op-736cc8d9b47f9610ed49e986"></a>
## updated_at

`struct_field` · `deltalake_catalog_unity::models::Schema::updated_at` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
updated_at: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L263).

Source: `crates/catalog-unity/src/models.rs:263`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Time at which this schema was created, in epoch milliseconds.

<a id="op-f1b4d45b17eaabb435375ed7"></a>
## updated_by

`struct_field` · `deltalake_catalog_unity::models::Schema::updated_by` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
updated_by: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L226).

Source: `crates/catalog-unity/src/models.rs:226`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Username of user who last modified schema.
