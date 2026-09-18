# `buoyant_kernel::table_configuration::ExpectedStatsSchemas`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_configuration.ExpectedStatsSchemas.json).

<a id="op-597c2f5e3c37ac7fd8adf8cb"></a>
## ExpectedStatsSchemas

`struct` · `buoyant_kernel::table_configuration::ExpectedStatsSchemas` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ExpectedStatsSchemas
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L48).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:48`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Expected schema for file statistics, using physical column names.

Wrapped in a struct so it can be extended with a logical-name variant if needed.

<a id="op-e34926565223e9ba5b3483ce"></a>
## clone

`function` · `buoyant_kernel::table_configuration::ExpectedStatsSchemas::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ExpectedStatsSchemas
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L46).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::ExpectedStatsSchemas", "path": "ExpectedStatsSchemas"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 17], "end": [46, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:46`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7ec81ee0fd2665971a1f01f"></a>
## fmt

`function` · `buoyant_kernel::table_configuration::ExpectedStatsSchemas::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L46).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_configuration::ExpectedStatsSchemas", "path": "ExpectedStatsSchemas"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 10], "end": [46, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:46`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92dbee4527527ad859caccde"></a>
## physical

`struct_field` · `buoyant_kernel::table_configuration::ExpectedStatsSchemas::physical` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
physical: schema::SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L50).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:50`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Stats schema using physical column names (for storage).
