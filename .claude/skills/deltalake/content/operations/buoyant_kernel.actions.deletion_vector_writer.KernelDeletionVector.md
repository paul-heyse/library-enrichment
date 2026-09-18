# `buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.deletion_vector_writer.KernelDeletionVector.json).

<a id="op-0310fc9c0836e83ec5940dc3"></a>
## KernelDeletionVector

`struct` · `buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct KernelDeletionVector
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L122).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:122`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A Kernel-provided deletion vector implementation backed by [`RoaringTreemap`].

This is the default implementation that engines can use. It provides memory-efficient
storage of deleted row indexes using compressed bitmaps.

# Examples

```rust
# use buoyant_kernel as delta_kernel;
use delta_kernel::actions::deletion_vector_writer::KernelDeletionVector;

let mut dv = KernelDeletionVector::new();
dv.add_deleted_row_indexes([0, 5, 10]);
```

Unresolved upstream links (retained, not inferred): ``RoaringTreemap``.

<a id="op-b002a160eea4f50f80be85fe"></a>
## IndexIterator

`assoc_type` · `buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector::IndexIterator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IndexIterator = IntoIter
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L158).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector", "path": "KernelDeletionVector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [157, 1], "end": [176, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::DeletionVector", "path": "DeletionVector"}, "trait_path": "buoyant_kernel::actions::deletion_vector_writer::DeletionVector"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:158`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-363fc079b568d95fbf55e054"></a>
## add_deleted_row_indexes

`function` · `buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector::add_deleted_row_indexes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add_deleted_row_indexes<I, T>(&mut self, iter: I) where I: IntoIterator<Item = T>, T: Borrow<u64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L141).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector", "path": "KernelDeletionVector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [155, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:141`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Adds indexes to be deleted to this deletion vector.

<a id="op-2c26ff77e29d9fa9c8b2e7d6"></a>
## cardinality

`function` · `buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector::cardinality` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn cardinality(&self) -> u64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L173).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector", "path": "KernelDeletionVector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [157, 1], "end": [176, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::DeletionVector", "path": "DeletionVector"}, "trait_path": "buoyant_kernel::actions::deletion_vector_writer::DeletionVector"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:173`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e3f16a2fa05779850aab48c"></a>
## cardinality

`function` · `buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector::cardinality` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn cardinality(&self) -> u64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L152).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector", "path": "KernelDeletionVector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [155, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:152`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the number of deleted rows in this deletion vector.

<a id="op-1d649d8045ff5354b9e67873"></a>
## clone

`function` · `buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> KernelDeletionVector
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L121).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector", "path": "KernelDeletionVector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 17], "end": [121, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:121`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-705c7c3cb4169eca8fe16d8f"></a>
## default

`function` · `buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L127).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector", "path": "KernelDeletionVector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [130, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:127`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-531a7a31a554ad9f38deab0a"></a>
## fmt

`function` · `buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L121).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector", "path": "KernelDeletionVector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 10], "end": [121, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:121`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5aeae04d36ff9e69526bbf86"></a>
## into_iter

`function` · `buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector::into_iter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_iter(self) -> Self::IndexIterator
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L160).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector", "path": "KernelDeletionVector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [157, 1], "end": [176, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::DeletionVector", "path": "DeletionVector"}, "trait_path": "buoyant_kernel::actions::deletion_vector_writer::DeletionVector"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:160`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24b98b8196d123d04dbc4787"></a>
## new

`function` · `buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L134).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector", "path": "KernelDeletionVector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [155, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:134`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new empty deletion vector.

<a id="op-723ffac04def23ef8ccd8500"></a>
## serialize

`function` · `buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize(self) -> DeltaResult<Bytes>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L165).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector", "path": "KernelDeletionVector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [157, 1], "end": [176, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs"}, "trait": {"args": null, "id": "buoyant_kernel::actions::deletion_vector_writer::DeletionVector", "path": "DeletionVector"}, "trait_path": "buoyant_kernel::actions::deletion_vector_writer::DeletionVector"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:165`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Optimized serialization that directly serializes the internal RoaringTreemap.

<a id="op-c07d93163a8f7ae42d502b40"></a>
## dv

`struct_field` · `buoyant_kernel::actions::deletion_vector_writer::KernelDeletionVector::dv` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
dv: roaring::RoaringTreemap
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector_writer.rs#L123).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/deletion_vector_writer.rs:123`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
