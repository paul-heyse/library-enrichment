# `deltalake_core::kernel::schema::partitions::FilterValue`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.schema.partitions.FilterValue.json).

<a id="op-89c72d822eff43314554e344"></a>
## FilterValue

`enum` · `deltalake_core::kernel::schema::partitions::FilterValue` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum FilterValue<'a>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L143).

Source: `crates/core/src/kernel/schema/partitions.rs:143`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The value of a `(column, op, value)` filter literal: a single partition-value
encoded string, or a set of them for `in` / `not in`.

<a id="op-62967e43e64b1aff0b054ae8"></a>
## Scalar

`variant` · `deltalake_core::kernel::schema::partitions::FilterValue::Scalar` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Scalar
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L145).

Source: `crates/core/src/kernel/schema/partitions.rs:145`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A single encoded value, compared with one of `=`, `!=`, `<`, `<=`, `>`, `>=`.

<a id="op-5a697c3a633958bdeb6ce700"></a>
## Set

`variant` · `deltalake_core::kernel::schema::partitions::FilterValue::Set` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Set
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L147).

Source: `crates/core/src/kernel/schema/partitions.rs:147`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A set of encoded values, compared with `in` or `not in`.

<a id="op-d94462acc3e8ded724b9455e"></a>
## clone

`function` · `deltalake_core::kernel::schema::partitions::FilterValue::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> FilterValue<'a>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L142).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::kernel::schema::partitions::FilterValue", "path": "FilterValue"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 10], "end": [142, 15], "filename": "crates/core/src/kernel/schema/partitions.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/schema/partitions.rs:142`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a17c68eb052252cf1877eb61"></a>
## eq

`function` · `deltalake_core::kernel::schema::partitions::FilterValue::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &FilterValue<'a>) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L142).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::kernel::schema::partitions::FilterValue", "path": "FilterValue"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 24], "end": [142, 33], "filename": "crates/core/src/kernel/schema/partitions.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/kernel/schema/partitions.rs:142`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-844573d1713c2bdf32a1b062"></a>
## fmt

`function` · `deltalake_core::kernel::schema::partitions::FilterValue::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L142).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::kernel::schema::partitions::FilterValue", "path": "FilterValue"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 17], "end": [142, 22], "filename": "crates/core/src/kernel/schema/partitions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/schema/partitions.rs:142`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
