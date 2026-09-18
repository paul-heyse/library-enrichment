# `buoyant_kernel::actions::visitors::CdcVisitor`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.visitors.CdcVisitor.json).

<a id="op-52d0d25049fa706a8fa0504b"></a>
## CdcVisitor

`struct` · `buoyant_kernel::actions::visitors::CdcVisitor` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CdcVisitor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L252).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:252`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-166766ac35730ea678266f18"></a>
## default

`function` · `buoyant_kernel::actions::visitors::CdcVisitor::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> CdcVisitor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L250).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::visitors::CdcVisitor", "path": "CdcVisitor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 10], "end": [250, 17], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:250`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f409540a8a4860de8aa29210"></a>
## selected_column_names_and_types

`function` · `buoyant_kernel::actions::visitors::CdcVisitor::selected_column_names_and_types` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn selected_column_names_and_types(&self) -> (&'static [ColumnName], &'static [DataType])
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L274).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::visitors::CdcVisitor", "path": "CdcVisitor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [295, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs"}, "trait": {"args": null, "id": "buoyant_kernel::engine_data::RowVisitor", "path": "RowVisitor"}, "trait_path": "buoyant_kernel::engine_data::RowVisitor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:274`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06ace7f274a758651d4c24b8"></a>
## visit

`function` · `buoyant_kernel::actions::visitors::CdcVisitor::visit` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn visit<'a>(&mut self, row_count: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L279).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::visitors::CdcVisitor", "path": "CdcVisitor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [295, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs"}, "trait": {"args": null, "id": "buoyant_kernel::engine_data::RowVisitor", "path": "RowVisitor"}, "trait_path": "buoyant_kernel::engine_data::RowVisitor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:279`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-072811ccf730ae1ed001aa00"></a>
## visit_cdc

`function` · `buoyant_kernel::actions::visitors::CdcVisitor::visit_cdc` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn visit_cdc<'a>(row_index: usize, path: String, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<Cdc>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L258).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::visitors::CdcVisitor", "path": "CdcVisitor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [271, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:258`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2bb7ea687afc2789989cabf"></a>
## cdcs

`struct_field` · `buoyant_kernel::actions::visitors::CdcVisitor::cdcs` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
cdcs: Vec<Cdc>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L253).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:253`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
