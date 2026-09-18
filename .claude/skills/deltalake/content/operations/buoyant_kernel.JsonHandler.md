# `buoyant_kernel::JsonHandler`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.JsonHandler.json).

<a id="op-8c2157a5cd619a78e70c9841"></a>
## JsonHandler

`trait` · `buoyant_kernel::JsonHandler` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait JsonHandler: AsAny
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L649).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:649`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Provides JSON handling functionality to Delta Kernel.

Delta Kernel can use this handler to parse JSON strings into Row or read content from JSON
files. Connectors can leverage this trait to provide their best implementation of the JSON
parsing capability to Delta Kernel.

<a id="op-b94fc78e8c763c8e8051a9ed"></a>
## parse_json

`function` · `buoyant_kernel::JsonHandler::parse_json` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_json(&self, json_strings: Box<dyn EngineData>, output_schema: SchemaRef) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L653).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:653`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Parse the given json strings and return the fields requested by output schema as columns in
[`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809). json_strings MUST be a single column batch of engine data, and the
column type must be string

<a id="op-fad9d2a21cfba1f126da6c44"></a>
## read_json_files

`function` · `buoyant_kernel::JsonHandler::read_json_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_json_files(&self, files: &[FileMeta], physical_schema: SchemaRef, predicate: Option<PredicateRef>) -> DeltaResult<FileDataReadResultIterator>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L681).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:681`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Read and parse the JSON format file at given locations and return the data as EngineData
with the columns requested by physical schema. Note: The [`FileDataReadResultIterator`](../operations/buoyant_kernel.FileDataReadResultIterator.md#op-2c38562abfe66da38e1b8f0e)
must emit data from files in the order that `files` is given. For example if files ["a",
"b"] is provided, then the engine data iterator must first return all the engine data
from file "a", _then_ all the engine data from file "b". Moreover, for a given file, all
of its [`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809) and constituent rows must be in order that they occur in the file.
Consider a file with rows (1, 2, 3). The following are legal iterator batches:
   iter: [EngineData(1, 2), EngineData(3)]
   iter: [EngineData(1), EngineData(2, 3)]
   iter: [EngineData(1, 2, 3)]
The following are illegal batches:
   iter: [EngineData(3), EngineData(1, 2)]
   iter: [EngineData(1), EngineData(3, 2)]
   iter: [EngineData(2, 1, 3)]

Additionally, engines may not merge engine data across file boundaries.

# Parameters

- `files` - File metadata for files to be read.
- `physical_schema` - Select list of columns to read from the JSON file.
- `predicate` - Optional push-down predicate hint (engine is free to ignore it).

<a id="op-57a7ab858fc6155af89e30e9"></a>
## write_json_file

`function` · `buoyant_kernel::JsonHandler::write_json_file` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn write_json_file(&self, path: &Url, data: DeltaResultIterator<'_, FilteredEngineData>, overwrite: bool) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L712).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:712`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Atomically (!) write a single JSON file. Each selected row of the input data must be
written as a new JSON object appended to the file; rows not selected by a batch's
selection vector (see [`FilteredEngineData`](../operations/buoyant_kernel.engine_data.FilteredEngineData.md#op-62b3837332f86d6b0fa41aab)) must not be written.
[`FilteredEngineData::apply_selection_vector`](../operations/buoyant_kernel.engine_data.FilteredEngineData.md#op-c5e31214bf7fcdf905d25151) produces the selected-rows view for
implementations that do not filter during serialization. This write must:
(1) serialize the selected rows to newline-delimited json (each row is a json object
    literal)
(2) write the data to storage atomically (i.e. if the file already exists, fail unless the
    overwrite flag is set)

For example, the JSON data should be written as { "column1": "val1", "column2": "val2", .. }
with each row on a new line.

NOTE: Null columns should not be written to the JSON file. For example, if a row has columns
["a", "b"] and the value of "b" is null, the JSON object should be written as
{ "a": "..." }. Note that including nulls is technically valid JSON, but would bloat the
log, therefore we recommend omitting them.

# Parameters

- `path` - URL specifying the location to write the JSON file
- `data` - Iterator of [`FilteredEngineData`](../operations/buoyant_kernel.engine_data.FilteredEngineData.md#op-62b3837332f86d6b0fa41aab) to write to the JSON file
- `overwrite` - If true, overwrite the file if it exists. If false, the call must fail if
  the file exists.
