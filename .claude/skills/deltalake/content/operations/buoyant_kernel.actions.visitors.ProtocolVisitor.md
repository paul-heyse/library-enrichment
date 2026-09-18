# `buoyant_kernel::actions::visitors::ProtocolVisitor`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.visitors.ProtocolVisitor.json).

<a id="op-42d5fe8dd457e6d291ee0937"></a>
## ProtocolVisitor

`struct` · `buoyant_kernel::actions::visitors::ProtocolVisitor` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ProtocolVisitor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L81).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:81`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-369b73253117bf2812c5a054"></a>
## default

`function` · `buoyant_kernel::actions::visitors::ProtocolVisitor::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> ProtocolVisitor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L79).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::visitors::ProtocolVisitor", "path": "ProtocolVisitor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 10], "end": [79, 17], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:79`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1616524261c101d44758ac44"></a>
## selected_column_names_and_types

`function` · `buoyant_kernel::actions::visitors::ProtocolVisitor::selected_column_names_and_types` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn selected_column_names_and_types(&self) -> (&'static [ColumnName], &'static [DataType])
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L86).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::visitors::ProtocolVisitor", "path": "ProtocolVisitor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [98, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs"}, "trait": {"args": null, "id": "buoyant_kernel::engine_data::RowVisitor", "path": "RowVisitor"}, "trait_path": "buoyant_kernel::engine_data::RowVisitor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:86`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d8c116072ec47e8e80d79f4"></a>
## visit

`function` · `buoyant_kernel::actions::visitors::ProtocolVisitor::visit` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn visit<'a>(&mut self, row_count: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L89).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::visitors::ProtocolVisitor", "path": "ProtocolVisitor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [98, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs"}, "trait": {"args": null, "id": "buoyant_kernel::engine_data::RowVisitor", "path": "RowVisitor"}, "trait_path": "buoyant_kernel::engine_data::RowVisitor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:89`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c987d8092ea564925c9ad40"></a>
## protocol

`struct_field` · `buoyant_kernel::actions::visitors::ProtocolVisitor::protocol` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
protocol: Option<Protocol>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/visitors.rs#L82).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/visitors.rs:82`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
