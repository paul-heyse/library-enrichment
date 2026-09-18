# `deltalake_core::operations::vacuum::VacuumMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.vacuum.VacuumMetrics.json).

<a id="op-e011b67e93384ae825c28089"></a>
## VacuumMetrics

`struct` · `deltalake_core::operations::vacuum::VacuumMetrics` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct VacuumMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L227).

Source: `crates/core/src/operations/vacuum.rs:227`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Details for the Vacuum operation including which files were

<a id="op-f701575e05cb3167092573d4"></a>
## default

`function` · `deltalake_core::operations::vacuum::VacuumMetrics::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> VacuumMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L226).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumMetrics", "path": "VacuumMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [226, 17], "end": [226, 24], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/operations/vacuum.rs:226`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb6a742e2c4de3559e5c59f2"></a>
## dry_run

`struct_field` · `deltalake_core::operations::vacuum::VacuumMetrics::dry_run` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
dry_run: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L229).

Source: `crates/core/src/operations/vacuum.rs:229`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Was this a dry run

<a id="op-f7887cc8361476018974f21b"></a>
## files_deleted

`struct_field` · `deltalake_core::operations::vacuum::VacuumMetrics::files_deleted` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
files_deleted: Vec<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L231).

Source: `crates/core/src/operations/vacuum.rs:231`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Files deleted successfully

<a id="op-9c44ead7a09564d8135d1817"></a>
## fmt

`function` · `deltalake_core::operations::vacuum::VacuumMetrics::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L226).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumMetrics", "path": "VacuumMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [226, 10], "end": [226, 15], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/vacuum.rs:226`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
