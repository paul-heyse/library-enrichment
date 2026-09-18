# `buoyant_kernel::table_changes::scan::TableChangesScan`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_changes.scan.TableChangesScan.json).

<a id="op-298dbadcbcd42cc5a5dc83e9"></a>
## TableChangesScan

`struct` · `buoyant_kernel::table_changes::scan::TableChangesScan` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct TableChangesScan
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L24).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:24`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The result of building a [`TableChanges`](../operations/buoyant_kernel.table_changes.TableChanges.md#op-9b64c4444ea3b8dc26e08d7e) scan over a table. This can be used to get the change
data feed from the table.

<a id="op-0469fced5888d4274e5fbe6b"></a>
## execute

`function` · `buoyant_kernel::table_changes::scan::TableChangesScan::execute` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn execute(&self, engine: Arc<dyn Engine>) -> DeltaResult<impl Iterator<Item = DeltaResult<Box<dyn EngineData>>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L203).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::scan::TableChangesScan", "path": "TableChangesScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [236, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:203`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Perform an "all in one" scan to get the change data feed. This will use the provided
`engine` to read and process all the data for the query. Each [`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809) in the
resultant iterator is a portion of the final set of data.

<a id="op-d6ce34a28c8d661a6053da98"></a>
## fmt

`function` · `buoyant_kernel::table_changes::scan::TableChangesScan::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L23).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::scan::TableChangesScan", "path": "TableChangesScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 10], "end": [23, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd10f2d1a4f268b7df180eee"></a>
## logical_schema

`function` · `buoyant_kernel::table_changes::scan::TableChangesScan::logical_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn logical_schema(&self) -> &SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L176).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::scan::TableChangesScan", "path": "TableChangesScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [236, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:176`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get a shared reference to the logical [`Schema`] of the table changes scan.

[`Schema`]: crate::schema::Schema

<a id="op-c16eb514e23d4e8b11b6969c"></a>
## physical_schema

`function` · `buoyant_kernel::table_changes::scan::TableChangesScan::physical_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn physical_schema(&self) -> &SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L183).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::scan::TableChangesScan", "path": "TableChangesScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [236, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:183`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get a shared reference to the physical [`Schema`] of the table changes scan.

[`Schema`]: crate::schema::Schema

<a id="op-9f2e3fd3459845290277c914"></a>
## table_root

`function` · `buoyant_kernel::table_changes::scan::TableChangesScan::table_root` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_root(&self) -> &Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L187).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_changes::scan::TableChangesScan", "path": "TableChangesScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [236, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:187`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11d4c9a070e7b02563810f43"></a>
## state_info

`struct_field` · `buoyant_kernel::table_changes::scan::TableChangesScan::state_info` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
state_info: std::sync::Arc<scan::state_info::StateInfo>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L28).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:28`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a00e6142fe26c9e7807be47b"></a>
## table_changes

`struct_field` · `buoyant_kernel::table_changes::scan::TableChangesScan::table_changes` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_changes: std::sync::Arc<super::TableChanges>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan.rs#L26).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
