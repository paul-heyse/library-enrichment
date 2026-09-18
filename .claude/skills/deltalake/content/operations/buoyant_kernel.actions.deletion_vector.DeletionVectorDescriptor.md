# `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.deletion_vector.DeletionVectorDescriptor.json).

<a id="op-7ddc769d032743db1b84b6be"></a>
## DeletionVectorDescriptor

`struct` · `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeletionVectorDescriptor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L133).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:133`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-173010fb0aa291ce60fbfa60"></a>
## absolute_path

`function` · `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor::absolute_path` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn absolute_path(&self, parent: &Url) -> DeltaResult<Option<Url>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L257).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor", "path": "DeletionVectorDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [164, 1], "end": [439, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:257`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21c639b4d3e65e69bc96fd07"></a>
## cardinality

`struct_field` · `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor::cardinality` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
cardinality: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L161).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:161`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Number of rows the given DV logically removes from the file.

<a id="op-f28081539d7c4e8400b1de06"></a>
## clone

`function` · `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeletionVectorDescriptor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L127).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor", "path": "DeletionVectorDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 17], "end": [127, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:127`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5358c7fb498964901d1091a"></a>
## eq

`function` · `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &DeletionVectorDescriptor) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L127).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor", "path": "DeletionVectorDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 24], "end": [127, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:127`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4172c291599b2392a7e729e"></a>
## fmt

`function` · `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L127).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor", "path": "DeletionVectorDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 10], "end": [127, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:127`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e26b7ec68060349557cf1654"></a>
## offset

`struct_field` · `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor::offset` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
offset: Option<i32>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L155).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:155`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Start of the data for this DV in number of bytes from the beginning of the file it is
stored in. Always None (absent in JSON) when `storageType = 'i'`.

<a id="op-0d1b08d3785d08ae59e25794"></a>
## path_or_inline_dv

`struct_field` · `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor::path_or_inline_dv` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
path_or_inline_dv: String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L151).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:151`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Three format options are currently proposed:
- If `storageType = 'u'` then `<random prefix - optional><base85 encoded uuid>`: The
  deletion vector is stored in a file with a path relative to the data directory of this
  Delta table, and the file name can be reconstructed from the UUID. See Derived Fields for
  how to reconstruct the file name. The random prefix is recovered as the extra characters
  before the (20 characters fixed length) uuid.
- If `storageType = 'i'` then `<base85 encoded bytes>`: The deletion vector is stored
  inline in the log. The format used is the `RoaringBitmapArray` format also used when the
  DV is stored on disk and described in [Deletion Vector Format].
- If `storageType = 'p'` then `<absolute path>`: The DV is stored in a file with an
  absolute path given by this path, which has the same format as the `path` field in the
  `add`/`remove` actions.

[Deletion Vector Format]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#Deletion-Vector-Format

<a id="op-723b2d268021188e80068e1a"></a>
## read

`function` · `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor::read` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read(&self, storage: Arc<dyn StorageHandler>, parent: &Url) -> DeltaResult<RoaringTreemap>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L292).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor", "path": "DeletionVectorDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [164, 1], "end": [439, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:292`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Read a dv in stored form into a [`RoaringTreemap`]

Unresolved upstream links (retained, not inferred): ``RoaringTreemap``.

<a id="op-b203f20ac056e78474b326d9"></a>
## row_indexes

`function` · `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor::row_indexes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn row_indexes(&self, storage: Arc<dyn StorageHandler>, parent: &Url) -> DeltaResult<Vec<u64>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L432).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor", "path": "DeletionVectorDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [164, 1], "end": [439, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:432`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Materialize the row indexes of the deletion vector as a `Vec<u64>` in which each element
represents a row index that is deleted from the table.

<a id="op-9efeeffa5e96fffa3bc54f4c"></a>
## size_in_bytes

`struct_field` · `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor::size_in_bytes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
size_in_bytes: i32
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L158).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:158`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Size of the serialized DV in bytes (raw data size, i.e. before base85 encoding, if inline).

<a id="op-6748513d0f5d3b2a4f6b88d1"></a>
## storage_type

`struct_field` · `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor::storage_type` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
storage_type: DeletionVectorStorageType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L135).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:135`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A single character to indicate how to access the DV. Legal options are: ['u', 'i', 'p'].

<a id="op-570b6ea6f40e1cef57f76f87"></a>
## to_schema

`function` · `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor::to_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L127).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor", "path": "DeletionVectorDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 39], "end": [127, 47], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs"}, "trait": {"args": null, "id": "buoyant_kernel::schema::ToSchema", "path": "ToSchema"}, "trait_path": "buoyant_kernel::schema::ToSchema"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:127`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91e65475e07a380c7104bdac"></a>
## try_new

`function` · `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor::try_new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(storage_type: DeletionVectorStorageType, path_or_inline_dv: impl Into<String>, offset: Option<i32>, size_in_bytes: i32, cardinality: i64) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L178).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor", "path": "DeletionVectorDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [164, 1], "end": [439, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:178`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Construct a validated [`DeletionVectorDescriptor`](../operations/buoyant_kernel.actions.deletion_vector.DeletionVectorDescriptor.md#op-7ddc769d032743db1b84b6be) from its raw fields.

Validates the protocol-level invariants from the "Deletion Vector Descriptor Schema"
section of the Delta protocol:
- `size_in_bytes` and `cardinality` must be non-negative.
- If `offset` is present, it must be non-negative.
- `Inline` descriptors must not carry an offset.
- `PersistedRelative` paths carry an optional random prefix followed by a 20-character
  z85-encoded UUID, so they must be at least 20 characters long.
- `PersistedAbsolute` paths must parse as a URL.

`Inline` payload bytes are accepted verbatim; the framing of the embedded RoaringBitmap
is only checked when the DV is later read via [`Self::read`](../operations/buoyant_kernel.actions.deletion_vector.DeletionVectorDescriptor.md#op-723b2d268021188e80068e1a).

<a id="op-754842307e7cfd11374082d4"></a>
## unique_id

`function` · `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor::unique_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn unique_id(&self) -> String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L239).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor", "path": "DeletionVectorDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [164, 1], "end": [439, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:239`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
