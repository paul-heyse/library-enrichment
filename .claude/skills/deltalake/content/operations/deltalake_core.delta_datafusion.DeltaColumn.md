# `deltalake_core::delta_datafusion::DeltaColumn`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.DeltaColumn.json).

<a id="op-f08a9546776688c57cff4318"></a>
## DeltaColumn

`struct` · `deltalake_core::delta_datafusion::DeltaColumn` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaColumn
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L580).

Source: `crates/core/src/delta_datafusion/mod.rs:580`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A wrapper for Deltafusion's Column to preserve case-sensitivity during string conversion

<a id="op-0ce3a55485a3197a8dec836b"></a>
## from

`function` · `deltalake_core::delta_datafusion::DeltaColumn::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(c: String) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L603).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::DeltaColumn", "path": "DeltaColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [602, 1], "end": [608, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/delta_datafusion/mod.rs:603`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6dbd8a80330a940bb5b14f05"></a>
## from

`function` · `deltalake_core::delta_datafusion::DeltaColumn::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(c: Column) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L618).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::DeltaColumn", "path": "DeltaColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [617, 1], "end": [621, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::column::Column", "path": "Column"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/delta_datafusion/mod.rs:618`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b348df31cefcd02b528c589e"></a>
## from

`function` · `deltalake_core::delta_datafusion::DeltaColumn::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(c: &str) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L585).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::DeltaColumn", "path": "DeltaColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [584, 1], "end": [590, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/delta_datafusion/mod.rs:585`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7335f377a04f885898a9068"></a>
## from

`function` · `deltalake_core::delta_datafusion::DeltaColumn::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(c: &String) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L594).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::DeltaColumn", "path": "DeltaColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [593, 1], "end": [599, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/delta_datafusion/mod.rs:594`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13e69369fb4bac8397d0e6d0"></a>
## inner

`struct_field` · `deltalake_core::delta_datafusion::DeltaColumn::inner` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: datafusion::common::Column
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L581).

Source: `crates/core/src/delta_datafusion/mod.rs:581`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
