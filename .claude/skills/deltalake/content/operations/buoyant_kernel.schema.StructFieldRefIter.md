# `buoyant_kernel::schema::StructFieldRefIter`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.schema.StructFieldRefIter.json).

<a id="op-90a6c1fea93239abdbdc2134"></a>
## StructFieldRefIter

`struct` · `buoyant_kernel::schema::StructFieldRefIter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct StructFieldRefIter<'a>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1521).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1521`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An iterator that yields references to [`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d)s from a [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98).

This iterator is returned by the [`IntoIterator`] implementation for `&StructType` and by
the [`StructType::fields()`] method. Unlike [`StructFieldIntoIter`](../operations/buoyant_kernel.schema.StructFieldIntoIter.md#op-b6864e01b9c97182b0d38901), this iterator does not
consume the original struct and yields references to the fields. It preserves the insertion
order maintained by the underlying [`IndexMap`].

This iterator implements [`Clone`], allowing you to create multiple independent iterators
over the same set of fields.

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

// Iterate over field references without consuming the struct_type
for field in &struct_type {
    println!("Field: {} ({})", field.name(), field.data_type());
}

// struct_type is still available for use
assert_eq!(struct_type.field("name").unwrap().name(), "name");

// Or use the fields() method explicitly
for field in struct_type.fields() {
    println!("Field type: {}", field.data_type());
}
# Ok::<(), Error>(())
```

[`StructType::fields()`]: StructType::fields
[`IndexMap`]: indexmap::IndexMap

Unresolved upstream links (retained, not inferred): `indexmap::IndexMap`, ``IntoIterator``, ``Clone``.

<a id="op-a7eb816d0287cd233632d1fc"></a>
## Item

`assoc_type` · `buoyant_kernel::schema::StructFieldRefIter::Item` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Item = &'a StructField
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1526).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::schema::StructFieldRefIter", "path": "StructFieldRefIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1525, 1], "end": [1535, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1526`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71948dfb16360e6627a42763"></a>
## clone

`function` · `buoyant_kernel::schema::StructFieldRefIter::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> StructFieldRefIter<'a>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1520).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::schema::StructFieldRefIter", "path": "StructFieldRefIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1520, 17], "end": [1520, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1520`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bf51fafc69de78f6e7855db"></a>
## fmt

`function` · `buoyant_kernel::schema::StructFieldRefIter::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1520).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::schema::StructFieldRefIter", "path": "StructFieldRefIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1520, 10], "end": [1520, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1520`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-003d0b9a7e7d1f8d2af8ccfe"></a>
## len

`function` · `buoyant_kernel::schema::StructFieldRefIter::len` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn len(&self) -> usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1538).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "buoyant_kernel::schema::StructFieldRefIter", "path": "StructFieldRefIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1537, 1], "end": [1541, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::exact_size::ExactSizeIterator", "path": "ExactSizeIterator"}, "trait_path": "core::iter::traits::exact_size::ExactSizeIterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1538`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bbfd9366c85b4cd1af0173fa"></a>
## next

`function` · `buoyant_kernel::schema::StructFieldRefIter::next` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn next(&mut self) -> Option<Self::Item>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1528).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::schema::StructFieldRefIter", "path": "StructFieldRefIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1525, 1], "end": [1535, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1528`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6bb25f4d32d032da29ab6dd9"></a>
## next_back

`function` · `buoyant_kernel::schema::StructFieldRefIter::next_back` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1546).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "buoyant_kernel::schema::StructFieldRefIter", "path": "StructFieldRefIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1545, 1], "end": [1549, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::double_ended::DoubleEndedIterator", "path": "DoubleEndedIterator"}, "trait_path": "core::iter::traits::double_ended::DoubleEndedIterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1546`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb05f537ee617695ab3572c0"></a>
## size_hint

`function` · `buoyant_kernel::schema::StructFieldRefIter::size_hint` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1532).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::schema::StructFieldRefIter", "path": "StructFieldRefIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1525, 1], "end": [1535, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1532`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0460750c06965b134cbbeaae"></a>
## inner

`struct_field` · `buoyant_kernel::schema::StructFieldRefIter::inner` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: indexmap::map::Values<'a, String, StructField>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1522).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1522`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
