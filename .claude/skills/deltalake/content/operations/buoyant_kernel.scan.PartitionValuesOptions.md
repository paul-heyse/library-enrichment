# `buoyant_kernel::scan::PartitionValuesOptions`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.scan.PartitionValuesOptions.json).

<a id="op-c151f7b66cbf5134df7c2604"></a>
## PartitionValuesOptions

`struct` · `buoyant_kernel::scan::PartitionValuesOptions` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct PartitionValuesOptions
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L196).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:196`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Engine-facing partition value options. Pass to [`ScanBuilder::with_partition_values`](../operations/buoyant_kernel.scan.ScanBuilder.md#op-8c97951876f2d3ff427f71f9) to
declare whether scan metadata output includes the typed `partitionValues_parsed` struct
alongside the raw string map (`fileConstantValues.partitionValues`), which is always present.

When the typed struct is requested, scan metadata output gains a top-level
`partitionValues_parsed` struct column with one typed nullable field per partition column
(physical names, table partition-column order). On non-partitioned tables the column is
omitted. Values come directly from the checkpoint's native `partitionValues_parsed` column
when present, otherwise from parsing the string map.

<a id="op-ef6ac9bd75d789f3579abb6d"></a>
## clone

`function` · `buoyant_kernel::scan::PartitionValuesOptions::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> PartitionValuesOptions
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L195).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::PartitionValuesOptions", "path": "PartitionValuesOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 10], "end": [195, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:195`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e14c0078331319d2836f641"></a>
## default

`function` · `buoyant_kernel::scan::PartitionValuesOptions::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> PartitionValuesOptions
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L195).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::PartitionValuesOptions", "path": "PartitionValuesOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 24], "end": [195, 31], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:195`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4a01f9cadaa40812f5c7245"></a>
## fmt

`function` · `buoyant_kernel::scan::PartitionValuesOptions::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L195).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::PartitionValuesOptions", "path": "PartitionValuesOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 17], "end": [195, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:195`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-250a5f39390d274423799215"></a>
## string_map_only

`function` · `buoyant_kernel::scan::PartitionValuesOptions::string_map_only` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn string_map_only() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L203).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::PartitionValuesOptions", "path": "PartitionValuesOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [214, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:203`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Raw string map only, no typed struct. Equivalent to [`Default::default`].

Unresolved upstream links (retained, not inferred): ``Default::default``.

<a id="op-82ae32148b14c2c6813c2520"></a>
## with_struct

`function` · `buoyant_kernel::scan::PartitionValuesOptions::with_struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_struct() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L209).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::PartitionValuesOptions", "path": "PartitionValuesOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [214, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:209`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Emit the typed `partitionValues_parsed` struct alongside the raw string map. Lets engines
consume `partitionValues_parsed` directly instead of parsing the string map per row.

<a id="op-a1589c8100f214d53af3a389"></a>
## parsed_struct

`struct_field` · `buoyant_kernel::scan::PartitionValuesOptions::parsed_struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
parsed_struct: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L198).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:198`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether to emit the typed `partitionValues_parsed` struct column.
