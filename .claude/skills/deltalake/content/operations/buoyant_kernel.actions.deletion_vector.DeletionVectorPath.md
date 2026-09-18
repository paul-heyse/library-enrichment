# `buoyant_kernel::actions::deletion_vector::DeletionVectorPath`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.deletion_vector.DeletionVectorPath.json).

<a id="op-e0c035c1bacd1df080e200fd"></a>
## DeletionVectorPath

`struct` · `buoyant_kernel::actions::deletion_vector::DeletionVectorPath` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeletionVectorPath
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L75).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:75`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Represents an abstract path to a deletion vector file.

This is used in the public API to construct the path to a deletion vector file and
has logic to convert [`crate::actions::deletion_vector_writer::DeletionVectorWriteResult`](../operations/buoyant_kernel.actions.deletion_vector_writer.DeletionVectorWriteResult.md#op-2c478f1fbf9c8c54eabc09d8)
to a [`DeletionVectorDescriptor`](../operations/buoyant_kernel.actions.deletion_vector.DeletionVectorDescriptor.md#op-7ddc769d032743db1b84b6be) with appropriate storage type and path.

<a id="op-27238f6d177414e0bbe7cb97"></a>
## absolute_path

`function` · `buoyant_kernel::actions::deletion_vector::DeletionVectorPath::absolute_path` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn absolute_path(&self) -> DeltaResult<Url>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L114).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector::DeletionVectorPath", "path": "DeletionVectorPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [125, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:114`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the absolute path to the deletion vector file.

<a id="op-563d2ef4f0c3716126920ec6"></a>
## prefix

`struct_field` · `buoyant_kernel::actions::deletion_vector::DeletionVectorPath::prefix` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
prefix: String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L82).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:82`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Optional directory prefix within the table path where the DV file will be located,
this is to allow for randomizing reads/writes to avoid object store throttling.

<a id="op-a4bd6dd5352d45073bb0116d"></a>
## table_path

`struct_field` · `buoyant_kernel::actions::deletion_vector::DeletionVectorPath::table_path` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_path: url::Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L77).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:77`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The base URL path to the Delta table

<a id="op-814d1ffb4173afe45a32aa0c"></a>
## uuid

`struct_field` · `buoyant_kernel::actions::deletion_vector::DeletionVectorPath::uuid` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
uuid: uuid::Uuid
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L79).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector.rs:79`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Unique identifier for this deletion vector file
