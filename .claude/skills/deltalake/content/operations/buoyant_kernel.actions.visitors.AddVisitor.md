# `buoyant_kernel::actions::visitors::AddVisitor`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.visitors.AddVisitor.json).

<a id="op-949dfe8032f4c97823c8e3b8"></a>
## AddVisitor

`struct` · `buoyant_kernel::actions::visitors::AddVisitor` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct AddVisitor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L103).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:103`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c875cd5b0641c8090a2208c6"></a>
## default

`function` · `buoyant_kernel::actions::visitors::AddVisitor::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> AddVisitor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L101).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::visitors::AddVisitor", "path": "AddVisitor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 10], "end": [101, 17], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:101`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb76bcf0a39c5d7778216f91"></a>
## selected_column_names_and_types

`function` · `buoyant_kernel::actions::visitors::AddVisitor::selected_column_names_and_types` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn selected_column_names_and_types(&self) -> (&'static [ColumnName], &'static [DataType])
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L159).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::visitors::AddVisitor", "path": "AddVisitor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [171, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs"}, "trait": {"args": null, "id": "buoyant_kernel::engine_data::RowVisitor", "path": "RowVisitor"}, "trait_path": "buoyant_kernel::engine_data::RowVisitor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:159`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-254d3ec24afcfdd6c8033197"></a>
## visit

`function` · `buoyant_kernel::actions::visitors::AddVisitor::visit` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn visit<'a>(&mut self, row_count: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L162).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::visitors::AddVisitor", "path": "AddVisitor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [171, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs"}, "trait": {"args": null, "id": "buoyant_kernel::engine_data::RowVisitor", "path": "RowVisitor"}, "trait_path": "buoyant_kernel::engine_data::RowVisitor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:162`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a03692c06817d44dac8ce38e"></a>
## visit_add

`function` · `buoyant_kernel::actions::visitors::AddVisitor::visit_add` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn visit_add<'a>(row_index: usize, path: String, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<Add>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L108).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::visitors::AddVisitor", "path": "AddVisitor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [156, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:108`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4af70003054b4feeb1786083"></a>
## adds

`struct_field` · `buoyant_kernel::actions::visitors::AddVisitor::adds` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
adds: Vec<Add>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L104).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:104`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
