# `buoyant_kernel::row_tracking::RowTrackingDomainMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.row_tracking.RowTrackingDomainMetadata.json).

<a id="op-b367b0d33ab4834102958e20"></a>
## RowTrackingDomainMetadata

`struct` · `buoyant_kernel::row_tracking::RowTrackingDomainMetadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct RowTrackingDomainMetadata
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/row_tracking.rs#L17).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/row_tracking.rs:17`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7077a17165ec2dd50722fe9"></a>
## deserialize

`function` · `buoyant_kernel::row_tracking::RowTrackingDomainMetadata::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/row_tracking.rs#L15).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::row_tracking::RowTrackingDomainMetadata", "path": "RowTrackingDomainMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [15, 17], "end": [15, 28], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/row_tracking.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/row_tracking.rs:15`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e885ce35e6d28362fdcbc1ab"></a>
## fmt

`function` · `buoyant_kernel::row_tracking::RowTrackingDomainMetadata::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/row_tracking.rs#L15).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::row_tracking::RowTrackingDomainMetadata", "path": "RowTrackingDomainMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [15, 10], "end": [15, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/row_tracking.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/row_tracking.rs:15`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a073c01e36651b5bdc1f976"></a>
## get_high_water_mark

`function` · `buoyant_kernel::row_tracking::RowTrackingDomainMetadata::get_high_water_mark` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_high_water_mark(snapshot: &Snapshot, engine: &dyn Engine) -> DeltaResult<Option<i64>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/row_tracking.rs#L60).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::row_tracking::RowTrackingDomainMetadata", "path": "RowTrackingDomainMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [70, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/row_tracking.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/row_tracking.rs:60`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Retrieves the row ID high water mark from the [`Snapshot`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-e760c13a7e6fcf7025cb3640)'s row tracking domain metadata.

This method searches through the snapshot's log segment for domain metadata actions
with the row tracking domain name and extracts the high water mark value.

# Returns

Returns `Ok(Some(high_water_mark))` if row tracking domain metadata is found,
`Ok(None)` if no row tracking domain metadata exists, or an error if the
metadata cannot be parsed or accessed.

# Errors

This method will return an error if:
- The domain metadata configuration cannot be read from the log segment
- The domain metadata JSON cannot be deserialized into `RowTrackingDomainMetadata`

<a id="op-948dd537c937e23729c84dac"></a>
## row_id_high_water_mark

`struct_field` · `buoyant_kernel::row_tracking::RowTrackingDomainMetadata::row_id_high_water_mark` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
row_id_high_water_mark: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/row_tracking.rs#L19).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/row_tracking.rs:19`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2cbb40d1dd845135c1c37ea"></a>
## serialize

`function` · `buoyant_kernel::row_tracking::RowTrackingDomainMetadata::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/row_tracking.rs#L15).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::row_tracking::RowTrackingDomainMetadata", "path": "RowTrackingDomainMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [15, 30], "end": [15, 39], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/row_tracking.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/row_tracking.rs:15`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
