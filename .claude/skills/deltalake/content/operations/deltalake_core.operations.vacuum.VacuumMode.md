# `deltalake_core::operations::vacuum::VacuumMode`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.vacuum.VacuumMode.json).

<a id="op-48c47efc7a5894e2e3ebdabf"></a>
## VacuumMode

`enum` · `deltalake_core::operations::vacuum::VacuumMode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum VacuumMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L172).

Source: `crates/core/src/operations/vacuum.rs:172`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Type of Vacuum operation to perform

<a id="op-e34ad220935972b20704baa2"></a>
## Full

`variant` · `deltalake_core::operations::vacuum::VacuumMode::Full` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Full
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L181).

Source: `crates/core/src/operations/vacuum.rs:181`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A `full` mode vacuum will remove _all_ data files no longer actively referenced in the
`_delta_log` table. For example, if parquet files exist in the table directory but are no
longer mentioned as `add` actions in the transaction log, then this mode will scan storage
and remove those files.

<a id="op-35d86e540c2208b39c65db88"></a>
## Lite

`variant` · `deltalake_core::operations::vacuum::VacuumMode::Lite` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Lite
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L176).

Source: `crates/core/src/operations/vacuum.rs:176`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The `lite` mode will only remove files which are referenced in the `_delta_log` associated
with `remove` action

<a id="op-2cb4bb75e0ed4856e596af24"></a>
## clone

`function` · `deltalake_core::operations::vacuum::VacuumMode::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> VacuumMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L171).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumMode", "path": "VacuumMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 26], "end": [171, 31], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/operations/vacuum.rs:171`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c02fb4ac6540a0b9f809d589"></a>
## default

`function` · `deltalake_core::operations::vacuum::VacuumMode::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> VacuumMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L171).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumMode", "path": "VacuumMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 17], "end": [171, 24], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/operations/vacuum.rs:171`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc71baab3c0db4ac03208b99"></a>
## eq

`function` · `deltalake_core::operations::vacuum::VacuumMode::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &VacuumMode) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L171).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumMode", "path": "VacuumMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 33], "end": [171, 42], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/operations/vacuum.rs:171`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e21535b2481a02e6a386eb4"></a>
## fmt

`function` · `deltalake_core::operations::vacuum::VacuumMode::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L171).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::vacuum::VacuumMode", "path": "VacuumMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 10], "end": [171, 15], "filename": "crates/core/src/operations/vacuum.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/vacuum.rs:171`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
