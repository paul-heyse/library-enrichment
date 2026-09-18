# `deltalake_core::kernel::models::actions::TableFeatures`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.models.actions.TableFeatures.json).

<a id="op-7bf10d7c5012289e7665730f"></a>
## TableFeatures

`enum` · `deltalake_core::kernel::models::actions::TableFeatures` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum TableFeatures
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L688).

Source: `crates/core/src/kernel/models/actions.rs:688`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

High level table features

<a id="op-6030074ccbf7b56e12c79f34"></a>
## AppendOnly

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::AppendOnly` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
AppendOnly
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L703).

Source: `crates/core/src/kernel/models/actions.rs:703`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Append Only Tables

<a id="op-750e01d591012389a5807d9e"></a>
## ChangeDataFeed

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::ChangeDataFeed` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ChangeDataFeed
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L709).

Source: `crates/core/src/kernel/models/actions.rs:709`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

CDF on a table

<a id="op-e13487f220efb2551e2bc5b3"></a>
## CheckConstraints

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::CheckConstraints` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
CheckConstraints
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L707).

Source: `crates/core/src/kernel/models/actions.rs:707`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Check constraints on columns

<a id="op-b9bb99c7ebb56e3712242fde"></a>
## ColumnMapping

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::ColumnMapping` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ColumnMapping
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L690).

Source: `crates/core/src/kernel/models/actions.rs:690`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Mapping of one column to another

<a id="op-449c9b1a7b39523209714900"></a>
## DeletionVectors

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::DeletionVectors` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
DeletionVectors
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L692).

Source: `crates/core/src/kernel/models/actions.rs:692`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Deletion vectors for merge, update, delete

<a id="op-f6b0f46729a54b595ef187bb"></a>
## DomainMetadata

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::DomainMetadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
DomainMetadata
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L717).

Source: `crates/core/src/kernel/models/actions.rs:717`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

domain specific metadata

<a id="op-c35e7c480881a9ef98f47ba3"></a>
## Err

`assoc_type` · `deltalake_core::kernel::models::actions::TableFeatures::Err` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Err = ()
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L731).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::TableFeatures", "path": "TableFeatures"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [730, 1], "end": [757, 2], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `crates/core/src/kernel/models/actions.rs:731`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19838d9f442ef772a9a3b0e7"></a>
## GeneratedColumns

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::GeneratedColumns` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
GeneratedColumns
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L711).

Source: `crates/core/src/kernel/models/actions.rs:711`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Columns with generated values

<a id="op-3386d383a5ee50a6c8f3fd98"></a>
## IcebergCompatV1

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::IcebergCompatV1` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
IcebergCompatV1
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L719).

Source: `crates/core/src/kernel/models/actions.rs:719`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Iceberg compatibility support

<a id="op-0f4481e0e4a037813e11a0a3"></a>
## IdentityColumns

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::IdentityColumns` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
IdentityColumns
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L713).

Source: `crates/core/src/kernel/models/actions.rs:713`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

ID Columns

<a id="op-a4da4996afdede1eb204ffb0"></a>
## Invariants

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::Invariants` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Invariants
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L705).

Source: `crates/core/src/kernel/models/actions.rs:705`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Table invariants

<a id="op-d4ca5f6faa056fb457fcb044"></a>
## MaterializePartitionColumns

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::MaterializePartitionColumns` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MaterializePartitionColumns
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L727).

Source: `crates/core/src/kernel/models/actions.rs:727`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Support for materializing partition column values into data files.

<a id="op-b6e2a8f8828f1b109ae314e8"></a>
## RowTracking

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::RowTracking` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
RowTracking
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L715).

Source: `crates/core/src/kernel/models/actions.rs:715`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Row tracking on tables

<a id="op-b23014654079900110b22283"></a>
## TimestampNanos

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::TimestampNanos` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
TimestampNanos
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L699).

Source: `crates/core/src/kernel/models/actions.rs:699`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Timestamps that are nanosecond resolution

<a id="op-e00f0c336bf7ee438ccd42d8"></a>
## TimestampWithoutTimezone

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::TimestampWithoutTimezone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
TimestampWithoutTimezone
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L695).

Source: `crates/core/src/kernel/models/actions.rs:695`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

timestamps without timezone support

<a id="op-7f54b2b2cee250109293df20"></a>
## V2Checkpoint

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::V2Checkpoint` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
V2Checkpoint
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L701).

Source: `crates/core/src/kernel/models/actions.rs:701`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

version 2 of checkpointing

<a id="op-3e5e9c73980ff436d9da23f8"></a>
## VariantShreddingPreview

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::VariantShreddingPreview` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
VariantShreddingPreview
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L725).

Source: `crates/core/src/kernel/models/actions.rs:725`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Preview shredded variant support

<a id="op-7a415ba24f8843e25de280a5"></a>
## VariantType

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::VariantType` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
VariantType
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L721).

Source: `crates/core/src/kernel/models/actions.rs:721`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Variant type support

<a id="op-2e62c14cf7a15be2dcabafd3"></a>
## VariantTypePreview

`variant` · `deltalake_core::kernel::models::actions::TableFeatures::VariantTypePreview` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
VariantTypePreview
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L723).

Source: `crates/core/src/kernel/models/actions.rs:723`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Preview variant type support

<a id="op-965c91420550effa8aa24bc0"></a>
## as_ref

`function` · `deltalake_core::kernel::models::actions::TableFeatures::as_ref` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_ref(&self) -> &str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L760).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::TableFeatures", "path": "TableFeatures"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [759, 1], "end": [783, 2], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `crates/core/src/kernel/models/actions.rs:760`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-247e2bdce413e33f81c29c27"></a>
## clone

`function` · `deltalake_core::kernel::models::actions::TableFeatures::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> TableFeatures
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L686).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::TableFeatures", "path": "TableFeatures"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [686, 41], "end": [686, 46], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/models/actions.rs:686`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b320cdc6c43b8ff7631faab"></a>
## deserialize

`function` · `deltalake_core::kernel::models::actions::TableFeatures::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L686).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::TableFeatures", "path": "TableFeatures"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [686, 21], "end": [686, 32], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/kernel/models/actions.rs:686`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2c665b4bb4765febf9497e2"></a>
## eq

`function` · `deltalake_core::kernel::models::actions::TableFeatures::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &TableFeatures) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L686).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::TableFeatures", "path": "TableFeatures"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [686, 52], "end": [686, 61], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/kernel/models/actions.rs:686`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b39f231bf63452794b49360"></a>
## fmt

`function` · `deltalake_core::kernel::models::actions::TableFeatures::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L786).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::TableFeatures", "path": "TableFeatures"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [785, 1], "end": [789, 2], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/core/src/kernel/models/actions.rs:786`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5b100d50d871633faf487ec"></a>
## fmt

`function` · `deltalake_core::kernel::models::actions::TableFeatures::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L686).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::TableFeatures", "path": "TableFeatures"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [686, 34], "end": [686, 39], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/models/actions.rs:686`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff657c20b1d622157afdd47b"></a>
## from_str

`function` · `deltalake_core::kernel::models::actions::TableFeatures::from_str` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_str(value: &str) -> Result<Self, Self::Err>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L733).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::TableFeatures", "path": "TableFeatures"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [730, 1], "end": [757, 2], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `crates/core/src/kernel/models/actions.rs:733`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9eeb3ff42c3aae6871e221a"></a>
## hash

`function` · `deltalake_core::kernel::models::actions::TableFeatures::hash` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L686).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::TableFeatures", "path": "TableFeatures"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [686, 63], "end": [686, 67], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `crates/core/src/kernel/models/actions.rs:686`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-607a8eb9d68b177bf9c871d1"></a>
## serialize

`function` · `deltalake_core::kernel::models::actions::TableFeatures::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L686).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::TableFeatures", "path": "TableFeatures"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [686, 10], "end": [686, 19], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/kernel/models/actions.rs:686`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7cf43041c5ee0566e01af0b"></a>
## to_reader_writer_features

`function` · `deltalake_core::kernel::models::actions::TableFeatures::to_reader_writer_features` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_reader_writer_features(&self) -> (Option<TableFeature>, Option<TableFeature>)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L801).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::TableFeatures", "path": "TableFeatures"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [799, 1], "end": [852, 2], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/models/actions.rs:801`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Convert table feature to respective reader or/and write feature
