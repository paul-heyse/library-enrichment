# `buoyant_kernel::actions::deletion_vector_writer::DeletionVectorWriteResult`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.deletion_vector_writer.DeletionVectorWriteResult.json).

<a id="op-2c478f1fbf9c8c54eabc09d8"></a>
## DeletionVectorWriteResult

`struct` · `buoyant_kernel::actions::deletion_vector_writer::DeletionVectorWriteResult` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeletionVectorWriteResult
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L75).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:75`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Metadata about a written deletion vector, excluding the storage path.

This structure contains the information needed to construct a full
[`DeletionVectorDescriptor`](../operations/buoyant_kernel.actions.deletion_vector.DeletionVectorDescriptor.md#op-7ddc769d032743db1b84b6be)
after writing the DV to storage.

<a id="op-7f27eb8c23d352f74c511818"></a>
## cardinality

`struct_field` · `buoyant_kernel::actions::deletion_vector_writer::DeletionVectorWriteResult::cardinality` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
cardinality: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L84).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:84`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Number of rows the deletion vector logically removes from the file.

<a id="op-4d3d9000db895bb0e8fd7459"></a>
## clone

`function` · `buoyant_kernel::actions::deletion_vector_writer::DeletionVectorWriteResult::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeletionVectorWriteResult
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L74).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::DeletionVectorWriteResult", "path": "DeletionVectorWriteResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 17], "end": [74, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80ae80fe2ac6023793a3f03c"></a>
## eq

`function` · `buoyant_kernel::actions::deletion_vector_writer::DeletionVectorWriteResult::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &DeletionVectorWriteResult) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L74).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::DeletionVectorWriteResult", "path": "DeletionVectorWriteResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 24], "end": [74, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a285dc0c65a42585d7d7573a"></a>
## fmt

`function` · `buoyant_kernel::actions::deletion_vector_writer::DeletionVectorWriteResult::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L74).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::DeletionVectorWriteResult", "path": "DeletionVectorWriteResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 10], "end": [74, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52800aaf0cd576487bfd0d58"></a>
## offset

`struct_field` · `buoyant_kernel::actions::deletion_vector_writer::DeletionVectorWriteResult::offset` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
offset: i32
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L78).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:78`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Start of the data for this DV in number of bytes from the beginning of the file.
Does not include CRC length or size in bytes prefix.

<a id="op-a752ea33d2ccd72ecb141abb"></a>
## size_in_bytes

`struct_field` · `buoyant_kernel::actions::deletion_vector_writer::DeletionVectorWriteResult::size_in_bytes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
size_in_bytes: i32
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L81).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:81`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Size of the serialized DV in bytes (raw data size).

<a id="op-4c3b91be82d7dba4079cb4ff"></a>
## to_descriptor

`function` · `buoyant_kernel::actions::deletion_vector_writer::DeletionVectorWriteResult::to_descriptor` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_descriptor(self, path: &DeletionVectorPath) -> DeletionVectorDescriptor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L96).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::DeletionVectorWriteResult", "path": "DeletionVectorWriteResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [105, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:96`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Convert the write result to a deletion vector descriptor.

As an implementation detail, this method will always use the persisted relative storage
type.

# Arguments

* `path` - The path to the deletion vector file.
