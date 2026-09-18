# `buoyant_kernel::Engine`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.Engine.json).

<a id="op-144f8dad57c79b7743fd1386"></a>
## Engine

`trait` · `buoyant_kernel::Engine` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait Engine: AsAny
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L977).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:977`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The `Engine` trait encapsulates all the functionality an engine or connector needs to provide
to the Delta Kernel in order to read the Delta table.

Engines/Connectors are expected to pass an implementation of this trait when reading a Delta
table.

<a id="op-1b1dc0134ee48baac011da16"></a>
## evaluation_handler

`function` · `buoyant_kernel::Engine::evaluation_handler` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn evaluation_handler(&self) -> Arc<dyn EvaluationHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L979).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:979`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the connector provided [`EvaluationHandler`](../operations/buoyant_kernel.EvaluationHandler.md#op-048603b8f393366d7f668758).

<a id="op-6a2a4fa6dadf41a504be9c88"></a>
## json_handler

`function` · `buoyant_kernel::Engine::json_handler` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn json_handler(&self) -> Arc<dyn JsonHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L985).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:985`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the connector provided [`JsonHandler`](../operations/buoyant_kernel.JsonHandler.md#op-8c2157a5cd619a78e70c9841).

<a id="op-c7ee15595fd89e53d2db6b42"></a>
## parquet_handler

`function` · `buoyant_kernel::Engine::parquet_handler` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parquet_handler(&self) -> Arc<dyn ParquetHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L988).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:988`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the connector provided [`ParquetHandler`](../operations/buoyant_kernel.ParquetHandler.md#op-935e1b04f902c9a95af7c376).

<a id="op-828942ecc20a9c9e6ab18ed5"></a>
## storage_handler

`function` · `buoyant_kernel::Engine::storage_handler` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn storage_handler(&self) -> Arc<dyn StorageHandler>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L982).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:982`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the connector provided [`StorageHandler`](../operations/buoyant_kernel.StorageHandler.md#op-925ea854f2a3b385f850512b)
