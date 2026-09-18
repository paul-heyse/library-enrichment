# `deltalake_core::errors::ColumnMappingOperation`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.ColumnMappingOperation.json).

<a id="op-c76922bfd6c0fd7163481fdb"></a>
## ColumnMappingOperation

`enum` · `deltalake_core::errors::ColumnMappingOperation` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum ColumnMappingOperation
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L13).

Source: `crates/core/src/errors.rs:13`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether an unsupported column-mapping access was a read or a write.

<a id="op-a2d5e2833fb432ce4887bf7e"></a>
## Read

`variant` · `deltalake_core::errors::ColumnMappingOperation::Read` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Read
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L15).

Source: `crates/core/src/errors.rs:15`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A read-path operation.

<a id="op-61dcd48d174060b8bea68860"></a>
## Write

`variant` · `deltalake_core::errors::ColumnMappingOperation::Write` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Write
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L17).

Source: `crates/core/src/errors.rs:17`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A write-path operation.

<a id="op-22b25837ba9795773fa6ab26"></a>
## clone

`function` · `deltalake_core::errors::ColumnMappingOperation::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ColumnMappingOperation
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L12).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::ColumnMappingOperation", "path": "ColumnMappingOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12, 17], "end": [12, 22], "filename": "crates/core/src/errors.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/errors.rs:12`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ea36a0b32006f03a820a1c7"></a>
## eq

`function` · `deltalake_core::errors::ColumnMappingOperation::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &ColumnMappingOperation) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L12).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::ColumnMappingOperation", "path": "ColumnMappingOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12, 30], "end": [12, 39], "filename": "crates/core/src/errors.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/errors.rs:12`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-580797d0fe70ce33b46bc72d"></a>
## fmt

`function` · `deltalake_core::errors::ColumnMappingOperation::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L12).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::ColumnMappingOperation", "path": "ColumnMappingOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12, 10], "end": [12, 15], "filename": "crates/core/src/errors.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/errors.rs:12`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60b44bcd790b5d12265144d0"></a>
## fmt

`function` · `deltalake_core::errors::ColumnMappingOperation::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L21).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::errors::ColumnMappingOperation", "path": "ColumnMappingOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 1], "end": [27, 2], "filename": "crates/core/src/errors.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/core/src/errors.rs:21`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
