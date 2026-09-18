# `buoyant_kernel::schema::StructFieldIntoIter`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.schema.StructFieldIntoIter.json).

<a id="op-b6864e01b9c97182b0d38901"></a>
## StructFieldIntoIter

`struct` · `buoyant_kernel::schema::StructFieldIntoIter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct StructFieldIntoIter
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1438).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1438`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An iterator that yields owned [`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d)s from a [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98).

This iterator is returned by the [`IntoIterator`] implementation for [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98) and
consumes the original struct. It yields each field in the order they were defined in the
schema, preserving the insertion order maintained by the underlying [`IndexMap`].

# Examples

```
# use buoyant_kernel as delta_kernel;
# use delta_kernel::Error;
use delta_kernel::schema::{StructType, StructField, DataType};

let fields = vec![
    StructField::new("name", DataType::STRING, false),
    StructField::new("age", DataType::INTEGER, true),
];
let struct_type = StructType::try_new(fields)?;

// Consume the struct_type and iterate over owned fields
for field in struct_type {
    println!("Field: {} ({})", field.name(), field.data_type());
}
# Ok::<(), Error>(())
```

[`IndexMap`]: indexmap::IndexMap

Unresolved upstream links (retained, not inferred): `indexmap::IndexMap`, ``IntoIterator``.

<a id="op-08b322f6ae482a27fbdb30f7"></a>
## Item

`assoc_type` · `buoyant_kernel::schema::StructFieldIntoIter::Item` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Item = StructField
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1443).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructFieldIntoIter", "path": "StructFieldIntoIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1442, 1], "end": [1464, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1443`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-021df06bfa313bdd556ba46d"></a>
## count

`function` · `buoyant_kernel::schema::StructFieldIntoIter::count` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn count(self) -> usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1453).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructFieldIntoIter", "path": "StructFieldIntoIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1442, 1], "end": [1464, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1453`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f1fde2ab4380224d31bcf43"></a>
## fmt

`function` · `buoyant_kernel::schema::StructFieldIntoIter::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1437).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructFieldIntoIter", "path": "StructFieldIntoIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1437, 10], "end": [1437, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1437`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1f16d0a6fc3e47cef32dc02"></a>
## last

`function` · `buoyant_kernel::schema::StructFieldIntoIter::last` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn last(self) -> Option<Self::Item>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1457).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructFieldIntoIter", "path": "StructFieldIntoIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1442, 1], "end": [1464, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1457`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-081eafe629f3f42efd00b02a"></a>
## len

`function` · `buoyant_kernel::schema::StructFieldIntoIter::len` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn len(&self) -> usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1467).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructFieldIntoIter", "path": "StructFieldIntoIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1466, 1], "end": [1470, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::exact_size::ExactSizeIterator", "path": "ExactSizeIterator"}, "trait_path": "core::iter::traits::exact_size::ExactSizeIterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1467`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5634b70493a5c70cb433d085"></a>
## next

`function` · `buoyant_kernel::schema::StructFieldIntoIter::next` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn next(&mut self) -> Option<Self::Item>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1445).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructFieldIntoIter", "path": "StructFieldIntoIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1442, 1], "end": [1464, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1445`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f397f8a44c49770e0b6224e6"></a>
## next_back

`function` · `buoyant_kernel::schema::StructFieldIntoIter::next_back` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1475).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructFieldIntoIter", "path": "StructFieldIntoIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1474, 1], "end": [1478, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::double_ended::DoubleEndedIterator", "path": "DoubleEndedIterator"}, "trait_path": "core::iter::traits::double_ended::DoubleEndedIterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1475`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db6b0638d83f771686ad846b"></a>
## nth

`function` · `buoyant_kernel::schema::StructFieldIntoIter::nth` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn nth(&mut self, n: usize) -> Option<Self::Item>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1461).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructFieldIntoIter", "path": "StructFieldIntoIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1442, 1], "end": [1464, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1461`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6927929eae376cec5652d989"></a>
## size_hint

`function` · `buoyant_kernel::schema::StructFieldIntoIter::size_hint` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1449).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructFieldIntoIter", "path": "StructFieldIntoIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1442, 1], "end": [1464, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1449`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8b7553beedbc7c8549df269"></a>
## inner

`struct_field` · `buoyant_kernel::schema::StructFieldIntoIter::inner` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: indexmap::map::IntoValues<String, StructField>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1439).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1439`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
