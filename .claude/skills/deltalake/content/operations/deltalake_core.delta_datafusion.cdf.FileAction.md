# `deltalake_core::delta_datafusion::cdf::FileAction`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.cdf.FileAction.json).

<a id="op-31f4fdea049b467ca08f7323"></a>
## FileAction

`trait` · `deltalake_core::delta_datafusion::cdf::FileAction` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait FileAction
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L78).

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:78`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

This trait defines a generic set of operations used by CDF Reader

<a id="op-a63dec41378ba6a27d319a55"></a>
## deletion_vector

`function` · `deltalake_core::delta_datafusion::cdf::FileAction::deletion_vector` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deletion_vector(&self) -> Option<DeletionVectorDescriptor>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L86).

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:86`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Possibly provide the deletion vector for the action

<a id="op-d0deee72c9df786b3dd6c6ef"></a>
## has_deletion_vector

`function` · `deltalake_core::delta_datafusion::cdf::FileAction::has_deletion_vector` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn has_deletion_vector(&self) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L88).

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:88`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether this file action contains a deletion vector

<a id="op-cdadb9ff8155fa2fa1a633d4"></a>
## partition_values

`function` · `deltalake_core::delta_datafusion::cdf::FileAction::partition_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn partition_values(&self) -> DeltaResult<&HashMap<String, Option<String>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L80).

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:80`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Adds partition values

<a id="op-b0135f6426082fc73cf42e79"></a>
## path

`function` · `deltalake_core::delta_datafusion::cdf::FileAction::path` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn path(&self) -> String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L82).

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:82`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Physical Path to the data

<a id="op-a926699056502e0a5c607355"></a>
## size

`function` · `deltalake_core::delta_datafusion::cdf::FileAction::size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn size(&self) -> DeltaResult<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L84).

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:84`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Byte size of the physical file
