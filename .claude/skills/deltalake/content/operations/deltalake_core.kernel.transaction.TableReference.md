# `deltalake_core::kernel::transaction::TableReference`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.TableReference.json).

<a id="op-3b8917d5d28388c565c309fb"></a>
## TableReference

`trait` · `deltalake_core::kernel::transaction::TableReference` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait TableReference: Send + Sync
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L251).

Source: `crates/core/src/kernel/transaction/mod.rs:251`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Reference to some structure that contains mandatory attributes for performing a commit.

<a id="op-6cb44517196f074e04c3d2b2"></a>
## config

`function` · `deltalake_core::kernel::transaction::TableReference::config` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn config(&self) -> &TableProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L253).

Source: `crates/core/src/kernel/transaction/mod.rs:253`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Well known table configuration

<a id="op-57724528dbd90ed69fb429b3"></a>
## eager_snapshot

`function` · `deltalake_core::kernel::transaction::TableReference::eager_snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eager_snapshot(&self) -> &EagerSnapshot
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L262).

Source: `crates/core/src/kernel/transaction/mod.rs:262`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Try to cast this table reference to a `EagerSnapshot`

<a id="op-c80012b333dfa220c6fe25c3"></a>
## metadata

`function` · `deltalake_core::kernel::transaction::TableReference::metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn metadata(&self) -> &Metadata
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L259).

Source: `crates/core/src/kernel/transaction/mod.rs:259`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table metadata of the snapshot

<a id="op-68ffe85f78990df4cf7712da"></a>
## protocol

`function` · `deltalake_core::kernel::transaction::TableReference::protocol` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn protocol(&self) -> &Protocol
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L256).

Source: `crates/core/src/kernel/transaction/mod.rs:256`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table protocol of the snapshot
