# `buoyant_kernel::actions::visitors::SetTransactionVisitor`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.visitors.SetTransactionVisitor.json).

<a id="op-007ebc10895e549b8ba46fdc"></a>
## SetTransactionVisitor

`struct` · `buoyant_kernel::actions::visitors::SetTransactionVisitor` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct SetTransactionVisitor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L308).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:308`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Extract application transaction actions from the log into a map

This visitor maintains the first entry for each application id it
encounters.  When a specific application id is required then
`application_id` can be set. This bounds the memory required for the
visitor to at most one entry and reduces the amount of processing
required.

<a id="op-d3feb9c765e39d5aa37f5a74"></a>
## default

`function` · `buoyant_kernel::actions::visitors::SetTransactionVisitor::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> SetTransactionVisitor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L306).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::visitors::SetTransactionVisitor", "path": "SetTransactionVisitor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 10], "end": [306, 17], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:306`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84f78514f6473b164cae0ca2"></a>
## fmt

`function` · `buoyant_kernel::actions::visitors::SetTransactionVisitor::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L306).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::visitors::SetTransactionVisitor", "path": "SetTransactionVisitor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 19], "end": [306, 24], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:306`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01fe99f2590372ee61bf8de8"></a>
## selected_column_names_and_types

`function` · `buoyant_kernel::actions::visitors::SetTransactionVisitor::selected_column_names_and_types` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn selected_column_names_and_types(&self) -> (&'static [ColumnName], &'static [DataType])
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L350).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::visitors::SetTransactionVisitor", "path": "SetTransactionVisitor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [349, 1], "end": [378, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs"}, "trait": {"args": null, "id": "buoyant_kernel::engine_data::RowVisitor", "path": "RowVisitor"}, "trait_path": "buoyant_kernel::engine_data::RowVisitor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:350`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65b659d2a3dcf9655acbb70e"></a>
## visit

`function` · `buoyant_kernel::actions::visitors::SetTransactionVisitor::visit` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn visit<'a>(&mut self, row_count: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L356).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::visitors::SetTransactionVisitor", "path": "SetTransactionVisitor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [349, 1], "end": [378, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs"}, "trait": {"args": null, "id": "buoyant_kernel::engine_data::RowVisitor", "path": "RowVisitor"}, "trait_path": "buoyant_kernel::engine_data::RowVisitor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:356`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af17da04d34b0df0bd301134"></a>
## visit_txn

`function` · `buoyant_kernel::actions::visitors::SetTransactionVisitor::visit_txn` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn visit_txn<'a>(row_index: usize, app_id: String, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<SetTransaction>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L327).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::visitors::SetTransactionVisitor", "path": "SetTransactionVisitor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [316, 1], "end": [347, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:327`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb77cc7904a97202f44f04e3"></a>
## application_id

`struct_field` · `buoyant_kernel::actions::visitors::SetTransactionVisitor::application_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
application_id: Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L310).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:310`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ce1e7af9def719c3790e104"></a>
## expiration_timestamp

`struct_field` · `buoyant_kernel::actions::visitors::SetTransactionVisitor::expiration_timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
expiration_timestamp: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L313).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:313`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Minimum timestamp for transaction retention. Transactions with last_updated
older than or equal to this timestamp will be filtered out. None means no filtering.

<a id="op-1bb19493f2ca55be25103573"></a>
## set_transactions

`struct_field` · `buoyant_kernel::actions::visitors::SetTransactionVisitor::set_transactions` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
set_transactions: std::collections::HashMap<String, SetTransaction>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L309).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:309`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
