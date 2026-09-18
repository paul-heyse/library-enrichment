# `deltalake_core::writer::WriteMode`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.writer.WriteMode.json).

<a id="op-1f477263f864cc45bd49fd0e"></a>
## WriteMode

`enum` · `deltalake_core::writer::WriteMode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum WriteMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/mod.rs#L164).

Source: `crates/core/src/writer/mod.rs:164`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Write mode for the [DeltaWriter](../operations/deltalake_core.writer.DeltaWriter.md#op-664f2195f9de70fb9633c7ad)

<a id="op-1a581445001e8bb26ea6d7fa"></a>
## Default

`variant` · `deltalake_core::writer::WriteMode::Default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Default
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/mod.rs#L166).

Source: `crates/core/src/writer/mod.rs:166`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Default write mode which will return an error if schemas do not match correctly

<a id="op-e42b4c72e462a41d8a8b74f6"></a>
## MergeSchema

`variant` · `deltalake_core::writer::WriteMode::MergeSchema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MergeSchema
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/mod.rs#L170).

Source: `crates/core/src/writer/mod.rs:170`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Merge the schema of the table with the newly written data

[Read more here](https://delta.io/blog/2023-02-08-delta-lake-schema-evolution/)

<a id="op-617d1c32984bee98cbc01dc6"></a>
## clone

`function` · `deltalake_core::writer::WriteMode::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> WriteMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/mod.rs#L163).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::WriteMode", "path": "WriteMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 16], "end": [163, 21], "filename": "crates/core/src/writer/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/writer/mod.rs:163`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68a8f52c75de4844dcc36c0a"></a>
## eq

`function` · `deltalake_core::writer::WriteMode::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &WriteMode) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/mod.rs#L163).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::WriteMode", "path": "WriteMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 30], "end": [163, 39], "filename": "crates/core/src/writer/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/writer/mod.rs:163`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3187e6b26698389d91c8adc6"></a>
## fmt

`function` · `deltalake_core::writer::WriteMode::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/mod.rs#L163).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::writer::WriteMode", "path": "WriteMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 23], "end": [163, 28], "filename": "crates/core/src/writer/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/writer/mod.rs:163`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
