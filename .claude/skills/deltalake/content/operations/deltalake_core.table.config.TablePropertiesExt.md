# `deltalake_core::table::config::TablePropertiesExt`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.table.config.TablePropertiesExt.json).

<a id="op-ee85d32ca06e264dfa68e5d1"></a>
## TablePropertiesExt

`trait` · `deltalake_core::table::config::TablePropertiesExt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait TablePropertiesExt
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L208).

Source: `crates/core/src/table/config.rs:208`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Convenience accessors for reading well-known Delta table properties with their defaults
applied, layered on top of the raw [`TableProperties`](../operations/buoyant_kernel.table_properties.TableProperties.md#op-a6b333464916b5b04ef86dcd) parsed from table metadata.

<a id="op-6c8ffff7adfb46ac0425d195"></a>
## append_only

`function` · `deltalake_core::table::config::TablePropertiesExt::append_only` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn append_only(&self) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L213).

Source: `crates/core/src/table/config.rs:213`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

true for this Delta table to be append-only. If append-only, existing records cannot be
deleted, and existing values cannot be updated. See [append-only tables] in the protocol.

[append-only tables]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#append-only-tables

<a id="op-da06f5b14be811a1ba0c512a"></a>
## checkpoint_interval

`function` · `deltalake_core::table::config::TablePropertiesExt::checkpoint_interval` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn checkpoint_interval(&self) -> NonZero<u64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L229).

Source: `crates/core/src/table/config.rs:229`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Interval (expressed as number of commits) after which a new checkpoint should be created.
E.g. if checkpoint interval = 10, then a checkpoint should be written every 10 commits.

<a id="op-713edebd2ddb8d8fc49b6f18"></a>
## deleted_file_retention_duration

`function` · `deltalake_core::table::config::TablePropertiesExt::deleted_file_retention_duration` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deleted_file_retention_duration(&self) -> Duration
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L241).

Source: `crates/core/src/table/config.rs:241`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

How long removed data files are retained before they may be physically deleted by vacuum.

<a id="op-931d2394068d60e49033733e"></a>
## enable_change_data_feed

`function` · `deltalake_core::table::config::TablePropertiesExt::enable_change_data_feed` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn enable_change_data_feed(&self) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L238).

Source: `crates/core/src/table/config.rs:238`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether the Change Data Feed is enabled for this table.

<a id="op-bdd8d692887d1630ba8ab9b5"></a>
## enable_expired_log_cleanup

`function` · `deltalake_core::table::config::TablePropertiesExt::enable_expired_log_cleanup` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn enable_expired_log_cleanup(&self) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L225).

Source: `crates/core/src/table/config.rs:225`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether to clean up expired checkpoints/commits in the delta log.

<a id="op-1d8c4aa6826ddb42fcabd09d"></a>
## get_constraints

`function` · `deltalake_core::table::config::TablePropertiesExt::get_constraints` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_constraints(&self) -> Vec<Constraint>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L247).

Source: `crates/core/src/table/config.rs:247`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The list of constraints (e.g. CHECK constraints) declared on the table.

<a id="op-c92f2dddfe58e49a5cec0031"></a>
## isolation_level

`function` · `deltalake_core::table::config::TablePropertiesExt::isolation_level` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn isolation_level(&self) -> IsolationLevel
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L244).

Source: `crates/core/src/table/config.rs:244`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The isolation level used when checking for conflicts during commits.

<a id="op-fde6944f74e56fcdd00c73ba"></a>
## log_retention_duration

`function` · `deltalake_core::table::config::TablePropertiesExt::log_retention_duration` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn log_retention_duration(&self) -> Duration
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L222).

Source: `crates/core/src/table/config.rs:222`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

How long the history for a Delta table is kept.

Each time a checkpoint is written, Delta Lake automatically cleans up log entries older
than the retention interval. If you set this property to a large enough value, many log
entries are retained. This should not impact performance as operations against the log are
constant time. Operations on history are parallel but will become more expensive as the log
size increases.

<a id="op-03269167539e88344dd2e7fb"></a>
## num_indexed_cols

`function` · `deltalake_core::table::config::TablePropertiesExt::num_indexed_cols` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn num_indexed_cols(&self) -> DataSkippingNumIndexedCols
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L232).

Source: `crates/core/src/table/config.rs:232`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of columns to be indexed.

<a id="op-de0ae184c4186be53bac2935"></a>
## target_file_size

`function` · `deltalake_core::table::config::TablePropertiesExt::target_file_size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn target_file_size(&self) -> NonZero<u64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/config.rs#L235).

Source: `crates/core/src/table/config.rs:235`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Target size in bytes for data files produced by writes and compaction.
