# `buoyant_kernel`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.json).

<a id="op-5fa674cbd0f805afb273382e"></a>
## buoyant_kernel

`module` · `buoyant_kernel` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod buoyant_kernel
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

# Delta Kernel

Delta-kernel-rs is an experimental [Delta](https://github.com/delta-io/delta/) implementation
focused on interoperability with a wide range of query engines. It supports reads and
(experimental) writes (only blind appends in the write path currently). This library defines a
number of traits which must be implemented to provide a working delta implementation. They are
detailed below. There is a provided "default engine" that implements all these traits and can
be used to ease integration work. See [`DefaultEngine`](engine/default/index.html) for more
information.

A full `rust` example for reading table data using the default engine can be found in the
[read-table-single-threaded] example (and for a more complex multi-threaded reader see the
[read-table-multi-threaded] example). An example for reading the table changes for a table
using the default engine can be found in the [read-table-changes] example. The [write-table]
example demonstrates how to write data to a Delta table using the default engine.

[read-table-single-threaded]:
https://github.com/delta-io/delta-kernel-rs/tree/main/kernel/examples/read-table-single-threaded
[read-table-multi-threaded]:
https://github.com/delta-io/delta-kernel-rs/tree/main/kernel/examples/read-table-multi-threaded
[read-table-changes]:
https://github.com/delta-io/delta-kernel-rs/tree/main/kernel/examples/read-table-changes
[write-table]:
https://github.com/delta-io/delta-kernel-rs/tree/main/kernel/examples/write-table

# Engine trait

The [`Engine`](../operations/buoyant_kernel.Engine.md#op-144f8dad57c79b7743fd1386) trait allows connectors to bring their own implementation of functionality such
as reading parquet files, listing files in a file system, parsing a JSON string etc. This
trait exposes methods to get sub-engines which expose the core functionalities customizable by
connectors.

## Expression handling

Expression handling is done via the [`EvaluationHandler`](../operations/buoyant_kernel.EvaluationHandler.md#op-048603b8f393366d7f668758), which in turn allows the creation of
[`ExpressionEvaluator`](../operations/buoyant_kernel.ExpressionEvaluator.md#op-02eea996b6bdb19503c4315a)s. These evaluators are created for a specific predicate [`Expression`](../operations/buoyant_kernel.expressions.Expression.md#op-b6e8a7405239f2ae57fc5e51)
and allow evaluation of that predicate for a specific batch of data.

## File system interactions

Delta Kernel needs to perform some basic operations against file systems like listing and
reading files. These interactions are encapsulated in the [`StorageHandler`](../operations/buoyant_kernel.StorageHandler.md#op-925ea854f2a3b385f850512b) trait.
Implementers must take care that all assumptions on the behavior of the functions - like sorted
results - are respected.

## Reading log and data files

Delta Kernel requires the capability to read and write json files and read parquet files, which
is exposed via the [`JsonHandler`](../operations/buoyant_kernel.JsonHandler.md#op-8c2157a5cd619a78e70c9841) and [`ParquetHandler`](../operations/buoyant_kernel.ParquetHandler.md#op-935e1b04f902c9a95af7c376) respectively. When reading files,
connectors are asked to provide the context information they require to execute the actual
operation. This is done by invoking methods on the [`StorageHandler`](../operations/buoyant_kernel.StorageHandler.md#op-925ea854f2a3b385f850512b) trait.
