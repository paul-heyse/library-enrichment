# `deltalake_core::kernel::models::actions::DeletionVectorDescriptor`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.models.actions.DeletionVectorDescriptor.json).

<a id="op-ce822833f66e7aced10003ce"></a>
## DeletionVectorDescriptor

`struct` · `deltalake_core::kernel::models::actions::DeletionVectorDescriptor` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeletionVectorDescriptor
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L903).

Source: `crates/core/src/kernel/models/actions.rs:903`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Defines a deletion vector

<a id="op-02fbd5dcffad44ce734f4ccb"></a>
## Error

`assoc_type` · `deltalake_core::kernel::models::actions::DeletionVectorDescriptor::Error` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = DeltaTableError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/scan_utils.rs#L317).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::DeletionVectorDescriptor", "path": "crate::kernel::DeletionVectorDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [316, 1], "end": [334, 2], "filename": "crates/core/src/delta_datafusion/cdf/scan_utils.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor", "path": "DeletionVectorDescriptor"}}}], "constraints": []}}, "id": "core::convert::TryInto", "path": "TryInto"}, "trait_path": "core::convert::TryInto"}`

Source: `crates/core/src/delta_datafusion/cdf/scan_utils.rs:317`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-936783806d1de01bd3cfd5b8"></a>
## cardinality

`struct_field` · `deltalake_core::kernel::models::actions::DeletionVectorDescriptor::cardinality` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
cardinality: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L932).

Source: `crates/core/src/kernel/models/actions.rs:932`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of rows the given DV logically removes from the file.

<a id="op-018cf9dc5d2fec2918350548"></a>
## clone

`function` · `deltalake_core::kernel::models::actions::DeletionVectorDescriptor::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeletionVectorDescriptor
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L900).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::DeletionVectorDescriptor", "path": "DeletionVectorDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [900, 41], "end": [900, 46], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/models/actions.rs:900`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04799e404300212419e5639a"></a>
## deserialize

`function` · `deltalake_core::kernel::models::actions::DeletionVectorDescriptor::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L900).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::DeletionVectorDescriptor", "path": "DeletionVectorDescriptor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [900, 21], "end": [900, 32], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/kernel/models/actions.rs:900`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2208bf2eb6e85df87318b12"></a>
## eq

`function` · `deltalake_core::kernel::models::actions::DeletionVectorDescriptor::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &DeletionVectorDescriptor) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L900).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::DeletionVectorDescriptor", "path": "DeletionVectorDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [900, 48], "end": [900, 57], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/kernel/models/actions.rs:900`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c73a8901192ea3540f5617c5"></a>
## fmt

`function` · `deltalake_core::kernel::models::actions::DeletionVectorDescriptor::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L900).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::DeletionVectorDescriptor", "path": "DeletionVectorDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [900, 34], "end": [900, 39], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/models/actions.rs:900`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ecabb78f88dfb4432cc8122"></a>
## offset

`struct_field` · `deltalake_core::kernel::models::actions::DeletionVectorDescriptor::offset` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
offset: Option<i32>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L926).

Source: `crates/core/src/kernel/models/actions.rs:926`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Start of the data for this DV in number of bytes from the beginning of the file it is stored in.
Always None (absent in JSON) when `storageType = 'i'`.

<a id="op-266f5baff77452966bc2ed2a"></a>
## path_or_inline_dv

`struct_field` · `deltalake_core::kernel::models::actions::DeletionVectorDescriptor::path_or_inline_dv` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
path_or_inline_dv: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L921).

Source: `crates/core/src/kernel/models/actions.rs:921`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Three format options are currently proposed:
- If `storageType = 'u'` then `<random prefix - optional><base85 encoded uuid>`:
  The deletion vector is stored in a file with a path relative to the data
  directory of this Delta table, and the file name can be reconstructed from
  the UUID. See Derived Fields for how to reconstruct the file name. The random
  prefix is recovered as the extra characters before the (20 characters fixed length) uuid.
- If `storageType = 'i'` then `<base85 encoded bytes>`: The deletion vector
  is stored inline in the log. The format used is the `RoaringBitmapArray`
  format also used when the DV is stored on disk and described in [Deletion Vector Format].
- If `storageType = 'p'` then `<absolute path>`: The DV is stored in a file with an
  absolute path given by this path, which has the same format as the `path` field
  in the `add`/`remove` actions.

[Deletion Vector Format]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#Deletion-Vector-Format

<a id="op-a0165306a9b507a514be3e17"></a>
## serialize

`function` · `deltalake_core::kernel::models::actions::DeletionVectorDescriptor::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L900).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::DeletionVectorDescriptor", "path": "DeletionVectorDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [900, 10], "end": [900, 19], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/kernel/models/actions.rs:900`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34b6d5cd9ddcb6a5d0c5fe57"></a>
## size_in_bytes

`struct_field` · `deltalake_core::kernel::models::actions::DeletionVectorDescriptor::size_in_bytes` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
size_in_bytes: i32
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L929).

Source: `crates/core/src/kernel/models/actions.rs:929`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Size of the serialized DV in bytes (raw data size, i.e. before base85 encoding, if inline).

<a id="op-f73051bb404c5117f0de0593"></a>
## storage_type

`struct_field` · `deltalake_core::kernel::models::actions::DeletionVectorDescriptor::storage_type` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
storage_type: StorageType
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L905).

Source: `crates/core/src/kernel/models/actions.rs:905`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A single character to indicate how to access the DV. Legal options are: ['u', 'i', 'p'].

<a id="op-ae4af575d7a533a51539a17f"></a>
## try_into

`function` · `deltalake_core::kernel::models::actions::DeletionVectorDescriptor::try_into` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_into(self) -> DeltaResult<DeletionVectorDescriptor>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/scan_utils.rs#L319).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::DeletionVectorDescriptor", "path": "crate::kernel::DeletionVectorDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [316, 1], "end": [334, 2], "filename": "crates/core/src/delta_datafusion/cdf/scan_utils.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor", "path": "DeletionVectorDescriptor"}}}], "constraints": []}}, "id": "core::convert::TryInto", "path": "TryInto"}, "trait_path": "core::convert::TryInto"}`

Source: `crates/core/src/delta_datafusion/cdf/scan_utils.rs:319`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
