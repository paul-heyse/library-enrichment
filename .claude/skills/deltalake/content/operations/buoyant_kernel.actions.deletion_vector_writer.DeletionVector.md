# `buoyant_kernel::actions::deletion_vector_writer::DeletionVector`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.deletion_vector_writer.DeletionVector.json).

<a id="op-5cf3dd6bd44c03eb57b5d5da"></a>
## DeletionVector

`trait` · `buoyant_kernel::actions::deletion_vector_writer::DeletionVector` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait DeletionVector: Sized
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L43).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:43`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A trait that allows engines to provide deletion vectors in various formats.

Engines can implement this trait to provide their own deletion vector implementations,
or use the provided [`KernelDeletionVector`](../operations/buoyant_kernel.actions.deletion_vector_writer.KernelDeletionVector.md#op-0310fc9c0836e83ec5940dc3) implementation backed by RoaringTreemap.

# Examples

```rust
# use buoyant_kernel as delta_kernel;
use delta_kernel::actions::deletion_vector_writer::DeletionVector;

struct MyDeletionVector {
    deleted_indexes: Vec<u64>,
}

impl DeletionVector for MyDeletionVector {
    type IndexIterator = std::vec::IntoIter<u64>;

    fn into_iter(self) -> Self::IndexIterator {
        self.deleted_indexes.into_iter()
    }

    fn cardinality(&self) -> u64 {
        self.deleted_indexes.len() as u64
    }
}
```

<a id="op-0d71114ffcea1a76d5c08a61"></a>
## IndexIterator

`assoc_type` · `buoyant_kernel::actions::deletion_vector_writer::DeletionVector::IndexIterator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IndexIterator
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L45).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:45`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Iterator type that yields deleted row indexes.

<a id="op-b6be75a0c7bc26f16b3410b2"></a>
## cardinality

`function` · `buoyant_kernel::actions::deletion_vector_writer::DeletionVector::cardinality` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn cardinality(&self) -> u64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L51).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:51`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Return the number of deleted rows in the deletion vector.

<a id="op-12d1ea68436a764056d751e2"></a>
## into_iter

`function` · `buoyant_kernel::actions::deletion_vector_writer::DeletionVector::into_iter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_iter(self) -> Self::IndexIterator
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L48).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:48`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Return an iterator over deleted row indexes.

<a id="op-a44ae007697053cd5f658c6b"></a>
## serialize

`function` · `buoyant_kernel::actions::deletion_vector_writer::DeletionVector::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize(self) -> DeltaResult<Bytes>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L59).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:59`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Serialize the deletion vector into bytes.

This serializes the deletion vector in the format expected by the Delta Lake protocol.
it may be overridden for more efficient serialization if the implementation already has the
data in a suitable format. But generally, only do this if you fully understand the the
format requirements.
